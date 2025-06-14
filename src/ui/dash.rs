use eframe::{
    egui::{self, Area, Color32, FontId, Layout, Rect, RichText, TextureOptions, UiBuilder, Vec2},
    epaint::CornerRadiusF32,
};
use image::ImageReader;
use std::{io::Cursor, sync::atomic::Ordering}; // 用于加载图片

use crate::{
    ui::index::{MyApp2, GAME_RACE_DATA, IS_MOUSE_PASS, LAST_TELE_DATA, TEXTURE_HANDLE_MAP},
    uitl::format_milliseconds_to_mmssms,
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
    let race_data = GAME_RACE_DATA.get().unwrap().lock().unwrap();
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

            let text_pos = rect.left_top() + Vec2::new(660.0, 56.0); // 距离左上角 10 像素
            let text_size = Vec2::new(180.0, 44.0); // 文本区域宽度比背景小 20，高度 50
            let text_rect_a = Rect::from_min_size(text_pos, text_size);
            ui.allocate_new_ui(UiBuilder::new().max_rect(text_rect_a), |ui_at_rect| {
                // if race_data.clutch > 0.0 {
                //   println!("🪵 [dash.rs:94]~ token ~ \x1b[0;32mrace_data.clutch\x1b[0m = {} {}", race_data.clutch,race_data.gear);
                // }
                let gear = if race_data.gear == 11 {
                    "N".to_string()
                } else {
                    race_data.gear.to_string()
                };
                ui_at_rect.label(
                    RichText::new(gear)
                        .color(Color32::WHITE)
                        .font(FontId::proportional(60.0)),
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
            let yoffset = 37.0+84.0-brake;
            let text_pos = rect.left_top() + Vec2::new(380.0, yoffset); // 距离左上角 10 像素
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
            let yoffset = 37.0+84.0-acc;
            let text_pos = rect.left_top() + Vec2::new(897.0, yoffset); // 距离左上角 10 像素
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
        })
        .response;
    if res.dragged() {
        app.hud_pos += res.drag_delta();
        // println!("🪵 [dash.rs:76]~ token ~ \x1b[0;32mapp.hud_pos\x1b[0m = {} {}", app.hud_pos.x,app.hud_pos.x,);
    }
}

pub fn load_img(ctx: &egui::Context, app: &mut MyApp2) {
    // 将图片数据嵌入到二进制文件中
    let image_data = include_bytes!("../../resource/GT_hud_LITE_VERSION5.png"); // 确保路径正确
    let image_data2 = include_bytes!("../../resource/fuel_background.png"); // 确保路径正确
    let image_data3 = include_bytes!("../../resource/turbo_background.png"); // 确保路径正确
    let mut texture_list = TEXTURE_HANDLE_MAP.get().unwrap().lock().unwrap();

    let mut load_fn = |img_data: &[u8], id: &str| {
        // 使用 image crate 解码图片
        let img = ImageReader::new(Cursor::new(image_data))
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
}
