// qwfy/rust/video_monitor/src/common/douyin_fetch.rs

use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT, COOKIE, ACCEPT, ACCEPT_LANGUAGE, CACHE_CONTROL, PRAGMA, UPGRADE_INSECURE_REQUESTS};

pub async fn fetch_url(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36"));
    headers.insert(ACCEPT, HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7"));
    headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("zh-CN,zh;q=0.9,en;q=0.8,zh-TW;q=0.7,ja;q=0.6"));
    headers.insert(CACHE_CONTROL, HeaderValue::from_static("no-cache"));
    headers.insert(PRAGMA, HeaderValue::from_static("no-cache"));
    headers.insert(UPGRADE_INSECURE_REQUESTS, HeaderValue::from_static("1"));
    headers.insert(COOKIE, HeaderValue::from_static("douyin.com; ttwid=1%7CcQzm1t4NhUgmmnoaO0MDobLGVWS5u7ZRVIpv7IRgYzg%7C1708924897%7C87d87217eeed2873c827299b60973aa37dba5e6275aae5b61c72142105a356e3; dy_swidth=1440; dy_sheight=900; FORCE_LOGIN=%7B%22videoConsumedRemainSeconds%22%3A180%7D; bd_ticket_guard_client_web_domain=2; passport_csrf_token=8bb2babf352a5313a2f60a0c7f4b637e; passport_csrf_token_default=8bb2babf352a5313a2f60a0c7f4b637e; douyin.com; device_web_cpu_core=8; device_web_memory_size=8; webcast_local_quality=null; live_use_vvc=%22false%22; odin_tt=ec0fc2db211a346d04bfa02b53c4a5f6c5a2ce01d4eda9a08b9b22eea74f93e686bb35e2241159f3d2af199ed9a0f69b10ec2ed4623af5470c9e3a8457722ebc356533d059911e1c5cd786ea75bc9334; s_v_web_id=verify_lt3vcqwl_911b53ff_cc14_a41c_204d_5d4fbbe4935f; xgplayer_device_id=7604811730; xgplayer_user_id=3761803984; volume_info=%7B%22isUserMute%22%3Afalse%2C%22isMute%22%3Atrue%2C%22volume%22%3A0.5%7D; SEARCH_RESULT_LIST_TYPE=%22single%22; download_guide=%223%2F20240305%2F1%22; __live_version__=%221.1.1.8561%22; webcast_leading_last_show_time=1709775865995; webcast_leading_total_show_times=4; live_can_add_dy_2_desktop=%221%22; __ac_nonce=065eaa0f100311bc760e1; __ac_signature=_02B4Z6wo00f01PItiBQAAIDDoNKoDXhA8VjyDYyAAFlnk88Dt9ofZcXHNonu7fzW.uGhaiUsXgxKDey0StgAQaYHi4YZJROn07xzXC-VDIHkdpsB-rdaNibCUzGI.vIU4mkpy3L2N6dQbqD10c; xg_device_score=7.658235294117647; strategyABtestKey=%221709875444.31%22; stream_player_status_params=%22%7B%5C%22is_auto_play%5C%22%3A0%2C%5C%22is_full_screen%5C%22%3A0%2C%5C%22is_full_webscreen%5C%22%3A0%2C%5C%22is_mute%5C%22%3A1%2C%5C%22is_speed%5C%22%3A1%2C%5C%22is_visible%5C%22%3A0%7D%22; stream_recommend_feed_params=%22%7B%5C%22cookie_enabled%5C%22%3Atrue%2C%5C%22screen_width%5C%22%3A1440%2C%5C%22screen_height%5C%22%3A900%2C%5C%22browser_online%5C%22%3Atrue%2C%5C%22cpu_core_num%5C%22%3A8%2C%5C%22device_memory%5C%22%3A8%2C%5C%22downlink%5C%22%3A9.7%2C%5C%22effective_type%5C%22%3A%5C%224g%5C%22%2C%5C%22round_trip_time%5C%22%3A200%7D%22; bd_ticket_guard_client_data=eyJiZC10aWNrZXQtZ3VhcmQtdmVyc2lvbiI6MiwiYmQtdGlja2V0LWd1YXJkLWl0ZXJhdGlvbi12ZXJzaW9uIjoxLCJiZC10aWNrZXQtZ3VhcmQtcmVlLXB1YmxpYy1rZXkiOiJCRll2WGhpWXJ3anNoRTdqUU1RWWlkY2FHU0JEcmZyQ0xwQTJheE5ZY1FTZ1B3N3Q4bkR0bHRSRWU0YjJ6emtXWlRqQlFnOTRvZHp2SnBKZGptWW04RXc9IiwiYmQtdGlja2V0LWd1YXJkLXdlYi12ZXJzaW9uIjoxfQ%3D%3D; home_can_add_dy_2_desktop=%221%22; msToken=htYQFp2w040GXiCY1ZOQupxpmQBIEURKrNj_TGtFB1vEaqypB8cvLgoVBTOmxRwwi593sAv_V3GMMyEGaoI3-W0P3ai2LetfG08pgc09VvKODi5Yld_eUg==; tt_scid=EA2UFeAwjsVLcLhyamXWxjsi9KXpRpVw8FHNNnuKBIPmMyYWvYewImsp0NHDt9ye88f6; msToken=z7nDpRJIR8SBh2Uk1K4sY4Aw6TqR_uYlQzEO9FD2C060Wt6kSIrGXnB7dypIIh7Uf26f1QRUpe7O9oU09tRbCw5ukMCgRHAwSfXPNaEdaESegfY_3lRQdA==; pwa2=%220%7C0%7C2%7C0%22; IsDouyinActive=false"));

    let client = reqwest::Client::new();
    let response = client.get(url)
        .headers(headers)
        .send()
        .await?;

    let content = response.text().await?;
    Ok(content)
}
