#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use std::sync::{
    atomic::{AtomicBool, Ordering}, Mutex, OnceLock
};

use eframe::{
    egui::{self, Area, Color32, Id, Pos2, Vec2},
    epaint::text::{FontInsert, InsertFontFamily},
};
use serde::{Deserialize, Serialize};

use crate::{enums::{TeleData}, ui::{
    other_logic::{check_first, check_is_focus, global_hk, listen_mouse_pass_event},
    sector::render_sector,
}};

pub static IS_FIRST: OnceLock<Mutex<AtomicBool>> = OnceLock::new();
pub static IS_MOUSE_PASS: OnceLock<Mutex<AtomicBool>> = OnceLock::new();
pub static LAST_IS_MOUSE_PASS: OnceLock<Mutex<AtomicBool>> = OnceLock::new();
pub static NEED_FIX_POS: OnceLock<Mutex<AtomicBool>> = OnceLock::new();
pub static SECTORID: OnceLock<Id> = OnceLock::new();
pub static CTX: OnceLock<&egui::Context> = OnceLock::new();
// pub static SHORTCUT_RX: OnceLock<mpsc::Receiver<keyData>> = OnceLock::new();
static DEFAULT_INNERSIZE: Vec2 = egui::vec2(2100.0, 1300.0);
static DEFAULT_POS2: Pos2 = egui::pos2(-180.0, -180.0);
pub static DEFAULT_SECTOR_POS: Pos2 = egui::pos2(700.0, 200.0);

pub fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
                        // let options = eframe::NativeOptions {
                        //     viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
                        //     ..Default::default()
                        // };
                        // eframe::run_native(
                        //     "fm8_sector",
                        //     options,
                        //     Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
                        // )
                        // key_listener();
    SECTORID.set(Id::new("sector")).unwrap();
    global_hk();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([DEFAULT_INNERSIZE.x, DEFAULT_INNERSIZE.y]) // Initial size of the window\
            .with_position([DEFAULT_POS2.x, DEFAULT_POS2.y])
            .with_decorations(false)
            .with_transparent(true) // Crucial: Make the native window transparent
            .with_fullscreen(true)
            .with_resizable(true)
            .with_maximize_button(true)
            .with_mouse_passthrough(true) // Start with mouse passthrough enabled
            .with_always_on_top(), // Crucial: Keep the window on top of others
        multisampling: 1,
        renderer: eframe::Renderer::Glow,
        // persist_window: false,
        // renderer: eframe::Renderer::Wgpu, // Explicitly tell eframe to use Wgpu
        vsync: true,
        ..Default::default()
    };

    eframe::run_native(
        "fm8_hud",
        options,
        // Box::new(|_cc| Ok(Box::new(MyApp::new(_cc)))),
        Box::new(|cc| {
            // 关键步骤 2: 在应用初始化时加载持久化状态
            // let mut app: MyApp = MyApp::new(cc);
            let mut app: MyApp = if let Some(storage) = cc.storage {
                // 如果有存储，尝试加载状态
                let a: Option<MyApp> = eframe::get_value(storage, eframe::APP_KEY);
                match a {
                    Some(a) => a,
                    None => MyApp::new(cc),
                }
            } else {
                // 如果没有存储（例如首次运行），使用默认值
                println!(
                    "🪵 [index.rs:61]~ token ~ \x1b[0;32m没有存储\x1b[0m = {}",
                    "没有存储"
                );
                MyApp::new(cc)
            };

            replace_fonts(&cc.egui_ctx);
            add_font(&cc.egui_ctx);
            reset_myapp(&mut app);
            // inputbot_listen();
            Ok(Box::new(app))
        }),
    )
}

// Demonstrates how to add a font to the existing ones
fn add_font(ctx: &egui::Context) {
    ctx.add_font(FontInsert::new(
        "my_font",
        egui::FontData::from_static(include_bytes!(
            "../../resource/arkitech_bold.ttf"
        )),
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
}

// Demonstrates how to replace all fonts.
fn replace_fonts(ctx: &egui::Context) {
    // Start with the default fonts (we will be adding to them rather than replacing them).
    let mut fonts = egui::FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters).
    // .ttf and .otf files supported.
    fonts.font_data.insert(
        "gt_font".to_owned(),
        std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
            "../../resource/arkitech_bold.ttf"
        ))),
    );

    fonts.font_data.insert(
        "base_font".to_owned(),
        std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
            "../../resource/ArialMonospace.ttf"
        ))),
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
        .push("base_font".to_owned());

    // Put my font as last fallback for monospace:
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("base_font".to_owned());

    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("gt_font".to_owned());

    // Tell egui to use these fonts:
    ctx.set_fonts(fonts);
}

fn reset_myapp(me: &mut MyApp) {
    me.show_ui = false;
    me.mouse_pass = true;
}

fn force_check_fullscreen(ui: &mut egui::Ui, ctx: &egui::Context) {
    let fullscreen = ui.input(|i| i.viewport().fullscreen.unwrap_or(false));
    if fullscreen {
        ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(false));
    }
}

fn get_cur_position(ctx: &egui::Context, app: &mut MyApp) {
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

fn set_need_fix_pos() {
    tokio::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_millis(15)).await;
        let need_fix_pos = NEED_FIX_POS
            .get_or_init(|| Mutex::new(AtomicBool::new(false)))
            .lock();
        let need_fix_pos = match need_fix_pos {
            Ok(h) => h,
            Err(e) => return,
        };
        need_fix_pos.store(true, Ordering::SeqCst);
    });
}

fn fix_viewport_size(ctx: &egui::Context) {
    // let need_fix_pos = NEED_FIX_POS
    //     .get_or_init(|| Mutex::new(AtomicBool::new(false)))
    //     .lock();
    // let need_fix_pos = match need_fix_pos {
    //     Ok(h) => h,
    //     Err(e) => return,
    // };
    // if need_fix_pos.load(Ordering::SeqCst) {
    //     ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(DEFAULT_INNERSIZE));
    //     ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(DEFAULT_POS2));
    // }
    ctx.send_viewport_cmd(egui::ViewportCommand::Decorations(true));
    ctx.send_viewport_cmd(egui::ViewportCommand::Transparent(true));
    ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(DEFAULT_INNERSIZE));
    ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(DEFAULT_POS2));
    ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(DEFAULT_POS2));
}
fn fix_transparent(ctx: &egui::Context) {
    ctx.send_viewport_cmd(egui::ViewportCommand::Transparent(true));
    // ctx.send_viewport_cmd(egui::ViewportCommand::Maximized(true));

    // ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(true));
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MyApp {
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
    pub tele_data:TeleData

}

impl MyApp {
    
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
            tele_data: TeleData::default(),
        }
    }
}

impl eframe::App for MyApp {

    
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        // Rgba::TRANSPARENT.to_array() // Alpha 值为 0.0，表示完全透明
        // Rgba::TRANSPARENT.to_array()
        [0.0, 0.0, 0.0, 0.0]
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        // egui 默认会将大部分 UI 状态（包括窗口位置）保存到 storage 中。
        // 你也可以保存你自己的应用状态，例如：
        storage.set_string("my_custom_label_text", "ee".to_owned());
        // storage.set_string("MyApp", serde_json::to_string(self).unwrap());
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
    // snip

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        
        // let ctx_ = CTX.set(&ctx);
        
        // fix_viewport_size(ctx);
        // fix_transparent(ctx);

        // if ctx.input(|i| i.key_pressed(egui::Key::F11)) {
        //     ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(false));
        // }

        if ctx.input(|i| i.key_pressed(egui::Key::F2)) {
            let handle = IS_MOUSE_PASS
                .get_or_init(|| Mutex::new(AtomicBool::new(true)))
                .lock();
            let handle = match handle {
                Ok(h) => h,
                Err(e) => return,
            };
            let is_mouse_pass = handle.load(Ordering::SeqCst);
            handle.store(!is_mouse_pass, Ordering::SeqCst);
        }
        // println!("update");
        listen_mouse_pass_event(ctx, self);

        check_first(ctx, self);

        check_is_focus(ctx, self);

        // get_cur_position(ctx, self);

        // test_transparent(ctx, self);

        render_sector(ctx, self);
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
                    ui.label(egui::RichText::new("fullscreen overlay")
                    .color(Color32::WHITE)
                    .size(32.0),);
                });
        }

        fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
            egui::Rgba::TRANSPARENT.to_array()
        }
    }
}
