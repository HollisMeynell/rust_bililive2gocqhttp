use std::collections::HashMap;
use std::{error, io};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::num::ParseIntError;
use std::process::id;
use std::ptr::null;
use std::sync::Mutex;
use std::thread::{sleep, Thread};
use std::time::Duration;
use lazy_static::lazy_static;
use reqwest;
use reqwest::{Error, Response};
use serde::{Deserialize, Serialize};
use serde::de::Unexpected::{Map, Option};

lazy_static! {
    static ref CONFIG_MAP: Mutex<HashMap<i64, Vec<i64>>> = Mutex::new(HashMap::new());
    static ref LIVER_MAP: Mutex<HashMap<i64, Vec<i64>>> = Mutex::new(HashMap::new());
    static ref LIVER_VEC: Mutex<Vec<i64>> = Mutex::new(Vec::new());
    static ref LIVE_STATUS_MAP: Mutex<HashMap<i64, RoomState>> = Mutex::new(HashMap::new());
}
fn init_config(path: &str) -> Result<i8, io::Error> {
    let file: File = File::open(path)?;
    let buffered: BufReader<File> = BufReader::new(file);
    let mut key: String = String::new();
    let mut new_line: String = String::new();

    for line in buffered.lines().map(|x| x.unwrap()) {
        new_line.clear();
        new_line.push_str(line.trim());
        // 定义注释为`#`, 遇到注释跳过
        if line.contains("#") {
            continue;
        } else if line.contains("[") && line.contains("]") { // 解析`[key]`为`key::`
            key.clear();
            new_line.pop();
            new_line.remove(0);
            key.push_str(new_line.as_str().trim());
        } else if let Ok(key) = key.parse::<i64>() {
            if let Ok(mut group_map) = CONFIG_MAP.lock(){
                let kvs: Vec<&str> = new_line.as_str().split(",").collect::<Vec<&str>>();
                if let Some(mut groups) = group_map.get_mut(&key) {
                    for s in kvs {
                        if let Ok(i) = s.parse::<i64>(){
                            groups.push(i);
                        }
                    }
                }else {
                    let mut values: Vec<i64> = Vec::new();
                    for s in kvs {
                        if let Ok(i) = s.parse::<i64>(){
                            values.push(i);
                        }
                    }
                    group_map.insert(key, values);
                }

            }
        }
    }
    for (key, value) in CONFIG_MAP.lock().unwrap().iter() {
        println!("k = {}, y = {:?}\n", key, value);
    }
    for (group_id, rooms) in CONFIG_MAP.lock().unwrap().iter() {
        for rooms_id_tmp in rooms {
            if let Ok(mut liver_map) = LIVER_MAP.lock() {
                if let Some(groups_tmp) = liver_map.get_mut(rooms_id_tmp) {
                    if !groups_tmp.contains(group_id) {
                        groups_tmp.push(group_id.clone());
                    }
                } else {
                    let mut new_groups_tmp: Vec<i64> = Vec::new();
                    new_groups_tmp.push(group_id.clone());
                    liver_map.insert(rooms_id_tmp.clone(), new_groups_tmp);
                }
            }
        }
    }
    for (key, value) in LIVER_MAP.lock().unwrap().iter() {
        println!("k = {}, y = {:?}", key, value);
    }
    for (room_id, _) in LIVER_MAP.lock().unwrap().iter() {
        let mut live_status = LIVE_STATUS_MAP.lock().unwrap();
        live_status.insert(room_id.clone(), RoomState::Close);
        LIVER_VEC.lock().unwrap().push(room_id.clone());
    }

    println!("load pass");
    return Ok(0);
}

const API: &str = "http://api.live.bilibili.com/room/v1/Room/get_status_info_by_uids";
const QQ_API: &str = "http://localhost:5700/send_group_msg";

#[tokio::main]
async fn main() {
    println!("start");

    if let Err(e) = init_config("init.conf") {
        println!("err = {:?}", e);
    }
    let client = reqwest::Client::new();
    loop {
        if let Ok(l) = LIVER_VEC.lock() {
            if l.len() == 0 {
                return;
            }
        }
        let rom = get_all_room(&client).await;
        match rom {
            Ok(map) => {
                println!("ok");
                do_send_qq(&map).await;
            }
            Err(e) => {
                println!("{:?}", &e);
            }
        }
        sleep(Duration::new(63, 18));
    }
}

async fn do_send_qq(map: &HashMap<String, Room>) {
    for (id, room) in map.into_iter() {
        let room_id: i64;
        if let Ok(i) = id.parse::<i64>() {
            room_id = i.clone();
        } else { continue; }
        let now_state = room.get_state();
        if room_status_change(&room_id, &now_state) {
            room_status_set(&room_id, &now_state);
            if now_state.eq(&RoomState::Open) {
                println!("{}start!!!", &room_id);
                if let Ok(liver_map) = LIVER_MAP.lock() {
                    if let Some(groups_tmp) = liver_map.get(&room_id) {
                        for group_id_temp in groups_tmp {
                            let data = get_open_message(&room, &group_id_temp);
                            do_post(data).await;
                        }
                    }
                }
            } else {
                println!("{}close!!!", &room_id);
                if let Ok(liver_map) = LIVER_MAP.lock() {
                    if let Some(groups_tmp) = liver_map.get(&room_id) {
                        for group_id_temp in groups_tmp {
                            let data = get_close_message(&room, &group_id_temp);
                            do_post(data).await;
                        }
                    }
                }
            }
        }
    }
}

fn room_status_change(room_id: &i64, room_status: &RoomState) -> bool {
    if let Ok(mut map) = LIVE_STATUS_MAP.lock() {
        return if let Some(room) = map.get(room_id) {
            !room.eq(room_status)
        } else {
            map.insert(room_id.clone(), room_status.clone());
            false
        };
    }
    return false;
}

fn room_status_set(room_id: &i64, room_status: &RoomState) {
    if let Ok(mut map) = LIVE_STATUS_MAP.lock() {
        map.insert(room_id.clone(), room_status.clone());
    }
}

fn get_open_message(room: &Room, group_id: &i64) -> HashMap<String, String> {
    let mut map = HashMap::new();
    map.insert("group_id".to_string(), group_id.to_string());
    let mut msg = String::new();
    let mid =
        if room.short_id != 0 { room.short_id.to_string() } else { room.room_id.to_string() };
    msg = msg + &room.uname + " 爷爷开始直播了!" + "\n";
    msg = msg + &room.title + "-" + &room.area_name + "\n";
    msg = msg + "快戳我围观!->https://live.bilibili.com/" + &mid + "\n";
    //msg = msg + "[CQ:image,url="+&room.keyframe+",type=show,id=40000]\n";
    map.insert("message".to_string(), msg.clone());
    return map;
}

fn get_close_message(room: &Room, group_id: &i64) -> HashMap<String, String> {
    let mut map = HashMap::new();
    map.insert("group_id".to_string(), group_id.to_string());
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
    println!("send");
    let n = client.post(QQ_API)
        .json(&data)
        .send();
    match n.await {
        Ok(a) => {
            if let Ok(p) = a.text().await {
                println!("qq 送达{} ->\n {}", &data.get("group_id").unwrap(), &p);
            }
        }

        Err(e) => {
            println!("qq 发送失败{:?}", e);
        }
    }
}

#[derive(PartialEq, Clone)]
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



async fn get_all_room(client: &reqwest::Client) -> Result<(HashMap<String, Room>), &str> {
    let mut map = HashMap::new();
    let mut uids: Vec<i64> = Vec::new();
    if let Ok(liver_id) = LIVER_VEC.lock() {
        uids = liver_id.clone();
    }
    println!("{:?}", uids);
    map.insert("uids".clone(), uids);/*, 545149341, 14172231*/

    let res = client.post(API)
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

