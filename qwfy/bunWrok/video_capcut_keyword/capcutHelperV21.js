const fs = require("fs");

// 从文件中读取 JSON 数据
function readJsonFromFile(filePath) {
  try {
    const jsonString = fs.readFileSync(filePath, "utf8");
    return JSON.parse(jsonString);
  } catch (error) {
    console.error("读取 JSON 文件时出错:", error);
    return null;
  }
}

// 将时间从微秒转换为毫秒
function convertTimeToMilliseconds(time) {
  return Math.floor(time / 1000);
}

function jsonToSrt(json, keywordList, timeThreshold = 200) {
  const srt = [];
  let index = 1;

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

          if (wordEndTime - currentLine.startTime <= timeThreshold) {
            currentLine.text += word;
            currentLine.endTime = wordEndTime;
          } else {
            subtitleLines.push(currentLine);
            currentLine = {
              text: word,
              startTime:
                wordEndTime -
                (material.words.end_time[i] - material.words.end_time[i - 1]),
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

        // 将分割后的字幕内容转换为 SRT 格式
        splitSubtitleLines.forEach((line) => {
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

// 格式化时间为 SRT 格式
function formatTime(time) {
  const totalSeconds = Math.floor(time / 1000);
  const milliseconds = time % 1000;
  const hours = Math.floor(totalSeconds / 3600);
  const minutes = Math.floor((totalSeconds % 3600) / 60);
  const seconds = totalSeconds % 60;

  return `${formatNumber(hours)}:${formatNumber(minutes)}:${formatNumber(
    seconds
  )},${formatNumber(milliseconds, 3)}`;
}

// 格式化数字,补零
function formatNumber(number, minimumIntegerDigits = 2) {
  return number.toString().padStart(minimumIntegerDigits, "0");
}

// 读取 JSON 文件
const jsonData = readJsonFromFile("./draft_info_example.json");

if (jsonData) {
  const keywordList = ["买的", "黑色的"];
  const srtData = jsonToSrt(jsonData, keywordList, 500);
  console.log(srtData);
}
