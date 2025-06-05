use crate::uitl::get_sys_username;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

pub const POT_GET_PROGRESS_TIME: u32 = 20484;

pub const REQ_TYPE: u32 = 1024;

pub const PLAY_LIST_SERVER_PATH: &str = "https://meamoe.top/record/temp/PotPlayerMini64.dpl";

pub const TEMP_LOCATION: &str = r"D:\MCode\test\PotPlayerMini64.dpl";

pub const POT_LOCATION: &str = r"C:\Program Files\DAUM\PotPlayer\PotPlayerMini64.exe";

pub const POT_LOCATION_KAF: &str = r"D:\Software\PotPlayer\PotPlayerMini64.exe";

pub const PLAY_LIST_LOCAL_LIST: &str =
    r"C:\Users\11038\AppData\Roaming\PotPlayerMini64\Playlist\PotPlayerMini64.dpl";

pub const PLAY_LIST_LOCAL_LIST_KAF: &str =
    r"D:\Software\PotPlayer\Playlist\PotPlayerMini64.dpl";

pub const HW_USER: &str = "huangwen";
pub const KAF_USER: &str = "kaf";

pub const STATIC_DIR : &str= r"D:\Software"; 

pub static USER: OnceLock<String> = OnceLock::new();

pub unsafe fn set_user() {
    USER.set(String::from(get_sys_username().trim_end_matches("\0"))).unwrap();
    println!(
        "🪵 [enums.rs:26]~ token ~ \x1b[0;32mUSER\x1b[0m = {}",
        get_user()
    );
}

pub fn get_pot_location() -> &'static str {
  let us: String = get_user();
    // println!(
    //     "🪵 [enums.rs:32]~ token ~ \x1b[0;32mUSER == HW_USER \x1b[0m ={} {} {}",
    //    us,
    //     HW_USER,
    //    us==HW_USER 
    // );
    if us==HW_USER  {
        POT_LOCATION
    } else {
        POT_LOCATION_KAF
    }
}

pub fn get_list_local_list() -> &'static str {
  
let us: String = get_user();
// println!(
//   "🪵 [enums.rs:322]~ token ~ \x1b[0;32mUSER == HW_USER \x1b[0m = {} {} {} {}",
//   us,
//   HW_USER,
//   us == HW_USER,
//   "huangwen"==us
// );
// assert_eq!(us, HW_USER);
    if us ==HW_USER   {
        PLAY_LIST_LOCAL_LIST
    } else {
        PLAY_LIST_LOCAL_LIST_KAF
    }
}

fn get_user() -> String {
    USER.get_or_init(|| "".to_string()).to_string()
}

pub struct PlayInfo {
    pub name: String,  //正在播放标题
    pub time: String,   //时间字符播放进度
    pub pg_time: isize, //ms播放进度
    pub ts: i64,        //正在播放日期ts

    pub play_time: String,  //播放进度ms
    pub play_bv: String,
}

#[derive(Debug, Deserialize)]
pub struct MyData<T> {
    pub data: T,
    // 其他字段
}


pub struct MyApp {
    pub value: f32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self { value: 0.5 }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TeleData {
    pub speed: f32,
}
impl Default for TeleData {
    fn default() -> Self {
        Self { speed: 0.0 }
    }
}
