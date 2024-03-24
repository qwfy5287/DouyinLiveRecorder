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

function jsonToSrt(json) {
  const srt = [];
  let index = 1;

  // 预处理 tracks 中的时间单位
  json.tracks.forEach((track) => {
    if (track.type === "text") {
      track.segments.forEach((segment) => {
        segment.target_timerange.start = convertTimeToMilliseconds(
          segment.target_timerange.start,
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
          (text) => text.id === segment.material_id,
        );

        // 构建字幕内容
        const subtitleLines = [];
        for (let i = 0; i < material.words.text.length; i++) {
          const word = material.words.text[i];
          const startTime =
            material.words.start_time[i] + segment.target_timerange.start;
          const endTime =
            material.words.end_time[i] + segment.target_timerange.start;

          subtitleLines.push({
            text: word,
            startTime: startTime,
            endTime: endTime,
          });
        }

        // 将字幕内容按起始时间排序
        subtitleLines.sort((a, b) => a.startTime - b.startTime);

        // 合并相邻的字幕内容
        const mergedSubtitleLines = [];
        let currentLine = subtitleLines[0];
        for (let i = 1; i < subtitleLines.length; i++) {
          if (subtitleLines[i].startTime - currentLine.endTime <= 200) {
            currentLine.text += subtitleLines[i].text;
            currentLine.endTime = subtitleLines[i].endTime;
          } else {
            mergedSubtitleLines.push(currentLine);
            currentLine = subtitleLines[i];
          }
        }
        mergedSubtitleLines.push(currentLine);

        // 将合并后的字幕内容转换为 SRT 格式
        mergedSubtitleLines.forEach((line) => {
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

  return `${formatNumber(hours)}:${formatNumber(minutes)}:${formatNumber(seconds)},${formatNumber(milliseconds, 3)}`;
}

// 格式化数字,补零
function formatNumber(number, minimumIntegerDigits = 2) {
  return number.toString().padStart(minimumIntegerDigits, "0");
}

// 读取 JSON 文件
const jsonData = readJsonFromFile("./draft_info_example.json");

if (jsonData) {
  const srtData = jsonToSrt(jsonData);
  console.log(srtData);
}