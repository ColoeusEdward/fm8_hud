use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc, Mutex,
    },
    thread,
};

use eframe::egui::{self, Pos2};
use global_hotkey::{hotkey::HotKey, GlobalHotKeyEvent};
use rdev::{listen, EventType, Key, ListenError};
use tokio::{task::LocalSet, time::sleep};

use crate::ui::index::{IS_FIRST, IS_MOUSE_PASS, LAST_IS_MOUSE_PASS};

pub struct keyData {
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

pub fn check_first(ctx: &egui::Context, app: &mut crate::ui::index::MyApp) {
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
            sleep(tokio::time::Duration::from_millis(1500)).await;
            let handle = IS_MOUSE_PASS
                .get_or_init(|| Mutex::new(AtomicBool::new(true)))
                .lock();
            let handle = match handle {
                Ok(h) => h,
                Err(e) => return,
            };
            handle.store(true, Ordering::SeqCst);
        });
        is_first_handle.store(false, Ordering::SeqCst);

        let handle = IS_MOUSE_PASS
            .get_or_init(|| Mutex::new(AtomicBool::new(true)))
            .lock();
        let handle = match handle {
            Ok(h) => h,
            Err(e) => return,
        };
        handle.store(false, Ordering::SeqCst);
    }
}

pub fn key_listener() {
    // SHORTCUT_RX.set(rx).unwrap();
    let res = listen(move |event| {
        // let name = event.name;
        // match name {                                                                                                                                                  32mevent\x1b[0m = {}", name),
        //     None => println!("🪵 [index.rs:25]~ token ~ \x1b[0;32mevent\x1b[0m = None")
        // }
        // println!("🪵 [index.rs:31]~ token ~ \x1b[0;32mevent.event_type\x1b[0m = {}", event.event_type);

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
    });
    match res {
        Ok(_) => {}
        Err(e) => println!("Key Listen Error"),
    }
    // tokio::spawn(async {

    // });
}

pub fn listen_mouse_pass_event(ctx: &egui::Context, app: &mut crate::ui::index::MyApp) {
    let handle = IS_MOUSE_PASS
        .get_or_init(|| Mutex::new(AtomicBool::new(true)))
        .lock();
    let handle = match handle {
        Ok(h) => h,
        Err(e) => return,
    };
    let handle_last = LAST_IS_MOUSE_PASS
        .get_or_init(|| Mutex::new(AtomicBool::new(true)))
        .lock();
    let handle_last = match handle_last {
        Ok(h) => h,
        Err(e) => return,
    };
    let is_mouse_pass = handle.load(Ordering::SeqCst);
    let last_is_mouse_pass = handle_last.load(Ordering::SeqCst);

    if is_mouse_pass && last_is_mouse_pass != is_mouse_pass {
        // println!("🪵 [index.rs:312]~ token ~ \x1b[0;32mlast_is_mouse_pass\x1b[0m = {}", last_is_mouse_pass);
        println!(
            "🪵 [index.rs:312]~ token ~ \x1b[0;32mis_mouse_pass\x1b[0m = {}",
            is_mouse_pass
        );
        app.show_ui = true;
        ctx.send_viewport_cmd(egui::ViewportCommand::MousePassthrough(true));
        // ctx.send_viewport_cmd(egui::ViewportCommand::WindowLevel(egui::WindowLevel::AlwaysOnTop));
        handle_last.store(is_mouse_pass, Ordering::SeqCst);
        // control_dec(app.show_ui);
    }
    if !is_mouse_pass && last_is_mouse_pass != is_mouse_pass {
        // println!("🪵 [index.rs:312]~ token ~ \x1b[0;32mlast_is_mouse_pass\x1b[0m = {}", last_is_mouse_pass);
        println!(
            "🪵 [index.rs:312]~ token ~ \x1b[0;32mis_mouse_pass\x1b[0m = {}",
            is_mouse_pass
        );
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

pub fn rev_gloabl_hk() {
    println!(
        "🪵 [other_logic.rs:207]~ token ~ \x1b[0;32mrev_gloabl_hk\x1b[0m = {}",
        "rev_gloabl_hk"
    );
    let res = GlobalHotKeyEvent::receiver().try_recv();
    let event = match res {
        Ok(e) => e,
        Err(e) => {
            println!("glbal hotkey rev error : {}", e);
            return;
        }
    };
    println!("eveve{:?}", event);

    // if let Ok(event) =  {
    //     println!("{:?}", event);
    // }
}

pub fn check_is_focus(ctx: &egui::Context,app: &mut crate::ui::index::MyApp){
  // ctx.input(|i| i.is_focus_changed());
  let is_focused = ctx.input(|i| i.raw.focused);
  if is_focused {
    // println!("is focused");
  }else{
    // println!("is not focused");
    ctx.request_repaint_after_secs(0.015);
  }
}