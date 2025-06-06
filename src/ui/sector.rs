
use eframe::{
    egui::{self, Area, Color32, Layout,  UiBuilder, ViewportCommand},
    epaint::CornerRadiusF32,
};

use crate::{ui::index::{MyApp, SECTORID}};

pub fn render_sector(ctx: &egui::Context, app: &mut MyApp) {
    let res = Area::new(*SECTORID.get().unwrap())
        .current_pos(egui::pos2(app.sector_pos.x, app.sector_pos.y)) // 位置, 400.0 + app.yoffset)) // 位置
        .movable(true) //
        .show(ctx, |ui| {
            // force_check_fullscreen(ui, ctx);

            // 这个文本会直接显示在透明的 viewport 上，没有任何背景

            // 定义圆角矩形的尺寸
            let desired_size = egui::vec2(210.0, 40.0);
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
                        let lb = ui.label(
                            egui::RichText::new("01:00:00")
                                .family(egui::FontFamily::Proportional)
                                .color(Color32::WHITE)
                                .size(24.0),
                        );
                        if lb.dragged() {
                            app.sector_pos += lb.drag_delta();
                            // println!("🪵 [sector.rs:65]~ token ~ \x1b[0;32msector_pos\x1b[0m = {} {}", app.sector_pos.x,app.sector_pos.y,);
                        }
                        ui.add_space(5.0); // 文本和按钮之间的空间
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

pub fn render_white_overlay(ctx: &egui::Context, app: &mut MyApp) {
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

pub fn test_transparent(ctx: &egui::Context, app: &mut MyApp) {
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

pub fn render_sight(ctx: &egui::Context, app: &mut MyApp) {
    let pos = if app.sight_pos.x == 0.0 && app.sight_pos.y == 0.0 {
        let pp = get_center_pos(ctx);
        app.sight_pos = pp;
        pp
    }else{
        app.sight_pos
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
                            .font(egui::FontId::monospace(14.0)) // 调整字体大小
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

pub fn get_center_pos(ctx: &egui::Context) -> egui::Pos2 {
    // 获取当前 egui 视窗的尺寸
    let screen_rect = ctx.screen_rect();
    let screen_center = screen_rect.center();
    return screen_center;
}