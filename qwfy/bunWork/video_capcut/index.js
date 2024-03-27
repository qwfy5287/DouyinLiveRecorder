// video_capcut/index.js

import {
  convertSubtitleTimeToMicroseconds,
  deepClone,
  generateId,
  getCanvasesItem,
  readJsonFile,
  srtStringToJson,
  writeJsonFile,
} from "./common/video.common";

let sourceJsonPath = "./draft_content.json";
let targetJsonPath = "./draft_content_new.json";

// 读取 JSON 文件
const draftJson = readJsonFile(sourceJsonPath);

if (draftJson) {
  // console.log(draftJson.materials.canvases);

  // // 1 添加 materials
  // // 1.1 添加 canvases item
  // let newCanvasId = generateId();
  // const newCanvasItem = {
  //   ...draftJson.materials.canvases[0],
  //   id: newCanvasId,
  // };
  // draftJson.materials.canvases.push(newCanvasItem);

  // // 1.2 添加 sound_channel_mappings
  // let newSoundChannelMappingId = generateId();
  // const newSoundChannelMappingItem = {
  //   ...draftJson.materials.sound_channel_mappings[0],
  //   id: newSoundChannelMappingId,
  // };
  // draftJson.materials.sound_channel_mappings.push(newSoundChannelMappingItem);

  // // 1.3 添加 speeds
  // let newSpeedId = generateId();
  // const newSpeedItem = {
  //   ...draftJson.materials.speeds[0],
  //   id: newSpeedId,
  // };
  // draftJson.materials.speeds.push(newSpeedItem);

  // // 1.4 添加 vocal_separations
  // let newVocalSeparationId = generateId();
  // const newVocalSeparationItem = {
  //   ...draftJson.materials.vocal_separations[0],
  //   id: newVocalSeparationId,
  // };
  // draftJson.materials.vocal_separations.push(newVocalSeparationItem);

  // // 1.5 添加 videos
  // let newVideoId = generateId();
  // const newVideoItem = {
  //   ...draftJson.materials.videos[0],
  //   id: newVideoId,
  // };
  // draftJson.materials.videos.push(newVideoItem);

  // // 2 添加 tracks
  // // 2.1 添加 tracks item
  // let newTrackId = generateId();
  // const newTrackItem = {
  //   ...deepClone(draftJson.tracks.filter((d) => d.type === "video")[0]),
  //   id: newTrackId,
  //   // 2 可能代表，不在主轨道上
  //   flag: 2,
  // };

  // // 2.2 添加 tracks item 的 segments item
  // let newSegmentId = generateId();
  // const newSegmentItem = {
  //   ...deepClone(newTrackItem.segments[0]),
  //   id: newSegmentId,
  //   render_index: 2,
  //   track_render_index: 2,
  //   extra_material_refs: [
  //     newCanvasId,
  //     newSoundChannelMappingId,
  //     newSpeedId,
  //     newVocalSeparationId,
  //   ],
  //   material_id: newVideoId,
  //   source_timerange: {
  //     duration: 43966666,
  //     start: 40000000,
  //   },
  //   target_timerange: {
  //     duration: 43966666,
  //     start: 40000000,
  //   },
  // };
  // newTrackItem.segments = [];
  // newTrackItem.segments.push(newSegmentItem);

  // draftJson.tracks.push(newTrackItem);

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

  let strString = `7
  00:00:12,166 --> 00:00:13,000
  跑量价结婚
  
  8
  00:00:13,000 --> 00:00:15,900
  这种的衣服结婚谁真的作
  
  9
  00:00:16,166 --> 00:00:17,600
  真心非常作
  
  10
  00:00:17,600 --> 00:00:22,266
  这件韩呃法国进口的一个原麻料
  
  11
  00:00:22,333 --> 00:00:24,600
  对又结合了拉架
  `;
  let json = srtStringToJson(strString);

  json.forEach((item) => {
    let start_time = convertSubtitleTimeToMicroseconds(item.start_time);
    let end_time = convertSubtitleTimeToMicroseconds(item.end_time);
    let duration = end_time - start_time;
    console.log(item);
    // 便利生成新的
    splitVideo(draftJson, newTrackItem, start_time, duration);
  });
  // newTrackItem.segments.shift();
  draftJson.tracks.push(newTrackItem);

  // 写入新的 JSON 文件
  writeJsonFile(targetJsonPath, draftJson);
}

function splitVideo(draftJson, newTrackItem, start_time, duration) {
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
    target_timerange: {
      // duration: 43966666,
      // start: 40000000,
      duration: duration,
      start: start_time,
    },
  };
  newTrackItem.segments = [];
  newTrackItem.segments.push(newSegmentItem);
}
