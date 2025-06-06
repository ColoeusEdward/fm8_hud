use eframe::{
    egui::{self, Area, Color32,UiBuilder, ViewportCommand},
    epaint::CornerRadiusF32,
};

use crate::ui::index::{MyApp, SECTORID};



pub fn render_sector(ctx: &egui::Context, app: &mut MyApp) {
    let res = Area::new(*SECTORID.get().unwrap())
        .current_pos(egui::pos2(app.sector_pos.x, app.sector_pos.y)) // 位置, 400.0 + app.yoffset)) // 位置
        // .movable(true) //
        .show(ctx, |ui| {
            // force_check_fullscreen(ui, ctx);

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
        }).response;

        // 如果 Area 被拖动，更新其位置
        if res.dragged() {
          app.sector_pos += res.drag_delta();
        }
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
