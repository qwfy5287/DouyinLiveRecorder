// video_capcut/common/video.common.js

const fs = require("fs");

// // 从文件中读取 JSON 数据
// export function readJsonFromFile(filePath) {
//   try {
//     const jsonString = fs.readFileSync(filePath, "utf8");
//     return JSON.parse(jsonString);
//   } catch (error) {
//     console.error("读取 JSON 文件时出错:", error);
//     return null;
//   }
// }

// 读取 JSON 文件
export function readJsonFile(filePath) {
  try {
    const data = fs.readFileSync(filePath, "utf8");
    return JSON.parse(data);
  } catch (error) {
    console.error("Error reading JSON file:", error);
    return null;
  }
}

// 写入 JSON 文件
export function writeJsonFile(filePath, data) {
  try {
    const jsonData = JSON.stringify(data, null, 2);
    fs.writeFileSync(filePath, jsonData, "utf8");
    console.log("JSON file updated successfully.");
  } catch (error) {
    console.error("Error writing JSON file:", error);
  }
}

/**
 * 生成 UUID
 * @returns {string} UUID
 */
export function generateId() {
  // 生成 8-4-4-4-12 格式的 UUID
  const parts = [];
  for (let i = 0; i < 32; i++) {
    const random = (Math.random() * 16) | 0;
    if (i === 8 || i === 12 || i === 16 || i === 20) {
      parts.push("-");
    }
    parts.push(
      (i === 12 ? 4 : i === 16 ? (random & 3) | 8 : random).toString(16)
    );
  }
  return parts.join("").toUpperCase();
}

export function getCanvasesItem(uuid) {
  return {
    album_image: "",
    blur: 0,
    color: "",
    id: uuid,
    image: "",
    image_id: "",
    image_name: "",
    source_platform: 0,
    team_id: "",
    type: "canvas_color",
  };
}

export function deepClone(obj) {
  if (typeof obj !== "object" || obj === null) {
    return obj;
  }

  let clone;

  if (Array.isArray(obj)) {
    clone = [];
    for (let i = 0; i < obj.length; i++) {
      clone[i] = deepClone(obj[i]);
    }
  } else {
    clone = {};
    for (let key in obj) {
      if (Object.prototype.hasOwnProperty.call(obj, key)) {
        clone[key] = deepClone(obj[key]);
      }
    }
  }

  return clone;
}

/**
 * 将字幕 时间字符串 转换为 毫秒数
 *  00:00:01,466 -> 1466
 * @param {*} timeString
 * @returns
 */
export function convertSubtitleTimeToNumber(timeString) {
  // 将时间字符串按照冒号和逗号分割成时、分、秒和毫秒
  const parts = timeString.split(/[:,]/);

  // 提取时、分、秒和毫秒
  const hours = parseInt(parts[0], 10);
  const minutes = parseInt(parts[1], 10);
  const seconds = parseInt(parts[2], 10);
  const milliseconds = parseInt(parts[3], 10);

  // 计算总毫秒数
  const totalMilliseconds =
    (hours * 3600 + minutes * 60 + seconds) * 1000 + milliseconds;

  return totalMilliseconds;
}

// // 示例用法
// const timeString = "00:00:01,466";
// const totalMilliseconds = convertSubtitleTimeToNumber(timeString);
// console.log(totalMilliseconds);  // 输出 1466

/**
 * 将字幕时间字符串转换为微秒数
 * 00:00:01,466.789 -> 1466789
 * 00:00:17,600 -> 17600000
 * @param {*} timeString
 * @returns
 */
export function convertSubtitleTimeToMicroseconds(timeString) {
  // 将时间字符串按照冒号、逗号和点分割成时、分、秒、毫秒和微秒
  const parts = timeString.split(/[:,\.]/);

  // 提取时、分、秒、毫秒和微秒
  const hours = parseInt(parts[0], 10);
  const minutes = parseInt(parts[1], 10);
  const seconds = parseInt(parts[2], 10);
  let milliseconds = 0;
  let microseconds = 0;
  if (parts.length > 3) {
    milliseconds = parseInt(parts[3], 10);
    if (parts.length > 4) {
      microseconds = parseInt(parts[4], 10);
    }
  }

  // 计算总微秒数
  const totalMicroseconds =
    ((hours * 3600 + minutes * 60 + seconds) * 1000 + milliseconds) * 1000 +
    microseconds;

  return totalMicroseconds;
}

// // 示例用法
// const timeString = "00:00:01,466.789";
// const totalMicroseconds = convertSubtitleTimeToMicroseconds(timeString);
// console.log(totalMicroseconds);  // 输出 1466789

export const srtStringToJson = (srtString) => {
  let result = null;

  const srtLines = srtString.trim().split("\n");
  const jsonResult = [];

  for (let i = 0; i < srtLines.length; i += 4) {
    const index = srtLines[i];
    const [start, end] = srtLines[i + 1].split(" --> ");
    const text = srtLines[i + 2];

    jsonResult.push({
      index: parseInt(index),
      start_time: start.trim(),
      end_time: end.trim(),
      text: text.trim(),
      keyword: text.trim(),
    });
  }

  result = jsonResult;

  return result;
};

export const jsonToSrtString = (subtitleJson) => {
  const jsonData = subtitleJson;
  let srtString = "";

  jsonData.forEach((item) => {
    srtString += `${item.index}\n`;
    srtString += `${item.start_time} --> ${item.end_time}\n`;
    srtString += `${item.text}\n\n`;
  });

  return srtString.trim();
};
