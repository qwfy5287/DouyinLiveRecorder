import { readFileSync } from "fs";

function readJsonFile(filePath) {
  try {
    const jsonData = readFileSync(filePath, "utf8");
    const data = JSON.parse(jsonData);
    console.log(data);
  } catch (error) {
    console.error("Error reading or parsing JSON file:", error);
  }
}

const jsonFilePath = process.argv[2];
if (jsonFilePath) {
  readJsonFile(jsonFilePath);
} else {
  console.log(
    "Please provide the path to the JSON file as a command-line argument.",
  );
}
