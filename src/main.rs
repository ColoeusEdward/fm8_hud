#![allow(unused_variables, dead_code)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use enums::MyApp;
// use std::io;
// use utf8_slice::slice;
use tokio::time::{sleep, Duration};
use tokio::{self};


// use std::thread::sleep;
// use std::time::Duration;

mod controllers;
mod enums;
mod uitl;
mod ui;
use eframe::{egui, NativeOptions};
use egui::{Color32, Rgba};

#[tokio::main]
async fn main() {
    
    let _ =ui::index::main();
    // let _ =ui::index::test();
    
    
}

// type File = String;

// fn open(f: &mut File) -> bool {
//     true
// }
// fn close(f: &mut File) -> bool {
//     true
// }

// #[allow(dead_code)]
// fn read(f: &mut File, save_to: &mut Vec<u8>) -> ! {
//     unimplemented!()
// }

// fn main() {
//     let mut f1 = File::from("f1.txt");
//     open(&mut f1);
//     read(&mut f1, &mut vec![]);
//     close(&mut f1);
// }


