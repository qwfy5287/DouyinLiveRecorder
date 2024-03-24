// qwfy/bunWrok/video_capcut_keyword/capcutHelper.js

import { readFileSync } from "bun:fs";

// 字幕片段类
class SubtitleSegment {
  constructor(text, startTime, endTime) {
    this.text = text;
    this.startTime = startTime;
    this.endTime = endTime;
  }
}

// SRT 格式化工具类
class SRTFormatter {
  static formatTime(timeInMicroseconds) {
    const timeInSeconds = timeInMicroseconds / 1000000;
    const hours = Math.floor(timeInSeconds / 3600);
    const minutes = Math.floor((timeInSeconds % 3600) / 60);
    const seconds = Math.floor(timeInSeconds % 60);
    const milliseconds = Math.floor((timeInSeconds % 1) * 1000);

    return (
      this.padZero(hours, 2) +
      ":" +
      this.padZero(minutes, 2) +
      ":" +
      this.padZero(seconds, 2) +
      "," +
      this.padZero(milliseconds, 3)
    );
  }

  static padZero(num, length) {
    return num.toString().padStart(length, "0");
  }

  static formatSubtitles(subtitles) {
    const srtLines = [];
    subtitles.forEach((subtitle, index) => {
      srtLines.push(index + 1);
      srtLines.push(
        `${this.formatTime(subtitle.startTime)} --> ${this.formatTime(
          subtitle.endTime,
        )}`,
      );

      const maxLineLength = 50;
      if (subtitle.text.length > maxLineLength) {
        const lines = [];
        let currentLine = "";
        subtitle.text.split(" ").forEach((word) => {
          if (currentLine.length + word.length + 1 <= maxLineLength) {
            currentLine += (currentLine ? " " : "") + word;
          } else {
            lines.push(currentLine);
            currentLine = word;
          }
        });
        if (currentLine) {
          lines.push(currentLine);
        }
        srtLines.push(lines.join("\n"));
      } else {
        srtLines.push(subtitle.text);
      }

      srtLines.push("");
    });
    return srtLines.join("\n");
  }
}

class SubtitleFilter {
  static filterKeywords(content, words, keywords) {
    if (!words || !words.text) {
      return {
        content: content,
        words: words,
      };
    }

    const pattern = new RegExp(`(${keywords.join("|")})`, "g");
    const parts = content.split(pattern).filter((part) => part.trim() !== "");

    const filteredContent = parts.join("");

    const filteredWords = {
      text: [],
      start_time: [],
      end_time: [],
    };

    let currentIndex = 0;

    parts.forEach((part) => {
      const startIndex = content.indexOf(part, currentIndex);
      const endIndex = startIndex + part.length - 1;

      const startWord = words.text.findIndex(
        (_, index) => words.end_time[index] >= startIndex,
      );
      const endWord = words.text.findIndex(
        (_, index) => words.end_time[index] >= endIndex,
      );

      filteredWords.text = filteredWords.text.concat(
        words.text.slice(startWord, endWord + 1),
      );
      filteredWords.start_time.push(words.start_time[startWord]);
      filteredWords.end_time.push(words.end_time[endWord]);

      currentIndex = endIndex + 1;
    });

    return {
      content: filteredContent,
      words: filteredWords,
    };
  }
}

class SubtitleConverter {
  constructor(draftInfo, keywords) {
    this.draftInfo = draftInfo;
    this.keywords = keywords;
  }

  convert() {
    const subtitles = [];
    const textTrack = this.draftInfo.tracks.find(
      (track) => track.type === "text",
    );

    textTrack.segments.forEach((segment) => {
      const material = this.draftInfo.materials.texts.find(
        (text) => text.id === segment.material_id,
      );

      const content = JSON.parse(material.content).text;
      const words = JSON.parse(material.content).words;
      const { content: filteredContent, words: filteredWords } =
        SubtitleFilter.filterKeywords(content, words, this.keywords);

      if (filteredContent) {
        if (filteredWords && filteredWords.start_time.length > 0) {
          const startTime = filteredWords.start_time[0];
          const endTime =
            filteredWords.end_time[filteredWords.end_time.length - 1];
          subtitles.push(
            new SubtitleSegment(filteredContent, startTime, endTime),
          );
        } else {
          subtitles.push(
            new SubtitleSegment(
              filteredContent,
              segment.target_timerange.start,
              segment.target_timerange.start +
                segment.target_timerange.duration,
            ),
          );
        }
      }
    });

    return SRTFormatter.formatSubtitles(subtitles);
  }
}

// 读取 draft_info_example.json 文件
const draftInfoJson = readFileSync("draft_info_example.json", "utf8");
const draftInfo = JSON.parse(draftInfoJson);

// 关键词列表
const keywords = ["然后呢", "这个"];

// 创建字幕转换器实例并转换为 SRT 格式
const converter = new SubtitleConverter(draftInfo, keywords);
const srtContent = converter.convert();

// 输出 SRT 内容
console.log(srtContent);
