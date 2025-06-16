use core::str;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, sync::OnceLock};

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
            close: false,
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
    pub show_dash: bool,
    pub show_history: bool,
    pub show_other: bool,
}

impl Default for ShowState {
    fn default() -> Self {
        Self {
            show_setting: false,
            show_info: false,
            show_sector: true,
            show_sight: true,
            show_complist: false,
            show_dash: true,
            show_history: true,
            show_other: false,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]

pub struct SettingData {
    pub ip: String,
    pub port: String,
    pub sector_len: String,
    pub sector_scale: f32,
    pub sector_base_len: f32,
    pub sector_delta_len: f32,
    pub sector_delta_scale: f32,
    // pub sector_delta_scale_ss4: f32,
    pub sight_len: String,
    pub sight_scale: f32,
    pub sight_base_len: f32,

    pub dash_len: String,
    pub dash_scale: f32,
    pub dash_base_len: f32,
}

impl Default for SettingData {
    fn default() -> Self {
        Self {
            ip: "127.0.0.1".to_string(),
            port: "8000".to_string(),

            sector_len: "210".to_string(),
            sector_scale: 5.25, //长宽比
            sector_base_len: 210.0,
            sector_delta_len: 90.0,
            sector_delta_scale: 2.25,
            //    sector_delta_scale_ss4: 3.0,
            sight_len: "14".to_string(),
            sight_scale: 1.0,
            sight_base_len: 14.0,

            dash_len: "1280".to_string(),
            dash_scale: 8.0,
            dash_base_len: 1280.0,
        }
    }
}

pub struct ErrorData {
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug,Clone)]

pub struct CarSetting {
    pub rpm: BTreeMap<u16, Vec<String>>, //车辆id , [最大转速, 红线转速]
}

impl Default for CarSetting {
    fn default() -> Self {
        Self {
            rpm: BTreeMap::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct  CurCarRpmSetting {
    pub car_id: u16,
    pub max_rpm: String,
    pub red_rpm: String
}

impl Default for CurCarRpmSetting {
    fn default() -> Self {
        Self {
           car_id:0,
           max_rpm: "".to_string(),
           red_rpm: "".to_string(),
        }
    }
}

// #[derive(Serialize, Deserialize, Debug)]

// pub struct CarSettingRpm {

// }

#[derive(Serialize, Deserialize, Debug)]

pub struct SectorRecord {
    pub s1: LapControl,
    pub s2: LapControl,
    pub s3: LapControl,
}
impl Default for SectorRecord {
    fn default() -> Self {
        Self {
            s1: LapControl::default(),
            s2: LapControl::default(),
            s3: LapControl::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)] // Add useful traits for debugging, default values, comparison, and cloning
pub struct LapControl {
    // Controle de tempo (Time control)
    pub lap_start: f64,
    pub sector_time: f64,
    // pub current_s1_time: f64, // Novo: armazena o tempo atual do S1
    // pub current_s2_time: f64, // Novo: armazena o tempo atual do S1
    pub current_s_time: f64,

    // Controle de posição (Position control)
    pub last_valid_distance: f64,
    pub sector_end: f64,
    pub has_passed_zero_point: bool,

    // Controle de estado (State control)
    pub current_lap: i32,
    pub is_done: bool,
    pub needs_reset: bool,
    pub was_in_pit: bool,
    pub time_shown: bool, // Novo: controla se já mostrou o tempo atual

    // Controle de recorde (Record control)
    pub best_time: f64,
    pub sub_best_time: f64,
    pub initialized: bool,
    pub last_track_id: u16,
    pub sector_start_time: f64,
    pub jumped_lap: i32, // Armazena o número da volta em que ocorreu o pulo
    pub track_length: f64,
    pub was_jumped: bool,
    pub custom_lap_counter: i32, // Novo contador independente
    pub started_counting: bool,  // Flag para saber se já começamos a contar
    pub delta: f64,
    pub delta_until: f64,
    pub two_laps_ago: f64,
    pub one_lap_ago: f64,

    pub s1_pass_time: f64,
    pub s2_pass_time: f64,

    pub s1_end: f64,
    pub s2_end: f64,

    pub s1_time: f64,
    pub s2_time: f64,
    pub s3_time: f64,
    pub s1_last_time: f64,
    pub s2_last_time: f64,
    pub s3_record: f64,
    pub last_lap_s: f64,
}

impl Default for LapControl {
    fn default() -> Self {
        LapControl {
            lap_start: 0.0,
            sector_time: 0.0,
            // current_s1_time: 0.0,
            // current_s2_time: 0.0,
            current_s_time: 0.0,

            last_valid_distance: 0.0,
            sector_end: 0.0,
            has_passed_zero_point: false,

            current_lap: -2,
            is_done: false,
            needs_reset: true,
            was_in_pit: false,
            time_shown: false,

            best_time: f64::INFINITY, // JavaScript's Infinity maps to f64::INFINITY
            sub_best_time: f64::INFINITY,
            initialized: false,
            last_track_id: 0,
            sector_start_time: 0.0,
            jumped_lap: 0, // null in JS becomes Option::None in Rust
            track_length: 0.0,
            was_jumped: false,
            custom_lap_counter: 0,
            started_counting: false,
            delta: 0.0,
            delta_until: 0.0,
            two_laps_ago: 0.0,
            one_lap_ago: 0.0,

            s1_pass_time: 0.0,
            s2_pass_time: 0.0,

            s1_end: 0.0,
            s2_end: 0.0,

            s1_time: 0.0,
            s2_time: 0.0,
            s3_time: 0.0,
            s1_last_time: 0.0,
            s2_last_time: 0.0,
            s3_record: 0.0,
            last_lap_s: 0.0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameRaceData {
    pub lap: i32,
    pub current_lap: i32,
    pub sub_current_lap: i32,
    pub distance: f64,
    pub sub_distance: f64,
    pub race_time: f64,
    pub fuel: f64,
    pub current_time: f64,
    pub track_id: u16,
    pub is_in_pit: bool,
    pub last_lap_time: f64,
    pub speed: f64,
    pub gear: i32,
    pub accel: f64,
    pub brake: f64,
    pub rpm: f64,
    pub clutch: f64,
    pub steer: f64,
    pub car_id: i32,
    pub best_lap_time: f64,
    pub max_rpm: f64,
    pub dash_is_blink:bool,
    pub dash_blink_ts:u128,
    pub boost: f64,
    pub tire_wear1:f64, //左前右前,左后右后
    pub tire_wear2:f64,
    pub tire_wear3:f64,
    pub tire_wear4:f64,
    pub last_lap_tire_wear1:f64,
    pub last_lap_tire_wear2:f64,
    pub last_lap_tire_wear3:f64,
    pub last_lap_tire_wear4:f64,
    pub lap_start_tire_wear1:f64,
    pub lap_start_tire_wear2:f64,
    pub lap_start_tire_wear3:f64,
    pub lap_start_tire_wear4:f64,
}
impl Default for GameRaceData {
    fn default() -> Self {
        Self {
            lap: 0,
            current_lap: 0,
            distance: 0.0,
            race_time: 0.0,
            current_time: 0.0,
            track_id: 0,
            is_in_pit: false,
            last_lap_time: 0.0,
            sub_current_lap: 0,
            sub_distance: 0.0,
            speed: 0.0,
            gear: 0,
            accel: 0.0,
            brake: 0.0,
            rpm: 0.0,
            clutch: 0.0,
            steer: 0.0,
            car_id: 0,
            best_lap_time: 0.0,
            max_rpm: 0.0,
            dash_is_blink:false,
            dash_blink_ts:0,
            fuel:0.0,
            boost:0.0,
            tire_wear1:0.0,
            tire_wear2:0.0,
            tire_wear3:0.0,
            tire_wear4:0.0,
            last_lap_tire_wear1:0.0,
            last_lap_tire_wear2:0.0,
            last_lap_tire_wear3:0.0,
            last_lap_tire_wear4:0.0,
            lap_start_tire_wear1:0.0,
            lap_start_tire_wear2:0.0,
            lap_start_tire_wear3:0.0,
            lap_start_tire_wear4:0.0,
        }
    }
}

pub struct DashData {}
