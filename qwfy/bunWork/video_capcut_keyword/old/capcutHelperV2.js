// qwfy/bunWrok/video_capcut_keyword/capcutHelper.js

import { readFileSync } from "bun:fs";

function convertToSRT(draftInfo) {
  const srtLines = [];

  // 获取文本轨道
  const textTrack = draftInfo.tracks.find((track) => track.type === "text");

  // 处理每个片段
  textTrack.segments.forEach((segment, index) => {
    const material = draftInfo.materials.texts.find(
      (text) => text.id === segment.material_id,
    );

    // 添加字幕序号
    srtLines.push(index + 1);

    // 添加字幕起始时间和结束时间
    const startTime = formatTime(segment.target_timerange.start);
    const endTime = formatTime(
      segment.target_timerange.start + segment.target_timerange.duration,
    );
    srtLines.push(`${startTime} --> ${endTime}`);

    // 添加字幕内容
    const content = JSON.parse(material.content).text;
    srtLines.push(content);

    // 添加空行作为分隔符
    srtLines.push("");
  });

  // 返回 SRT 字符串
  return srtLines.join("\n");
}

// 格式化时间为 SRT 格式
function formatTime(timeInMicroseconds) {
  const timeInSeconds = timeInMicroseconds / 1000000;
  const hours = Math.floor(timeInSeconds / 3600);
  const minutes = Math.floor((timeInSeconds % 3600) / 60);
  const seconds = Math.floor(timeInSeconds % 60);
  const milliseconds = Math.floor((timeInSeconds % 1) * 1000);

  return (
    padZero(hours, 2) +
    ":" +
    padZero(minutes, 2) +
    ":" +
    padZero(seconds, 2) +
    "," +
    padZero(milliseconds, 3)
  );
}

// 补零函数,用于格式化时间
function padZero(num, length) {
  return num.toString().padStart(length, "0");
}

// 读取 draft_info_example.json 文件
const draftInfoJson = readFileSync("draft_info_example.json", "utf8");
const draftInfo = JSON.parse(draftInfoJson);

// 转换为 SRT 格式
const srtContent = convertToSRT(draftInfo);

// 输出 SRT 内容
console.log(srtContent);
