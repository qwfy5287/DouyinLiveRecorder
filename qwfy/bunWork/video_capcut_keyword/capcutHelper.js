import {
  doFlow,
  filterLinesByKeywords,
  formatTime,
  jsonToSrtString,
  mergeSubtitleLines,
  srtStringToJson,
} from "./capcut.common";
import {
  writeSrtFile,
  readJsonFile,
  convertTimeToMilliseconds,
} from "../video_capcut/common/video.common";

function dartJsonToSrt(json, keywordList) {
  const srt = [];
  let index = 1;

  if (json.tracks.filter((track) => track.type === "text").length === 0) {
    console.log("JSON 中没有文本轨道");
    return "";
  }

  // 预处理 tracks 中的时间单位
  json.tracks.forEach((track) => {
    if (track.type === "text") {
      track.segments.forEach((segment) => {
        segment.target_timerange.start = convertTimeToMilliseconds(
          segment.target_timerange.start
        );
      });
    }
  });

  // 遍历每个文本轨道
  json.tracks.forEach((track) => {
    if (track.type === "text") {
      // 遍历轨道中的每个片段
      track.segments.forEach((segment) => {
        const material = json.materials.texts.find(
          (text) => text.id === segment.material_id
        );

        // 构建字幕内容
        const subtitleLines = [];
        let currentLine = {
          text: "",
          startTime: segment.target_timerange.start,
          endTime: segment.target_timerange.start,
        };
        for (let i = 0; i < material.words.text.length; i++) {
          const word = material.words.text[i];
          const wordEndTime =
            material.words.end_time[i] + segment.target_timerange.start;

          if (wordEndTime - currentLine.endTime <= 200) {
            currentLine.text += word;
            currentLine.endTime = wordEndTime;
          } else {
            subtitleLines.push(currentLine);
            currentLine = {
              text: word,
              startTime: currentLine.endTime,
              endTime: wordEndTime,
            };
          }
        }
        subtitleLines.push(currentLine);

        // 根据关键词列表分割字幕内容
        const splitSubtitleLines = [];
        subtitleLines.forEach((line) => {
          let startIndex = 0;
          keywordList.forEach((keyword) => {
            const keywordIndex = line.text.indexOf(keyword, startIndex);
            if (keywordIndex !== -1) {
              if (keywordIndex > startIndex) {
                splitSubtitleLines.push({
                  text: line.text.slice(startIndex, keywordIndex),
                  startTime: line.startTime,
                  endTime:
                    line.startTime +
                    Math.floor(
                      (line.endTime - line.startTime) *
                        ((keywordIndex - startIndex) / line.text.length)
                    ),
                });
              }
              splitSubtitleLines.push({
                text: keyword,
                startTime:
                  line.startTime +
                  Math.floor(
                    (line.endTime - line.startTime) *
                      (keywordIndex / line.text.length)
                  ),
                endTime:
                  line.startTime +
                  Math.floor(
                    (line.endTime - line.startTime) *
                      ((keywordIndex + keyword.length) / line.text.length)
                  ),
              });
              startIndex = keywordIndex + keyword.length;
            }
          });
          if (startIndex < line.text.length) {
            splitSubtitleLines.push({
              text: line.text.slice(startIndex),
              startTime:
                line.startTime +
                Math.floor(
                  (line.endTime - line.startTime) *
                    (startIndex / line.text.length)
                ),
              endTime: line.endTime,
            });
          }
        });

        // console.log(splitSubtitleLines);

        // 过滤掉分割后的关键词
        let filterLines = filterLinesByKeywords(
          splitSubtitleLines,
          keywordList
        );

        // 合并分割后的相邻的关键词
        let mergedLines = mergeSubtitleLines(filterLines);
        // console.log(mergedLines);

        // 将分割后的字幕内容转换为 SRT 格式
        mergedLines.forEach((line) => {
          const startTime = formatTime(line.startTime);
          const endTime = formatTime(line.endTime);

          srt.push(index);
          srt.push(`${startTime} --> ${endTime}`);
          srt.push(line.text);
          srt.push("");

          index++;
        });
      });
    }
  });

  return srt.join("\n");
}

export function doDartJsonToSrt(sourceJsonPath) {
  if (!sourceJsonPath) {
    sourceJsonPath =
      "/Users/qwfy/Movies/JianyingPro/User Data/Projects/com.lveditor.draft/3月27日-cut-small/draft_info.json";
  }

  // 读取 JSON 文件
  const jsonData = readJsonFile(sourceJsonPath);

  if (jsonData) {
    // const keywordList = ["买的", "黑色的", "好不好", "然后呢", "呃", "对"];
    const keywordList = ["好不", "好不好", "然后呢", "呃", "对"];
    const srtData = dartJsonToSrt(jsonData, keywordList);
    if (!srtData) {
      console.log("没有字幕");
      return;
    }

    // subTitle 精细化处理
    let subtitleJson = doFlow(srtData);

    // console.log(subtitleJson.length);
    // console.log(subtitleJson);

    // let pickArr = [
    //   51, 55, 59, 61, 62, 63, 64, 72, 73, 74, 82, 83, 84, 126, 130, 135, 136, 137,
    //   162, 163, 191, 192,
    // ];

    // let pickJson = subtitleJson.filter((item) => {
    //   return pickArr.includes(item.index);
    // });

    let srt = jsonToSrtString(subtitleJson);

    // console.log("🚀 ~ srt:");
    // console.log(srt);

    writeSrtFile("output_srt.srt", srt);

    return srt;

    // console.log(json);

    // const srtLines = srtData.trim().split("\\n");
    // console.log(srtLines[0]);
    // const jsonString = convertSrtToJson(srtLines);
    // console.log(jsonString);

    // console.log(srtData);
  }
}

function main() {
  doDartJsonToSrt();
}

main();
