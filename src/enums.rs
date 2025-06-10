use serde::{Deserialize, Serialize};
use std::{ sync::OnceLock};

// pub const GT_FONT_PATH: &str = "./arkitech_medium.ttf";
pub const GT_FONT_PATH: &str = "./resource/arkitech_medium.ttf";

// pub const BASE_FONT_PATH: &str = "./ArialMonospace.ttf";
pub const BASE_FONT_PATH: &str = "./resource/ArialMonospace.ttf";


pub static USER: OnceLock<String> = OnceLock::new();


// pub fn get_gt_font_path() -> String {
//     let current_exe_path = env::current_exe().expect("获取当前可执行文件路径失败");
//     let current_exe_path = current_exe_path.parent().expect("获取当前可执行文件路径失败");
//     if !cfg!(debug_assertions){
//         current_exe_path.join(GT_FONT_PATH_DEV).to_str().expect("获取当前可执行文件路径失败").to_string()
//     }else{
//         current_exe_path.join(GT_FONT_PATH).to_str().expect("获取当前可执行文件路径失败").to_string()
//     }
// }

// pub fn get_base_font_path() -> String {
//     let current_exe_path = env::current_exe().expect("获取当前可执行文件路径失败");
//     let current_exe_path = current_exe_path.parent().expect("获取当前可执行文件路径失败");
//     if !cfg!(debug_assertions){
//         current_exe_path.join(BASE_FONT_PATH_DEV).to_str().expect("获取当前可执行文件路径失败").to_string()
//     }else{
//         current_exe_path.join(BASE_FONT_PATH).to_str().expect("获取当前可执行文件路径失败").to_string()
//     }
// }

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
    pub close: bool,
}
impl Default for TeleData {
    fn default() -> Self {
        Self { 
            speed: 0.0,
            close: false 
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]

pub struct ShowState {
    pub show_setting: bool,
    pub show_info: bool,
    pub show_sector: bool,
    pub show_sight: bool,
    pub show_complist: bool,
}

impl Default for ShowState {
    fn default() -> Self {
        Self { 
            show_setting: false,
            show_info: false,
            show_sector: true,
            show_sight: true,
            show_complist: false
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]

pub struct  SettingData{
    pub ip: String,
    pub port: String,
    pub sector_len: String,
    pub sector_scale: f32,
    pub sector_base_len: f32,

    pub sight_len: String,
    pub sight_scale:f32,
    pub sight_base_len: f32,
}

impl Default for SettingData {
    fn default() -> Self {
        Self { 
           ip: "127.0.0.1".to_string(),
           port: "8000".to_string(),

           sector_len: "210".to_string(),
           sector_scale:5.25,
           sector_base_len: 210.0,

           sight_len: "14".to_string(),
           sight_scale: 1.0,
           sight_base_len: 14.0,

        }
    }
}

pub struct  ErrorData {
    pub message: String
}