use std::{
    collections::BTreeMap,
    sync::{atomic::Ordering, MutexGuard},
};

use eframe::{
    egui::{self, Area, Color32, Rect, RichText, Stroke, StrokeKind, UiBuilder, Vec2},
    epaint::CornerRadiusF32,
};

use crate::{
    enums::GameRaceData,
    ui::index::{MyApp2, GAME_RACE_DATA, IS_MOUSE_PASS, SECTOR_RECORD_DATA, TEXTURE_HANDLE_MAP},
    uitl::format_milliseconds_to_mmssms,
};

pub fn render_history(ctx: &egui::Context, app: &mut MyApp2) {
    if !app.show_state.show_history {
        return;
    }
    let mut race_data = GAME_RACE_DATA.get().unwrap().lock().unwrap();
    let texture_map = TEXTURE_HANDLE_MAP.get().unwrap().lock().unwrap();
    let is_mouse_pass = IS_MOUSE_PASS
        .get()
        .unwrap()
        .lock()
        .unwrap()
        .load(Ordering::SeqCst);

    let res = Area::new("history".into())
        .current_pos(egui::pos2(app.history_pos.x, app.history_pos.y)) // 位置, 400.0 + app.yoffset)) // 位置
        .movable(true) //
        .show(ctx, |ui| {
            let len = app.setting_data.history_len.parse::<f32>();
            let len = match len {
                Ok(len) => len,
                Err(e) => app.setting_data.history_base_len,
            };
            let height = len / app.setting_data.history_scale;
            let scale_to_base = len / app.setting_data.history_base_len;
            let desired_size = egui::vec2(len, height);
            // 分配一个精确大小的区域，这将是我们绘制矩形的边界
            let (rect, _response) = ui.allocate_exact_size(desired_size, egui::Sense::hover());
            if !is_mouse_pass {
                let painter = ui.painter();
                painter.rect_stroke(
                    rect,
                    CornerRadiusF32::same(6.0), // 所有角的圆角半径相同
                    Stroke::new(3.0, Color32::WHITE),
                    StrokeKind::Outside,
                );
            }

            render_lap_table(
                ctx,
                ui,
                app,
                rect,
                &mut race_data,
                &texture_map,
                len,
                scale_to_base,
                desired_size,
            );
        })
        .response;

    if res.dragged() {
        app.history_pos += res.drag_delta();
    }
}

fn render_lap_table(
    ctx: &egui::Context,
    ui: &mut egui::Ui,
    app: &mut MyApp2,
    rect: Rect,
    race_data: &mut MutexGuard<'_, GameRaceData>,
    texture_map: &MutexGuard<'_, BTreeMap<String, egui::TextureHandle>>,
    len: f32,
    scale_to_base: f32,
    desired_size: egui::Vec2,
) {
    let height = 30.0 * scale_to_base;
    let left_width = 42.0 * scale_to_base;
    let cur_time_str = format_milliseconds_to_mmssms((race_data.current_time * 1000.0) as u32);
    let hisoty_str_list = &race_data.lap_history_str;
    let hisoty_list = &race_data.lap_history;
    let his_list_len = hisoty_str_list.len() as i32;

    let sector_data = SECTOR_RECORD_DATA.get().unwrap().lock().unwrap();
    let opt_best_time =
        sector_data.s1.best_time + sector_data.s2.best_time + sector_data.s3.best_time;
    let opt_best_time = if his_list_len == 0 {"--'--.---".to_string()} else {format_milliseconds_to_mmssms((opt_best_time * 1000.0) as u32)};

    let mut render_one_lap = |i: i32, time_str: &str, lap_str: &str| {
        let y_other_offset= if i < 0 {-8.0} else {0.0};
        let lap_x_other_offset= if i < 0 {-5.0} else {0.0};
        let lap_y_other_offset= if i < 0 {-2.0} else {0.0};
        let time_s = if i > 0 || i == -1 {
            time_str
        } else {
            &cur_time_str
        };
        let lap_str = if i > 0 {
            i.to_string()
        } else if i == -1 {
            lap_str.to_string()
        } else {
            (race_data.current_lap + 1).to_string()
        };

        let y_pos = if i > 0 {
            (his_list_len - i + 1) as f32 + height * (his_list_len - i + 1) as f32
        } else {
            height * i as f32 + y_other_offset
        };
        let text_pos = rect.left_top() + Vec2::new(left_width, y_pos); // 距离左上角 10 像素
        let text_size = Vec2::new(len - left_width, height); // 文本区域宽度比背景小 20，高度 50
        let text_rect_a = Rect::from_min_size(text_pos, text_size);
        ui.allocate_new_ui(UiBuilder::new().max_rect(text_rect_a), |ui_at_rect| {
            let img_id = if race_data.current_lap + 1 > 3
                && i > 0
                && hisoty_list[(i - 1) as usize] == race_data.best_lap_time
            {
                "history_best_img"
            } else if race_data.current_lap + 1 > 3
                && i > 0
                && hisoty_list[(i - 1) as usize] <= race_data.best_lap_time * 1.00125
            {
                "history_good_img"
            } else {
                "history_img"
            };
            ui_at_rect.painter().image(
                texture_map.get(img_id).unwrap().id(),
                text_rect_a, // 图片将填充整个面板
                egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)), // UV 坐标 (0.0-1.0)
                Color32::WHITE, // 图片的色调 (White 表示原色)
            );
        });
        let text_pos = rect.left_top() + Vec2::new(left_width, 9.0 * scale_to_base + y_pos); // 距离左上角 10 像素
        let text_rect_a = Rect::from_min_size(text_pos, text_size);
        ui.allocate_new_ui(UiBuilder::new().max_rect(text_rect_a), |ui_at_rect| {
            ui_at_rect.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
                let idx = if i > 0 {
                    i as usize
                } else {
                    (race_data.current_lap + 1) as usize
                };
                let p_str = if race_data.out_pit_lap_list.contains(&idx) {
                    "P"
                } else {
                    ""
                };
                // ui.add_space(12.0);
                ui.label(
                    RichText::new(format!("  {}   {}", time_s, p_str))
                        .color(Color32::WHITE)
                        .family(egui::FontFamily::Name("base".into()))
                        .size(18.0 * scale_to_base),
                );
            });
        });

        let text_pos = rect.left_top() + Vec2::new(0.0, 0.0 + y_pos); // 距离左上角 10 像素
        let text_size = Vec2::new(left_width, height); // 文本区域宽度比背景小 20，高度 50
        let text_rect_a = Rect::from_min_size(text_pos, text_size);
        ui.allocate_new_ui(UiBuilder::new().max_rect(text_rect_a), |ui_at_rect| {
            // 定义圆角矩形的尺寸
            let desired_size = egui::vec2(left_width, height);
            // 分配一个精确大小的区域，这将是我们绘制矩形的边界
            let (rect, _response) =
                ui_at_rect.allocate_exact_size(desired_size, egui::Sense::hover());
            // 获取 painter
            let painter = ui_at_rect.painter();
            // 定义填充颜色: #A2000000 (ARGB) -> 64% 透明度的黑色 (RGBA: 0,0,0,162)
            let fill_color = Color32::from_rgba_premultiplied(215, 219, 225, 210);
            // 定义圆角半径
            let corner_radius = 3.0; // 较大的圆角，更明显
                                     // 绘制填充的圆角矩形
            painter.rect_filled(
                text_rect_a,
                CornerRadiusF32::same(corner_radius), // 所有角的圆角半径相同
                fill_color,
            );
        });

        let x_diff = if lap_str.len() >= 2 { -6.0+lap_x_other_offset } else { 0.0 };
        let text_pos =
            rect.left_top() + Vec2::new((16.0 + x_diff) * scale_to_base, 0.0 + y_pos - 1.0 + lap_y_other_offset); // 距离左上角 10 像素
        let text_rect_a = Rect::from_min_size(text_pos, text_size);
        ui.allocate_new_ui(UiBuilder::new().max_rect(text_rect_a), |ui_at_rect| {
            ui_at_rect.add_space(11.0 * scale_to_base);
            ui_at_rect.label(
                RichText::new(format!("{}", lap_str))
                    .color(Color32::BLACK)
                    .family(egui::FontFamily::Name("base".into()))
                    .size(18.0 * scale_to_base),
            );
        });
    };

    render_one_lap(0, &cur_time_str, "");

    if app.show_state.show_opt_time {
        render_one_lap(-1, &opt_best_time, "Opt");
    }

    let min_idx = if his_list_len > 10 {
        his_list_len - 10 + 1
    } else {
        1
    };
    if his_list_len > 0 {
        for i in (min_idx..=his_list_len).rev() {
            // println!("🪵 [lap_history.rs:149]~ token ~ \x1b[0;32m&hisoty_str_list[i - 1]\x1b[0m = {}", &hisoty_str_list[i - 1]);
            render_one_lap(i, &hisoty_str_list[(i - 1) as usize], "");
        }
    }

    // hisoty_str.iter().for_each(|s| {
    //     // let text_pos = rect.left_top() + Vec2::new(455.0, 56.0); // 距离左上角 10 像素
    //     // let text_size = Vec2::new(180.0, 44.0); // 文本区域宽度比背景小 20，高度 50
    //     // let text_rect_a = Rect::from_min_size(text_pos, text_size);

    //     // ui.allocate_new_ui(UiBuilder::new().max_rect(text_rect_a), |ui_at_rect| {});
    // });
}
