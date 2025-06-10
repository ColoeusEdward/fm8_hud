use std::sync::{
    atomic::{AtomicBool, Ordering},
    Mutex,
};

use eframe::{
    egui::{self, Area, Color32, Layout, UiBuilder, ViewportCommand},
    epaint::CornerRadiusF32,
};

use crate::ui::index::{MyApp2, IS_MOUSE_PASS, SECTORID};

pub fn render_sector(ctx: &egui::Context, app: &mut MyApp2) {
    if !app.show_state.show_sector {
        return;
    }
    let res = Area::new(*SECTORID.get().unwrap())
        .current_pos(egui::pos2(app.sector_pos.x, app.sector_pos.y)) // 位置, 400.0 + app.yoffset)) // 位置
        .movable(true) //
        .show(ctx, |ui| {
            // force_check_fullscreen(ui, ctx);

            // 这个文本会直接显示在透明的 viewport 上，没有任何背景

            // 定义圆角矩形的尺寸
            let len = app.setting_data.sector_len.parse::<f32>();
            let len = match len {
                Ok(len) => len,
                Err(e) => app.setting_data.sector_base_len,
            };
            let scale_to_base = len / app.setting_data.sector_base_len;
            let desired_size = egui::vec2(len, len / app.setting_data.sector_scale);
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
                        ui.add_space(5.0 * scale_to_base); // 顶部一点空间
                                                           // ui.label(egui::RichText::new("Area 中的圆角矩形").color(Color32::WHITE).size(22.0));
                        let lb = ui.label(
                            egui::RichText::new("01:00:00")
                                .family(egui::FontFamily::Proportional)
                                .color(Color32::WHITE)
                                .size(24.0 * scale_to_base),
                        );
                        if lb.dragged() {
                            app.sector_pos += lb.drag_delta();
                            // println!("🪵 [sector.rs:65]~ token ~ \x1b[0;32msector_pos\x1b[0m = {} {}", app.sector_pos.x,app.sector_pos.y,);
                        }
                        ui.add_space(5.0 * scale_to_base); // 文本和按钮之间的空间
                                                           // if ui.button("点击我").clicked() {
                                                           //     println!("按钮在 Area 中被点击了!");
                                                           // }
                    },
                );
            });
        })
        .response;

    // 如果 Area 被拖动，更新其位置
    if res.dragged() {}
}

pub fn render_white_overlay(ctx: &egui::Context, app: &mut MyApp2) {
    Area::new("white_overlay".into())
        .fixed_pos(egui::pos2(0.0, 0.0))
        .show(ctx, |ui| {
            // 定义圆角矩形的尺寸
            let desired_size = egui::vec2(1920.0, 1080.0);
            // 分配一个精确大小的区域，这将是我们绘制矩形的边界
            let (rect, _response) = ui.allocate_exact_size(desired_size, egui::Sense::hover());

            // 获取 painter
            let painter = ui.painter();

            // 定义填充颜色: #A2000000 (ARGB) -> 64% 透明度的黑色 (RGBA: 0,0,0,162)
            let fill_color = Color32::from_rgba_unmultiplied(255, 255, 255, 80);

            // 定义圆角半径
            let corner_radius = 0.0; // 较大的圆角，更明显

            // 绘制填充的圆角矩形
            painter.rect_filled(
                rect,
                CornerRadiusF32::same(corner_radius), // 所有角的圆角半径相同
                fill_color,
            );
        });
}

pub fn test_transparent(ctx: &egui::Context, app: &mut MyApp2) {
    ctx.send_viewport_cmd(ViewportCommand::Transparent(true));
    egui::CentralPanel::default()
        .frame(egui::Frame::default().fill(Color32::TRANSPARENT))
        .show(ctx, |ui| {
            ui.label(format!("fullscreen: {}", app.fullscreen));
            if ui.button("toggle fullscreen").clicked() {
                app.fullscreen = !app.fullscreen;
                ctx.send_viewport_cmd(ViewportCommand::Fullscreen(app.fullscreen));
            }

            ui.label(format!("transparent: {}", app.transparent));
            if ui.button("toggle transparency").clicked() {
                app.transparent = !app.transparent;
                ctx.send_viewport_cmd(ViewportCommand::Transparent(app.transparent));
            }
        });
}

pub fn render_sight(ctx: &egui::Context, app: &mut MyApp2) {
    if !app.show_state.show_sight {
        return;
    }
    let pos = if app.sight_pos.x == 0.0 && app.sight_pos.y == 0.0 {
        let pp = get_center_pos(ctx);
        app.sight_pos = pp;
        pp
    } else {
        app.sight_pos
    };
    let size = app.setting_data.sight_len.parse::<f32>();
    let size = match size {
        Ok(s) => s,
        Err(e) => app.setting_data.sight_base_len,
    };
    Area::new("sight".into())
        .current_pos(pos)
        .movable(true)
        .show(ctx, |ui| {
            // 使用 with_layout 来将内容水平和垂直居中
            ui.with_layout(
                Layout::centered_and_justified(egui::Direction::TopDown),
                |ui| {
                    // 你可以在这里添加任何你想要居中的内容
                    let lb = ui.label(
                        egui::RichText::new("o")
                            .font(egui::FontId::monospace(size)) // 调整字体大小
                            .color(Color32::from_hex("#00FFFF").expect("hex error")), // 设置文字颜色
                    );
                    if lb.dragged() {
                        app.sight_pos += lb.drag_delta();
                    }
                    // 如果需要，可以添加一个加载指示器
                    // ui.spinner();
                },
            );
        });
}

pub fn render_cross_line(ctx: &egui::Context) {
    let is_mouse_pass = IS_MOUSE_PASS
        .get_or_init(|| Mutex::new(AtomicBool::new(true)))
        .lock()
        .unwrap()
        .load(Ordering::SeqCst);
    if is_mouse_pass {
        return;
    }
    // 获取屏幕尺寸
    let screen_rect = ctx.screen_rect();
    let center = screen_rect.center();
    let width = screen_rect.max.x - screen_rect.min.x;

    // 获取 Painter，用于直接绘图
    let painter = ctx.layer_painter(egui::LayerId::new(
        egui::Order::Foreground,
        egui::Id::new("crosshair"),
    ));

    let line_color = egui::Color32::from_rgba_premultiplied(0, 0, 0, 80);
    let thickness = 1.0;
    let len = width / 2.0; // 十字线长度（从中心往两边）

    // 画水平线
    painter.line_segment(
        [
            egui::pos2(center.x - len, center.y),
            egui::pos2(center.x + len, center.y),
        ],
        egui::Stroke::new(thickness, line_color),
    );

    // 画垂直线
    painter.line_segment(
        [
            egui::pos2(center.x, center.y - len),
            egui::pos2(center.x, center.y + len),
        ],
        egui::Stroke::new(thickness, line_color),
    );

    // // 可选：正常 UI
    // egui::CentralPanel::default().show(ctx, |ui| {
    //     // ui.label("屏幕中央有一个十字辅助线");
    // });
}

pub fn get_center_pos(ctx: &egui::Context) -> egui::Pos2 {
    // 获取当前 egui 视窗的尺寸
    let screen_rect = ctx.screen_rect();
    let screen_center = screen_rect.center();
    println!(
        "🪵 [sector.rs:152]~ token ~ \x1b[0;32mscreen_center\x1b[0m = {} {}",
        screen_center.x, screen_center.y
    );
    return screen_center;
}

pub fn render_bg(
    ctx: &egui::Context,
    ui: &mut egui::Ui,
    size: [f32; 2],
    add_contents: impl FnOnce(&mut egui::Ui) -> (),
) {
    // 定义圆角矩形的尺寸
    let desired_size = egui::vec2(size[0], size[1]);
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

    ui.allocate_new_ui(UiBuilder::new().max_rect(rect), |ui| {
        ui.with_layout(
            egui::Layout::centered_and_justified(egui::Direction::TopDown),
            |ui| {
                add_contents(ui);
                // ui.heading("用户信息表单");

                // ui.separator();

                // ui.horizontal(|ui| {
                //     ui.label("游戏遥测IP:");
                //     ui.text_edit_singleline(&mut app.setting_data.ip);
                // });

                // ui.horizontal(|ui| {
                //     ui.label("游戏遥测端口:");
                //     ui.text_edit_singleline(&mut app.setting_data.port);
                // });

                // ui.separator();

                // if ui.button("提交").clicked() {
                //     println!("游戏遥测IP: {}", app.setting_data.ip);
                //     // println!("邮箱: {}", self.email);
                //     // println!("订阅: {}", self.subscribe);
                // }
            },
        )
    });
}
