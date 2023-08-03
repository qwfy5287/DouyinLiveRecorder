# -*- encoding: utf-8 -*-

"""
Author: Hmily
Github: https://github.com/ihmily
Date: 2023-07-17 23:52:05
Update: 2023-08-04 06:30:00
Copyright (c) 2023 by Hmily, All Rights Reserved.
Function: Record live stream video.
"""

import random
import os
import urllib.parse
import time
import configparser
import subprocess
import threading
import logging
import datetime
import shutil
import hashlib
from spider import *
from web_rid import *

# 版本号
version = 202300803.08

# --------------------------log日志-------------------------------------
# 创建一个logger
logger = logging.getLogger('抖音直播录制%s版' % str(version))
logger.setLevel(logging.INFO)
# 创建一个handler，用于写入日志文件
if not os.path.exists("./log"):
    os.makedirs("./log")
fh = logging.FileHandler("./log/错误日志文件.log", encoding="utf-8-sig", mode="a")
fh.setLevel(logging.WARNING)
# 定义handler的输出格式
formatter = logging.Formatter('%(asctime)s - %(message)s')
fh.setFormatter(formatter)
# 给logger添加handler
logger.addHandler(fh)

# --------------------------全局变量-------------------------------------
recording = set()
unrecording = set()
warning_count = 0
max_request = 0
runing_list = []
url_tuples_list = []
textNoRepeatUrl = []
create_var = locals()
first_start = True
name_list = []
firstRunOtherLine = True
live_list = []
not_record_list = []
start5_time = datetime.datetime.now()
global_proxy = False
recording_time_list={}
headers = {
    'User-Agent': 'Mozilla/5.0 (Linux; Android 11; SAMSUNG SM-G973U) AppleWebKit/537.36 (KHTML, like Gecko) SamsungBrowser/14.2 Chrome/87.0.4280.141 Mobile Safari/537.36'
}
config_file = './config/config.ini'
url_config_file = './config/URL_config.ini'
backup_dir = './backup_config'
encoding = 'utf-8-sig'
rstr = r"[\/\\\:\*\?\"\<\>\|&u]"
ffmpeg_path = "ffmpeg"  # ffmpeg文件路径
default_path = os.getcwd()


# --------------------------用到的函数-------------------------------------

def display_info():
    # TODO: 显示当前录制配置信息
    global start5_time
    global recording_time_list
    time.sleep(5)
    while True:
        try:
            time.sleep(5)
            os.system("cls")
            print("\r共监测" + str(Monitoring) + "个直播中", end=" | ")
            print("同一时间访问网络的线程数:", max_request, end=" | ")
            if len(video_save_path) > 0:
                if not os.path.exists(video_save_path):
                    print("配置文件里,直播保存路径并不存在,请重新输入一个正确的路径.或留空表示当前目录,按回车退出")
                    input("程序结束")
                    os._exit(0)

            if use_proxy:
                is_use_proxy='是'
            else:
                is_use_proxy = '否'
            print(f"是否开启代理录制: {is_use_proxy}", end=" | ")

            if Splitvideobysize:
                print("TS录制分段开启，录制分段大小为 %d M" % Splitsize, end=" | ")

            print("Cookies录制", end=" | ")
            print("录制视频质量为: " + str(video_quality), end=" | ")
            print("录制视频格式为: " + str(video_save_type), end=" | ")
            print("目前瞬时错误数为: " + str(warning_count), end=" | ")
            nowdate = time.strftime("%H:%M:%S", time.localtime())
            print(f"当前时间: {nowdate}")

            if len(recording) == 0 and len(unrecording) == 0:
                time.sleep(5)
                print("\r没有正在录制的直播 " + nowdate, end="")
                print("")
                continue
            else:
                now_time = datetime.datetime.now()
                if len(recording) > 0:
                    print("x" * 60)
                    NoRepeatrecording = list(set(recording))
                    print(f"正在录制{len(NoRepeatrecording)}个直播: ")
                    for recording_live in NoRepeatrecording:
                        have_record_time=now_time-recording_time_list[recording_live]
                        print(f"{recording_live} 正在录制中 "+str(have_record_time).split('.')[0])

                    # print('\n本软件已运行：'+str(now_time - start5_time).split('.')[0])
                    print("x" * 60)
                else:
                    start5_time = now_time
        except Exception as e:
            print("错误信息644:" + str(e) + "\r\n发生错误的行数: "+str(e.__traceback__.tb_lineno))
            logger.warning("错误信息: " + str(e) + " 发生错误的行数: " + str(e.__traceback__.tb_lineno))


def update_file(file, old_str, new_str):
    # TODO: 更新文件操作
    file_data = ""
    with open(file, "r", encoding="utf-8-sig") as f:
        for text_line in f:
            if old_str in text_line:
                text_line = text_line.replace(old_str, new_str)
            file_data += text_line
    with open(file, "w", encoding="utf-8-sig") as f:
        f.write(file_data)


def converts_mp4(address):
    if tsconvert_to_mp4:
        _output = subprocess.check_output([
            "ffmpeg", "-i", address,
            "-c:v", "copy",
            "-c:a", "copy",
            "-f", "mp4", address.split('.')[0] + ".mp4",
        ], stderr=subprocess.STDOUT)
        if delFilebeforeconversion:
            time.sleep(1)
            if os.path.exists(address):
                os.remove(address)


def converts_m4a(address):
    if tsconvert_to_m4a:
        _output = subprocess.check_output([
            "ffmpeg", "-i", address,
            "-n", "-vn",
            "-c:a", "aac", "-bsf:a", "aac_adtstoasc", "-ab", "320k",
            address.split('.')[0] + ".m4a",
        ], stderr=subprocess.STDOUT)
        if delFilebeforeconversion:
            time.sleep(1)
            if os.path.exists(address):
                os.remove(address)


def create_ass_file(filegruop):
    # TODO:  录制时生成ass格式的字幕文件
    anchor_name = filegruop[0]
    ass_filename = filegruop[1]
    index_time = -1
    finish = 0
    today = datetime.datetime.now()
    re_datatime = today.strftime('%Y-%m-%d %H:%M:%S')

    while True:
        index_time += 1
        txt = str(index_time) + "\n" + tranform_int_to_time(index_time) + ',000 --> ' + tranform_int_to_time(
            index_time + 1) + ',000' + "\n" + str(re_datatime) + "\n"

        with open(ass_filename + ".ass", 'a', encoding='utf8') as f:
            f.write(txt)

        if anchor_name not in recording:
            finish += 1
            offset = datetime.timedelta(seconds=1)
            # 获取修改后的时间并格式化
            re_datatime = (today + offset).strftime('%Y-%m-%d %H:%M:%S')
            today = today + offset
        else:
            time.sleep(1)
            today = datetime.datetime.now()
            re_datatime = today.strftime('%Y-%m-%d %H:%M:%S')

        if finish > 15:
            break


def change_max_connect():
    global max_request
    global warning_count
    # 动态控制连接次数

    preset = max_request
    # 记录当前时间
    start_time = time.time()

    while True:
        time.sleep(5)
        if 10 <= warning_count <= 20:
            if preset > 5:
                max_request = 5
            else:
                max_request //= 2  # 将max_request除以2（向下取整）
                if max_request > 0:  # 如果得到的结果大于0，则直接取该结果
                    max_request = preset
                else:  # 否则将其设置为1
                    preset = 1

            print("同一时间访问网络的线程数动态改为", max_request)
            warning_count = 0
            time.sleep(5)

        elif 20 < warning_count:
            max_request = 1
            print("同一时间访问网络的线程数动态改为", max_request)
            warning_count = 0
            time.sleep(10)

        elif warning_count < 10 and time.time() - start_time > 60:
            max_request = preset
            warning_count = 0
            start_time = time.time()
            print("同一时间访问网络的线程数动态改为", max_request)


def get_douyin_stream_url(json_data):
    # TODO: 获取直播源地址
    data = []  # 定义一个返回数据列表

    roomStore = json_data['app']['initialState']['roomStore']
    roomInfo = roomStore['roomInfo']
    anchor_name = roomInfo['anchor']['nickname']
    data.append(anchor_name)
    status=4
    # 获取直播间状态
    if 'room' in roomInfo:
        status = roomInfo["room"]["status"]  # 直播状态2是正在直播.4是未开播

    if status == 4:
        data = [anchor_name, False, '', '']
    else:
        stream_url = roomInfo['room']['stream_url']
        # flv视频流链接
        flv_url_list = stream_url['flv_pull_url']
        # m3u8视频流链接
        m3u8_url_list = stream_url['hls_pull_url_map']

        # origin蓝光1080P、720超清hd、720高清sd、540标清ld
        if video_quality == "原画" or video_quality == "蓝光":
            m3u8_url = m3u8_url_list["FULL_HD1"]
            flv_url = flv_url_list["FULL_HD1"]
        elif video_quality == "超清":
            m3u8_url = m3u8_url_list["HD1"]
            flv_url = flv_url_list["HD1"]
        elif video_quality == "高清":
            m3u8_url = m3u8_url_list["SD1"]
            flv_url = flv_url_list["SD1"]
        elif video_quality == "标清":
            m3u8_url = m3u8_url_list["SD2"]
            flv_url = flv_url_list["SD2"]

        data = [anchor_name, True, m3u8_url, flv_url]
    return data

def get_tiktok_stream_url(json_data):
    # TODO: 获取Tiktok直播源地址
    data = []  # 定义一个返回数据列表

    LiveRoom = json_data['LiveRoom']['liveRoomUserInfo']
    anchor_name = LiveRoom['user']['nickname']
    data.append(anchor_name)
    # 获取直播间状态
    status = LiveRoom['user']["status"]  # 直播状态2是正在直播.4是未开播

    if status == 4:
        data = [anchor_name, False, '', '']
    else:
        # 画质从高到低：origin>uhd>sd>sd>ld
        # {origin:'原画质或蓝光',uhd:'1080P或720P',sd:'540P或480P',ld:'360P标清'}
        # 上面画质对应只是一般情况，具体情况有可能不一样 可以看对应画质的sdk_params参数，里面有如1080P等参数
        stream_data = LiveRoom['liveRoom']['streamData']['pull_data']['stream_data']
        stream_data = json.loads(stream_data)['data']
        if video_quality == "原画" or video_quality == "蓝光":
            m3u8_url = stream_data["origin"]['main']['hls']
            flv_url = stream_data["origin"]['main']['flv']
        elif video_quality == "超清":
            m3u8_url = stream_data["uhd"]['main']['hls']
            flv_url = stream_data["uhd"]['main']['flv']
        elif video_quality == "高清":
            m3u8_url = stream_data["sd"]['main']['hls']
            flv_url = stream_data["sd"]['main']['flv']
        elif video_quality == "标清":
            m3u8_url = stream_data["ld"]['main']['hls']
            flv_url = stream_data["ld"]['main']['flv']
        # 注意，这里要将链接改为http协议，否则无法使用ffmpeg录制，原因是代理大都是http
        m3u8_url = re.sub("https", "http", m3u8_url)
        flv_url = re.sub("https", "http", flv_url)
        data = [anchor_name, True, m3u8_url, flv_url]
    return data

def get_kuaishou_stream_url(json_data):
    # TODO: 获取快手直播源地址
    data = []  # 定义一个返回数据列表


    liveroom = json_data['liveroom']
    anchor_name = liveroom['author']['name']
    data.append(anchor_name)
    # 获取直播间状态
    status = liveroom['isLiving']  # 直播状态True是正在直播.False是未开播

    if not status:
        data = [anchor_name, False, '', '']
    else:
        stream_data = liveroom['liveStream']['playUrls'][0]['adaptationSet']['representation']
        # stream_data数组中索引从小到大分别是高清、超清、蓝光4M、蓝光8M （每个直播间不一样）
        if video_quality == "原画" or video_quality == "蓝光":
            flv_url = stream_data[-1]['url']
        elif video_quality == "超清":
            flv_url = stream_data[-2]['url']
        elif video_quality == "高清":
            flv_url = stream_data[1]['url']
        elif video_quality == "标清":
            flv_url = stream_data[0]['url']

        data = [anchor_name, True, flv_url, flv_url]  # 快手只有flv视频流
    return data

def start_record(line, count_variable=-1):
    global warning_count
    global video_save_path
    global live_list
    global not_record_list
    global recording_time_list
    while True:
        try:
            record_finished = False
            record_finished_2 = False
            Runonce = False
            is_long_url = False
            count_time = time.time()
            url_tuple = line
            record_url = url_tuple[0]
            anchor_name = url_tuple[1]
            print("\r运行新线程,传入地址 " + record_url)

            while True:
                try:
                    port_info = []
                    if record_url.find("https://live.douyin.com/") > -1:
                        # 判断如果是浏览器长链接
                        with semaphore:
                            # 使用semaphore来控制同时访问资源的线程数量
                            json_data = get_douyin_stream_data(record_url,cookies_set)  # 注意这里需要配置文件中的cookie
                            port_info = get_douyin_stream_url(json_data)
                    elif record_url.find("https://v.douyin.com/") > -1:
                        # 判断如果是app分享链接
                        is_long_url = True
                        room_id, sec_user_id = get_sec_user_id(record_url)
                        web_rid = get_live_room_id(room_id, sec_user_id)
                        if len(web_rid) == 0:
                            print('web_rid 获取失败，若多次失败请联系作者修复或者使用浏览器打开后的长链接')
                        new_record_url = "https://live.douyin.com/" + str(web_rid)
                        not_record_list.append(new_record_url)
                        with semaphore:
                            json_data = get_douyin_stream_data(new_record_url, cookies_set)
                            port_info = get_douyin_stream_url(json_data)

                    elif record_url.find("https://www.tiktok.com/") > -1:
                        with semaphore:
                            if use_proxy or (global_proxy and proxy_addr !=''):
                                json_data = get_tiktok_stream_data(record_url,proxy_addr)
                                port_info = get_tiktok_stream_url(json_data)

                    elif record_url.find("https://live.kuaishou.com/") > -1:
                        with semaphore:
                            json_data = get_kuaishou_stream_data(record_url)
                            port_info = get_kuaishou_stream_url(json_data)

                    # print("端口信息:" + str(port_info))
                    # port_info=['主播名','状态码','m3u8地址','flv地址']
                    if len(port_info) != 4:
                        print(f'序号{count_variable} 网址内容获取失败,进行重试中...获取失败的地址是:{line}')
                        warning_count += 1
                    else:
                        anchor_name = port_info[0]
                        anchor_name = re.sub(rstr, "_", anchor_name)  # 过滤不能作为文件名的字符，替换为下划线

                        if anchor_name in live_list:
                            print(f"新增的地址: {anchor_name} 已经存在,本条线程将会退出")
                            name_list.append(f'{record_url}|#{record_url}')
                            exit(0)

                        if url_tuple[1] == "" and Runonce is False:
                            if is_long_url:
                                name_list.append(f'{record_url}|{new_record_url},主播: {anchor_name.strip()}')
                            else:
                                name_list.append(f'{record_url}|{record_url},主播: {anchor_name.strip()}')
                            Runonce = True

                        # 判断状态码 如果是2则正在直播，如果是4则未在直播
                        if not port_info[1]:
                            print(f"序号{count_variable} {port_info[0]} 等待直播... ")
                            anchor_name = port_info[0]
                        else:
                            print(f"序号{count_variable} {port_info[0]} 正在直播中...")

                            # 是否显示直播地址
                            if video_m3u8:
                                if video_save_type == "FLV":
                                    print(f"{port_info[0]} 直播地址为:{port_info[3]}")
                                else:
                                    print(f"{port_info[0]} 直播地址为:{port_info[2]}")

                            real_url = port_info[2]  # 默认使用m3u8地址进行下载

                            if real_url == "":
                                print('解析错误，直播间视频流未找到...')
                                pass
                            else:
                                live_list.append(anchor_name)
                                now = time.strftime("%Y-%m-%d-%H-%M-%S", time.localtime(time.time()))
                                try:
                                    full_path = f'{default_path}/{anchor_name}'
                                    if len(video_save_path) > 0:
                                        if video_save_path[-1] != "/":
                                            video_save_path = video_save_path + "/"
                                        full_path = f'{video_save_path}/{anchor_name}'
                                        if not os.path.exists(full_path):
                                            os.makedirs(full_path)
                                    else:
                                        if not os.path.exists(anchor_name):
                                            os.makedirs('./' + anchor_name)

                                except Exception as e:
                                    print("路径错误信息708: " + str(e) + " 发生错误的行数: " + str(e.__traceback__.tb_lineno))
                                    logger.warning("错误信息: " + str(e) + " 发生错误的行数: " + str(e.__traceback__.tb_lineno))

                                if not os.path.exists(full_path):
                                    print("保存路径不存在,不能生成录制.请避免把本程序放在c盘,桌面,下载文件夹,qq默认传输目录.请重新检查设置")
                                    video_save_path = ""
                                    print(f"因为配置文件的路径错误,本次录制在程序目录 {default_path}")


                                ffmpeg_command = [
                                    ffmpeg_path, "-y",
                                    "-v", "verbose",
                                    "-rw_timeout", "10000000",  # 10s
                                    "-loglevel", "error",
                                    "-hide_banner",
                                    "-user_agent", headers["User-Agent"],
                                    "-protocol_whitelist", "rtmp,crypto,file,http,https,tcp,tls,udp,rtp",
                                    "-thread_queue_size", "1024",
                                    "-analyzeduration", "2147483647",
                                    "-probesize", "2147483647",
                                    "-fflags", "+discardcorrupt",
                                    "-i", real_url,
                                    "-bufsize", "5000k",
                                    # "-map", "0",  # 不同点
                                    "-sn", "-dn",
                                    # "-bsf:v","h264_mp4toannexb",
                                    # "-c","copy",  # 直接用copy的话体积特别大.
                                    # "-c:v","libx264",   # 后期可以用crf来控制大小
                                    "-reconnect_delay_max", "30",
                                    "-reconnect_streamed", "-reconnect_at_eof",
                                    # "-c:v", "copy",  # 不同点
                                    "-c:a", "copy",
                                    "-max_muxing_queue_size", "64",
                                    "-correct_ts_overflow", "1",
                                    # "-f", "matroska",  # 不同点
                                    # "{path}".format(path=file),  # 不同点
                                ]

                                # 添加代理参数
                                if 'tiktok' in real_url:
                                    if use_proxy and proxy_addr != '':
                                        # os.environ["http_proxy"] = proxy_addr
                                        ffmpeg_command.insert(1, "-http_proxy")
                                        ffmpeg_command.insert(2, proxy_addr)


                                record_name=f'序号{count_variable} ' + anchor_name
                                recording.add(record_name)
                                start_record_time = datetime.datetime.now()
                                recording_time_list[record_name] = start_record_time
                                rec_info=f"\r{anchor_name} 录制视频中: {full_path}"
                                filename_short = full_path + '/' + anchor_name + '_' + now

                                if video_save_type == "FLV":
                                    filename = anchor_name + '_' + now + '.flv'
                                    print(f'{rec_info}/{filename}')

                                    if create_time_file:
                                        filename_gruop = [anchor_name, filename_short]
                                        create_var[str(filename_short)] = threading.Thread(target=create_ass_file,
                                                                                           args=(filename_gruop,))
                                        create_var[str(filename_short)].daemon = True
                                        create_var[str(filename_short)].start()

                                    try:
                                        # “port_info[3]”对应的是flv地址，使用老方法下载（直接请求下载flv）只能是下载flv流的。
                                        real_url = port_info[3]
                                        _filepath, _ = urllib.request.urlretrieve(real_url,full_path + '/' + filename)
                                        record_finished = True
                                        record_finished_2 = True
                                        count_time = time.time()

                                    except:
                                        print('\r' + time.strftime('%Y-%m-%d %H:%M:%S  ') + anchor_name + ' 未开播')

                                    # 注意，只有录制完后才会执行到这里
                                    if anchor_name in recording:
                                        recording.remove(anchor_name)
                                    if anchor_name in unrecording:
                                        unrecording.add(anchor_name)

                                elif video_save_type == "MKV":
                                    filename = anchor_name + '_' + now + ".mkv"
                                    print(f'{rec_info}/{filename}')
                                    file = full_path + '/' + filename
                                    if create_time_file:
                                        filename_gruop = [anchor_name, filename_short]
                                        create_var[str(filename_short)] = threading.Thread(target=create_ass_file,
                                                                                           args=(filename_gruop,))
                                        create_var[str(filename_short)].daemon = True
                                        create_var[str(filename_short)].start()

                                    try:
                                        command=[
                                            "-map", "0",
                                            "-c:v", "copy",  # 直接用copy的话体积特别大.
                                            "-f", "matroska",
                                            "{path}".format(path=file),
                                        ]
                                        ffmpeg_command.extend(command)

                                        _output = subprocess.check_output(ffmpeg_command, stderr=subprocess.STDOUT)

                                        record_finished = True
                                        record_finished_2 = True
                                        count_time = time.time()
                                    except subprocess.CalledProcessError as e:
                                        # logging.warning(str(e.output))
                                        print(str(e.output) + " 发生错误的行数: " + str(e.__traceback__.tb_lineno))
                                        logger.warning(
                                            "错误信息: " + str(e) + " 发生错误的行数: " + str(e.__traceback__.tb_lineno))
                                    if anchor_name in recording:
                                        recording.remove(anchor_name)
                                    if anchor_name in unrecording:
                                        unrecording.add(anchor_name)

                                elif video_save_type == "MP4":

                                    filename = anchor_name + '_' + now + ".mp4"
                                    print(f'{rec_info}/{filename}')
                                    file = full_path + '/' + filename

                                    if create_time_file:
                                        filename_gruop = [anchor_name, filename_short]
                                        create_var[str(filename_short)] = threading.Thread(target=create_ass_file,
                                                                                           args=(filename_gruop,))
                                        create_var[str(filename_short)].daemon = True
                                        create_var[str(filename_short)].start()

                                    try:
                                        command = [
                                            "-map", "0",
                                            "-c:v", "copy",  # 直接用copy的话体积特别大.
                                            "-f", "mp4",
                                            "{path}".format(path=file),
                                        ]
                                        ffmpeg_command.extend(command)
                                        _output = subprocess.check_output(ffmpeg_command, stderr=subprocess.STDOUT)

                                        # 取消http_proxy环境变量设置
                                        # if proxy_addr:
                                        #     del os.environ["http_proxy"]

                                        record_finished = True
                                        record_finished_2 = True
                                        count_time = time.time()

                                    except subprocess.CalledProcessError as e:
                                        # logging.warning(str(e.output))
                                        print(str(e.output) + " 发生错误的行数: " + str(e.__traceback__.tb_lineno))
                                        logger.warning(
                                            "错误信息: " + str(e) + " 发生错误的行数: " + str(e.__traceback__.tb_lineno))
                                    if anchor_name in recording:
                                        recording.remove(anchor_name)
                                    if anchor_name in unrecording:
                                        unrecording.add(anchor_name)

                                elif video_save_type == "MKV音频":
                                    filename = anchor_name + '_' + now + ".mkv"
                                    print(f'{rec_info}/{filename}')
                                    file = full_path + '/' + filename

                                    try:
                                        command = [
                                            "-map", "0:a", # 不同点
                                            "-f", "matroska", # 不同点
                                            "{path}".format(path=file),
                                        ]
                                        ffmpeg_command.extend(command)
                                        _output = subprocess.check_output(ffmpeg_command, stderr=subprocess.STDOUT)

                                        record_finished = True
                                        record_finished_2 = True
                                        count_time = time.time()

                                        if tsconvert_to_m4a:
                                            threading.Thread(target=converts_m4a, args=(file,)).start()
                                    except subprocess.CalledProcessError as e:
                                        # logging.warning(str(e.output))
                                        print(str(e.output) + " 发生错误的行数: " + str(e.__traceback__.tb_lineno))
                                        logger.warning(
                                            "错误信息: " + str(e) + " 发生错误的行数: " + str(e.__traceback__.tb_lineno))
                                    if anchor_name in recording:
                                        recording.remove(anchor_name)
                                    if anchor_name in unrecording:
                                        unrecording.add(anchor_name)

                                elif video_save_type == "TS音频":
                                    filename = anchor_name + '_' + now + ".ts"
                                    print(f'{rec_info}/{filename}')
                                    file = full_path + '/' + filename

                                    try:
                                        command = [
                                            "-map", "0:a",  # 不同点
                                            "-f", "mpegts",
                                            "{path}".format(path=file),
                                        ]
                                        ffmpeg_command.extend(command)
                                        _output = subprocess.check_output(ffmpeg_command, stderr=subprocess.STDOUT)

                                        record_finished = True
                                        record_finished_2 = True
                                        count_time = time.time()

                                        if tsconvert_to_m4a:
                                            threading.Thread(target=converts_m4a, args=(file,)).start()
                                    except subprocess.CalledProcessError as e:
                                        # logging.warning(str(e.output))
                                        print(str(e.output) + " 发生错误的行数: " + str(e.__traceback__.tb_lineno))
                                        logger.warning(
                                            "错误信息: " + str(e) + " 发生错误的行数: " + str(e.__traceback__.tb_lineno))
                                    if anchor_name in recording:
                                        recording.remove(anchor_name)
                                    if anchor_name in unrecording:
                                        unrecording.add(anchor_name)

                                else:

                                    if Splitvideobysize:  # 这里默认是启用/不启用视频分割功能
                                        while True:
                                            now = time.strftime("%Y-%m-%d-%H-%M-%S", time.localtime(time.time()))
                                            filename = anchor_name + '_' + now + ".ts"
                                            print(f'{rec_info}/{filename}')
                                            file = full_path + '/' + filename

                                            if create_time_file:
                                                filename_gruop = [anchor_name, filename_short]
                                                create_var[str(filename_short)] = threading.Thread(
                                                    target=create_ass_file,
                                                    args=(filename_gruop,))
                                                create_var[str(filename_short)].daemon = True
                                                create_var[str(filename_short)].start()

                                            try:
                                                command = [
                                                    "-c:v", "copy",
                                                    "-map", "0",  # 不同点
                                                     "-f", "mpegts",
                                                    "-fs", str(Splitsizes),  # 不同点
                                                    "{path}".format(path=file),
                                                ]

                                                ffmpeg_command.extend(command)
                                                _output = subprocess.check_output(ffmpeg_command,
                                                                                  stderr=subprocess.STDOUT)

                                                record_finished = True  # 这里表示正常录制成功一次
                                                record_finished_2 = True
                                                count_time = time.time()  # 这个记录当前时间, 用于后面 1分钟内快速2秒循环 这个值不能放到后面

                                                if tsconvert_to_mp4:
                                                    threading.Thread(target=converts_mp4, args=(file,)).start()
                                                if tsconvert_to_m4a:
                                                    threading.Thread(target=converts_m4a, args=(file,)).start()
                                                if anchor_name in recording:
                                                    recording.remove(anchor_name)
                                                if anchor_name in unrecording:
                                                    unrecording.add(anchor_name)
                                            except subprocess.CalledProcessError as e:
                                                # logging.warning(str(e.output))
                                                # logger.warning("错误信息: "+str(e)  +" 发生错误的行数: "+str(e.__traceback__.tb_lineno))
                                                if anchor_name in recording:
                                                    recording.remove(anchor_name)
                                                if anchor_name in unrecording:
                                                    unrecording.add(anchor_name)
                                                break


                                    else:
                                        filename = anchor_name + '_' + now + ".ts"

                                        print(f'{rec_info}/{filename}')
                                        file = full_path + '/' + filename

                                        if create_time_file:
                                            filename_gruop = [anchor_name, filename_short]
                                            create_var[str(filename_short)] = threading.Thread(target=create_ass_file,
                                                                                               args=(filename_gruop,))
                                            create_var[str(filename_short)].daemon = True
                                            create_var[str(filename_short)].start()

                                        try:
                                            command = [
                                                "-c:v", "copy",
                                                "-map", "0",  # 不同点
                                                "-f", "mpegts",
                                                "{path}".format(path=file),
                                            ]

                                            ffmpeg_command.extend(command)
                                            _output = subprocess.check_output(ffmpeg_command,
                                                                              stderr=subprocess.STDOUT)
                                            record_finished = True
                                            record_finished_2 = True
                                            count_time = time.time()

                                            if tsconvert_to_mp4:
                                                threading.Thread(target=converts_mp4, args=(file,)).start()
                                            if tsconvert_to_m4a:
                                                threading.Thread(target=converts_m4a, args=(file,)).start()


                                        except subprocess.CalledProcessError as e:
                                            # logging.warning(str(e.output))
                                            print(str(e.output) + " 发生错误的行数: " + str(e.__traceback__.tb_lineno))
                                            logger.warning(
                                                "错误信息: " + str(e) + " 发生错误的行数: " + str(e.__traceback__.tb_lineno))
                                        if anchor_name in recording:
                                            recording.remove(anchor_name)
                                        if anchor_name in unrecording:
                                            unrecording.add(anchor_name)

                            if record_finished_2 == True:
                                if anchor_name in recording:
                                    recording.remove(anchor_name)
                                if anchor_name in unrecording:
                                    unrecording.add(anchor_name)
                                print('\n' + anchor_name + " " + time.strftime('%Y-%m-%d %H:%M:%S  ') + '直播录制完成\n')
                                record_finished_2 = False

                except Exception as e:
                    print(
                        "错误信息644:" + str(e) + "\r\n读取的地址为: " + str(record_url) + " 发生错误的行数: " + str(e.__traceback__.tb_lineno))
                    logger.warning("错误信息: " + str(e) + " 发生错误的行数: " + str(e.__traceback__.tb_lineno))
                    warning_count += 1

                num = random.randint(-5, 5) + delay_default  # 生成-5到5的随机数，加上delay_default
                if num < 0:  # 如果得到的结果小于0，则将其设置为0
                    num = 0
                x = num

                # 如果出错太多,就加秒数
                if warning_count > 100:
                    x = x + 60
                    print("瞬时错误太多,延迟加60秒")

                # 这里是.如果录制结束后,循环时间会暂时变成30s后检测一遍. 这样一定程度上防止主播卡顿造成少录
                # 当30秒过后检测一遍后. 会回归正常设置的循环秒数
                if record_finished == True:
                    count_time_end = time.time() - count_time
                    if count_time_end < 60:
                        x = 30
                    record_finished = False

                else:
                    x = num

                # 这里是正常循环
                while x:
                    x = x - 1
                    # print('\r循环等待%d秒 '%x)
                    if loop_time:
                        print('\r' + anchor_name + ' 循环等待%d秒 ' % x, end="")
                    time.sleep(1)
                if loop_time:
                    print('\r检测直播间中...', end="")
        except Exception as e:
            print("错误信息644:" + str(e) + "\r\n发生错误的行数: " + str(e.__traceback__.tb_lineno))
            logger.warning("错误信息: " + str(e) + " 发生错误的行数: " + str(e.__traceback__.tb_lineno))
            print("线程崩溃2秒后重试.错误信息: " + str(e) + " 发生错误的行数: " + str(e.__traceback__.tb_lineno))
            warning_count += 1
            time.sleep(2)


def check_md5(file_path):
    """
    计算文件的md5值
    """
    with open(file_path, 'rb') as fp:
        file_md5 = hashlib.md5(fp.read()).hexdigest()
    return file_md5


def backup_file(file_path, backup_dir):
    """
    备份配置文件到备份目录
    """
    try:
        if not os.path.exists(backup_dir):
            os.makedirs(backup_dir)
        # 拼接备份文件名，年-月-日-时-分-秒
        timestamp = datetime.datetime.now().strftime('%Y-%m-%d-%H-%M-%S')
        backup_file_name = os.path.basename(file_path) + '_' + timestamp
        # 拷贝文件到备份目录
        backup_file_path = os.path.join(backup_dir, backup_file_name)
        shutil.copy2(file_path, backup_file_path)
        print(f'\r已备份配置文件 {file_path} 到 {backup_file_path}')
    except Exception as e:
        print(f'\r备份配置文件 {file_path} 失败：{str(e)}')


def backup_file_start():

    config_md5 = ''
    url_config_md5 = ''

    while True:
        try:
            if os.path.exists(config_file):
                new_config_md5 = check_md5(config_file)
                if new_config_md5 != config_md5:
                    backup_file(config_file, backup_dir)
                    config_md5 = new_config_md5

            if os.path.exists(url_config_file):
                new_url_config_md5 = check_md5(url_config_file)
                if new_url_config_md5 != url_config_md5:
                    backup_file(url_config_file, backup_dir)
                    url_config_md5 = new_url_config_md5
            time.sleep(60)  # 每1分钟检测一次文件是否有修改
        except Exception as e:
            print(f'执行脚本异常：{str(e)}')


# --------------------------检测是否存在ffmpeg-------------------------------------
ffmepg_file_check = subprocess.getoutput(["ffmpeg"])
if ffmepg_file_check.find("run") > -1:
    # print("ffmpeg存在")
    pass
else:
    print("重要提示:")
    print("检测到ffmpeg不存在,请将ffmpeg.exe放到同目录,或者设置为环境变量,没有ffmpeg将无法录制")

# --------------------------初始化程序-------------------------------------
print('--------------- 抖音直播录制 程序当前配置-----------------')
print(f"版本号：{version}")
print(f"作者：Hmily")
print('......................................................')

if not os.path.exists('./config'):
    os.makedirs('./config')

# 备份配置
t3 = threading.Thread(target=backup_file_start, args=(), daemon=True)
t3.start()
Monitoring = 0

# 如果开启了电脑全局/规则代理，可以不用再在配置文件中填写代理地址
try:
    # 检测电脑是否开启了全局/规则代理
    print('系统代理检测中...')
    response_g = urllib.request.urlopen("https://www.tiktok.com/", timeout=5)
    global_proxy = True
    print('系统代理已开启√ 注意：配置文件中的代理设置也要开启才会生效哦！')

except Exception as e:
    print('INFO：未检测到网络代理，请检查代理是否生效（若无需录制Tiktok直播请忽略此条提示）')


def read_config_value(config, section, option, default_value):
    try:
        config.read(config_file, encoding=encoding)
        if '1' not in config.sections():
            config.add_section('1')
        return config.get(section, option)
    except (configparser.NoSectionError, configparser.NoOptionError):
        config.set(section, option, str(default_value))
        with open(config_file, 'w', encoding=encoding) as f:
            config.write(f)
        return default_value


while True:
    # 循环读取配置
    config = configparser.RawConfigParser()

    try:
        with open(config_file, 'r', encoding=encoding) as f:
            config.read_file(f)
    except IOError:
        with open(config_file, 'w', encoding=encoding) as f:
            pass

    if os.path.isfile(url_config_file):
        with open(url_config_file, 'r', encoding=encoding) as f:
            inicontent = f.read()
    else:
        inicontent = ""

    if len(inicontent) == 0:
        inurl = input('请输入要录制的抖音主播的直播间网址（尽量使用PC网页端的直播间地址）:\n')
        with open(url_config_file, 'a+', encoding=encoding) as f:
            f.write(inurl)

    video_save_path = read_config_value(config, '1', '直播保存路径（不填则默认）', "")
    video_save_type = read_config_value(config, '1', '视频保存格式TS|MKV|FLV|MP4|TS音频|MKV音频', "MP4")
    video_quality = read_config_value(config, '1', '原画|超清|高清|标清', "原画")
    use_proxy = read_config_value(config, '1', '是否使用代理ip（是/否）', "否")
    proxy_addr = read_config_value(config, '1', '代理地址', "")  # 暂时没有用到
    max_request = int(read_config_value(config, '1', '同一时间访问网络的线程数', 3))
    semaphore = threading.Semaphore(max_request)
    delay_default = int(read_config_value(config, '1', '循环时间(秒)', 60))
    local_delay_default = int(read_config_value(config, '1', '排队读取网址时间(秒)', 0))
    video_m3u8 = read_config_value(config, '1', '是否显示直播地址', "否")
    loop_time = read_config_value(config, '1', '是否显示循环秒数', "否")
    Splitvideobysize = read_config_value(config, '1', 'TS格式分段录制是否开启', "否")
    Splitsize = int(read_config_value(config, '1', '视频分段大小(兆)', '1000'))
    tsconvert_to_mp4 = read_config_value(config, '1', 'TS录制完成后自动增加生成MP4格式', "否")
    tsconvert_to_m4a = read_config_value(config, '1', 'TS录制完成后自动增加生成m4a格式', "否")
    delFilebeforeconversion = read_config_value(config, '1', '追加格式后删除原文件', "否")
    create_time_file = read_config_value(config, '1', '生成时间文件', "否")
    cookies_set = read_config_value(config, '1', 'cookies（不可为空）', '')

    if len(video_save_type) > 0:
        if video_save_type.upper().lower() == "FLV".lower():
            video_save_type = "FLV"
            # print("直播视频保存为FLV格式")
        elif video_save_type.upper().lower() == "MKV".lower():
            video_save_type = "MKV"
            # print("直播视频保存为MKV格式")
        elif video_save_type.upper().lower() == "TS".lower():
            video_save_type = "TS"
            # print("直播视频保存为TS格式")
        elif video_save_type.upper().lower() == "MP4".lower():
            video_save_type = "MP4"
            # print("直播视频保存为MP4格式")
        elif video_save_type.upper().lower() == "TS音频".lower():
            video_save_type = "TS音频"
            # print("直播视频保存为TS音频格式")
        elif video_save_type.upper().lower() == "MKV音频".lower():
            video_save_type = "MKV音频"
            # print("直播视频保存为MKV音频格式")
        else:
            video_save_type = "TS"
            print("直播视频保存格式设置有问题,这次录制重置为默认的TS格式")
    else:
        video_save_type = "TS"
        print("直播视频保存为TS格式")

    # 这里是控制TS分段大小
    if Splitsize < 5:
        Splitsize = 5  # 分段大小最低不能小于5m
    Splitsizes = Splitsize * 1024 * 1024  # 分割视频大小,转换为字节


    def tranform_int_to_time(seconds):
        m, s = divmod(seconds, 60)
        h, m = divmod(m, 60)
        return ("%d:%02d:%02d" % (h, m, s))

    options = {
        "是": True,
        "否": False
    }
    use_proxy = options.get(use_proxy, False)  # 是否使用代理ip
    video_m3u8 = options.get(video_m3u8, False)  # 是否显示直播地址
    loop_time = options.get(loop_time, False)  # 是否显示循环秒数
    Splitvideobysize = options.get(Splitvideobysize, False)  # 这里是控制TS是否分段
    create_time_file = options.get(create_time_file, False)  # 这里控制是否生成时间文件
    delFilebeforeconversion = options.get(delFilebeforeconversion, False)  # 追加格式后,是否删除原文件
    tsconvert_to_m4a = options.get(tsconvert_to_m4a, False)  # 这里是控制TS是否追加m4a格式
    tsconvert_to_mp4 = options.get(tsconvert_to_mp4, False)  # 这里是控制TS是否追加mp4格式


    # 读取url_config.ini文件
    try:
        with open(url_config_file, "r", encoding=encoding) as file:
            for line in file:
                line = line.strip()
                if line.startswith("#") or len(line) < 20:
                    continue

                if re.search('[,，]', line):
                    split_line = re.split('[,，]', line)
                else:
                    split_line = [line, '']
                url = split_line[0]
                url_host=url.split('/')[2]
                host_list=['live.douyin.com','v.douyin.com','www.tiktok.com','live.kuaishou.com']
                if url_host in host_list:
                    new_line = (url, split_line[1])
                    url_tuples_list.append(new_line)
                else:
                    print(f"{url} 未知链接.此条跳过")

        while len(name_list):
            a = name_list.pop()
            replacewords = a.split('|')
            if replacewords[0] != replacewords[1]:
                update_file(url_config_file, replacewords[0], replacewords[1])

        # print('url_tuples_list：',url_tuples_list)
        if len(url_tuples_list) > 0:
            textNoRepeatUrl = list(set(url_tuples_list))
        if len(textNoRepeatUrl) > 0:
            for url_tuple in textNoRepeatUrl:
                if url_tuple[0] in not_record_list:
                    continue

                if url_tuple[0] not in runing_list:
                    if first_start == False:
                        print("新增链接: " + url_tuple[0])
                    Monitoring = Monitoring + 1
                    args = [url_tuple, Monitoring]
                    # TODO: 执行开始录制的操作
                    create_var['thread' + str(Monitoring)] = threading.Thread(target=start_record, args=args)
                    create_var['thread' + str(Monitoring)].daemon = True
                    create_var['thread' + str(Monitoring)].start()
                    runing_list.append(url_tuple[0])
                    time.sleep(local_delay_default)
        url_tuples_list = []
        first_start = False

    except Exception as e:
        print("错误信息644:" + str(e) + "\r\n发生错误的行数: " + str(e.__traceback__.tb_lineno))
        logger.warning("错误信息: " + str(e) + " 发生错误的行数: " + str(e.__traceback__.tb_lineno))

    # 这个是第一次运行其他线程.因为有变量前后顺序的问题,这里等全局变量完成后再运行def函数
    if firstRunOtherLine:
        t = threading.Thread(target=display_info, args=(), daemon=True)
        t.start()
        t2 = threading.Thread(target=change_max_connect, args=(), daemon=True)
        t2.start()

        firstRunOtherLine = False

    # 总体循环3s
    time.sleep(3)