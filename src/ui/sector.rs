use std::{
    collections::BTreeMap,
    sync::{
        atomic::{AtomicBool, Ordering},
        Mutex, MutexGuard,
    },
    time::Duration,
};

use eframe::{
    egui::{self, Area, Color32, Layout, UiBuilder, ViewportCommand},
    epaint::CornerRadiusF32,
};

use crate::{
    config::{get_track_data_map, TrackData},
    enums::{GameRaceData, LapControl, SectorRecord},
    ui::index::{
        MyApp2,  GAME_RACE_DATA, IS_MOUSE_PASS, LAST_TELE_DATA, SECTORID,
        SECTOR_RECORD_DATA,
    },
    uitl::{format_milliseconds_to_mmssms, get_now_ts},
};

pub fn render_sector(ctx: &egui::Context, app: &mut MyApp2) {
    if !app.show_state.show_sector {
        return;
    }
    let tele_data = LAST_TELE_DATA.get().unwrap().lock().unwrap();
    let cur_lap_time = tele_data.get("CurrentLap");
    let cur_lap_time = match cur_lap_time {
        Some(cur_lap_time) => cur_lap_time * 1000.0,
        None => 0.0,
    } as u32;
    let test_lap = cur_lap_time;
    let cur_lap_time = format_milliseconds_to_mmssms(cur_lap_time);

    let is_mouse_pass = IS_MOUSE_PASS
        .get()
        .unwrap()
        .lock()
        .unwrap()
        .load(Ordering::SeqCst);

    // let is_race_on = tele_data.get("IsRaceOn");
    // let is_race_on = match is_race_on {
    //     Some(is_race_on) => is_race_on,
    //     None => &0.0,
    // }.clone() as i32;
    if cur_lap_time == "00:00:000" && is_mouse_pass {
        // let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        // println!("🪵 [sector.rs:48]~ token ~ \x1b[0;32mcur_lap_time\x1b[0m = {} {} {}", cur_lap_time,ts,test_lap);
        return;
    }
    let (sector_time, delta_show, delta) = sector_logic2(&tele_data);
    let mut scale_to_base_s: f32 = 1.0;
    // println!("🪵 [sector.rs:17]~ token ~ \x1b[0;32mtele_data\x1b[0m = {}", is_race_on);
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
            scale_to_base_s = scale_to_base;
            let desired_size = egui::vec2(len, len / app.setting_data.sector_scale);
            // 分配一个精确大小的区域，这将是我们绘制矩形的边界
            let (rect, _response) = ui.allocate_exact_size(desired_size, egui::Sense::hover());

            // 获取 painter
            let painter = ui.painter();

            // 定义填充颜色: #A2000000 (ARGB) -> 64% 透明度的黑色 (RGBA: 0,0,0,162)
            let fill_color = Color32::from_rgba_premultiplied(0, 0, 0, 80);

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
                        ui.add_space(7.0 * scale_to_base); // 顶部一点空间
                                                           // ui.label(egui::RichText::new("Area 中的圆角矩形").color(Color32::WHITE).size(22.0));
                        let lb = ui.label(
                            egui::RichText::new(sector_time)
                                .family(egui::FontFamily::Proportional)
                                .color(Color32::WHITE)
                                .size(20.0 * scale_to_base),
                        );
                        if lb.dragged() {
                            app.sector_pos += lb.drag_delta();
                            app.sector_delta_pos += lb.drag_delta();
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

    if delta_show {
        let delta = Area::new("delta".into())
            .current_pos(egui::pos2(
                app.sector_pos.x + 220.0 * scale_to_base_s,
                app.sector_pos.y,
            ))
            .show(ctx, |ui| {
                // 定义圆角矩形的尺寸
                let len = app.setting_data.sector_delta_len * scale_to_base_s;

                let scale_to_base = scale_to_base_s;
                let desired_size = egui::vec2(len, len / app.setting_data.sector_delta_scale);
                // 分配一个精确大小的区域，这将是我们绘制矩形的边界
                let (rect, _response) = ui.allocate_exact_size(desired_size, egui::Sense::hover());

                // 获取 painter
                let painter = ui.painter();

                // 定义填充颜色: #A2000000 (ARGB) -> 64% 透明度的黑色 (RGBA: 0,0,0,162)
                let fill_color = if delta.contains("+") {
                    Color32::from_rgba_premultiplied(177, 45, 44, 255)
                } else {
                    Color32::from_rgba_premultiplied(44, 153, 50, 255)
                };
                // let fill_color = Color32::from_rgba_premultiplied(177, 45, 44 ,128);

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
                            ui.add_space(10.5 * scale_to_base); // 顶部一点空间
                                                                // ui.label(egui::RichText::new("Area 中的圆角矩形").color(Color32::WHITE).size(22.0));
                            let lb = ui.label(
                                egui::RichText::new(delta)
                                    .family(egui::FontFamily::Name("base".into()))
                                    .color(Color32::WHITE)
                                    .weak()
                                    .size(24.0 * scale_to_base),
                            );

                            ui.add_space(5.0 * scale_to_base); // 文本和按钮之间的空间
                                                               // if ui.button("点击我").clicked() {
                                                               //     println!("按钮在 Area 中被点击了!");
                                                               // }
                        },
                    );
                });
            })
            .response;
    }

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

pub fn sector_logic2(tele_data: &MutexGuard<BTreeMap<String, f32>>) -> (String, bool, String) {
    //return (sector_time,delta_show,delta)
    let mut sector_data = SECTOR_RECORD_DATA.get().unwrap().lock().unwrap();
    // let game_race_data = GAME_RACE_DATA.get().unwrap().lock().unwrap();
    update_race_data(tele_data);
    let mut race_data = GAME_RACE_DATA.get().unwrap().lock().unwrap();

    let track_info = get_track_data_map(&race_data.track_id);
    // println!("🪵 [sector.rs:398]~ token ~ \x1b[0;32m&race_data.track_id\x1b[0m = {}", &race_data.track_id);
    let cur_sector_time = race_data.race_time - race_data.current_time;
    if race_data.race_time <= 0.3 {
        println!(
            "🪵 [sector.rs:401]~ token ~ \x1b[0;32mrace_data.race_time <= 0.3 \x1b[0m = {}",
            race_data.race_time <= 0.3
        );
        reset_lap_control(&mut sector_data.s1);
        reset_lap_control(&mut sector_data.s2);
        reset_lap_control(&mut sector_data.s3);
        return (
            format_milliseconds_to_mmssms((race_data.current_time * 1000.0) as u32),
            false,
            "0.00".to_string(),
        );
    }
    if race_data.track_id != sector_data.s1.last_track_id {
        change_track(&mut sector_data.s1, &race_data);
        change_track(&mut sector_data.s2, &race_data);
        change_track(&mut sector_data.s3, &race_data);
    }
    if !sector_data.s1.initialized {
        init_s_record(&mut sector_data.s1, &track_info, 1);
    }
    if !sector_data.s2.initialized {
        init_s_record(&mut sector_data.s2, &track_info, 2);
    }
    if !sector_data.s3.initialized {
        // init_s_record(&mut sector_data.s2, &track_info, 2);
        init_s_record(&mut sector_data.s3, &track_info, 3);
    }

    //---------------------
    if race_data.lap == 0 && race_data.distance == -(track_info.length as f64) {
        //跳圈检测
        sector_data.s1.jumped_lap = -1;
        sector_data.s2.jumped_lap = -1;
        sector_data.s3.jumped_lap = -1;
        // sector_data.s3.jumped_lap = -1;
    } else if race_data.distance < -100.0 {
        // println!(
        //     "🪵 [sector.rs:437]~ token ~ \x1b[0;32m-100.0 \x1b[0m = {}",
        //     -100
        // );
        //数据重置

        set_lap_control_when_nega_distence(&mut sector_data.s1, &race_data, 1);
        set_lap_control_when_nega_distence(&mut sector_data.s2, &race_data, 2);
        set_lap_control_when_nega_distence(&mut sector_data.s3, &race_data, 3);
    }
    //-------------------------------------------------

    if race_data.current_time > 0.0 {
        if race_data.current_lap > race_data.sub_current_lap && race_data.distance > 0.0 {
            race_data.sub_current_lap = race_data.current_lap;
            race_data.sub_distance = race_data.distance;
            // println!("🪵 [sector.rs:458]~ token ~ \x1b[0;32mrace_data.distance;\x1b[0m = {}", race_data.distance);
        }
        if race_data.distance < race_data.sub_distance {
            race_data.sub_distance = 0.0;
        }
        let distence = race_data.distance - race_data.sub_distance;
        // println!(
        //     "🪵 [sector.rs:460]~ token ~ \x1b[0;32mdistence\x1b[0m = {} {}",
        //     // race_data.distance,
        //     distence,race_data.current_time
        // );

        // let distence = if race_data.distance > track_info.length as f64 {
        //     race_data.distance - track_info.length as f64 * (race_data.lap - 1) as f64
        // } else {
        //     race_data.distance
        // };
        // println!("🪵 [sector.rs:454]~ token ~ \x1b[0;32mdistence\x1b[0m = {}", track_info.length as f64 * race_data.current_lap as f64);
        if distence > 0.0 && distence < track_info.s1_end as f64 {
            sector_data.s1.is_done = false;

                // println!("🪵 [sector.rs:477]~ token ~ \x1b[0;32mace_data.lap \x1b[0m = {}", race_data.lap );
            if !sector_data.s3.is_done && race_data.lap > 1 {
                // println!("🪵 [sector.rs:477]~ token ~ \x1b[0;32mace_data.lap \x1b[0m = {}", race_data.lap );
                sector_data.s3.is_done = true;
                sector_data.s3.delta = sector_data.s3.current_s_time - sector_data.s3.best_time;
                if sector_data.s3.current_s_time < sector_data.s3.best_time {
                    sector_data.s3.sub_best_time = sector_data.s3.best_time;
                    sector_data.s3.best_time = sector_data.s3.current_s_time;
                }
                sector_data.s3.time_shown = true;
                println!("🪵 [sector.rs:507]~ token ~ \x1b[0;32msector_data.s1.delta\x1b[0m = s3/{}", sector_data.s3.delta);

                tokio::spawn(async move {
                    tokio::time::sleep(Duration::from_millis(5000)).await;
                    let mut sector_data = SECTOR_RECORD_DATA.get().unwrap().lock().unwrap();
                    sector_data.s3.time_shown = false;
                });
            }

            sector_data.s1.current_s_time = race_data.current_time;
        } else if distence >= track_info.s1_end as f64 && distence < track_info.s2_end as f64 {
            // sector_data.s1.is_done = true;
            sector_data.s3.is_done = false;
            sector_data.s2.is_done = false;

            if !sector_data.s1.is_done {
                sector_data.s1.is_done = true;
                sector_data.s1.delta = sector_data.s1.current_s_time - sector_data.s1.best_time;
                if sector_data.s1.current_s_time < sector_data.s1.best_time {
                    sector_data.s1.sub_best_time = sector_data.s1.best_time;
                    sector_data.s1.best_time = sector_data.s1.current_s_time;
                }
                sector_data.s1.time_shown = true;
                println!("🪵 [sector.rs:507]~ token ~ \x1b[0;32msector_data.s1.delta\x1b[0m = s1/{}", sector_data.s1.delta);
                tokio::spawn(async move {
                    tokio::time::sleep(Duration::from_millis(5000)).await;
                    let mut sector_data = SECTOR_RECORD_DATA.get().unwrap().lock().unwrap();
                    sector_data.s1.time_shown = false;
                });
            }

            let sector_start_time = sector_data.s1.current_s_time;
            sector_data.s2.current_s_time = race_data.current_time - sector_start_time;
        } else if distence >= track_info.s2_end as f64 && distence < track_info.length as f64 {
            // sector_data.s2.is_done = true;

            if !sector_data.s2.is_done {
                sector_data.s2.is_done = true;
                sector_data.s2.delta = sector_data.s2.current_s_time - sector_data.s2.best_time;
                if sector_data.s2.current_s_time < sector_data.s2.best_time {
                    sector_data.s2.sub_best_time = sector_data.s2.best_time;
                    sector_data.s2.best_time = sector_data.s2.current_s_time;
                }
                sector_data.s2.time_shown = true;
                println!("🪵 [sector.rs:507]~ token ~ \x1b[0;32msector_data.s1.delta\x1b[0m = s2/{}", sector_data.s2.delta);
                tokio::spawn(async move {
                    tokio::time::sleep(Duration::from_millis(5000)).await;
                    let mut sector_data = SECTOR_RECORD_DATA.get().unwrap().lock().unwrap();
                    sector_data.s2.time_shown = false;
                });
            }

            let sector_start_time = sector_data.s2.current_s_time + sector_data.s1.current_s_time;
            // println!("🪵 [sector.rs:539]~ token ~ \x1b[0;32msector_start_time\x1b[0m = {} {}",race_data.current_time, sector_start_time);
            // if race_data.current_time > 10.0 { //确保没有用到下一圈的current time
            // }
            sector_data.s3.current_s_time = race_data.current_time - sector_start_time;

        }
    }

    let output = if (!sector_data.s1.is_done && sector_data.s3.is_done) || (!sector_data.s1.is_done && !sector_data.s2.is_done && !sector_data.s3.is_done) {  //正常的下一圈或初始圈
        // println!(
        //     "🪵 [sector.rs:517]~ token ~ \x1b[0;32msector_data.s1\x1b[0m = {}",
        //     "s1"
        // );
        let ctime = if sector_data.s3.time_shown { sector_data.s3.current_s_time } else { sector_data.s1.current_s_time };
        format_milliseconds_to_mmssms((ctime * 1000.0) as u32)
    } else if !sector_data.s1.is_done && !sector_data.s3.is_done && sector_data.s2.is_done {  //?到达第三赛段
        // println!(
        //     "🪵 [sector.rs:517]~ token ~ \x1b[0;32msector_data.s1\x1b[0m = {}",
        //     "s3"
        // );
        let ctime = if sector_data.s2.time_shown { sector_data.s2.current_s_time } else { sector_data.s3.current_s_time };
        format_milliseconds_to_mmssms((ctime * 1000.0) as u32)
    } else if !sector_data.s2.is_done && sector_data.s1.is_done && !sector_data.s3.is_done {
        // println!(
        //     "🪵 [sector.rs:517]~ token ~ \x1b[0;32msector_data.s1\x1b[0m = {}",
        //     "s2"
        // );
        let ctime = if sector_data.s1.time_shown { sector_data.s1.current_s_time } else { sector_data.s2.current_s_time };
        format_milliseconds_to_mmssms((ctime * 1000.0) as u32)
    } else if sector_data.s2.is_done && !sector_data.s3.is_done && sector_data.s1.is_done {  //正常到达第三赛段
        // println!(
        //     "🪵 [sector.rs:517]~ token ~ \x1b[0;32msector_data.s1\x1b[0m = {}",
        //     "s33"
        // );
        let ctime = if sector_data.s2.time_shown { sector_data.s2.current_s_time } else { sector_data.s3.current_s_time };

        format_milliseconds_to_mmssms((ctime * 1000.0) as u32)
    } else {
        // println!(
        //     "🪵 [sector.rs:517]~ token ~ \x1b[0;32msector_data.s1\x1b[0m = {}",
        //     "defa"
        // );

        format_milliseconds_to_mmssms((race_data.current_time * 1000.0) as u32)
    };

    let delta_show =
        sector_data.s1.time_shown || sector_data.s2.time_shown || sector_data.s3.time_shown;

    let mut delta = if sector_data.s1.time_shown {
        let str = if sector_data.s1.delta > 0.0  { "+" } else { "" };
        format!("{}{:.2}",str, sector_data.s1.delta)
    } else if sector_data.s2.time_shown {
        let str = if sector_data.s2.delta > 0.0  { "+" } else { "" };
        format!("{}{:.2}",str, sector_data.s2.delta)
    } else if sector_data.s3.time_shown {
        let str = if sector_data.s3.delta > 0.0  { "+" } else { "" };
        format!("{}{:.2}",str, sector_data.s3.delta)
    } else {
        "-:--".to_string()
    };
    if delta.contains("inf") {
        delta = "-:--".to_string();
    }

    return (output, delta_show, delta);
}

pub fn sector_logic(tele_data: &MutexGuard<BTreeMap<String, f32>>) -> (String, bool, String) {
    //return (sector_time,delta_show,delta)
    let mut sector_data = SECTOR_RECORD_DATA.get().unwrap().lock().unwrap();
    // let game_race_data = GAME_RACE_DATA.get().unwrap().lock().unwrap();
    update_race_data(tele_data);
    let race_data = GAME_RACE_DATA.get().unwrap().lock().unwrap();

    let track_info = get_track_data_map(&race_data.track_id);
    // println!("🪵 [sector.rs:398]~ token ~ \x1b[0;32m&race_data.track_id\x1b[0m = {}", &race_data.track_id);
    let cur_sector_time = race_data.race_time - race_data.current_time;
    if race_data.race_time <= 0.3 {
        println!(
            "🪵 [sector.rs:401]~ token ~ \x1b[0;32mrace_data.race_time <= 0.3 \x1b[0m = {}",
            race_data.race_time <= 0.3
        );
        reset_lap_control(&mut sector_data.s1);
        reset_lap_control(&mut sector_data.s2);
        reset_lap_control(&mut sector_data.s3);
        return (
            format_milliseconds_to_mmssms((race_data.current_time * 1000.0) as u32),
            false,
            "0.00".to_string(),
        );
    }
    if race_data.track_id != sector_data.s1.last_track_id {
        change_track(&mut sector_data.s1, &race_data);
        change_track(&mut sector_data.s2, &race_data);
        change_track(&mut sector_data.s3, &race_data);
    }
    if !sector_data.s1.initialized {
        init_s_record(&mut sector_data.s1, &track_info, 1);
    }
    if !sector_data.s2.initialized {
        init_s_record(&mut sector_data.s2, &track_info, 2);
    }
    init_s_record(&mut sector_data.s3, &track_info, 3);

    //---------------------
    if race_data.lap == 0 && race_data.distance == -(track_info.length as f64) {
        //跳圈检测
        sector_data.s1.jumped_lap = -1;
        sector_data.s2.jumped_lap = -1;
        sector_data.s3.jumped_lap = -1;
        // sector_data.s3.jumped_lap = -1;
    } else if race_data.distance < -100.0 {
        //数据重置
        set_lap_control_when_nega_distence(&mut sector_data.s1, &race_data, 1);
        set_lap_control_when_nega_distence(&mut sector_data.s2, &race_data, 2);
        set_lap_control_when_nega_distence(&mut sector_data.s3, &race_data, 3);
    }
    //-------------------------------------------------

    if race_data.race_time < 0.0
        || race_data.distance < 0.0
        || race_data.is_in_pit == true
        || race_data.lap == 0
    {
        // println!("🪵 [sector.rs:443]~ token ~ \x1b[0;32mrace_data.lap == 0\x1b[0m = {}", race_data.lap == 0);
        return (
            format_milliseconds_to_mmssms((race_data.current_time * 1000.0) as u32),
            false,
            "0.00".to_string(),
        );
    }

    when_cur_lap_diff(&mut sector_data.s1, &race_data, 1);
    when_cur_lap_diff(&mut sector_data.s2, &race_data, 2);
    when_cur_lap_diff(&mut sector_data.s3, &race_data, 3);

    if race_data.race_time < 0.0 || race_data.distance < 0.0 || race_data.is_in_pit == true {
        return (
            format_milliseconds_to_mmssms((race_data.current_time * 1000.0) as u32),
            false,
            "0.00".to_string(),
        );
    }

    let lap_distence_s1 = calc_lap_distence(&mut sector_data.s1, &race_data, 1);
    let lap_distence_s2 = calc_lap_distence(&mut sector_data.s2, &race_data, 2);
    let lap_distence_s3 = calc_lap_distence(&mut sector_data.s3, &race_data, 3);

    let (is_moving_forward_s1, prev_distance_s1) =
        calc_moving_forward(&mut sector_data.s1, &race_data, 1, lap_distence_s1);
    let (is_moving_forward_s2, prev_distance_s2) =
        calc_moving_forward(&mut sector_data.s2, &race_data, 2, lap_distence_s2);
    let (is_moving_forward_s3, prev_distance_s3) =
        calc_moving_forward(&mut sector_data.s3, &race_data, 3, lap_distence_s3);

    check_is_done(
        &mut sector_data.s1,
        &race_data,
        1,
        lap_distence_s1,
        is_moving_forward_s1,
    );
    check_is_done(
        &mut sector_data.s2,
        &race_data,
        2,
        lap_distence_s2,
        is_moving_forward_s2,
    );

    if sector_data.s3.s1_time == 0.0 && lap_distence_s3 >= sector_data.s3.s1_end {
        sector_data.s3.s1_time = sector_data.s1.current_s_time;
        sector_data.s3.s1_last_time = sector_data.s3.s1_time;
        // if lap_distence_s3 >= sector_data.s3.s1_end{
        // }
    }
    if sector_data.s3.s1_time > 0.0
        && sector_data.s3.s2_time == 0.0
        && lap_distence_s3 >= sector_data.s3.s2_end
    {
        sector_data.s3.s2_time = sector_data.s2.current_s_time;
        sector_data.s3.s2_last_time = sector_data.s3.s2_time;
        sector_data.s3.current_s_time =
            race_data.current_time - sector_data.s3.s1_time - sector_data.s3.s2_time;
    }

    let show_current_time_s1 =
        race_data.race_time <= sector_data.s1.delta_until && race_data.current_time > 0.3;
    let show_current_time_s2 =
        race_data.race_time <= sector_data.s2.delta_until && race_data.current_time > 0.3;
    let show_current_time_s3 =
        race_data.race_time <= sector_data.s3.delta_until && race_data.current_time > 0.01;

    let output: String;
    if show_current_time_s1 || (!sector_data.s1.is_done && !show_current_time_s3) {
        // println!("🪵 [sector.rs:517]~ token ~ \x1b[0;32mis_done\x1b[0m = {}", "s1");
        let cur_time = if !show_current_time_s1 {
            race_data.current_time
        } else {
            sector_data.s1.current_s_time
        };
        output = format_milliseconds_to_mmssms((cur_time * 1000.0) as u32);
    } else if show_current_time_s2
        || (sector_data.s1.is_done && !sector_data.s2.is_done && !show_current_time_s3)
    {
        // println!("🪵 [sector.rs:517]~ token ~ \x1b[0;32mis_done\x1b[0m = {}", "s2");
        let cur_time = if !show_current_time_s2 {
            println!(
                "🪵 [sector.rs:528]~ token ~ \x1b[0;32mrace_data.current_time \x1b[0m = {} {}",
                race_data.current_time, sector_data.s2.sector_start_time
            );
            race_data.current_time - sector_data.s2.sector_start_time
        } else {
            sector_data.s2.current_s_time
        };
        output = format_milliseconds_to_mmssms((cur_time * 1000.0) as u32);
    } else {
        println!(
            "🪵 [sector.rs:517]~ token ~ \x1b[0;32mis_done\x1b[0m = {}",
            "s3"
        );

        let cur_time = if !show_current_time_s3 {
            race_data.current_time - sector_data.s3.sector_start_time
        } else {
            sector_data.s3.s3_time
        };
        output = format_milliseconds_to_mmssms((cur_time * 1000.0) as u32);
    }

    let mut delta = 0.0;
    if show_current_time_s1 {
        delta = sector_data.s1.delta;
    } else if show_current_time_s2 {
        delta = sector_data.s2.delta;
    } else if show_current_time_s3 {
        delta = sector_data.s3.delta;
    }
    let delta = if delta <= 0.0 {
        format!("{:.2}", delta)
    } else {
        format!("+{:.2}", delta)
    };
    // println!("🪵 [sector.rs:537]~ token ~ \x1b[0;32moutput\x1b[0m = {}", output);
    return (
        output,
        show_current_time_s1 || show_current_time_s2 || show_current_time_s3,
        delta,
    );
}

pub fn update_race_data(tele_data: &MutexGuard<BTreeMap<String, f32>>) -> () {
    let mut game_race_data = GAME_RACE_DATA.get().unwrap().lock().unwrap();
    // let mut new_data = GameRaceData::default();
    game_race_data.current_lap = match tele_data.get("LapNumber") {
        Some(lap) => lap.clone() as i32,
        None => 0,
    };
    game_race_data.lap = game_race_data.current_lap - 1;

    game_race_data.distance = match tele_data.get("DistanceTraveled") {
        Some(distance) => distance.clone() as f64,
        None => 0.0,
    };
    game_race_data.race_time = match tele_data.get("CurrentRaceTime") {
        Some(race_time) => race_time.clone() as f64,
        None => 0.0,
    };
    game_race_data.current_time = match tele_data.get("CurrentLap") {
        Some(current_time) => current_time.clone() as f64,
        None => 0.0,
    };
    game_race_data.track_id = match tele_data.get("TrackOrdinal") {
        Some(track_id) => track_id.clone() as u16,
        None => 0,
    };
    game_race_data.is_in_pit = match tele_data.get("IsInPit") {
        Some(is_in_pit) => is_in_pit.clone() == 0.0,
        None => false,
    };
    game_race_data.last_lap_time = match tele_data.get("LastLap") {
        Some(last_lap_time) => last_lap_time.clone() as f64,
        None => 0.0,
    };
    // let old_map = mem::replace(&mut *game_race_data, new_data.clone());
    // new_data
    // *game_race_data = new_data;
}

fn reset_lap_control(lap_control: &mut LapControl) {
    lap_control.jumped_lap = -1;
    lap_control.was_jumped = false;
    lap_control.current_lap = -2;
    lap_control.has_passed_zero_point = false;
    lap_control.sector_time = 0.0;
    lap_control.is_done = false;
    lap_control.was_in_pit = false;
    lap_control.sector_start_time = 0.0;
    lap_control.last_valid_distance = -9999.0;
    lap_control.custom_lap_counter = 0;
    lap_control.started_counting = false;
    lap_control.best_time = f64::INFINITY;
    lap_control.delta_until = 0.0;
    lap_control.current_s_time = 0.0;
    lap_control.s1_time = 0.0;
    lap_control.s2_time = 0.0;
    lap_control.s1_last_time = 0.0;
    lap_control.s2_last_time = 0.0;
    lap_control.last_lap_s = 0.0;
}
fn change_track(lap_control: &mut LapControl, race_data: &GameRaceData) {
    lap_control.best_time = f64::INFINITY;
    lap_control.last_track_id = race_data.track_id;
    lap_control.needs_reset = true;
    lap_control.initialized = false;
}
fn init_s_record(lap_control: &mut LapControl, track_info: &TrackData, order: u16) {
    if order == 1 {
        lap_control.sector_end = track_info.s1_end.into();
    } else if order == 2 {
        lap_control.s1_end = track_info.s1_end.into();
        lap_control.s2_end = track_info.s2_end.into();
    } else {
        lap_control.s1_end = track_info.s1_end.into();
        lap_control.s2_end = track_info.s2_end.into();
        // lap_control.sector_end = track_info.length.into();
    }
    lap_control.track_length = track_info.length.into();
    lap_control.initialized = true;
}

fn set_lap_control_when_nega_distence(
    lap_control: &mut LapControl,
    race_data: &GameRaceData,
    order: u16,
) {
    lap_control.jumped_lap = race_data.lap;
    lap_control.was_jumped = true;
    lap_control.custom_lap_counter = 0;
    lap_control.started_counting = false;
    lap_control.time_shown = false;

    lap_control.sector_time = 0.0;
    lap_control.is_done = false;
    lap_control.has_passed_zero_point = false;
    lap_control.sector_start_time = 0.0;
    lap_control.last_valid_distance = 0.0;
    lap_control.current_s_time = 0.0;

    if order == 2 {
        lap_control.lap_start = race_data.race_time;
    }
    if order == 3 {
        lap_control.s1_time = 0.0;
        lap_control.s2_time = 0.0;
        lap_control.last_lap_s = 0.0;
    }
}

fn when_cur_lap_diff(lap_control: &mut LapControl, race_data: &GameRaceData, order: u16) {
    let s3_need_reset = order == 3 && lap_control.needs_reset;
    if race_data.lap != lap_control.current_lap || s3_need_reset {
        lap_control.current_lap = race_data.lap;
        lap_control.sector_time = 0.0;
        lap_control.is_done = false;
        lap_control.has_passed_zero_point = false;
        if order == 1 {
            lap_control.last_valid_distance = 0.0;
            lap_control.sector_start_time = race_data.race_time;
        }
        if order == 2 {
            lap_control.s1_pass_time = 0.0;
        }
        if order == 3 {
            lap_control.lap_start = if race_data.race_time > 0.0 {
                race_data.current_time
            } else {
                get_now_ts() / 1000.0
            };
            lap_control.s1_time = 0.0;
            lap_control.s2_time = 0.0;
            if lap_control.current_lap >= 0 && lap_control.current_s_time > 0.0 {
                lap_control.last_lap_s = lap_control.current_s_time;

                lap_control.two_laps_ago = lap_control.one_lap_ago;
                lap_control.one_lap_ago = lap_control.best_time;
            }

            if race_data.lap > 0
                && race_data.last_lap_time > 0.0
                && lap_control.s1_last_time > 0.0
                && lap_control.s2_last_time > 0.0
            {
                let s3_calc =
                    race_data.last_lap_time - (lap_control.s1_last_time + lap_control.s2_last_time);
                println!("🪵 [sector.rs:684]~ token ~ \x1b[0;32mrace_data.last_lap_time\x1b[0m = {} {} {}", race_data.last_lap_time,lap_control.s1_last_time,lap_control.s2_last_time );
                if s3_calc > 1.0 {
                    lap_control.s3_time = s3_calc;
                    lap_control.delta = lap_control.s3_time - lap_control.best_time;

                    if s3_calc != lap_control.best_time {
                        lap_control.delta_until = race_data.race_time + 5.0;
                    }

                    if s3_calc < lap_control.best_time {
                        lap_control.best_time = s3_calc;
                    }
                }
            }
            lap_control.current_s_time = 0.0;
            lap_control.needs_reset = false;
            lap_control.is_done = false;
            lap_control.has_passed_zero_point = false;
            lap_control.last_valid_distance = 0.0;
        }

        if lap_control.jumped_lap != 0
            && race_data.lap != lap_control.jumped_lap
            && lap_control.was_jumped == true
        {
            lap_control.jumped_lap = 0;
        }
        if lap_control.jumped_lap == 0 && !lap_control.started_counting {
            lap_control.custom_lap_counter = 1;
            lap_control.started_counting = true;
        } else if lap_control.jumped_lap == 0 && lap_control.started_counting {
            lap_control.custom_lap_counter = lap_control.custom_lap_counter + 1;
        }
    }
}

fn calc_lap_distence(lap_control: &mut LapControl, race_data: &GameRaceData, order: u16) -> f64 {
    #[allow(unused_assignments)]
    let mut lap_distence = 0.0;
    if lap_control.jumped_lap != 0 && race_data.lap == lap_control.jumped_lap {
        lap_distence = race_data.distance;
    } else if lap_control.jumped_lap == 0 {
        lap_distence = race_data.distance
            - ((lap_control.custom_lap_counter as f64) * lap_control.track_length);
    } else {
        lap_distence =
            race_data.distance - ((lap_control.current_lap as f64) * lap_control.track_length);
    }
    return lap_distence;
}

fn calc_moving_forward(
    lap_control: &mut LapControl,
    race_data: &GameRaceData,
    order: u16,
    lap_distence: f64,
) -> (bool, f64) {
    let is_moving_forward = lap_distence > lap_control.last_valid_distance;
    let prev_distance = lap_control.last_valid_distance;
    lap_control.last_valid_distance = lap_distence;
    if !lap_control.has_passed_zero_point {
        if lap_distence < 50.0 && is_moving_forward && lap_distence >= 0.0 {
            //正常返回
            lap_control.has_passed_zero_point = true;
            lap_control.sector_start_time = race_data.race_time;
        } else if prev_distance < 0.0 && lap_distence >= 0.0 {
            //跳圈后返回
            lap_control.has_passed_zero_point = true;
            lap_control.sector_start_time = race_data.race_time;
        }
    }
    return (is_moving_forward, prev_distance);
}

fn check_is_done(
    lap_control: &mut LapControl,
    race_data: &GameRaceData,
    order: u16,
    lap_distence: f64,
    is_moving_forward: bool,
) -> bool {
    let mut is_done = !lap_control.is_done;
    if order == 1 {
        is_done = !lap_control.is_done && lap_control.has_passed_zero_point;
    };

    if is_done {
        let mut current_time = 0.0;
        if order == 1 {
            current_time = race_data.current_time
        } else if order == 2 {
            current_time = race_data.current_time
        }

        if order == 2 {
            if lap_control.s1_pass_time == 0.0
                && lap_distence >= lap_control.s1_end
                && is_moving_forward
            {
                lap_control.s1_pass_time = current_time;
            }
        }

        let mut judge_end = lap_distence >= lap_control.sector_end && is_moving_forward;
        if order == 2 {
            judge_end = lap_control.s1_pass_time > 0.0
                && lap_distence >= lap_control.s2_end
                && is_moving_forward;
        }

        if judge_end {
            lap_control.sector_time = current_time;
            lap_control.is_done = true;
            lap_control.time_shown = false;
            if order == 2 {
                lap_control.current_s_time = lap_control.sector_time;
                lap_control.sector_time = current_time - lap_control.s1_pass_time;
            }

            let mut judge_lap = true;
            if order == 1 {
                judge_lap = race_data.lap >= 0;
            }

            if judge_lap {
                if order == 1 {
                    lap_control.current_s_time = current_time;
                }

                lap_control.two_laps_ago = lap_control.one_lap_ago;
                lap_control.one_lap_ago = lap_control.best_time;

                lap_control.delta = lap_control.current_s_time - lap_control.best_time;

                if current_time != lap_control.one_lap_ago {
                    lap_control.delta_until = race_data.race_time + 5.0;
                }

                if order == 1 {
                    if current_time < lap_control.best_time {
                        lap_control.best_time = current_time;
                    }
                    // if lap_distence >= lap_control.s1_end && is_moving_forward {
                    //     lap_control.delta_until = race_data.race_time + 5.0;
                    // }
                }
                if order == 2 {
                    if lap_control.sector_time < lap_control.best_time {
                        lap_control.best_time = lap_control.sector_time;
                    }
                }
            }
        }
    }
    is_done
}

fn new_lap_reset(sector_data: &mut SectorRecord, race_data: &GameRaceData, order: u16) {
    sector_data.s1.is_done = false;
    sector_data.s2.is_done = false;
    sector_data.s3.is_done = false;
}
