import {
  filterExactTexts,
  filterKeywords,
  sortBrands,
  sortEmotions,
  sortMaterials,
  sortSizes,
  sortSpecialties,
} from "./subtitle.common";

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

export const filterJson = (subtitleJson) => {
  let result = null;

  const keywords = filterKeywords;
  const exactTexts = filterExactTexts;
  const jsonData = subtitleJson;

  const filteredData = jsonData.filter((item) => {
    const includesKeyword = keywords.some((keyword) =>
      item.text.includes(keyword)
    );
    const isExactText = exactTexts.includes(item.text);

    return !includesKeyword && !isExactText;
  });

  result = filteredData;

  return result;
};

export const sortJson = (filteredSubtitleJson) => {
  let result = null;
  const specialties = sortSpecialties;
  const emotions = sortEmotions;
  const brands = sortBrands;
  const materials = sortMaterials;
  const sizes = sortSizes;

  const jsonData = filteredSubtitleJson;

  jsonData.forEach((item) => {
    if (specialties.some((keyword) => item.text.includes(keyword))) {
      item.sort = 1;
    } else if (emotions.some((keyword) => item.text.includes(keyword))) {
      item.sort = 2;
    } else if (brands.some((keyword) => item.text.includes(keyword))) {
      item.sort = 3;
    } else if (materials.some((keyword) => item.text.includes(keyword))) {
      item.sort = 4;
    } else if (sizes.some((keyword) => item.text.includes(keyword))) {
      item.sort = 5;
    } else {
      item.sort = 9;
    }
  });

  const sortedJsonData = jsonData.sort((a, b) => a.sort - b.sort);

  result = sortedJsonData;

  return result;
};

// 格式化数字,补零
export function formatNumber(number, minimumIntegerDigits = 2) {
  return number.toString().padStart(minimumIntegerDigits, "0");
}

// 格式化时间为 SRT 格式
export function formatTime(time) {
  // console.log("🚀 ~ formatTime ~ time:", time);
  const totalSeconds = Math.floor(time / 1000);
  const milliseconds = time % 1000;
  const hours = Math.floor(totalSeconds / 3600);
  const minutes = Math.floor((totalSeconds % 3600) / 60);
  const seconds = totalSeconds % 60;

  return `${formatNumber(hours)}:${formatNumber(minutes)}:${formatNumber(
    seconds
  )},${formatNumber(milliseconds, 3)}`;
}

export function filterLinesByKeywords(splitSubtitleLines, keywordList) {
  // 过滤掉分割后的关键词
  let filterLines = splitSubtitleLines.filter((line) => {
    return (
      // keywordList.filter((keyword) => keyword.includes(line.text.trim()))
      keywordList.filter((keyword) => line.text.trim().includes(keyword))
        .length === 0
    );
  });
  return filterLines;
}

export function mergeSubtitleLines(subtitleLines) {
  const mergedLines = [];

  for (let i = 0; i < subtitleLines.length; i++) {
    const currentLine = subtitleLines[i];

    if (i === 0) {
      mergedLines.push(currentLine);
    } else {
      const prevLine = mergedLines[mergedLines.length - 1];

      if (prevLine.endTime === currentLine.startTime) {
        prevLine.text += "" + currentLine.text;
        prevLine.endTime = currentLine.endTime;
      } else {
        mergedLines.push(currentLine);
      }
    }
  }

  return mergedLines;
}

/**
 * subTitle 精细化处理
 * 1. 将 SRT 字幕转换为 JSON 格式
 */
export const doFlow = (srtData) => {
  let result = null;

  const subtitleJsonResult = srtStringToJson(srtData);
  console.log("json:", subtitleJsonResult.length);
  //   console.log(jsonString);

  const filterJsonResult = filterJson(subtitleJsonResult);
  console.log("filter:", filterJsonResult.length);

  //   console.log(filterJsonResult);

  const sortJsonResult = sortJson(filterJsonResult);
  console.log("sort:", sortJsonResult.length);

  const sortJsonGood = sortJsonResult.filter((item) => item.sort < 9);
  const sortJsonNormal = sortJsonResult.filter((item) => item.sort >= 9);
  console.log("sort good:", sortJsonGood.length);
  console.log("sort normal", sortJsonNormal.length);
  //   console.log(sortJsonGood);
  // console.log(sortJsonNormal);

  result = sortJsonGood;
  return result;
};
