// video_capcut/index.js

import {
  convertSubtitleTimeToMicroseconds,
  deepClone,
  generateId,
  readJsonFile,
  readSrtFile,
  srtStringToJson,
  writeJsonFile,
} from "./common/video.common";
import { doDartJsonToSrt } from "../video_capcut_keyword/capcutHelper";

function splitVideo(
  draftJson,
  newTrackItem,
  start_time,
  duration,
  sort_start_time
) {
  // 1 添加 materials
  // 1.1 添加 canvases item
  let newCanvasId = generateId();
  const newCanvasItem = {
    ...draftJson.materials.canvases[0],
    id: newCanvasId,
  };
  draftJson.materials.canvases.push(newCanvasItem);

  // 1.2 添加 sound_channel_mappings
  let newSoundChannelMappingId = generateId();
  const newSoundChannelMappingItem = {
    ...draftJson.materials.sound_channel_mappings[0],
    id: newSoundChannelMappingId,
  };
  draftJson.materials.sound_channel_mappings.push(newSoundChannelMappingItem);

  // 1.3 添加 speeds
  let newSpeedId = generateId();
  const newSpeedItem = {
    ...draftJson.materials.speeds[0],
    id: newSpeedId,
  };
  draftJson.materials.speeds.push(newSpeedItem);

  // 1.4 添加 vocal_separations
  let newVocalSeparationId = generateId();
  const newVocalSeparationItem = {
    ...draftJson.materials.vocal_separations[0],
    id: newVocalSeparationId,
  };
  draftJson.materials.vocal_separations.push(newVocalSeparationItem);

  // 1.5 添加 videos
  let newVideoId = generateId();
  const newVideoItem = {
    ...draftJson.materials.videos[0],
    id: newVideoId,
  };
  draftJson.materials.videos.push(newVideoItem);

  // 2.2 添加 tracks item 的 segments item
  let newSegmentId = generateId();
  const newSegmentItem = {
    ...deepClone(newTrackItem.segments[0]),
    id: newSegmentId,
    render_index: 2,
    track_render_index: 2,
    extra_material_refs: [
      newCanvasId,
      newSoundChannelMappingId,
      newSpeedId,
      newVocalSeparationId,
    ],
    material_id: newVideoId,
    source_timerange: {
      // duration: 43966666,
      // start: 40000000,
      duration: duration,
      start: start_time,
    },
    // target_timerange: {
    //   // duration: 43966666,
    //   // start: 40000000,
    //   duration: duration,
    //   start: start_time,
    // },
    target_timerange: {
      // duration: 43966666,
      // start: 40000000,
      duration: duration,
      start: sort_start_time,
    },
  };
  // newTrackItem.segments = [];
  newTrackItem.segments.push(newSegmentItem);
}

function splitTrack(sourceJsonPath, targetJsonPath, srtString) {
  // 读取 JSON 文件
  const draftJson = readJsonFile(sourceJsonPath);

  if (!draftJson) {
    console.warn("读取 JSON 文件失败");
    return;
  }

  /**
   * flow
   */

  // 2 添加 tracks
  // 2.1 添加 tracks item
  let newTrackId = generateId();
  const newTrackItem = {
    ...deepClone(draftJson.tracks.filter((d) => d.type === "video")[0]),
    id: newTrackId,
    // 2 可能代表，不在主轨道上
    flag: 2,
  };

  // let srtPath = "../video_capcut_keyword/output_srt.srt";
  // let srtString = readSrtFile(srtPath);

  let srtJson = srtStringToJson(srtString);

  let sort_start_time = 0;
  srtJson.forEach((item) => {
    let start_time = convertSubtitleTimeToMicroseconds(item.start_time);
    let end_time = convertSubtitleTimeToMicroseconds(item.end_time);
    let duration = end_time - start_time;
    // 便利生成新的
    splitVideo(draftJson, newTrackItem, start_time, duration, sort_start_time);
    sort_start_time += duration;
  });
  newTrackItem.segments.shift();
  draftJson.tracks.push(newTrackItem);

  // 写入新的 JSON 文件
  writeJsonFile(targetJsonPath, draftJson);
}

function main() {
  // 从命令行参数获取 folderName
  let folderName = "3月27日-cut-small";

  if (process.argv[2]) {
    folderName = process.argv[2];
  }

  // let sourceJsonPath = "./draft_content.json";
  let sourceJsonPath = `/Users/qwfy/Movies/JianyingPro/User Data/Projects/com.lveditor.draft/${folderName}/draft_info.json`;
  let targetJsonPath = "./draft_content_new.json";

  let srtString = doDartJsonToSrt(sourceJsonPath);
  console.log("🚀 ~ main ~ ss:", srtString);
  splitTrack(sourceJsonPath, targetJsonPath, srtString);
}

main();
