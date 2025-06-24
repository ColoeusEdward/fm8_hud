use std::{
    collections::{hash_set, BTreeMap},
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc, Arc, Mutex, OnceLock,
    },
};

use eframe::{
    egui::{self, Area, Color32, Id, Pos2, Vec2},
    egui_glow::ShaderVersion,
    epaint::text::{FontInsert, InsertFontFamily},
};
use serde::{Deserialize, Serialize};

use crate::{
    enums::{CarSetting, CurCarRpmSetting, ErrorData, GameRaceData, SectorRecord, SettingData, ShowState, TeleData},
    ui::{
        dash::render_dash, lap_history::render_history, other_logic::{
            check_first, check_is_focus, check_is_min, check_udp_run, global_hk, key_listener_focus, listen_mouse_pass_event, receive_upd_data, render_error, rev_rx
        }, sector::{ render_cross_line, render_sector, render_sight}, setting::{load_car_json, render_setting, save_car_json}
    },
    uitl::get_sreen_info,
};

pub static IS_FIRST: OnceLock<Mutex<AtomicBool>> = OnceLock::new();
pub static IS_MOUSE_PASS: OnceLock<Mutex<AtomicBool>> = OnceLock::new();
pub static IS_MIN: OnceLock<Mutex<AtomicBool>> = OnceLock::new();
pub static IS_UDP_REDIRECT: OnceLock<Mutex<AtomicBool>> = OnceLock::new();
pub static LAST_IS_MOUSE_PASS: OnceLock<Mutex<AtomicBool>> = OnceLock::new();
pub static NEED_FIX_POS: OnceLock<Mutex<AtomicBool>> = OnceLock::new();
pub static SECTORID: OnceLock<Id> = OnceLock::new();
pub static CTX: OnceLock<&egui::Context> = OnceLock::new();
pub static KEYRECORD: OnceLock<Mutex<hash_set::HashSet<rdev::Key>>> = OnceLock::new();
pub static RXHOLDER: OnceLock<Mutex<mpsc::Receiver<TeleData>>> = OnceLock::new();
pub static TXHOLDER: OnceLock<Mutex<mpsc::Sender<TeleData>>> = OnceLock::new();
pub static ERROR_RX: OnceLock<Mutex<mpsc::Receiver<ErrorData>>> = OnceLock::new();
pub static ERROR_TX: OnceLock<Mutex<mpsc::Sender<ErrorData>>> = OnceLock::new();
pub static APP_RX: OnceLock<Mutex<mpsc::Receiver<&mut MyApp2>>> = OnceLock::new();
pub static APP_TX: OnceLock<Mutex<mpsc::Sender<&mut MyApp2>>> = OnceLock::new();
pub static TELE_DATA_TX: OnceLock<Mutex<mpsc::Sender<BTreeMap<String, f32>>>> = OnceLock::new();
pub static TELE_DATA_RX: OnceLock<Mutex<mpsc::Receiver<BTreeMap<String, f32>>>> = OnceLock::new();
pub static BOOL_TX: OnceLock<Mutex<mpsc::Sender<bool>>> = OnceLock::new();
pub static BOOL_RX: OnceLock<Mutex<mpsc::Receiver<bool>>> = OnceLock::new();
pub static LAST_TELE_DATA: OnceLock<Mutex<BTreeMap<String, f32>>> = OnceLock::new();
pub static SECTOR_RECORD_DATA: OnceLock<Mutex<SectorRecord>> = OnceLock::new();
pub static GAME_RACE_DATA: OnceLock<Mutex<GameRaceData>> = OnceLock::new();
pub static TEXTURE_HANDLE_MAP: OnceLock<Mutex<BTreeMap<String, egui::TextureHandle>>> = OnceLock::new();
pub static CAR_SETTING: OnceLock<Mutex<CarSetting>> = OnceLock::new();
pub static CUR_CAR_RPM_SETTING: OnceLock<Mutex<CurCarRpmSetting>> = OnceLock::new();

pub static RESTART_UDP_FLAG: OnceLock<AtomicBool> = OnceLock::new();
pub static ERROR_SHOW_FLAG: OnceLock<AtomicBool> = OnceLock::new();
// pub static SHORTCUT_RX: OnceLock<mpsc::Receiver<keyData>> = OnceLock::new();
static DEFAULT_INNERSIZE: Vec2 = egui::vec2(1970.0, 1120.0);
static DEFAULT_INNERSIZE_DIFF: Vec2 = egui::vec2(50.0, 40.0);
static DEFAULT_POS2: Pos2 = egui::pos2(-50.0, -32.0);
pub static DEFAULT_SECTOR_POS: Pos2 = egui::pos2(852.0, 159.0);
pub static DEFAULT_SECTOR_DELTA_POS: Pos2 = egui::pos2(1072.0, 159.0);

#[derive(Debug, Serialize, Deserialize)]
pub struct MyApp2 {
    pub text: String,
    pub value: String,
    pub show_ui: bool, // Toggle UI visibility
    pub yoffset: f32,
    pub xoffset: f32,
    pub pox: f32,
    pub poy: f32,
    pub mouse_pass: bool,
    pub sector_size: Vec<f32>,
    pub fullscreen: bool,
    pub transparent: bool,
    pub sector_pos: Pos2,
    pub sector_delta_pos: Pos2,
    pub tele_data: TeleData,
    pub sight_pos: Pos2,
    pub show_state: ShowState,
    pub setting_data: SettingData,
    pub hud_pos: Pos2,
    pub history_pos: Pos2,
}

impl MyApp2 {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            text: "Edit this text field if you want".to_owned(),
            value: "Edit this text field if you want".to_owned(),
            show_ui: false,
            mouse_pass: true,
            yoffset: 0.0,
            xoffset: 0.0,
            pox: 0.0,
            poy: 0.0,
            sector_size: vec![560.0, 180.0],
            fullscreen: true,
            transparent: true,
            sector_pos: DEFAULT_SECTOR_POS,
            sector_delta_pos: DEFAULT_SECTOR_DELTA_POS,
            tele_data: TeleData::default(),
            sight_pos: Pos2 { x: 0.0, y: 0.0 },
            show_state: ShowState::default(),
            setting_data: SettingData::default(),
            hud_pos: Pos2 { x: 339.0, y: 339.0 },
            history_pos: Pos2 { x: 1209.0, y: 472.0 },
        }
    }
}

pub fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
                        // let options = eframe::NativeOptions {
                        //     viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
                        //     ..Default::default()
                        // };
                        // eframe::run_native(
                        //     "fm8_sector",
                        //     options,
                        //     Box::new(|cc| Ok(Box::new(MyApp2::new(cc)))),
                        // )
                        // key_listener();
    load_car_json();
    SECTORID.set(Id::new("sector")).unwrap();
    let (tx, rx) = mpsc::channel::<TeleData>();
    RXHOLDER.set(Mutex::new(rx)).unwrap();
    TXHOLDER.set(Mutex::new(tx)).unwrap();
    let (tx, rx) = mpsc::channel::<ErrorData>();
    ERROR_RX.set(Mutex::new(rx)).unwrap();
    ERROR_TX.set(Mutex::new(tx)).unwrap();
    let (tx, rx) = mpsc::channel::<MyApp2>();
    // APP_RX.set(Mutex::new(rx)).unwrap();
    // APP_TX.set(Mutex::new(tx)).unwrap();
    let (tx, rx) = mpsc::channel::<BTreeMap<String, f32>>();
    TELE_DATA_RX.set(Mutex::new(rx)).unwrap();
    TELE_DATA_TX.set(Mutex::new(tx)).unwrap();
    let (tx, rx) = mpsc::channel::<bool>();
    BOOL_TX.set(Mutex::new(tx)).unwrap();
    BOOL_RX.set(Mutex::new(rx)).unwrap();
    LAST_TELE_DATA.set(Mutex::new(BTreeMap::new())).unwrap();
    SECTOR_RECORD_DATA.set(Mutex::new(SectorRecord::default())).unwrap();
    GAME_RACE_DATA.set(Mutex::new(GameRaceData::default())).unwrap();
    let _ = TEXTURE_HANDLE_MAP.set(Mutex::new(BTreeMap::new()));
    CUR_CAR_RPM_SETTING.set(Mutex::new(CurCarRpmSetting::default())).unwrap();
    IS_MIN.set(Mutex::new(AtomicBool::new(false))).unwrap();
    global_hk();

    receive_upd_data();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_transparent(true) // Crucial: Make the native window transparent
            .with_fullscreen(false)
            .with_resizable(false)
            .with_maximize_button(true)
            .with_mouse_passthrough(true) // Start with mouse passthrough enabled
            .with_visible(true)
            .with_always_on_top(), // Crucial: Keep the window on top of others
        multisampling: 1,
        renderer: eframe::Renderer::Glow,
        hardware_acceleration: eframe::HardwareAcceleration::Required,
        run_and_return: false,
        window_builder: Some(Box::new(|vp| {
            let (screenx, screeny) = get_sreen_info();
            vp.with_fullscreen(false)
                .with_mouse_passthrough(true)
                .with_transparent(true)
                .with_decorations(true)
                .with_position([DEFAULT_POS2.x, DEFAULT_POS2.y])
                .with_inner_size([
                    DEFAULT_INNERSIZE_DIFF.x + screenx,
                    DEFAULT_INNERSIZE_DIFF.y + screeny,
                ]) // Initial size of the window\
        })),
        // persist_window: false,
        // renderer: eframe::Renderer::Wgpu, // Explicitly tell eframe to use Wgpu
        // vsync: true,
        shader_version: Some(ShaderVersion::Es300),
        ..Default::default()
    };

    eframe::run_native(
        "fm8_hud",
        options,
        // Box::new(|_cc| Ok(Box::new(MyApp2::new(_cc)))),
        Box::new(|cc| {
            // 关键步骤 2: 在应用初始化时加载持久化状态
            // let mut app: MyApp2 = MyApp2::new(cc);
            let mut app: MyApp2 = if let Some(storage) = cc.storage {
                // 如果有存储，尝试加载状态
                let a = eframe::get_value(storage, eframe::APP_KEY);
                match a {
                    Some(a) => a,
                    None => MyApp2::new(cc),
                }
            } else {
                // 如果没有存储（例如首次运行），使用默认值
                println!(
                    "🪵 [index.rs:61]~ token ~ \x1b[0;32m没有存储\x1b[0m = {}",
                    "没有存储"
                );
                MyApp2::new(cc)
            };

            IS_UDP_REDIRECT.set(Mutex::new(AtomicBool::new(app.setting_data.is_redirect))).unwrap();
            // replace_fonts(&cc.egui_ctx);
            add_font(&cc.egui_ctx);
            reset_myapp(&mut app);
            Ok(Box::new(app))
        }),
    )
}

// Demonstrates how to add a font to the existing ones
fn add_font(ctx: &egui::Context) {
    ctx.add_font(FontInsert::new(
        "my_font",
        egui::FontData::from_static(include_bytes!("../../resource/gt7-MyTimingFont.ttf")),
        // egui::FontData::from_static(include_bytes!("../../resource/gt7-MyTimingFontOutline.ttf")),
        vec![
            InsertFontFamily {
                family: egui::FontFamily::Proportional,
                priority: egui::epaint::text::FontPriority::Highest,
            },
            InsertFontFamily {
                family: egui::FontFamily::Monospace,
                priority: egui::epaint::text::FontPriority::Lowest,
            },
        ],
    ));

    ctx.add_font(FontInsert::new(
        "ms_font",
        egui::FontData::from_static(include_bytes!(r"C:\Windows\Fonts\msyh.ttc")),
        vec![
            InsertFontFamily {
                family: egui::FontFamily::Monospace,
                priority: egui::epaint::text::FontPriority::Highest,
            },
            InsertFontFamily {
                family: egui::FontFamily::Proportional,
                priority: egui::epaint::text::FontPriority::Lowest,
            },
        ],
    ));

    ctx.add_font(FontInsert::new(
        "gt_base_font",
        egui::FontData::from_static(include_bytes!(r"../../resource/gt7-MyFont-Light.ttf")),
        vec![
            InsertFontFamily {
                family: egui::FontFamily::Name("base".into()),
                priority: egui::epaint::text::FontPriority::Highest,
            },
            InsertFontFamily {
                family: egui::FontFamily::Proportional,
                priority: egui::epaint::text::FontPriority::Lowest,
            },
        ],
    ));
}

// Demonstrates how to replace all fonts.
fn replace_fonts(ctx: &egui::Context) {
    // Start with the default fonts (we will be adding to them rather than replacing them).
    let mut fonts = egui::FontDefinitions::default();
    // 从系统路径加载字体
    #[cfg(target_os = "windows")]
    let font_path = r"C:\Windows\Fonts\msyh.ttc";

    // Install my own font (maybe supporting non-latin characters).
    // .ttf and .otf files supported.
    fonts.font_data.insert(
        "gt_font".to_owned(),
        std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
            "../../resource/gt7-MyTimingFont.ttf"
        ))),
    );

    fonts.font_data.insert(
        "base_font".to_owned(),
        std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
            "../../resource/ArialMonospace.ttf"
        ))),
    );

    let font_data = std::fs::read(font_path).expect("无法读取字体文件");
    fonts.font_data.insert(
        "default".to_owned(),
        Arc::new(egui::FontData::from_owned(font_data)),
    );

    // Put my font first (highest priority) for proportional text:
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .push("gt_font".to_owned());

    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .push("default".to_owned());

    // fonts
    //     .families
    //     .entry(egui::FontFamily::Proportional)
    //     .or_default()
    //     .push("default".to_owned());

    // // Put my font as last fallback for monospace:
    // fonts
    //     .families
    //     .entry(egui::FontFamily::Monospace)
    //     .or_default()
    //     .push("base_font".to_owned());

    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("default".to_owned());

    // Tell egui to use these fonts:
    ctx.set_fonts(fonts);
}

fn reset_myapp(me: &mut MyApp2) {
    me.show_ui = false;
    me.mouse_pass = true;
    me.show_state.show_info = false;
    me.show_state.show_setting = false;
}

// fn force_check_fullscreen(ui: &mut egui::Ui, ctx: &egui::Context) {
//     let fullscreen = ui.input(|i| i.viewport().fullscreen.unwrap_or(false));
//     if fullscreen {
//         ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(false));
//     }
// }

fn get_cur_position(ctx: &egui::Context, app: &mut MyApp2) {
    let mut desired_pos: Option<Vec2> = None;
    ctx.input(|input| {
        if let Some(size) = input.viewport().monitor_size {
            // let center = pos2(size.x / 2.0, size.y / 2.0) - pos2(310.0, 120.0);
            desired_pos = input.viewport().outer_rect.and_then(|current_position| {
                Option::Some(Vec2::new(current_position.min.x, current_position.min.y))
            })
        }
    });
    if let Some(pos) = desired_pos {
        // println!("🪵 [index.rs:191]~ token ~ \x1b[0;32mdesired_pos\x1b[0m = {} {}", pos.x,pos.y);
        app.pox = pos.x;
        app.poy = pos.y;
    }
}

// fn set_need_fix_pos() {
//     tokio::spawn(async move {
//         tokio::time::sleep(tokio::time::Duration::from_millis(15)).await;
//         let need_fix_pos = NEED_FIX_POS
//             .get_or_init(|| Mutex::new(AtomicBool::new(false)))
//             .lock();
//         let need_fix_pos = match need_fix_pos {
//             Ok(h) => h,
//             Err(e) => return,
//         };
//         need_fix_pos.store(true, Ordering::SeqCst);
//     });
// }

impl eframe::App for MyApp2 {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        // Rgba::TRANSPARENT.to_array() // Alpha 值为 0.0，表示完全透明
        // Rgba::TRANSPARENT.to_array()
        let is_mouse_pass = IS_MOUSE_PASS
            .get_or_init(|| Mutex::new(AtomicBool::new(true)))
            .lock()
            .expect("failed to lock");
        let is_mouse_pass = is_mouse_pass.load(Ordering::SeqCst);
        if is_mouse_pass {
            return [0.0, 0.0, 0.0, 0.0];
        } else {
            return [0.5, 0.5, 0.5, 0.1];
        }
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        save_car_json();
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        // egui 默认会将大部分 UI 状态（包括窗口位置）保存到 storage 中。
        // 你也可以保存你自己的应用状态，例如：
        storage.set_string("my_custom_label_text", "ee".to_owned());
        // storage.set_string("MyApp", serde_json::to_string(self).unwrap());
        eframe::set_value(storage, eframe::APP_KEY, self);
        // println!("🪵 [index.rs:320]~ token ~ \x1b[0;32mself\x1b[0m = {} {}", self.sector_pos.x,self.sector_pos.y);
        // println!("🪵 [index.rs:320]~ token ~ \x1b[0;32mself\x1b[0m = {} {}", self.sight_pos.x,self.sight_pos.y);
    }
    // snip

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        key_listener_focus(ctx, self);
        // println!("update");
        listen_mouse_pass_event(ctx, self);

        check_first(ctx, self);

        check_is_focus(ctx, self);
        check_udp_run(ctx, self);

        rev_rx(ctx, self, _frame);
        // get_cur_position(ctx, self);

        // test_transparent(ctx, self);
        // render_white_overlay(ctx, self);
        
        render_cross_line(ctx);

        render_sector(ctx, self);
        render_sight(ctx, self);
        render_setting(ctx, self);
        if check_is_min(ctx, self) { return; } // render_min(ctx, app)

        render_dash(ctx, self);
        render_history(ctx, self);

        render_error(ctx, self, _frame);
    }
}

#[derive(Default)]
struct TestApp {
    fullscreen: bool,
    transparent: bool,
}

impl eframe::App for TestApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        Area::new("fullscreen overlay".into())
            .current_pos(egui::pos2(200.0, 200.0))
            .movable(true)
            .show(ctx, |ui| {
                ui.label(
                    egui::RichText::new("fullscreen overlay")
                        .color(Color32::WHITE)
                        .size(32.0),
                );
            });
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }
}
pub fn test() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_transparent(true)
            .with_fullscreen(true),

        //The following two lines solve the problem
        multisampling: 1,
        renderer: eframe::Renderer::Glow,

        ..Default::default()
    };
    let _ = eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Ok(Box::<TestApp>::default())),
    );
}
