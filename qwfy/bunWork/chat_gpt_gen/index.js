const fs = require("fs");
const path = require("path");

function convertSrtToMd(folderPath, outputPath) {
  // 读取文件夹下的所有文件
  const files = fs.readdirSync(folderPath);

  // 筛选出 .srt 文件
  const srtFiles = files.filter(
    (file) => path.extname(file).toLowerCase() === ".srt"
  );

  // 存储转换后的 Markdown 内容
  let markdownContent = "";

  // 遍历每个 .srt 文件
  srtFiles.forEach((file) => {
    const filePath = path.join(folderPath, file);
    const content = fs.readFileSync(filePath, "utf-8");

    // 提取产品信息
    const { brand, name, productId } = parseProductInfo(file);

    // 生成 Markdown 标题
    const title = `## 【${brand}】${name}`;
    markdownContent += title + "\n\n";

    // 将 .srt 内容转换为 Markdown 格式
    const lines = content.split("\n");
    lines.forEach((line) => {
      if (line.trim() !== "") {
        markdownContent += line + "\n";
      }
    });

    markdownContent += "\n";
  });

  // 将转换后的 Markdown 内容写入输出文件
  fs.writeFileSync(outputPath, markdownContent);

  console.log("转换完成!");
}

function parseProductInfo(filename) {
  // 提取品牌名
  const brand = filename.match(/【(.*?)】/)[1];

  // 提取产品名称
  const nameMatch = filename.match(/】(.*?)\d/);
  const name = nameMatch ? nameMatch[1].trim() : "";

  // 提取产品编号
  const productId = filename.match(/\d+/)[0];

  // 提取文件扩展名
  const extension = filename.split(".").pop();

  // 返回解析后的产品信息对象
  return {
    brand,
    name,
    productId,
    extension,
  };
}

function main() {
  // 示例用法
  const folderPath = "./data/srt";
  const outputPath = "output-wlb-srt-example.md";

  convertSrtToMd(folderPath, outputPath);
}

main();
