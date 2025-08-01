use std::{
    fs,
    sync::{
        atomic::{AtomicBool, Ordering},
        Mutex, MutexGuard,
    },
};

use crate::{
    enums::CarSetting,
    ui::index::{CAR_SETTING, CUR_CAR_RPM_SETTING, GAME_RACE_DATA, IS_MOUSE_PASS, IS_UDP_REDIRECT, SECTOR_RECORD_DATA}, uitl::{format_milliseconds_to_mmssms, format_milliseconds_to_mmssms2},
};
use eframe::egui::{self, Align, Area, FontId, Layout, Pos2, RichText, TextEdit};

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
                ui.add_space(10.0);
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
                                ui.horizontal(|ui| {
                                    let res = ui.checkbox(
                                        &mut app.setting_data.is_redirect,
                                        RichText::new("开启UDP转发(固定转发至8003端口)")
                                            .color(egui::Color32::WHITE)
                                            .font(egui::FontId::monospace(14.0)),
                                    );
                                    if res.changed() {
                                        IS_UDP_REDIRECT
                                            .get()
                                            .unwrap()
                                            .lock()
                                            .unwrap()
                                            .store(app.setting_data.is_redirect, Ordering::SeqCst);
                                    }
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

                render_other(ctx, app, ui);

                // 右侧内容：用 Spacer 推开，或者使用 with_layout(Align::Max)
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    ui.label(
                        egui::RichText::new("         极限竞速8 hud overlay")
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
                    RichText::new(r"F2           切换编辑状态, 编辑状态可拖动组件")
                        .color(egui::Color32::WHITE)
                        .font(egui::FontId::monospace(14.0)),
                );
                ui.label(
                    RichText::new(r"F10          整体隐藏或显示HUD")
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
        Area::new("complist".into())
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
                    ui.heading("组件选项");
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.checkbox(
                            &mut app.show_state.show_sector,
                            RichText::new("赛段计时")
                                .color(egui::Color32::WHITE)
                                .font(egui::FontId::proportional(14.0)),
                        );

                        ui.add_space(40.0);
                        ui.label(
                            RichText::new("长度")
                                .color(egui::Color32::WHITE)
                                .font(egui::FontId::proportional(14.0)),
                        );
                        ui.add(
                            TextEdit::singleline(&mut app.setting_data.sector_len)
                                .desired_width(80.0),
                        );
                    });

                    ui.horizontal(|ui| {
                        ui.checkbox(
                            &mut app.show_state.show_sight,
                            RichText::new("准星")
                                .color(egui::Color32::WHITE)
                                .font(egui::FontId::proportional(14.0)),
                        );

                        ui.add_space(68.0);
                        ui.label(
                            RichText::new("长度")
                                .color(egui::Color32::WHITE)
                                .font(egui::FontId::proportional(14.0)),
                        );
                        ui.add(
                            TextEdit::singleline(&mut app.setting_data.sight_len)
                                .desired_width(80.0),
                        );
                    });

                    ui.horizontal(|ui| {
                        ui.checkbox(
                            &mut app.show_state.show_dash,
                            RichText::new("仪表盘")
                                .color(egui::Color32::WHITE)
                                .font(egui::FontId::proportional(14.0)),
                        );

                        ui.add_space(68.0);
                        // ui.label(
                        //     RichText::new("长度")
                        //         .color(egui::Color32::WHITE)
                        //         .font(egui::FontId::proportional(14.0)),
                        // );
                        // ui.add(
                        //     TextEdit::singleline(&mut app.setting_data.dash_len)
                        //         .desired_width(80.0),
                        // );
                    });

                    ui.horizontal(|ui| {
                        ui.checkbox(
                            &mut app.show_state.show_history,
                            RichText::new("圈速历史")
                                .color(egui::Color32::WHITE)
                                .font(egui::FontId::proportional(14.0)),
                        );

                        ui.add_space(58.0);
                        ui.label(
                            RichText::new("长度")
                                .color(egui::Color32::WHITE)
                                .font(egui::FontId::proportional(14.0)),
                        );
                        ui.add(
                            TextEdit::singleline(&mut app.setting_data.history_len)
                                .desired_width(80.0),
                        );
                        ui.add_space(10.0);
                        ui.checkbox(
                            &mut app.show_state.show_opt_time,
                            RichText::new("Opt圈速")
                                .color(egui::Color32::WHITE)
                                .font(egui::FontId::proportional(14.0)),
                        );
                    });

                    // ui.horizontal(|ui| {
                    //     ui.checkbox(
                    //         &mut app.show_state.show_opt_time,
                    //         RichText::new("Opt圈速")
                    //             .color(egui::Color32::WHITE)
                    //             .font(egui::FontId::proportional(14.0)),
                    //     );

                    //     ui.add_space(68.0);
                    //     ui.label(
                    //         RichText::new("长度")
                    //             .color(egui::Color32::WHITE)
                    //             .font(egui::FontId::proportional(14.0)),
                    //     );
                    //     ui.add(
                    //         TextEdit::singleline(&mut app.setting_data.opt_time_len)
                    //             .desired_width(80.0),
                    //     );
                    // });

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

pub fn render_other(ctx: &egui::Context, app: &mut crate::ui::index::MyApp2, ui: &mut egui::Ui) {
    let btn = ui.button("☉ 其他");
    if btn.clicked() {
        app.show_state.show_other = !app.show_state.show_other;
    }
    let mut race_data = GAME_RACE_DATA.get().unwrap().lock().unwrap();
    let mut cur_car_rpm_setting = CUR_CAR_RPM_SETTING.get().unwrap().lock().unwrap();
    let mut car_setting = CAR_SETTING.get().unwrap().lock().unwrap();
    let id = if race_data.car_id == 0 {
        race_data.last_car_id
    } else {
        race_data.car_id
    } as u16;
    if id == 0 {
        return;
    } else {
        race_data.last_car_id = id as i32;
    }
    if car_setting.rpm.get(&id).is_none() {
        let mut vec = Vec::<String>::new();
        let maxr = race_data.max_rpm as i32;
        let red_rpm = (race_data.max_rpm * 0.9) as i32;
        vec.push(maxr.to_string());
        vec.push(red_rpm.to_string());
        car_setting.rpm.insert(id, vec);
    }
    if cur_car_rpm_setting.car_id == 0 || cur_car_rpm_setting.car_id != id {
        let car_rpm_item: &Vec<String> = car_setting.rpm.get(&id).unwrap();
        cur_car_rpm_setting.car_id = id;
        cur_car_rpm_setting.max_rpm = car_rpm_item[0].clone();
        cur_car_rpm_setting.red_rpm = car_rpm_item[1].clone();
    }

    if app.show_state.show_other {
        let screen_rect = ctx.screen_rect();
        let  sector_data = SECTOR_RECORD_DATA.get().unwrap().lock().unwrap();
        let opt_best_time = sector_data.s1.best_time + sector_data.s2.best_time + sector_data.s3.best_time;
        let opt_best_time = format_milliseconds_to_mmssms((opt_best_time * 1000.0) as u32);
        let s1_time = format_milliseconds_to_mmssms2((sector_data.s1.best_time * 1000.0) as u32,true);
        let s2_time = format_milliseconds_to_mmssms2((sector_data.s2.best_time * 1000.0) as u32,true);
        let s3_time = format_milliseconds_to_mmssms2((sector_data.s3.best_time * 1000.0) as u32,true);

        let pos = Pos2::new(screen_rect.right() + 100.0, screen_rect.top() + 28.0);
        Area::new("setting_other".into())
            .current_pos(pos)
            .show(ctx, |ui| {
                // 创建一个自定义的 Frame 样式
                let frame = egui::Frame::window(&ctx.style())
                    // .widget_rect(Rect::new(0.0, 0.0, 300.0, 300.0))
                    .fill(egui::Color32::from_rgb(50, 50, 50)) // 矩形背景颜色
                    .stroke(egui::Stroke::new(2.0, egui::Color32::LIGHT_BLUE)) // 边框
                    .corner_radius(5.0) // 圆角
                    .inner_margin(10.0); // 内部边距

                // 在这个 Frame 中绘制内容
                frame.show(ui, |ui| {
                    ui.heading("其他信息");
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("当前赛道Id: ")
                                .color(egui::Color32::WHITE)
                                .font(FontId::monospace(14.0)),
                        );

                        ui.label(
                            RichText::new(race_data.track_id.to_string())
                                .color(egui::Color32::WHITE)
                                .font(FontId::monospace(14.0)),
                        );
                        // ui.checkbox(&mut app.show_state.show_sector, RichText::new("当前赛道Id")
                        // .color(egui::Color32::WHITE)
                        // .font(egui::FontId::proportional(14.0)),);

                        // ui.add_space(40.0);
                        // ui.label(RichText::new("长度")
                        // .color(egui::Color32::WHITE)
                        // .font(egui::FontId::proportional(14.0)));
                        // ui.add(TextEdit::singleline(&mut app.setting_data.sector_len).desired_width(80.0));
                    });
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("当前车辆Id: ")
                                .color(egui::Color32::WHITE)
                                .font(FontId::monospace(14.0)), // .family(egui::FontFamily::Name("base".into()))
                                                                // .size(14.0),
                        );

                        ui.label(
                            RichText::new(id.to_string())
                                .color(egui::Color32::WHITE)
                                .font(FontId::monospace(14.0)),
                        );

                        ui.add_space(10.0);
                        ui.label(
                            RichText::new("最高转速")
                                .color(egui::Color32::WHITE)
                                .font(egui::FontId::monospace(14.0)),
                        );
                        let res = ui.add(
                            TextEdit::singleline(&mut cur_car_rpm_setting.max_rpm)
                                .desired_width(70.0),
                        );

                        ui.add_space(10.0);
                        ui.label(
                            RichText::new("红线转速")
                                .color(egui::Color32::WHITE)
                                .font(egui::FontId::monospace(14.0)),
                        );
                        let res = ui.add(
                            TextEdit::singleline(&mut cur_car_rpm_setting.red_rpm)
                                .desired_width(70.0),
                        );

                        ui.add_space(10.0);
                        let btn = ui.button("保存");
                        if btn.clicked() {
                            car_setting.rpm.insert(
                                id,
                                vec![
                                    cur_car_rpm_setting.max_rpm.clone(),
                                    cur_car_rpm_setting.red_rpm.clone(),
                                ],
                            );
                            tokio::spawn(async move { save_car_json() });
                        }
                    });

                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("上一圈胎耗: ")
                                .color(egui::Color32::WHITE)
                                .font(FontId::monospace(14.0)), // .family(egui::FontFamily::Name("base".into()))
                                                                // .size(14.0),
                        );
                        ui.add_space(10.0);
                        ui.label(
                            RichText::new(format!(
                                " {:.1} / {:.1} / {:.1} / {:.1}",
                                race_data.last_save_lap_tire_wear1,
                                race_data.last_save_lap_tire_wear2,
                                race_data.last_save_lap_tire_wear3,
                                race_data.last_save_lap_tire_wear4
                            ))
                            .color(egui::Color32::WHITE)
                            .font(FontId::monospace(14.0)), // .family(egui::FontFamily::Name("base".into()))
                                                            // .size(14.0),
                        );
                    });


                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("赛段Opt: ")
                                .color(egui::Color32::WHITE)
                                .font(FontId::monospace(14.0)), // .family(egui::FontFamily::Name("base".into()))
                                                                // .size(14.0),
                        );
                        ui.add_space(10.0);
                        ui.label(
                            RichText::new(format!(
                                " {} / {} / {} / {}",
                                s1_time,
                                s2_time,
                                s3_time,
                                opt_best_time
                            ))
                            .color(egui::Color32::WHITE)
                            .font(FontId::monospace(14.0)), // .family(egui::FontFamily::Name("base".into()))
                                                            // .size(14.0),
                        );
                    });


                    ui.separator();

                    ui.horizontal(|ui| {
                        ui.add_space(460.0);

                        if ui.button("关闭").clicked() {
                            app.show_state.show_other = false;
                        }
                    });
                });
            });
    }
}

pub fn load_car_json() {
    let json_string_read = fs::read_to_string("car_config.json");
    let json_string_read = match json_string_read {
        Ok(s) => s,
        Err(_) => "".to_string(),
    };
    let app_settings_read: CarSetting = if json_string_read.len() > 0 {
        let res = serde_json::from_str(&json_string_read);
        let res = match res {
            Ok(s) => s,
            Err(_) => CarSetting::default(),
        };
        res
    } else {
        CarSetting::default()
    };
    CAR_SETTING.set(Mutex::new(app_settings_read)).unwrap();
}

pub fn save_car_json() {
    let app_settings: MutexGuard<'_, CarSetting> = CAR_SETTING.get().unwrap().lock().unwrap();
    let app_settings: CarSetting = app_settings.clone();
    let json_string = serde_json::to_string_pretty(&app_settings).unwrap();
    fs::write("car_config.json", json_string).unwrap();
}
