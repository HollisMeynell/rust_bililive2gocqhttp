pub mod postbili{
    use std::collections::HashMap;
    use std::fs::File;
    use std::future::Future;
    use std::io::{BufRead, BufReader};
    use serde::{Deserialize, Serialize};
    use std::thread;
    use std::thread::sleep;
    use std::time::Duration;
    use futures::future::{join_all, ok, err};

    const API: &str = "http://api.live.bilibili.com/room/v1/Room/get_status_info_by_uids";
    const QQ_API: &str = "http://localhost:5700/send_group_msg";


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
    #[derive(PartialEq, Clone)]
    enum RoomState {
        Close,
        Open,
        Pass,
    }
    #[derive(Deserialize, Serialize, Debug)]
    struct ResponseDate {
        code: u8,
        message: String,
        msg: String,
        data: HashMap<String, Room>,
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
    pub struct RunDate{
        CONFIG_MAP: HashMap<i64, Vec<i64>>,
        LIVER_MAP: HashMap<i64, Vec<i64>>,
        LIVER_VEC: Vec<i64>,
        LIVE_STATUS_MAP: HashMap<i64, RoomState>
    }
    impl RunDate {
        fn init(&mut self, path: &str){
            let file: File = File::open(path)?;
            let buffered: BufReader<File> = BufReader::new(file);
            let mut key: String = String::new();
            let mut new_line: String = String::new();
            //按行读取配置文件
            for line in buffered.lines().map(|x| x.unwrap()){
                new_line.clear();
                new_line.push_str(line.trim());

                if line.contains("#") { // 注释跳过
                    continue;
                } else if line.contains("[") && line.contains("]") { // 解析`[key]`为`key::`
                    key.clear();
                    new_line.pop();
                    new_line.remove(0);
                } else if let Ok(key) = key.parse::<i64>() { //解析值 value
                    let group_map = &mut self.CONFIG_MAP;
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

            //解析 room -- groups表
            for (group_id, rooms) in self.CONFIG_MAP.iter() {
                for rooms_id_tmp in rooms {
                    // room -> 读取group 并插入
                    let liver_map = &mut self.LIVER_MAP;
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

            // 初始化拉取直播的状态表
            for (room_id, _) in self.LIVER_MAP.iter() {
                self.LIVE_STATUS_MAP.insert(room_id.clone(), RoomState::Close);
                self.LIVER_VEC.push(room_id.clone());
            }
        }
        pub fn get_instance(path: &str) -> RunDate{
            let mut data = RunDate{
                CONFIG_MAP: HashMap::new(),
                LIVER_MAP: HashMap::new(),
                LIVER_VEC: Vec::new(),
                LIVE_STATUS_MAP: HashMap::new()
            };
            data.init(path);
            return data;
        }
    }
    pub fn dt(date: &mut RunDate){
        if date.LIVER_VEC.len() == 0 {
            return;
        }
        let client = reqwest::Client::new();
        loop {
            thread::spawn(|| async {
                let rom = get_all_room(&client, &date.LIVER_VEC).await;
                match rom {
                    Ok(room_map) => {
                        let mut works = Vec::new();
                        for (id, room) in room_map.into_iter() {
                            let room_id: i64;
                            if let Ok(i) = id.parse::<i64>() {
                                room_id = i.clone();
                            } else { continue; }

                            let now_state = &room.get_state();
                            let live_status = &mut date.LIVE_STATUS_MAP;
                            let live_map = &mut date.LIVER_MAP;

                            // 检查直播状态是否改变  初始默认为停止
                            if room_status_change(&room_id, &now_state, live_status) {
                                // 更新记录状态
                                live_status.insert(room_id.clone(), now_state.clone());

                                // 直播开播 live start
                                if now_state.eq(&RoomState::Open) {
                                    println!("{}start!!!", &room_id);
                                    if let Some(groups_tmp) = live_map.get(&room_id) {
                                        for group_id_temp in groups_tmp {
                                            let data = get_open_message(&room, &group_id_temp);
                                            works.push(do_post(data));
                                        }
                                    }
                                } else {
                                    println!("{}close!!!", &room_id);
                                    if let Some(groups_tmp) = live_map.get(&room_id) {
                                        for group_id_temp in groups_tmp {
                                            let data = get_close_message(&room, &group_id_temp);
                                            works.push(do_post(data));
                                        }
                                    }
                                }
                            }
                        }
                        join_all(works.iter());
                    },
                    Err(e) => {
                        println!("{}",e)
                    }
                }
            });
            sleep(Duration::new(60, 00));
        }
    }
    async fn get_all_room(client: &reqwest::Client, lever_all: &Vec<i64>) -> Result<(HashMap<String, Room>), &str> {
        let mut map = HashMap::new();
        let mut uids: Vec<i64> = lever_all.clone();

        println!("{:?}", uids);

        map.insert("uids", uids);

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
                        return Err("json error, change struct ResponseDate!");
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
    fn room_status_change(room_id: &i64, room_status: &RoomState, statue_map: &mut HashMap<i64, RoomState>) -> bool {
        return if let Some(room) = statue_map.get(room_id) {
            !room.eq(room_status)
        } else {
            statue_map.insert(room_id.clone(), room_status.clone());
            false
        };
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
        msg = msg + "[CQ:image,url="+&room.keyframe+",type=show,id=40000]\n";
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
}