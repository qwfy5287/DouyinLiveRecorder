const fs = require("fs");

function exportSRT(jsonFilePath) {
  const jsonData = fs.readFileSync(jsonFilePath, "utf8");
  const json = JSON.parse(jsonData);

  const { materials, tracks } = json;
  const textMaterials = materials.texts;

  const srtLines = [];
  let srtIndex = 1;

  tracks.forEach((track) => {
    if (track.type === "text") {
      track.segments.forEach((segment) => {
        const { material_id, target_timerange } = segment;
        const { start, duration } = target_timerange;

        const textMaterial = textMaterials.find(
          (item) => item.id === material_id,
        );
        if (textMaterial) {
          const { words } = textMaterial;
          const { text } = words;

          const startTime = formatTime(start);
          const endTime = formatTime(start + duration);
          const subtitle = text.join("");

          const srtLine = `${srtIndex}\n${startTime} --> ${endTime}\n${subtitle}\n\n`;
          srtLines.push(srtLine);
          srtIndex++;
        }
      });
    }
  });

  return srtLines.join("");
}

function formatTime(microseconds) {
  const milliseconds = Math.floor(microseconds / 1000);
  const totalSeconds = Math.floor(milliseconds / 1000);
  const hours = Math.floor(totalSeconds / 3600);
  const minutes = Math.floor((totalSeconds % 3600) / 60);
  const seconds = totalSeconds % 60;
  const formattedMilliseconds = (milliseconds % 1000)
    .toString()
    .padStart(3, "0");

  return `${padZero(hours)}:${padZero(minutes)}:${padZero(seconds)},${formattedMilliseconds}`;
}

function padZero(number) {
  return number.toString().padStart(2, "0");
}

// 读取 JSON 文件并导出 SRT 字幕
const jsonFilePath = "./draft_info_example.json";
const srtContent = exportSRT(jsonFilePath);
console.log(srtContent);
