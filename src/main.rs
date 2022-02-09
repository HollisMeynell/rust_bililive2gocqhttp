use std::collections::HashMap;
use std::error;
use std::process::id;
use std::ptr::null;
use std::thread::{sleep, Thread};
use std::time::Duration;
use reqwest;
use serde::{Deserialize, Serialize};
use serde::de::Unexpected::Option;
use serde::ser::StdError;

const UID: u64 = 4995808;
const GROUP: u64 = 446316073;
const API: &str = "http://api.live.bilibili.com/room/v1/Room/get_status_info_by_uids";

const QQ_API: &str = "http://localhost:5700/send_group_msg";

#[tokio::main]
async fn main() {
    println!("start");
    let post = get_default();
    let mut last_state = RoomState::Close;
    loop {
        sleep(Duration::new(63, 18));
        let rom = post.get_all_room().await;
        match rom {
            Ok(map) => {
                for (id, room) in map.into_iter() {
                    println!("{:?}", &room);
                    let now_state = room.get_state();
                    if !now_state.eq(&last_state) {
                        last_state = now_state;
                        if last_state.eq(&RoomState::Open) {
                            println!("start!!!");
                            let data = get_open_message(&room);
                            do_post(data);
                        } else {
                            println!("close!!!");
                            let data = get_close_message(&room);
                            do_post(data);
                        }
                    }
                }
            }
            Err(e) => {
                println!("{:?}", &e);
            }
        }
    }
}

fn get_open_message(room: &Room) -> HashMap<String, String> {
    let mut map = HashMap::new();
    map.insert("group_id".to_string(), GROUP.to_string());
    let mut msg = String::new();
    let mid =
        if room.short_id != 0 { room.short_id.to_string() } else { room.room_id.to_string() };
    msg = msg + &room.uname + " 爷爷开始直播了!" + "\n";
    msg = msg + &room.title + "-" + &room.area_name + "\n";
    msg = msg + "快戳我围观!->https://live.bilibili.com/" + &mid + "\n";
    map.insert("message".to_string(), msg.clone());
    return map;
}

fn get_close_message(room: &Room) -> HashMap<String, String> {
    let mut map = HashMap::new();
    map.insert("group_id".to_string(), GROUP.to_string());
    let mut msg = String::new();
    let mid =
        if room.short_id != 0 { room.short_id.to_string() } else { room.room_id.to_string() };
    msg = msg + &room.uname + " 爷爷的直播结束了>_<" + "\n";
    msg = msg + &room.title + "-" + &room.area_name + "\n";
    msg = msg + "快戳我关注直播间!->https://live.bilibili.com/" + &mid + "\n";
    map.insert("message".to_string(), msg.clone());
    return map;
}

async fn do_post(data: HashMap<String, String>) {
    let client = reqwest::Client::new();
    client.post(QQ_API)
        .json(&data)
        .send()
        .await;
}

#[derive(PartialEq)]
enum RoomState {
    Close,
    Open,
    Pass,
}

impl RoomState {
    fn eq(&self, other: &RoomState) -> bool {
        match (self, other) {
            (RoomState::Close, RoomState::Close) => {
                true
            }
            (RoomState::Open, RoomState::Open) => {
                true
            }
            (RoomState::Pass, RoomState::Pass) => {
                true
            }
            _ => { false }
        }
    }
}

struct PostData {
    uid: u64,
    group: u64,
    all_room_api: String,
    client: reqwest::Client,
}

impl PostData {
    async fn get_all_room(&self) -> Result<(HashMap<String, Room>), &str> {
        let mut map = HashMap::new();
        map.insert("uids", [UID]);/*, 545149341, 14172231*/

        let res = self.client.post(&self.all_room_api)
            .json(&map)
            .send()
            .await;
        let mut package;
        match res {
            Ok(res) => {
                match res.json::<ResponseDate>().await {
                    Ok(pack) => { package = pack; }
                    Err(e) => {
                        println!("{:?}", e);
                        return Err("json error");
                    }
                };
            }
            Err(e) => {
                println!("{:?}", e);
                return Err("post error");
            }
        }

        if package.code == 0 {
            let mut room_data = package.data;
            Ok(room_data)
        } else {
            Err("biliapi request error")
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
struct ResponseDate {
    code: u8,
    message: String,
    msg: String,
    data: HashMap<String, Room>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Room {
    //live_status 0：未开播 1：直播中 2：轮播中   live_time 秒时间戳
    title: String,
    uname: String,
    area_name: String,
    cover_from_user: String,
    keyframe: String,
    room_id: u64,
    online: u32,
    live_time: u64,
    live_status: u8,
    short_id: u32,
}

impl Room {
    fn get_state(&self) -> RoomState {
        return match self.live_status {
            0 => { RoomState::Close }
            1 => { RoomState::Open }
            _ => { RoomState::Pass }
        };
    }
    fn is_open(&self) -> bool {
        self.live_status == 1
    }
}

fn get_default() -> PostData {
    PostData {
        uid: UID,
        group: GROUP,
        all_room_api: API.to_string(),
        client: reqwest::Client::new(),
    }
}

