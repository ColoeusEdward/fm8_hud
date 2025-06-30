use eframe::{
    egui::{
        self, Area, Color32, FontId, Layout, Pos2, Rect, RichText, Stroke, StrokeKind,
        TextureOptions, UiBuilder, Vec2,
    },
    epaint::{CornerRadiusF32, PathShape, PathStroke},
};
use image::ImageReader;
use std::{
    collections::BTreeMap,
    io::Cursor,
    sync::{atomic::Ordering, MutexGuard},
};

use crate::{
    enums::{CarSetting, CurCarRpmSetting, GameRaceData},
    ui::index::{
        MyApp2, CAR_SETTING, CUR_CAR_RPM_SETTING, GAME_RACE_DATA, IS_MOUSE_PASS, LAST_TELE_DATA,
        TEXTURE_HANDLE_MAP,
    },
    uitl::{format_milliseconds_to_mmssms, get_now_ts_mill},
};

pub fn render_dash(ctx: &egui::Context, app: &mut MyApp2) {
    if !app.show_state.show_dash {
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
    let mut race_data = GAME_RACE_DATA.get().unwrap().lock().unwrap();
    let cur_car_rpm_setting_mutex = CUR_CAR_RPM_SETTING.get().unwrap().lock().unwrap();
    let car_setting = CAR_SETTING.get().unwrap().lock().unwrap();
    let texture_map = TEXTURE_HANDLE_MAP.get().unwrap().lock().unwrap();

    let res = Area::new("gt_hud".into())
        .current_pos(egui::pos2(app.hud_pos.x, app.hud_pos.y)) // 位置, 400.0 + app.yoffset)) // 位置
        .movable(true) //
        .show(ctx, |ui| {
            let len = app.setting_data.dash_len.parse::<f32>();
            let len = match len {
                Ok(len) => len,
                Err(e) => app.setting_data.dash_base_len,
            };
            let scale_to_base = len / app.setting_data.dash_base_len;
            let desired_size = egui::vec2(len, len / app.setting_data.dash_scale);
            let (rect, _response) = ui.allocate_exact_size(desired_size, egui::Sense::hover());
            render_arc(
                ctx,
                ui,
                app,
                rect,
                &mut race_data,
                &car_setting,
                &cur_car_rpm_setting_mutex,
            );

            ui.painter().image(
                texture_map.get("gt_hud_img").unwrap().id(),
                rect, // 图片将填充整个面板
                egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)), // UV 坐标 (0.0-1.0)
                Color32::WHITE, // 图片的色调 (White 表示原色)
            );

            let text_pos = rect.left_top() + Vec2::new(455.0, 56.0); // 距离左上角 10 像素
            let text_size = Vec2::new(180.0, 44.0); // 文本区域宽度比背景小 20，高度 50
            let text_rect_a = Rect::from_min_size(text_pos, text_size);

            ui.allocate_new_ui(UiBuilder::new().max_rect(text_rect_a), |ui_at_rect| {
                ui_at_rect.with_layout(Layout::right_to_left(egui::Align::TOP), |ui_at_rect| {
                    // ui_at_rect.disable(); // 通常不需要交互，所以可以禁用
                    ui_at_rect.label(
                        RichText::new(format!("{:.0}", race_data.speed))
                            .color(Color32::WHITE)
                            .font(FontId::proportional(42.0)),
                    );
                });
                ui_at_rect.add_space(-10.0);
                ui_at_rect.with_layout(Layout::right_to_left(egui::Align::TOP), |ui_at_rect| {
                    ui_at_rect.label(
                        RichText::new("Km/h")
                            .color(Color32::WHITE)
                            .weak()
                            .family(egui::FontFamily::Name("base".into())),
                    );
                });
            });

            let text_pos = rect.left_top() + Vec2::new(668.0, 56.0); // 距离左上角 10 像素
            let text_size = Vec2::new(180.0, 44.0); // 文本区域宽度比背景小 20，高度 50
            let text_rect_a = Rect::from_min_size(text_pos, text_size);
            ui.allocate_new_ui(UiBuilder::new().max_rect(text_rect_a), |ui_at_rect| {
                // if race_data.clutch > 0.0 {
                //   println!("🪵 [dash.rs:94]~ token ~ \x1b[0;32mrace_data.clutch\x1b[0m = {} {}", race_data.clutch,race_data.gear);
                // }
                let gear = if race_data.gear == 11 {
                    "N".to_string()
                } else if race_data.gear == 0 {
                    "R".to_string()
                }
                else {
                    race_data.gear.to_string()
                };
                ui_at_rect.label(
                    RichText::new(gear)
                        .color(Color32::WHITE)
                        .font(FontId::proportional(64.0)),
                );
            });

            let text_pos = rect.left_top() + Vec2::new(760.0, 62.0); // 距离左上角 10 像素
            let text_size = Vec2::new(120.0, 44.0); // 文本区域宽度比背景小 20，高度 50
            let text_rect_a = Rect::from_min_size(text_pos, text_size);
            ui.allocate_new_ui(UiBuilder::new().max_rect(text_rect_a), |ui_at_rect| {
                ui_at_rect.label(
                    RichText::new(format!("{:.0}", race_data.rpm))
                        .color(Color32::WHITE)
                        .font(FontId::proportional(18.0)),
                );
            });

            let brake = (race_data.brake / 3.03) as f32;
            let yoffset = 38.0 + 84.0 - brake;
            let text_pos = rect.left_top() + Vec2::new(387.0, yoffset); // 距离左上角 10 像素
                                                                        // let text_po2 = rect.left_top() + Vec2::new(776.0, 28.0); // 距离左上角 10 像素
            let text_size = Vec2::new(20.0, 100.0); // 文本区域宽度比背景小 20，高度 50
            let text_rect_a = Rect::from_min_size(text_pos, text_size);
            ui.allocate_new_ui(UiBuilder::new().max_rect(text_rect_a), |ui_at_rect| {
                let desired_size = egui::vec2(16.0, brake); //最高92px

                let (rect, _response) =
                    ui_at_rect.allocate_exact_size(desired_size, egui::Sense::hover());
                // 获取 painter
                let painter = ui_at_rect.painter();

                // 定义填充颜色: #A2000000 (ARGB) -> 64% 透明度的黑色 (RGBA: 0,0,0,162)
                let fill_color = Color32::from_rgba_premultiplied(255, 255, 255, 255);

                // 定义圆角半径
                let corner_radius = 1.0; // 较大的圆角，更明显

                // 绘制填充的圆角矩形
                painter.rect_filled(
                    rect,
                    CornerRadiusF32::same(corner_radius), // 所有角的圆角半径相同
                    fill_color,
                );
            });

            let acc = (race_data.accel / 3.03) as f32;
            let yoffset = 38.0 + 84.0 - acc;
            let text_pos = rect.left_top() + Vec2::new(911.0, yoffset); // 距离左上角 10 像素
                                                                        // let text_po2 = rect.left_top() + Vec2::new(776.0, 28.0); // 距离左上角 10 像素
            let text_size = Vec2::new(20.0, 100.0); // 文本区域宽度比背景小 20，高度 50
            let text_rect_a = Rect::from_min_size(text_pos, text_size);
            ui.allocate_new_ui(UiBuilder::new().max_rect(text_rect_a), |ui_at_rect| {
                let desired_size = egui::vec2(16.0, acc); //最高92px

                let (rect, _response) =
                    ui_at_rect.allocate_exact_size(desired_size, egui::Sense::hover());
                // 获取 painter
                let painter = ui_at_rect.painter();

                // 定义填充颜色: #A2000000 (ARGB) -> 64% 透明度的黑色 (RGBA: 0,0,0,162)
                let fill_color = Color32::from_rgba_premultiplied(255, 255, 255, 255);

                // 定义圆角半径
                let corner_radius = 1.0; // 较大的圆角，更明显

                // 绘制填充的圆角矩形
                painter.rect_filled(
                    rect,
                    CornerRadiusF32::same(corner_radius), // 所有角的圆角半径相同
                    fill_color,
                );
            });

            render_fuel(
                ctx,
                ui,
                app,
                rect,
                &mut race_data,
                &cur_car_rpm_setting_mutex,
                &texture_map,
            );

            render_tire(
                ctx,
                ui,
                app,
                rect,
                &mut race_data,
                &cur_car_rpm_setting_mutex,
            );
            render_boost(
                ctx,
                ui,
                app,
                rect,
                &mut race_data,
                &cur_car_rpm_setting_mutex,
                &texture_map,
            );
            render_dot(
                ctx,
                ui,
                app,
                rect,
                &mut race_data,
                &cur_car_rpm_setting_mutex,
                &texture_map,
            );
        })
        .response;

    if res.dragged() {
        app.hud_pos += res.drag_delta();
        // println!("🪵 [dash.rs:76]~ token ~ \x1b[0;32mapp.hud_pos\x1b[0m = {} {}", app.hud_pos.x,app.hud_pos.x,);
    }
}

fn render_arc(
    ctx: &egui::Context,
    ui: &mut egui::Ui,
    app: &mut MyApp2,
    rect: Rect,
    race_data: &mut MutexGuard<'_, GameRaceData>,
    car_setting: &MutexGuard<'_, CarSetting>,
    car_setting_mutex: &MutexGuard<'_, CurCarRpmSetting>,
) {
    let text_pos = rect.left_top() + Vec2::new(455.0, 56.0);
    let text_size = Vec2::new(380.0, 180.0); //
    let text_rect_a = Rect::from_min_size(text_pos, text_size);

    ui.allocate_new_ui(UiBuilder::new().max_rect(text_rect_a), |ui_at_rect| {
        // ui_at_rect.label(RichText::new("弧线").size(20.0).color(Color32::WHITE).font(FontId::proportional(16.0)));
        // 获取 Painter 对象用于自定义绘图
        let painter = ui_at_rect.painter();
        let half_total_len = 9.54;
        let total_len = half_total_len * 2.0;
        let mut end_degree = 270.0 - half_total_len;
        let car_id = race_data.car_id as u16;
        let cur_car_rpm_setting = car_setting.rpm.get(&car_id);
        let cur_car_rpm_setting = match cur_car_rpm_setting {
            Some(cur_car_rpm_setting) => cur_car_rpm_setting,
            None => &vec![
                car_setting_mutex.max_rpm.clone(),
                car_setting_mutex.red_rpm.clone(),
            ],
        };

        let max_rpm = cur_car_rpm_setting[0].parse::<f64>();
        let max_rpm = match max_rpm {
            Ok(max_rpm) => max_rpm,
            Err(_) => race_data.max_rpm,
        };
        let red_rpm = cur_car_rpm_setting[1].parse::<f64>();
        let red_rpm = match red_rpm {
            Ok(red_rpm) => red_rpm,
            Err(_) => max_rpm * 0.9,
        };
        let min_show_rpm = max_rpm * 0.70;
        let mut cur_rpm = race_data.rpm;
        if cur_rpm > red_rpm {
            cur_rpm = red_rpm
        }
        let percent = (cur_rpm - min_show_rpm) / (red_rpm - min_show_rpm);
        if race_data.rpm > min_show_rpm {
            end_degree = end_degree + total_len * percent;
        }

        // let mut dash_color = Color32::from_rgba_premultiplied(0, 255, 255, 250);
        let dash_color_blink = Color32::from_rgba_premultiplied(0, 255, 255, 255);
        let color_per = if percent > 0.40 {
            let val = (percent - 0.40) / (1.0-0.40-0.1);
            if val > 1.0 {
                1.0
            } else{
                val
            }
        } else {
            0.0
        };
        // 定义起始和结束颜色
        let r1: f32 = 255.0;
        let g1: f32 = 0.0;
        let b1: f32 = 0.0;

        let r2: f32 = 253.0;
        let g2: f32 = 153.0;
        let b2: f32 = 255.0;
        // 使用线性插值计算新的 R, G, B 值
        let new_r = r1 + (r2 - r1) * color_per as f32;
        let new_g = g1 + (g2 - g1) * color_per as f32;
        let new_b = b1 + (b2 - b1) * color_per as f32;
        let mut dash_color =
            Color32::from_rgba_premultiplied(new_r as u8, new_g as u8, new_b as u8, 255);

        if race_data.rpm <= red_rpm {
            race_data.dash_is_blink = false;
        } else {
            dash_color = dash_color_blink;

            let now = get_now_ts_mill();
            // println!(
            //     "🪵 [dash.rs:263]~ token ~ \x1b[0;32mnow - race_data.dash_blink_ts\x1b[0m = {} {}",
            //     now, race_data.dash_blink_ts
            // );
            if now - race_data.dash_blink_ts > 52 {
                race_data.dash_is_blink = !race_data.dash_is_blink;
                race_data.dash_blink_ts = now;
            }
        }
        if race_data.dash_is_blink {
            // dash_color = dash_color_blink;
            end_degree = 270.0 - half_total_len;
        }

        // 定义圆弧的中心位置
        // 我们将其放置在标签旁边以便观察
        let center = ui_at_rect.cursor().min + Vec2::new(202.0, 1263.0);

        // **设置半径为 20px**
        let radius = 1290.0;

        // 定义顶部圆弧的角度范围
        // 顶部意味着围绕 270 度（或 -90 度）。
        // 我们选择一个 90 度的扇形（从 225 度到 315 度）来表示 "顶部一小段"。
        // to_radians() 将角度转换为弧度
        let start_angle = ((270.0 - half_total_len) as f32).to_radians(); // 225 度
        let end_angle = (end_degree as f32).to_radians(); // 315 度

        // 生成构成圆弧的点
        let n_points = 140; // 对于小圆弧，不需要太多点
        let points: Vec<Pos2> = (0..=n_points)
            .map(|i| {
                let angle = start_angle + (end_angle - start_angle) * (i as f32 / n_points as f32);
                let x = center.x + radius * angle.cos();
                let y = center.y + radius * angle.sin();
                Pos2::new(x, y)
            })
            .collect();

        // 创建一个路径形状 (PathShape)
        let arc_shape = PathShape {
            points,
            closed: false,              // 设置为 false，使其成为一条线而不是闭合图形
            fill: Color32::TRANSPARENT, // 无填充色
            stroke: PathStroke::new(18.0, dash_color), // 设置线条粗细和颜色
        };

        // 使用 painter 将形状添加到 UI
        painter.add(arc_shape);
    });
}

fn render_fuel(
    ctx: &egui::Context,
    ui: &mut egui::Ui,
    app: &mut MyApp2,
    rect: Rect,
    race_data: &mut MutexGuard<'_, GameRaceData>,
    car_setting: &MutexGuard<'_, CurCarRpmSetting>,
    texture_map: &MutexGuard<'_, BTreeMap<String, egui::TextureHandle>>,
) {
    let text_pos = rect.left_top() + Vec2::new(132.0, 30.0); // 距离左上角 10 像素
    let text_size = Vec2::new(133.0, 66.0); // 文本区域宽度比背景小 20，高度 50
    let text_rect_a = Rect::from_min_size(text_pos, text_size);

    ui.allocate_new_ui(UiBuilder::new().max_rect(text_rect_a), |ui_at_rect| {
        ui_at_rect.painter().image(
            texture_map.get("fuel_img").unwrap().id(),
            text_rect_a, // 图片将填充整个面板
            egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)), // UV 坐标 (0.0-1.0)
            Color32::WHITE, // 图片的色调 (White 表示原色)
        );
    });

    ui.allocate_new_ui(UiBuilder::new().max_rect(text_rect_a), |ui_at_rect| {
        let half_total_len = 90.0;
        let total_len = half_total_len * 2.0;
        let mut end_degree = 270.0 - half_total_len;
        let fuel_percent = race_data.fuel;
        // println!("🪵 [dash.rs:355]~ token ~ \x1b[0;32mrace_data.fuel\x1b[0m = {}", race_data.fuel);
        end_degree = end_degree + total_len * fuel_percent;
        // 定义圆弧的中心位置
        // 我们将其放置在标签旁边以便观察
        // ui_at_rect.label(RichText::new("FUEL").color(Color32::WHITE));
        let center = ui_at_rect.cursor().min + Vec2::new(65.0, 57.0);
        // let  dash_color = Color32::from_rgba_premultiplied(0, 255, 255, 250);

        // **设置半径为 20px**
        let radius = 44.0;

        // 定义顶部圆弧的角度范围
        // 顶部意味着围绕 270 度（或 -90 度）。
        // 我们选择一个 90 度的扇形（从 225 度到 315 度）来表示 "顶部一小段"。
        // to_radians() 将角度转换为弧度
        let start_angle = ((270.0 - half_total_len) as f32).to_radians(); // 225 度
        let end_angle = (end_degree as f32).to_radians(); // 315 度

        // 生成构成圆弧的点
        let n_points = 100; // 对于小圆弧，不需要太多点
        let points: Vec<Pos2> = (0..=n_points)
            .map(|i| {
                let angle = start_angle + (end_angle - start_angle) * (i as f32 / n_points as f32);
                let x = center.x + radius * angle.cos();
                let y = center.y + radius * angle.sin();
                Pos2::new(x, y)
            })
            .collect();

        // 创建一个路径形状 (PathShape)
        let arc_shape = PathShape {
            points,
            closed: false,              // 设置为 false，使其成为一条线而不是闭合图形
            fill: Color32::TRANSPARENT, // 无填充色
            stroke: PathStroke::new(4.0, Color32::WHITE), // 设置线条粗细和颜色
        };
        // 使用 painter 将形状添加到 UI
        ui_at_rect.painter().add(arc_shape);
    });


    let text_pos = rect.left_top() + Vec2::new(182.0, 68.0); // 距离左上角 10 像素
    let text_size = Vec2::new(35.5, 35.5); // 文本区域宽度比背景小 20，高度 50
    let text_rect_a = Rect::from_min_size(text_pos, text_size);
    ui.allocate_new_ui(UiBuilder::new().max_rect(text_rect_a), |ui_at_rect| {
        // 获取 painter
        let painter = ui_at_rect.painter();

        // 定义填充颜色: #A2000000 (ARGB) -> 64% 透明度的黑色 (RGBA: 0,0,0,162)
        let fill_color = Color32::from_rgba_premultiplied(67, 65, 65, 108);

        // 定义圆角半径
        let corner_radius = 18.0; // 较大的圆角，更明显

        // 绘制填充的圆角矩形
        painter.rect_filled(
            text_rect_a,
            CornerRadiusF32::same(corner_radius), // 所有角的圆角半径相同
            fill_color,
        );
    });
    let text_pos = text_pos + Vec2::new(7.0, 5.0); // 距离左上角 10 像素
    let text_size = Vec2::new(23.1 ,24.85); // 文本区域宽度比背景小 20，高度 50
    let text_rect_a = Rect::from_min_size(text_pos, text_size);
    ui.allocate_new_ui(UiBuilder::new().max_rect(text_rect_a), |ui_at_rect| {
        ui_at_rect.painter().image(
            texture_map.get("fuel_icon").unwrap().id(),
            text_rect_a, // 图片将填充整个面板
            egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)), // UV 坐标 (0.0-1.0)
            Color32::WHITE, // 图片的色调 (White 表示原色)
        );
    });

    let text_pos = rect.left_top() + Vec2::new(138.0, 106.0); // 距离左上角 10 像素
    let text_size = Vec2::new(120.0, 26.0); // 文本区域宽度比背景小 20，高度 50
    let text_rect_a = Rect::from_min_size(text_pos, text_size);

    ui.allocate_new_ui(UiBuilder::new().max_rect(text_rect_a), |ui_at_rect| {
        // 获取 painter
        let painter = ui_at_rect.painter();

        // 定义填充颜色: #A2000000 (ARGB) -> 64% 透明度的黑色 (RGBA: 0,0,0,162)
        let fill_color = Color32::from_rgba_premultiplied(0, 0, 0, 108);

        // 定义圆角半径
        let corner_radius = 6.0; // 较大的圆角，更明显

        // 绘制填充的圆角矩形
        painter.rect_filled(
            text_rect_a,
            CornerRadiusF32::same(corner_radius), // 所有角的圆角半径相同
            fill_color,
        );
    });

    ui.allocate_new_ui(UiBuilder::new().max_rect(text_rect_a), |ui_at_rect| {
        ui_at_rect.with_layout(
            egui::Layout::centered_and_justified(egui::Direction::TopDown),
            |ui| {
                ui.label(
                    RichText::new(format!("已行驶: {:.2}Km", race_data.distance / 1000.0))
                        .font(FontId::monospace(14.0))
                        .color(Color32::WHITE), // .size(16.0),
                );
            },
        );
    });
}

fn render_tire(
    ctx: &egui::Context,
    ui: &mut egui::Ui,
    app: &mut MyApp2,
    rect: Rect,
    race_data: &mut MutexGuard<'_, GameRaceData>,
    car_setting: &MutexGuard<'_, CurCarRpmSetting>,
) {
    let tire1 = race_data.tire_wear1 * 100.0;
    let remaind_tire1 = 100.0 - tire1;
    let tire2 = race_data.tire_wear2 * 100.0;
    let remaind_tire2 = 100.0 - tire2;
    let tire3 = race_data.tire_wear3 * 100.0;
    let remaind_tire3 = 100.0 - tire3;
    let tire4 = race_data.tire_wear4 * 100.0;
    let remaind_tire4 = 100.0 - tire4;
    let slip1 = race_data.tire_slip1;
    // println!("🪵 [dash.rs:491]~ token ~ \x1b[0;32mslip1\x1b[0m = {}", slip1);
    let slip2 = race_data.tire_slip2;
    let slip3 = race_data.tire_slip3;
    let slip4 = race_data.tire_slip4;

    let mut render_single_tire = |pos: Vec2,
                                  pos_cost_tire: Vec2,
                                  tire: f64,
                                  remaind_tire: f64,
                                  tire_last_lap: f64,
                                  slip: f64| {
        let text_pos = rect.left_top() + pos; // 距离左上角 10 像素
        let text_size = Vec2::new(14.0, 42.0); // 文本区域宽度比背景小 20，高度 50
        let text_rect_a = Rect::from_min_size(text_pos, text_size);
        let text_rect_a_outline = Rect::from_min_size(
            text_pos + Vec2 { x: -1.42, y: -1.52 },
            text_size + Vec2 { x: 2.0, y: 2.0 },
        );
        ui.allocate_new_ui(UiBuilder::new().max_rect(text_rect_a), |ui_at_rect| {
            // 获取 painter
            let painter = ui_at_rect.painter();

            // 定义填充颜色: #A2000000 (ARGB) -> 64% 透明度的黑色 (RGBA: 0,0,0,162)
            let mut fill_color = Color32::from_rgba_premultiplied(255, 255, 255, 250);
            if remaind_tire < 50.0 && remaind_tire > 40.0 {
                fill_color = Color32::from_rgba_premultiplied(227, 191, 12, 250);
            } else if remaind_tire < 40.0 {
                fill_color = Color32::from_rgba_premultiplied(255, 140, 0, 250);
            }
            // 定义圆角半径
            let corner_radius = 2.0; // 较大的圆角，更明显
            let mut outline_color = Color32::from_rgba_premultiplied(0, 0, 0, 80);
            if slip > 0.90 {
                let percent = slip - 0.90 / 1.15 - 0.90;
                let zero_rgb = [248, 110, 113];
                let end_rgb = [255, 0, 0];
                fn calc_rgb(percent: f32, zero_rgb: [u8; 3], end_rgb: [u8; 3]) -> [u8; 3] {
                    let mut rgb = [0u8; 3];
                    for i in 0..3 {
                        // rgb[i] = (zero_rgb[i] as f32 * (1.0 - percent) + end_rgb[i] as f32 * percent) as u8;
                        rgb[i] = (zero_rgb[i] as f32  + ((end_rgb[i] as f32 - zero_rgb[i] as f32) * percent)) as u8;
                    }
                    rgb
                }
                // let r1 = (r1 + (r2 - r1) * percent) as f32;
                let rgb = calc_rgb(percent as f32, zero_rgb, end_rgb);
                outline_color = Color32::from_rgba_premultiplied(rgb[0], rgb[1], rgb[2], 80);
            }
            // 绘制填充的圆角矩形
            painter.rect_filled(
                text_rect_a,
                CornerRadiusF32::same(corner_radius), // 所有角的圆角半径相同
                fill_color,
            );
            painter.rect_stroke(
                text_rect_a_outline,
                CornerRadiusF32::same(corner_radius), // 所有角的圆角半径相同
                Stroke::new(3.0, outline_color),
                StrokeKind::Outside,
            );
        });
        // println!("🪵 [dash.rs:471]~ token ~ \x1b[0;32m(race_data.tire_wear1 * 42.0)\x1b[0m = {}", (race_data.tire_wear1 * 42.0));
        let text_size = Vec2::new(14.0, (tire * 42.0) as f32); // 文本区域宽度比背景小 20，高度 50
        let text_rect_a = Rect::from_min_size(text_pos, text_size);
        ui.allocate_new_ui(UiBuilder::new().max_rect(text_rect_a), |ui_at_rect| {
            // 获取 painter
            let painter = ui_at_rect.painter();
            // 定义填充颜色: #A2000000 (ARGB) -> 64% 透明度的黑色 (RGBA: 0,0,0,162)
            let fill_color = Color32::from_rgba_premultiplied(191, 61, 55, 250);
            // 定义圆角半径
            let corner_radius = 2.0; // 较大的圆角，更明显
                                     // 绘制填充的圆角矩形
            painter.rect_filled(
                text_rect_a,
                CornerRadiusF32::same(corner_radius), // 所有角的圆角半径相同
                fill_color,
            );
        });
        let text_pos = rect.left_top() + pos_cost_tire; // 距离左上角 10 像素
        let text_size = Vec2::new(32.0, 26.0); // 文本区域宽度比背景小 20，高度 50
        let text_rect_a = Rect::from_min_size(text_pos, text_size);
        ui.allocate_new_ui(UiBuilder::new().max_rect(text_rect_a), |ui_at_rect| {
            ui_at_rect.label(
                RichText::new(format!("{:.1}", tire_last_lap))
                    .font(FontId::monospace(14.0))
                    .color(Color32::WHITE),
            );
        });
    };

    render_single_tire(
        Vec2 { x: 10.0, y: 28.0 },
        Vec2 { x: 5.0, y: 6.0 },
        race_data.tire_wear1,
        remaind_tire1,
        race_data.last_lap_tire_wear1,
        slip1,
    );
    render_single_tire(
        Vec2 { x: 90.0, y: 28.0 },
        Vec2 { x: 86.0, y: 6.0 },
        race_data.tire_wear2,
        remaind_tire2,
        race_data.last_lap_tire_wear2,
        slip2,
    );
    render_single_tire(
        Vec2 { x: 10.0, y: 88.0 },
        Vec2 { x: 5.0, y: 133.0 },
        race_data.tire_wear3,
        remaind_tire3,
        race_data.last_lap_tire_wear3,
        slip3,
    );
    render_single_tire(
        Vec2 { x: 90.0, y: 88.0 },
        Vec2 { x: 86.0, y: 133.0 },
        race_data.tire_wear4,
        remaind_tire4,
        race_data.last_lap_tire_wear4,
        slip4,
    );
}

fn render_boost(
    ctx: &egui::Context,
    ui: &mut egui::Ui,
    app: &mut MyApp2,
    rect: Rect,
    race_data: &mut MutexGuard<'_, GameRaceData>,
    car_setting: &MutexGuard<'_, CurCarRpmSetting>,
    texture_map: &MutexGuard<'_, BTreeMap<String, egui::TextureHandle>>,
) {
    let text_pos = rect.left_top() + Vec2::new(1060.0, 4.0); // 距离左上角 10 像素
    let text_size = Vec2::new(136.8, 148.8); // 文本区域宽度比背景小 20，高度 50
    let text_rect_a = Rect::from_min_size(text_pos, text_size);
    let mut boost = race_data.boost;
    // println!(
    //     "🪵 [dash.rs:546]~ token ~ \x1b[0;32mboost\x1b[0m = {}",
    //     boost
    // );
    // if boost > 100.0 {
    //     boost = 100.0;
    // }
    boost = boost / 100.0;
   

    ui.allocate_new_ui(UiBuilder::new().max_rect(text_rect_a), |ui_at_rect| {
        ui_at_rect.painter().image(
            texture_map.get("turbo_img").unwrap().id(),
            text_rect_a, // 图片将填充整个面板
            egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)), // UV 坐标 (0.0-1.0)
            Color32::WHITE, // 图片的色调 (White 表示原色)
        );

        if boost >= 0.0 {
            let half_total_len = 90.0;
            let total_len = half_total_len * 2.0;
            let mut end_degree = 270.0 - half_total_len;
            // println!("🪵 [dash.rs:355]~ token ~ \x1b[0;32mrace_data.fuel\x1b[0m = {}", race_data.fuel);
            // end_degree = end_degree + total_len * 1.0;
            end_degree = end_degree + total_len * boost;
            // * 2.5;
            // 定义圆弧的中心位置
            // 我们将其放置在标签旁边以便观察
            // ui_at_rect.label(RichText::new("FUEL").color(Color32::WHITE));
            let center = ui_at_rect.cursor().min + Vec2::new(68.0, 75.0);
            // let  dash_color = Color32::from_rgba_premultiplied(0, 255, 255, 250);

            // **设置半径为 20px**
            let radius = 46.0;

            // 定义顶部圆弧的角度范围
            // 顶部意味着围绕 270 度（或 -90 度）。
            // 我们选择一个 90 度的扇形（从 225 度到 315 度）来表示 "顶部一小段"。
            // to_radians() 将角度转换为弧度
            let start_angle = ((270.0 - half_total_len) as f32).to_radians(); // 225 度
            let end_angle = (end_degree as f32).to_radians(); // 315 度
                                                              // 生成构成圆弧的点
            let n_points = 100; // 对于小圆弧，不需要太多点
            let points: Vec<Pos2> = (0..=n_points)
                .map(|i| {
                    let angle =
                        start_angle + (end_angle - start_angle) * (i as f32 / n_points as f32);
                    let x = center.x + radius * angle.cos();
                    let y = center.y + radius * angle.sin();
                    Pos2::new(x, y)
                })
                .collect();

            // 创建一个路径形状 (PathShape)
            let arc_shape = PathShape {
                points,
                closed: false, // 设置为 false，使其成为一条线而不是闭合图形
                fill: Color32::TRANSPARENT, // 无填充色
                stroke: PathStroke::new(4.0, Color32::WHITE), // 设置线条粗细和颜色
            };
            // 使用 painter 将形状添加到 UI
            ui_at_rect.painter().add(arc_shape);
        } else {
            let boost = boost.abs();
            let mut zero_degree = 180.0;
            let half_total_len = 90.0;
            let total_len = half_total_len;
            let diff = total_len * boost * 100.0 / 20.0;
            zero_degree = zero_degree + total_len - diff;
            // let total_len = half_total_len * 2.0;
            let mut end_degree = zero_degree - half_total_len; //起点
                                                               // println!("🪵 [dash.rs:355]~ token ~ \x1b[0;32mrace_data.fuel\x1b[0m = {}", race_data.fuel);
                                                               // end_degree = end_degree + total_len * 1.0;
            end_degree = end_degree + diff;
            // println!("🪵 [dash.rs:625]~ token ~ \x1b[0;32mend_degree\x1b[0m = {}", total_len * boost/20.0);
            // 定义圆弧的中心位置
            // 我们将其放置在标签旁边以便观察
            // ui_at_rect.label(RichText::new("FUEL").color(Color32::WHITE));
            let center = ui_at_rect.cursor().min + Vec2::new(68.0, 75.0);
            // let  dash_color = Color32::from_rgba_premultiplied(0, 255, 255, 250);

            // **设置半径为 20px**
            let radius = 45.0;

            // 定义顶部圆弧的角度范围
            // 顶部意味着围绕 270 度（或 -90 度）。
            // 我们选择一个 90 度的扇形（从 225 度到 315 度）来表示 "顶部一小段"。
            // to_radians() 将角度转换为弧度
            let start_angle = ((zero_degree - half_total_len) as f32).to_radians(); // 225 度
            let end_angle = (end_degree as f32).to_radians(); // 315 度
                                                              // 生成构成圆弧的点
            let n_points = 100; // 对于小圆弧，不需要太多点
            let points: Vec<Pos2> = (0..=n_points)
                .map(|i| {
                    let angle =
                        start_angle + (end_angle - start_angle) * (i as f32 / n_points as f32);
                    let x = center.x + radius * angle.cos();
                    let y = center.y + radius * angle.sin();
                    Pos2::new(x, y)
                })
                .collect();

            // 创建一个路径形状 (PathShape)
            let arc_shape = PathShape {
                points,
                closed: false, // 设置为 false，使其成为一条线而不是闭合图形
                fill: Color32::TRANSPARENT, // 无填充色
                stroke: PathStroke::new(4.0, Color32::WHITE), // 设置线条粗细和颜色
            };
            // 使用 painter 将形状添加到 UI
            ui_at_rect.painter().add(arc_shape);
        }
    });
}

fn render_dot(
    ctx: &egui::Context,
    ui: &mut egui::Ui,
    app: &mut MyApp2,
    rect: Rect,
    race_data: &mut MutexGuard<'_, GameRaceData>,
    car_setting: &MutexGuard<'_, CurCarRpmSetting>,
    texture_map: &MutexGuard<'_, BTreeMap<String, egui::TextureHandle>>,
) {
    let steer = race_data.steer;
    let zero_pos = rect.left_top() + Vec2::new(658.0, 10.0);

    let y = (steer / 127.0).abs().powf(2.0) * 13.0;
    let diff = Vec2::new((209.0 * steer / 127.0) as f32, y as f32);
    // --- 绘制小圆点 ---
    let dot_pos = zero_pos + diff;
    let dot_radius = 3.5;
    let dot_color = Color32::RED;
    ui.painter().circle_filled(dot_pos, dot_radius, dot_color);
}

pub fn load_img(ctx: &egui::Context, app: &mut MyApp2) {
    // 将图片数据嵌入到二进制文件中
    let image_data = include_bytes!("../../resource/GT_hud_LITE_VERSION5.png"); // 确保路径正确
    let image_data2 = include_bytes!("../../resource/fuel_background.png"); // 确保路径正确
    let image_data3 = include_bytes!("../../resource/turbo_background.png"); // 确保路径正确
    let image_data4 = include_bytes!("../../resource/gap_gradient.png"); // 确保路径正确
    let image_data5 = include_bytes!("../../resource/fl_gradient.png"); // 确保路径正确
    let image_data6 = include_bytes!("../../resource/fuel_inner.png"); // 确保路径正确
    let image_data7 = include_bytes!("../../resource/gap_gradient_good.png"); // 确保路径正确
    let mut texture_list = TEXTURE_HANDLE_MAP.get().unwrap().lock().unwrap();

    let mut load_fn = |imgd: &[u8], id: &str| {
        // 使用 image crate 解码图片
        let img = ImageReader::new(Cursor::new(imgd))
            .with_guessed_format()
            .expect("Failed to guess image format")
            .decode()
            .expect("Failed to decode image")
            .into_rgba8(); // 转换为 RGBA8 格式

        let dimensions = img.dimensions();
        let pixels = img.into_flat_samples();

        // 将图像数据转换为 egui::ColorImage
        let color_image = egui::ColorImage::from_rgba_unmultiplied(
            [dimensions.0 as usize, dimensions.1 as usize],
            pixels.as_slice(),
        );
        // 将 ColorImage 转换为 egui 纹理
        let texture = Some(ctx.load_texture(
            id, // 纹理的唯一 ID
            color_image,
            TextureOptions::LINEAR, // 纹理过滤，影响图片缩放时的平滑度
        ))
        .unwrap();
        texture_list.insert(id.to_string(), texture);
    };
    load_fn(image_data, "gt_hud_img");
    load_fn(image_data2, "fuel_img");
    load_fn(image_data3, "turbo_img");
    load_fn(image_data4, "history_img");
    load_fn(image_data5, "history_best_img");
    load_fn(image_data6, "fuel_icon");
    load_fn(image_data7, "history_good_img");
}
