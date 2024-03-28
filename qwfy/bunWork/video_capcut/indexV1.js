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

  let strString = `21
  00:00:41,700 --> 00:00:43,120
  哇这上身的质感
  
  25
  00:00:50,366 --> 00:00:53,680
  夏天穿起来是嫩感通透肉感的
  
  32
  00:01:08,266 --> 00:01:11,400
  你就算袖子别一点点起来穿也很好看
  
  33
  00:01:11,400 --> 00:01:11,960
  很好看
  
  2
  00:00:02,933 --> 00:00:05,120
  做真正的高端线
  
  8
  00:00:13,000 --> 00:00:15,880
  这种的衣服结婚谁真的作
  
  11
  00:00:19,160 --> 00:00:22,280
  法国进口的一个原麻料
  
  15
  00:00:29,933 --> 00:00:31,720
  结合其他的全亚麻
  
  19
  00:00:37,400 --> 00:00:39,360
  一个灰一个粉色
  
  20
  00:00:40,500 --> 00:00:41,520
  确认一下这里面
  
  22
  00:00:43,133 --> 00:00:44,720
  真的你们会爱惨我
  
  23
  00:00:45,466 --> 00:00:47,920
  这件是真正进口的一个法国
  
  26
  00:00:54,088 --> 00:00:55,640
  你看一下这垂的感觉
  
  27
  00:00:56,200 --> 00:00:58,200
  你就算挂在那里有一点褶皱
  
  28
  00:00:58,200 --> 00:01:00,200
  挂在那里一会直接又回弹了
  
  30
  00:01:03,333 --> 00:01:05,560
  真的每天看你直播一点不带寂寞的
  
  34
  00:01:12,733 --> 00:01:14,000
  它真的很贵
  
  36
  00:01:15,500 --> 00:01:16,960
  这件应该只有三个尺码
  
  37
  00:01:17,166 --> 00:01:19,320
  m是1百s码s码105
  
  38
  00:01:19,333 --> 00:01:21,880
  m码120 加l穿到145斤
  
  39
  00:01:21,900 --> 00:01:23,480
  我觉得这件卡码都拍小
  `;
  let json = srtStringToJson(strString);

  let sort_start_time = 0;
  json.forEach((item) => {
    // console.log(item.start_time);
    let start_time = convertSubtitleTimeToMicroseconds(item.start_time);
    let end_time = convertSubtitleTimeToMicroseconds(item.end_time);
    let duration = end_time - start_time;
    // console.log(start_time);
    // console.log(duration);
    // console.log(item);
    // console.log(sort_start_time);
    // 便利生成新的
    splitVideo(draftJson, newTrackItem, start_time, duration, sort_start_time);
    sort_start_time += duration;
  });
  newTrackItem.segments.shift();
  draftJson.tracks.push(newTrackItem);

  // 写入新的 JSON 文件
  writeJsonFile(targetJsonPath, draftJson);
}

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
