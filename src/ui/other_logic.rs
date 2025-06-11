use std::{
    collections::{BTreeMap, HashSet},
    mem,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{self},
        Mutex, MutexGuard,
    },
    thread,
    time::Duration,
};

use eframe::{
    egui::{self, Align2, Color32, FontId, Modifiers, RichText},
    Frame,
};
use egui_toast::{Toast, ToastKind, ToastOptions, Toasts};
use rdev::{listen, EventType, Key};
use tokio::time::sleep;

use crate::{
    controllers::udp::{init_udp, THREAD_RUNINNG_FLAG},
    enums::{GameRaceData, TeleData},
    ui::index::{
        ERROR_RX, ERROR_SHOW_FLAG, GAME_RACE_DATA, IS_FIRST, IS_MOUSE_PASS, KEYRECORD, LAST_IS_MOUSE_PASS, LAST_TELE_DATA, RESTART_UDP_FLAG, RXHOLDER, TELE_DATA_RX, TXHOLDER
    },
};

pub struct KeyData {
    pub key: Key,
    pub mouse_pass: bool,
}

fn test_mouse_pass_change(bb: bool) {
    let handle = IS_MOUSE_PASS
        .get_or_init(|| Mutex::new(AtomicBool::new(true)))
        .lock();
    let handle = match handle {
        Ok(h) => h,
        Err(e) => return,
    };
    handle.store(bb, Ordering::SeqCst);
    // sleep(tokio::time::Duration::from_millis(100)).await;
    // handle.store(true, Ordering::SeqCst);
}

pub fn check_first(ctx: &egui::Context, app: &mut crate::ui::index::MyApp2) {
    // println!("check first");
    let is_first_handle = IS_FIRST
        .get_or_init(|| Mutex::new(AtomicBool::new(true)))
        .lock();
    let is_first_handle = match is_first_handle {
        Ok(h) => h,
        Err(e) => return,
    };
    if is_first_handle.load(Ordering::SeqCst) {
        println!(
            "🪵 [other_logic.rs:33]~ token ~ \x1b[0;32mis_first_handle\x1b[0m = {}",
            ""
        );
        // ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(Pos2::new(
        //     app.pox, app.poy,
        // )));
        // println!(
        //     "🪵 [index.rs:202]~ token ~ \x1b[0;32mapp.pox,\x1b[0m = {} {}",
        //     app.pox, app.poy
        // );
        // test_mouse_pass_change(false);

        tokio::spawn(async move {
            sleep(tokio::time::Duration::from_millis(800)).await;
            let handle = IS_MOUSE_PASS
                .get_or_init(|| Mutex::new(AtomicBool::new(true)))
                .lock();
            let handle = match handle {
                Ok(h) => h,
                Err(e) => {
                    println!("IS_MOUSE_PASS error: {}", e);
                    return;
                }
            };
            handle.store(true, Ordering::SeqCst);
        });
        is_first_handle.store(false, Ordering::SeqCst);

        let handle = IS_MOUSE_PASS
            .get_or_init(|| Mutex::new(AtomicBool::new(true)))
            .lock();
        let handle = match handle {
            Ok(h) => h,
            Err(e) => {
                println!("IS_MOUSE_PASS error: {}", e);
                return;
            }
        };
        handle.store(false, Ordering::SeqCst);
        // ctx.request_repaint_after(Duration::from_millis(850));
    }
}

pub fn key_listener() {
    // SHORTCUT_RX.set(rx).unwrap();
    let res = listen(move |event| {
        if event.event_type == EventType::KeyPress(Key::F2) {
            // println!("🪵 [index.rs:33]~ token ~ \x1b[0;32mF2\x1b[0m = ",);
            {
                let handle = IS_MOUSE_PASS
                    .get_or_init(|| Mutex::new(AtomicBool::new(true)))
                    .lock();
                let handle = match handle {
                    Ok(h) => h,
                    Err(e) => {
                        println!(
                            "🪵 [index.rs:36]~ token ~ \x1b[0;32mhandle\x1b[0m = {}",
                            "locking"
                        );
                        return;
                    }
                };
                let val = handle.load(Ordering::SeqCst);
                // println!("🪵 [index.rs:36]~ token ~ \x1b[0;32mval\x1b[0m = {}", !val);
                handle.store(!val, Ordering::SeqCst);
            }
        }
        if event.event_type == EventType::KeyPress(Key::ControlLeft) {
            let mut key_record = KEYRECORD
                .get_or_init(|| Mutex::new(HashSet::new()))
                .lock()
                .expect("lock error");
            key_record.insert(Key::ControlLeft);
            check_ctrl_q(key_record);
        }
        if event.event_type == EventType::KeyRelease(Key::ControlLeft) {
            let mut key_record = KEYRECORD
                .get_or_init(|| Mutex::new(HashSet::new()))
                .lock()
                .expect("lock error");
            key_record.remove(&Key::ControlLeft);
        }
        if event.event_type == EventType::KeyPress(Key::KeyQ) {
            let mut key_record = KEYRECORD
                .get_or_init(|| Mutex::new(HashSet::new()))
                .lock()
                .expect("lock error");
            key_record.insert(Key::KeyC);
            check_ctrl_q(key_record);
        }
        if event.event_type == EventType::KeyRelease(Key::KeyQ) {
            let mut key_record = KEYRECORD
                .get_or_init(|| Mutex::new(HashSet::new()))
                .lock()
                .expect("lock error");
            key_record.remove(&Key::KeyC);
        }
    });
    match res {
        Ok(_) => {}
        Err(e) => println!("Key Listen Error"),
    }
}

pub fn key_listener_focus(ctx: &egui::Context, app: &mut crate::ui::index::MyApp2) {
    ctx.input(|input| {
        if input.key_pressed(egui::Key::F2) {
            let handle = IS_MOUSE_PASS
                .get_or_init(|| Mutex::new(AtomicBool::new(true)))
                .lock();
            let handle = match handle {
                Ok(h) => h,
                Err(e) => return,
            };
            let is_mouse_pass = handle.load(Ordering::SeqCst);
            handle.store(!is_mouse_pass, Ordering::SeqCst);
        }

        if input
            .modifiers
            .matches_exact(Modifiers::COMMAND | Modifiers::CTRL)
            && input.key_pressed(egui::Key::Q)
        {
            // This block will execute if Ctrl+C (or Cmd+C on macOS) is pressed
            // println!("Ctrl+Q detected!");
            let handle = IS_MOUSE_PASS
                .get_or_init(|| Mutex::new(AtomicBool::new(true)))
                .lock();
            let handle = match handle {
                Ok(h) => h,
                Err(e) => return,
            };
            let is_mouse_pass = handle.load(Ordering::SeqCst);
            handle.store(!is_mouse_pass, Ordering::SeqCst);
            tokio::spawn(async {
                tokio::time::sleep(tokio::time::Duration::from_millis(15)).await;
                let tx = TXHOLDER.get().unwrap().lock().unwrap();
                let mut tdata = TeleData::default();
                tdata.close = true;
                tx.send(tdata).unwrap();
            });
        }
    });
}

pub fn listen_mouse_pass_event(ctx: &egui::Context, app: &mut crate::ui::index::MyApp2) {
    let handle = IS_MOUSE_PASS
        .get_or_init(|| Mutex::new(AtomicBool::new(true)))
        .lock();
    let handle = match handle {
        Ok(h) => h,
        Err(e) => {
            println!("IS_MOUSE_PASS error: {}", e);
            return;
        }
    };
    let handle_last = LAST_IS_MOUSE_PASS
        .get_or_init(|| Mutex::new(AtomicBool::new(true)))
        .lock();
    let handle_last = match handle_last {
        Ok(h) => h,
        Err(e) => {
            println!("IS_MOUSE_PASS error: {}", e);
            return;
        }
    };
    let is_mouse_pass = handle.load(Ordering::SeqCst);
    let last_is_mouse_pass = handle_last.load(Ordering::SeqCst);

    if is_mouse_pass && last_is_mouse_pass != is_mouse_pass {
        // println!("🪵 [index.rs:312]~ token ~ \x1b[0;32mlast_is_mouse_pass\x1b[0m = {}", last_is_mouse_pass);
        // println!(
        //     "🪵 [index.rs:312]~ token ~ \x1b[0;32mis_mouse_pass\x1b[0m = {}",
        //     is_mouse_pass
        // );
        app.show_ui = true;
        ctx.send_viewport_cmd(egui::ViewportCommand::MousePassthrough(true));
        // ctx.send_viewport_cmd(egui::ViewportCommand::WindowLevel(egui::WindowLevel::AlwaysOnTop));
        handle_last.store(is_mouse_pass, Ordering::SeqCst);
        // control_dec(app.show_ui);
    }
    if !is_mouse_pass && last_is_mouse_pass != is_mouse_pass {
        // println!("🪵 [index.rs:312]~ token ~ \x1b[0;32mlast_is_mouse_pass\x1b[0m = {}", last_is_mouse_pass);
        // println!(
        //     "🪵 [index.rs:312]~ token ~ \x1b[0;32mis_mouse_pass\x1b[0m = {}",
        //     is_mouse_pass
        // );
        app.show_ui = false;
        ctx.send_viewport_cmd(egui::ViewportCommand::MousePassthrough(false));
        // ctx.send_viewport_cmd(egui::ViewportCommand::WindowLevel(
        //     egui::WindowLevel::AlwaysOnTop,
        // ));
        handle_last.store(is_mouse_pass, Ordering::SeqCst);
        // set_need_fix_pos();
        // fix_viewport_size(ctx);
    }
}

pub fn global_hk() {
    let handle = thread::spawn(move || {
        key_listener();
    });
}

pub fn check_is_focus(ctx: &egui::Context, app: &mut crate::ui::index::MyApp2) {
    ctx.request_repaint_after(Duration::from_millis(16));
    // ctx.request_repaint_after(Duration::from_millis(32));

    //   // ctx.input(|i| i.is_focus_changed());
    //   let is_focused = ctx.input(|i| i.raw.focused);
    //   if is_focused {
    //     // println!("is focused");
    //   }else{
    //     // println!("is not focused");
    //     ctx.request_repaint_after(Duration::from_millis(16));

    //   }
}

fn check_ctrl_q(key_record: MutexGuard<'_, HashSet<rdev::Key>>) {
    let tx = TXHOLDER.get().unwrap().lock().unwrap();
    if key_record.contains(&Key::KeyC) && key_record.contains(&Key::ControlLeft) {
        let mut tdata = TeleData::default();
        tdata.close = true;
        tx.send(tdata).unwrap();
    }
}

pub fn rev_rx(ctx: &egui::Context, app: &mut crate::ui::index::MyApp2, frame: &mut Frame) {
    let rx = RXHOLDER.get().expect("rx get error");
    let rx = rx.lock().expect("lock error");
    match rx.try_recv() {
        Ok(msg) => {
            // println!("[Receiver] 收到 (非阻塞): {}", msg.close);
            if msg.close {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
        }
        Err(mpsc::TryRecvError::Empty) => {
            // 通道为空，没有新消息
            // println!("[Receiver] 通道为空，执行其他工作...");
            // thread::sleep(Duration::from_millis(200)); // 模拟做其他工作
        }
        Err(mpsc::TryRecvError::Disconnected) => {
            // 所有发送端都已关闭，通道已断开
            // println!("[Receiver] 所有发送端已断开，退出接收循环。");
        }
    }
}

pub fn check_udp_run(ctx: &egui::Context, app: &mut crate::ui::index::MyApp2) {
    let running = THREAD_RUNINNG_FLAG.get_or_init(|| AtomicBool::new(false));
    let restart_udp_flag = RESTART_UDP_FLAG.get_or_init(|| AtomicBool::new(true));
    if !running.load(Ordering::SeqCst) && restart_udp_flag.load(Ordering::SeqCst) {
        let _ = init_udp(app);
        restart_udp_flag.store(false, Ordering::SeqCst);
        tokio::spawn(async move {
            sleep(Duration::from_millis(200)).await;
            restart_udp_flag.store(true, Ordering::SeqCst);
        });
    }
}

pub fn render_error(ctx: &egui::Context, app: &mut crate::ui::index::MyApp2, frame: &mut Frame) {
    let err_rx = ERROR_RX.get().expect("rx get error");
    let err_rx = err_rx.lock().expect("lock error");
    let err_show_flag = ERROR_SHOW_FLAG.get_or_init(|| AtomicBool::new(true));
    let mut toasts = Toasts::new()
        .anchor(Align2::LEFT_TOP, (10.0, 10.0))
        .direction(egui::Direction::TopDown);
    match err_rx.try_recv() {
        Ok(err) => {
            println!("[Receiver] 收到 (非阻塞): {}", err.message);
            if err_show_flag.load(Ordering::SeqCst) {
                toasts.add(Toast {
                    text: RichText::new(err.message)
                        .font(FontId::monospace(14.0))
                        .color(Color32::WHITE)
                        .into(),
                    kind: ToastKind::Error,
                    options: ToastOptions::default()
                        .duration_in_seconds(3.0)
                        .show_progress(true)
                        .show_icon(true),
                    ..Default::default()
                });

                // Show all toasts

                err_show_flag.store(false, Ordering::SeqCst);
                tokio::spawn(async move {
                    sleep(Duration::from_millis(4000)).await;
                    err_show_flag.store(true, Ordering::SeqCst);
                });
            }
        }
        Err(mpsc::TryRecvError::Empty) => {
            // 通道为空，没有新消息
            // println!("[Receiver] 通道为空，执行其他工作...");
            // thread::sleep(Duration::from_millis(200)); // 模拟做其他工作
        }
        Err(mpsc::TryRecvError::Disconnected) => {
            // 所有发送端都已关闭，通道已断开
            // println!("[Receiver] 所有发送端已断开，退出接收循环。");
        }
    }
    toasts.show(ctx);
}

pub fn receive_upd_data() {
    tokio::spawn(async move {
        loop {
            let tele_rx = TELE_DATA_RX.get().unwrap().lock().unwrap();
            

            let tele_data = match tele_rx.recv() {
                Ok(data) => {
                    let mut last_tele_data = LAST_TELE_DATA.get().unwrap().lock().unwrap();
                    // println!("[Receiver] 收到 (非阻塞): {}", msg.close);
                    let old_map = mem::replace(&mut *last_tele_data, data);
                }
                Err(e)=>{

                }
            };
            // thread::sleep(Duration::from_millis(4));
            // tokio::time::sleep(tokio::time::Duration::from_millis(4)).await;
        }
    });
}


