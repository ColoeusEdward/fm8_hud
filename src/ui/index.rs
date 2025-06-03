#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use std::sync::{atomic::{AtomicBool, Ordering}, OnceLock};

use eframe::{
    egui::{self, pos2, Area, Color32,  Id, Pos2, Rect, Rgba,  UiBuilder, Vec2},
    epaint::{
        text::{FontInsert, InsertFontFamily},
        CornerRadiusF32,
    },
};
use serde::{Deserialize, Serialize};


pub static IS_FIRST: OnceLock<AtomicBool> = OnceLock::new();


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

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([560.0, 180.0]) // Initial size of the window\
            // .with_position([500.0, 500.0])
            .with_transparent(true) // Crucial: Make the native window transparent
            .with_decorations(false) // Crucial: Remove native window decorations (title bar, borders)
            .with_always_on_top() // Crucial: Keep the window on top of others
            .with_resizable(false)
            .with_maximize_button(false)
            .with_mouse_passthrough(false), // Start with mouse passthrough enabled
            persist_window: true,
            renderer: eframe::Renderer::Wgpu, // Explicitly tell eframe to use Wgpu
            
            
        ..Default::default()
    };

    eframe::run_native(
        "egui Transparent Overlay",
        options,
        // Box::new(|_cc| Ok(Box::new(MyApp::new(_cc)))),
        Box::new(|cc| {
            // 关键步骤 2: 在应用初始化时加载持久化状态
            let mut app: MyApp = if let Some(storage) = cc.storage {
                // 如果有存储，尝试加载状态
                let a: Option<MyApp> = eframe::get_value(storage, eframe::APP_KEY);
                match a {
                    Some(a) => a,
                    None => MyApp::new(cc),
                }
            } else {
                // 如果没有存储（例如首次运行），使用默认值
                println!("🪵 [index.rs:61]~ token ~ \x1b[0;32m没有存储\x1b[0m = {}", "没有存储");
                MyApp::new(cc)
            };
            replace_fonts(&cc.egui_ctx);
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
        egui::FontData::from_static(include_bytes!(
            "D:\\Software\\SimHub\\DashFonts\\arkitech_bold.ttf"
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
            "D:\\Software\\SimHub\\DashFonts\\arkitech_bold.ttf"
        ))),
    );

    fonts.font_data.insert(
        "base_font".to_owned(),
        std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
            "D:\\Software\\SimHub\\DashFonts\\bahnschrift.ttf"
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
}

#[derive(Debug, Serialize,Deserialize)]
struct MyApp {
    pub text: String,
    pub value: String,
    pub show_ui: bool, // Toggle UI visibility
    pub yoffset: f32,
    pub xoffset: f32,
    pub pox: f32,
    pub poy: f32,
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        
        Self {
            text: "Edit this text field if you want".to_owned(),
            value: "Edit this text field if you want".to_owned(),
            show_ui: false,
            yoffset: 0.0,
            xoffset: 0.0,
            pox: 500.0,
            poy: 500.0,
        }
    }
}

impl eframe::App for MyApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        Rgba::TRANSPARENT.to_array() // Alpha 值为 0.0，表示完全透明
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        // egui 默认会将大部分 UI 状态（包括窗口位置）保存到 storage 中。
        // 你也可以保存你自己的应用状态，例如：
        storage.set_string("my_custom_label_text", "ee".to_owned());
        // storage.set_string("MyApp", serde_json::to_string(self).unwrap());
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
// snip

        
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame,) {
        if ctx.input(|i| i.key_pressed(egui::Key::F1)) {
            self.show_ui = !self.show_ui;
            // Optionally, toggle mouse passthrough when UI visibility changes
            // ctx.send_viewport_cmd(egui::ViewportCommand::MousePassthrough(!self.show_ui));
            ctx.send_viewport_cmd(egui::ViewportCommand::Decorations(self.show_ui));
            self.yoffset = if self.show_ui { 10.0 } else { 43.0 };
            self.xoffset = if self.show_ui { 0.0 } else { 10.0 };
        }
        if IS_FIRST.get_or_init(|| AtomicBool::new(true)).load(Ordering::SeqCst) {
            ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(Pos2::new(self.pox, self.poy)));
            IS_FIRST.get().unwrap().store(false, Ordering::SeqCst);

        }

        let mut desired_pos: Option<Vec2>= None;
        ctx.input(|input| {
            if let Some(size) = input.viewport().monitor_size {
                let center = pos2(size.x/2.0, size.y /2.0) - pos2(310.0, 120.0) ;
                desired_pos = input.viewport().outer_rect.and_then(|current_position| {
                    Option::Some(Vec2::new(current_position.min.x, current_position.min.y))
                })
            }
        });
        if let Some(pos) = desired_pos {
            // println!("🪵 [index.rs:191]~ token ~ \x1b[0;32mdesired_pos\x1b[0m = {} {}", pos.x,pos.y);
            self.pox = pos.x;
            self.poy = pos.y;
        }

        // if let Some(pos) = desired_pos {
        //     let move_command = egui::ViewportCommand::OuterPosition(pos2(pos.x, pos.y));
        //     ctx.send_viewport_cmd(move_command);
        // }
        // ctx.input(|input| {
        //     if let Some(size) = input.viewport().monitor_size {
        //         let res: Option<Rect> = input.viewport().outer_rect.and_then(|current_position| {
        //             Some(current_position)
        //         });
        //     }
        // });
        // let pos = ctx.viewport(|viewport_state| {
        //     // 这个闭包就是 `reader`
        //     // 它可以访问 `&ViewportState`
        //     ;et pos = viewport_state.
        // });
        // if !self.show_ui {
        //     // Create a custom window that acts as your overlay UI

        // }

        Area::new(Id::new("test"))
            .fixed_pos(egui::pos2(100.0+self.xoffset, self.yoffset)) // 固定位置
            .show(ctx, |ui| {
                // 这个文本会直接显示在透明的 viewport 上，没有任何背景

                // 定义圆角矩形的尺寸
                let desired_size = egui::vec2(210.0, 50.0);
                // 分配一个精确大小的区域，这将是我们绘制矩形的边界
                let (rect, _response) = ui.allocate_exact_size(desired_size, egui::Sense::hover());

                // 获取 painter
                let painter = ui.painter();

                // 定义填充颜色: #A2000000 (ARGB) -> 64% 透明度的黑色 (RGBA: 0,0,0,162)
                let fill_color = Color32::from_rgba_unmultiplied(0, 0, 0, 80);

                // 定义圆角半径
                let corner_radius = 6.0; // 较大的圆角，更明显

                // 绘制填充的圆角矩形
                painter.rect_filled(
                    rect,
                    CornerRadiusF32::same(corner_radius), // 所有角的圆角半径相同
                    fill_color,
                );

                // 在矩形中央添加一些文本，以显示其半透明效果
                // 确保文本位于绘制的矩形内部
                ui.allocate_new_ui(UiBuilder::new().max_rect(rect), |ui| {
                    ui.with_layout(
                        egui::Layout::centered_and_justified(egui::Direction::TopDown),
                        |ui| {
                            ui.add_space(5.0); // 顶部一点空间
                                               // ui.label(egui::RichText::new("Area 中的圆角矩形").color(Color32::WHITE).size(22.0));
                            ui.label(
                                egui::RichText::new("01:00:00")
                                    .family(egui::FontFamily::Proportional)
                                    .color(Color32::WHITE)
                                    .size(24.0),
                            );
                            ui.add_space(5.0); // 文本和按钮之间的空间
                                               // if ui.button("点击我").clicked() {
                                               //     println!("按钮在 Area 中被点击了!");
                                               // }
                        },
                    );
                });
            });
    }
}
