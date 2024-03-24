const fs = require("fs");

// function msToSrtTime(ms) {
//   const hours = Math.floor(ms / 3600000)
//     .toString()
//     .padStart(2, "0");
//   const minutes = Math.floor((ms % 3600000) / 60000)
//     .toString()
//     .padStart(2, "0");
//   const seconds = Math.floor((ms % 60000) / 1000)
//     .toString()
//     .padStart(2, "0");
//   const milliseconds = (ms % 1000).toString().padStart(3, "0");

//   return `${hours}:${minutes}:${seconds},${milliseconds}`;
// }

// function convertJsonToSrt(jsonData) {
//   let srtData = "";
//   let counter = 1;

//   jsonData.materials.texts.forEach((item) => {
//     item.words.text.forEach((text, index) => {
//       const startTime = item.words.start_time[index];
//       const endTime = item.words.end_time[index];
//       srtData += `${counter}\n`;
//       srtData += `${msToSrtTime(startTime)} --> ${msToSrtTime(endTime)}\n`;
//       srtData += `${text}\n\n`;
//       counter++;
//     });
//   });

//   return srtData;
// }

function msToSrtTime(ms) {
  let seconds = ms / 1000;
  const hours = parseInt(seconds / 3600, 10)
    .toString()
    .padStart(2, "0");
  seconds %= 3600;
  const minutes = parseInt(seconds / 60, 10)
    .toString()
    .padStart(2, "0");
  seconds = (seconds % 60).toFixed(3);
  const [sec, mil] = seconds.split(".");
  return `${hours}:${minutes}:${sec},${mil}`;
}

function convertJsonToSrt(jsonData) {
  let srtData = "";
  jsonData.materials.texts.forEach((material, index) => {
    const { start_time, end_time, text } = material.words;
    const startTime = start_time[0];
    const endTime = end_time[end_time.length - 1];
    const textLine = text.join("");

    srtData += `${index + 1}\n`;
    srtData += `${msToSrtTime(startTime)} --> ${msToSrtTime(endTime)}\n`;
    srtData += `${textLine}\n\n`;
  });

  return srtData;
}

async function main() {
  const data = fs.readFileSync("draft_info_example.json", {
    encoding: "utf-8",
  });
  const jsonData = JSON.parse(data);
  const srtData = convertJsonToSrt(jsonData);

  console.log(srtData);
  // 你也可以选择将 SRT 数据保存到文件中
  // fs.writeFileSync('output.srt', srtData);
}

main();
