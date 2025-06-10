use std::sync::{
    atomic::{AtomicBool, Ordering},
    Mutex,
};

use eframe::egui::{
        self, Align, Area, Layout, Pos2, RichText, TextEdit
    };
use crate::ui::index::IS_MOUSE_PASS;

pub fn render_setting(ctx: &egui::Context, app: &mut crate::ui::index::MyApp2) {
    let is_mouse_pass = IS_MOUSE_PASS
        .get_or_init(|| Mutex::new(AtomicBool::new(true)))
        .lock()
        .unwrap()
        .load(Ordering::SeqCst);
    if is_mouse_pass {
        return;
    }

    // 顶部工具栏
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            ui.horizontal(|ui| {
                let btn = ui.button("⚙ 设置");
                // 设置按钮
                if btn.clicked() {
                    let show_set = app.show_state.show_setting;
                    app.show_state.show_setting = !show_set;
                }
                if app.show_state.show_setting {
                    let screen_rect = ctx.screen_rect();
                    let pos = Pos2::new(screen_rect.right() + 100.0, screen_rect.top() + 28.0);
                    Area::new("setting".into())
                        .current_pos(pos)
                        .show(ctx, |ui| {
                            // 创建一个自定义的 Frame 样式
                            let frame = egui::Frame::window(&ctx.style())
                                .fill(egui::Color32::from_rgb(50, 50, 50)) // 矩形背景颜色
                                .stroke(egui::Stroke::new(2.0, egui::Color32::LIGHT_BLUE)) // 边框
                                .corner_radius(5.0) // 圆角
                                .inner_margin(10.0); // 内部边距

                            // 在这个 Frame 中绘制内容
                            frame.show(ui, |ui| {
                                ui.heading("配置");
                                ui.separator();

                                ui.horizontal(|ui| {
                                    ui.label(
                                        RichText::new("游戏遥测IP:")
                                            .color(egui::Color32::WHITE)
                                            .font(egui::FontId::monospace(14.0)),
                                    );
                                    ui.text_edit_singleline(&mut app.setting_data.ip);
                                });

                                ui.horizontal(|ui| {
                                    ui.label(
                                        RichText::new("游戏遥测端口:")
                                            .color(egui::Color32::WHITE)
                                            .font(egui::FontId::proportional(14.0)),
                                    );
                                    ui.add(TextEdit::singleline(&mut app.setting_data.port));
                                });

                                ui.separator();

                                ui.horizontal(|ui| {
                                    ui.add_space(340.0);

                                    // if ui.button("提交").clicked() {
                                    //     println!("游戏遥测IP: {}", app.setting_data.ip);
                                    //     // println!("邮箱: {}", self.email);
                                    //     // println!("订阅: {}", self.subscribe);
                                    // }
                                    ui.spacing();

                                    if ui.button("关闭").clicked() {
                                        app.show_state.show_setting = false;
                                    }
                                });
                            });
                        });
                }

                render_info(ctx, app, ui);

                render_complist(ctx, app, ui);

                // 右侧内容：用 Spacer 推开，或者使用 with_layout(Align::Max)
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    ui.label(
                        egui::RichText::new("极限竞速8 hud overlay")
                            .font(egui::FontId::monospace(14.0)), // 调整字体大小
                                                                  // .color(Color32::from_hex("#00FFFF").expect("hex error")), // 设置文字颜色
                    );
                });
            });
        });
    });
}

pub fn render_info(ctx: &egui::Context, app: &mut crate::ui::index::MyApp2, ui: &mut egui::Ui) {
    let btn = ui.button("☰ 说明");
    if btn.clicked() {
        let show_set = app.show_state.show_info;
        app.show_state.show_info = !show_set;
    }
    if app.show_state.show_info {
        let screen_rect = ctx.screen_rect();
        let pos = Pos2::new(screen_rect.right() + 100.0, screen_rect.top() + 28.0);
        Area::new("info".into()).current_pos(pos).show(ctx, |ui| {
            // 创建一个自定义的 Frame 样式
            let frame = egui::Frame::window(&ctx.style())
                .fill(egui::Color32::from_rgb(50, 50, 50)) // 矩形背景颜色
                .stroke(egui::Stroke::new(2.0, egui::Color32::LIGHT_BLUE)) // 边框
                .corner_radius(5.0) // 圆角
                .inner_margin(10.0); // 内部边距

            // 在这个 Frame 中绘制内容
            frame.show(ui, |ui| {
                ui.heading("说明");
                ui.separator();

                ui.label(
                    RichText::new(r"F2          切换编辑状态, 编辑状态可拖动组件")
                        .color(egui::Color32::WHITE)
                        .font(egui::FontId::monospace(14.0)),
                );
                ui.label(
                    RichText::new(r"Ctrl+Q      退出程序")
                        .color(egui::Color32::WHITE)
                        .font(egui::FontId::monospace(14.0)),
                );

                ui.separator();

                ui.horizontal(|ui| {
                    ui.add_space(320.0);

                    if ui.button("关闭").clicked() {
                        app.show_state.show_info = false;
                    }
                });
            });
        });
    }
}

pub fn render_complist(ctx: &egui::Context, app: &mut crate::ui::index::MyApp2, ui: &mut egui::Ui) {
    let btn = ui.button("☉ 组件");
    if btn.clicked() {
        let show_set = app.show_state.show_complist;
        app.show_state.show_complist = !show_set;
    }
    if app.show_state.show_complist {
        let screen_rect = ctx.screen_rect();
        let pos = Pos2::new(screen_rect.right() + 150.0, screen_rect.top() + 28.0);
        Area::new("complist".into()).current_pos(pos).show(ctx, |ui| {
            // 创建一个自定义的 Frame 样式
            let frame = egui::Frame::window(&ctx.style())
                .fill(egui::Color32::from_rgb(50, 50, 50)) // 矩形背景颜色
                .stroke(egui::Stroke::new(2.0, egui::Color32::LIGHT_BLUE)) // 边框
                .corner_radius(5.0) // 圆角
                .inner_margin(10.0); // 内部边距

            // 在这个 Frame 中绘制内容
            frame.show(ui, |ui| {
                ui.heading("组件选项");
                ui.separator();
                ui.horizontal(|ui| {
                    ui.checkbox(&mut app.show_state.show_sector, RichText::new("赛段计时")
                    .color(egui::Color32::WHITE)
                    .font(egui::FontId::proportional(14.0)),);
                    // ui.add_space(5.0);
                    // if ui.button("↑").clicked() {

                    // }
                    // if ui.button("↓").clicked() {

                    // }
                    // if ui.button("←").clicked() {
                        
                    // }
                    // if ui.button("→").clicked() {
                        
                    // }
                    ui.add_space(40.0);
                    ui.label(RichText::new("长度")
                    .color(egui::Color32::WHITE)
                    .font(egui::FontId::proportional(14.0)));
                    ui.add(TextEdit::singleline(&mut app.setting_data.sector_len).desired_width(80.0));
                    
                });

                ui.horizontal(|ui| {
                    ui.checkbox(&mut app.show_state.show_sight, RichText::new("准星")
                    .color(egui::Color32::WHITE)
                    .font(egui::FontId::proportional(14.0)),);
                    // ui.add_space(5.0);
                    // if ui.button("↑").clicked() {

                    // }
                    // if ui.button("↓").clicked() {

                    // }
                    // if ui.button("←").clicked() {
                        
                    // }
                    // if ui.button("→").clicked() {
                        
                    // }
                    ui.add_space(68.0);
                    ui.label(RichText::new("长度")
                    .color(egui::Color32::WHITE)
                    .font(egui::FontId::proportional(14.0)));
                    ui.add(TextEdit::singleline(&mut app.setting_data.sight_len).desired_width(80.0));
                });

                ui.separator();

                ui.horizontal(|ui| {
                    ui.add_space(320.0);

                    if ui.button("关闭").clicked() {
                        app.show_state.show_complist = false;
                    }
                });
            });
        });
    }
}
