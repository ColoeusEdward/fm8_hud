use chrono::Local;
use std::{
    collections::BTreeMap,
    fs, io,
    net::UdpSocket,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc, Mutex, OnceLock,
    },
};
use tokio::time::{sleep, Duration};

use crate::{
    config::TELEMETRY_FIELDS,
    enums::ErrorData,
    ui::index::{MyApp2, ERROR_TX, TELE_DATA_TX},
    uitl::{get_local_data_list, is_port_available, load_raw_bytes_from_file, read_fn_map, save_raw_bytes_to_file},
};

pub static ISSTART: OnceLock<AtomicUsize> = OnceLock::new();
pub static THREAD_RUNINNG_FLAG: OnceLock<AtomicBool> = OnceLock::new();
pub static SAING_DATA: OnceLock<AtomicBool> = OnceLock::new();
pub static TEMP_SAVING_BUFFER: OnceLock<Mutex<Vec<Vec<u8>>>> = OnceLock::new();

// pub static POWER_CHART_DATA2: OnceLock<Arc<Mutex<Vec<Vec<i32>>>>> = OnceLock::new();
// pub static TORQUE_CHART_DATA2: OnceLock<Arc<Mutex<Vec<Vec<i32>>>>> = OnceLock::new();

pub static POWER_CHART_DATA: OnceLock<Arc<Mutex<BTreeMap<i32, Vec<f32>>>>> = OnceLock::new();
pub static TORQUE_CHART_DATA: OnceLock<Arc<Mutex<BTreeMap<i32, Vec<f32>>>>> = OnceLock::new();

// pub static READ: OnceLock<tauri::ipc::Channel<UdpDataEvent>> = OnceLock::new();
// pub static READ: OnceLock<Box<dyn tauri::ipc::Channel<UdpDataEvent>>> = OnceLock::new();

// pub static SOCKET_INSTANCE: OnceLock<UdpSocket> = OnceLock::new();

pub fn init_udp(app: &mut MyApp2) -> Result<(), String> {
    println!(
        "🪵 [udp.rs:7]~ token ~ \x1b[0;32mconfig\x1b[0m = {} {}",
        app.setting_data.ip, app.setting_data.port
    );
    get_local_data_list().unwrap();
    // READ.set(reader);
    let is_start = ISSTART.get_or_init(|| AtomicUsize::new(0));

    if is_start.load(std::sync::atomic::Ordering::SeqCst) > 0 {
        println!("UDP listener is already running");
        return Ok(());
    }
    is_start.store(1, std::sync::atomic::Ordering::SeqCst);
    let port = app.setting_data.port.clone();
    let ip = app.setting_data.ip.clone();

    // let running_flag = Arc::new(AtomicBool::new(true)); // Flag for the new thread
    // let thread_running_flag = Arc::clone(&running_flag); // Clone for the thread closure
    let thread_running_flag = THREAD_RUNINNG_FLAG.get_or_init(|| AtomicBool::new(true));
    thread_running_flag.store(true, Ordering::SeqCst);
    //  &state.thread_running_flag;
    //  = thread_running_flag;
    // state.

    let socket_addr = format!("{}:{}", ip, port); // Listen on all interfaces
    let socket = match UdpSocket::bind(&socket_addr) {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to bind UDP socket: {}", e);
            is_start.store(0, std::sync::atomic::Ordering::SeqCst);
            thread_running_flag.store(false, Ordering::SeqCst);
            let mut err = format!("Failed to bind UDP socket:  {}", e);
            if e.kind() == io::ErrorKind::AddrInUse {
                err = format!("Failed to bind UDP socket:  {}", "当前IP端口已被其他程序占用");
            } else if err.contains("invalid port") {
                err = format!("Failed to bind UDP socket:  {}", "端口格式错误");
            } else if err.contains("不知道这样的") {
                err = format!("Failed to bind UDP socket:  {}", "IP格式错误");
            }
            let _ = ERROR_TX.get().unwrap().lock().unwrap().send(ErrorData {
                message: err.clone(),
            });
            return Err(err);
        }
    };

    println!("Listening on UDP: {}", socket_addr);

    let _ = tokio::spawn(async move {
        // let mut buffer = [0u8; 1024]; // Adjust buffer size as needed
        let mut buffer = [0u8; 1500]; // Typical MTU for Ethernet
                                      // let (tx, _) = mpsc::channel::<UdpDataPayload>(1024); // Buffer size 1024
        let name_list = ["IsRaceOn","CurrentEngineRpm","CarOrdinal","CarClass", "CarPerformanceIndex","Speed","Power", "Torque","TireTempFrontLeft","TireTempFrontRight","TireTempRearLeft","TireTempRearRight","Boost","Fuel","DistanceTraveled","BestLap","LastLap","CurrentLap","CurrentRaceTime","LapNumber","RacePosition","Accel","Brake","Gear","Steer","TireWearFrontLeft","TireWearFrontRight","TireWearRearLeft","TireWearRearRight","TrackOrdinal"];
        // let field_map:BTreeMap<String, > = BTreeMap::new();
        let field_vec = TELEMETRY_FIELDS
            .iter()
            .filter(|item| name_list.contains(&item.name))
            .collect::<Vec<_>>();

        let mut data_map:BTreeMap<String, f32> = BTreeMap::new();
        name_list.iter().for_each(|s| {data_map.insert(s.to_string(), 0.0);});

        let pcdata = POWER_CHART_DATA.get_or_init(|| Arc::new(Mutex::new(BTreeMap::new())));
        let todata = TORQUE_CHART_DATA.get_or_init(|| Arc::new(Mutex::new(BTreeMap::new())));
        reset_data();
        let tele_tx = TELE_DATA_TX.get().unwrap();
        // let pcdata =  POWER_CHART_DATA2.get_or_init(|| Arc::new(Mutex::new(Vec::new())));
        // let todata = TORQUE_CHART_DATA2.get_or_init(|| Arc::new(Mutex::new(Vec::new())));
        while thread_running_flag.load(Ordering::SeqCst) {
            // Use a non-blocking or timed receive in a real app to allow checking the flag
            // For simplicity here, we'll use blocking, which makes stopping immediate less trivial
            // A better approach would be using a channel or tokio with select!

            // Example with a small timeout to check the flag (less efficient but works with std::net)

            match socket.set_read_timeout(Some(std::time::Duration::from_millis(3000))) {
                Ok(_) => {}
                Err(_) => {
                    println!("Failed to set read timeout");

                    break;
                } // Error setting timeout, exit thread
            }

            match socket.recv_from(&mut buffer) {
                Ok((bytes_received, _)) => {
                    save_temp_data(buffer).unwrap();

                    // // Emit event to the frontend
                    // let payload = serde_json::json!({
                    //     "sender": src.to_string(),
                    //     "data": received_text, // Send as string
                    //     "rawData": data // Or send raw bytes if needed
                    // });
                    // win.emit("udp_data",  payload).unwrap();

                    // let mut data_vec: Vec<UdpDataItem> = Vec::new();
                                                       // let mut vv: Vec<i32> = Vec::new();//rpm,power,,torque

                    for item in field_vec.iter() {
                        let mut buf: Vec<u8> = Vec::new();
                        for i in 0..item.bytes {
                            buf.push(buffer[item.offset + i]);
                        }
                        let val = read_fn_map(**item, buf);
                        // let res = match val.parse::<i32>() {
                        //     Ok(val) => val,
                        //     Err(e) => {
                        //         println!("Error parsing value: {} {}",val, e);
                        //         0
                        //     }
                        // };
                        // vv.push(val);
                        data_map.insert(item.name.to_string(), val);
                    }
                    tele_tx.lock().unwrap().send(data_map.clone()).unwrap();
                    

                    // todata.lock().unwrap().insert(vv[0], [vv[0], vv[2]].to_vec());
                    // println!(
                    //     "🪵 [udp.rs:118]~ token ~ \x1b[0;32mdata_vec\x1b[0m = {}",
                    //     vv[0]
                    // );
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    // Timeout occurred, check the running flag and continue or exit
                    if !thread_running_flag.load(Ordering::SeqCst) {
                        break; // Exit loop if flag is false
                    }
                }
                Err(e) => {
                    eprintln!("UDP receive error: {}", e);
                    let mut err = e.to_string();
                    let por = &port.parse::<u16>();
                    match por {
                        Ok(por) => {
                            if !is_port_available(&ip, *por) {
                                err = format!("端口 {} 已被程序占用", por);
                            }
                        }
                        Err(e) => {
                            err = format!("端口 {} 格式错误", &port);
                        }
                    };
                    if !err.contains("没有反应") {
                        let error_tx = ERROR_TX.get().unwrap();
                        let errdata = ErrorData { message: err };
                        error_tx.lock().unwrap().send(errdata).unwrap();
                    }
                    // Handle other errors, maybe emit an error event
                    break; // Exit loop on other errors
                }
            }
        }
        println!("UDP listener thread stopped.");
        is_start.store(0, std::sync::atomic::Ordering::SeqCst);
        thread_running_flag.store(false, Ordering::SeqCst);
        save_data_to_file();
        // pcdata.lock().unwrap().clear();
        // todata.lock().unwrap().clear();
    });
    Ok(())
}

pub fn stop_udp() {
    let thread_running_flag = THREAD_RUNINNG_FLAG.get_or_init(|| AtomicBool::new(false));
    thread_running_flag.store(false, Ordering::SeqCst);
}

pub fn reset_data() {
    let pcdata = POWER_CHART_DATA.get_or_init(|| Arc::new(Mutex::new(BTreeMap::new())));
    let todata = TORQUE_CHART_DATA.get_or_init(|| Arc::new(Mutex::new(BTreeMap::new())));
    pcdata.lock().unwrap().clear();
    todata.lock().unwrap().clear();
}

pub fn set_saving_data_flag(is_open: bool) {
    let start_flag = SAING_DATA.get_or_init(|| AtomicBool::new(false));
    start_flag.store(is_open, Ordering::SeqCst);
}

fn save_data_to_file() {
    let start_flag = SAING_DATA.get_or_init(|| AtomicBool::new(false));
    if !start_flag.load(Ordering::SeqCst) {
        return;
    }
    let _ = tokio::spawn(async move {
        // start_flag.store(false, Ordering::SeqCst);
        sleep(Duration::from_millis(10)).await;
        let temp_buf_list = TEMP_SAVING_BUFFER
            .get_or_init(|| Mutex::new(Vec::new()))
            .lock();
        let mut temp_buf_list = match temp_buf_list {
            Ok(temp_buf_list) => temp_buf_list,
            Err(e) => {
                eprintln!("Failed to acquire lock on TEMP_SAVING_BUFFER: {}", e);
                return;
            }
        };
        //date time string yyyyMMdd_hhmmss
        let now = Local::now();
        let date_time_string = now.format("%Y%m%d_%H%M%S").to_string();
        let name = format!("fm_{}.data", date_time_string);
        let save_res = save_raw_bytes_to_file(&*temp_buf_list, &name);
        match save_res {
            Ok(_) => {
                println!("🪵 [udp.rs:222]~ token ~ \x1b[0;32mok\x1b[0m = {}", "ok");
                temp_buf_list.clear();
            }
            Err(e) => {
                println!("🪵 [udp.rs:222]~ token ~ \x1b[0;32merror\x1b[0m = {}", e);
            }
        }
    });
}

pub fn local_data_test_mode() -> () {
    let name_list = get_local_data_list().unwrap();
    let name = match name_list.get(0) {
        Some(first_element) => first_element.to_string(),
        None => "fm.data".to_string(),
    };
    let data = load_raw_bytes_from_file(&name);
    let data = match data {
        Ok(data) => data,
        Err(e) => {
            println!("Error loading data: {}", e);
            return;
        }
    };
    let thread_running_flag = THREAD_RUNINNG_FLAG.get_or_init(|| AtomicBool::new(true));
    thread_running_flag.store(true, Ordering::SeqCst);
    // println!("🪵 [udp.rs:264]~ token ~ \x1b[0;32mdata\x1b[0m = {} {} {} {} {}", data.len(),data[0].len(),data[1].len(),data[2].len(),data[20].len());

    let _ = tokio::spawn(async move {
        let name_list = ["Power", "CurrentEngineRpm", "Torque"];
        let field_vec = TELEMETRY_FIELDS
            .iter()
            .filter(|item| name_list.contains(&item.name))
            .collect::<Vec<_>>();

        let pcdata = POWER_CHART_DATA.get_or_init(|| Arc::new(Mutex::new(BTreeMap::new())));
        let todata = TORQUE_CHART_DATA.get_or_init(|| Arc::new(Mutex::new(BTreeMap::new())));
        reset_data();

        // let pcdata = POWER_CHART_DATA2.get_or_init(|| Arc::new(Mutex::new(Vec::new())));
        // let todata = TORQUE_CHART_DATA2.get_or_init(|| Arc::new(Mutex::new(Vec::new())));

        for buffer in data {
            // println!("🪵 [udp.rs:280]~ token ~ \x1b[0;32mbuffer\x1b[0m = {}", buffer.len());
            let mut vv: Vec<f32> = Vec::new(); //rpm_val,power_val,,torque_val

            // let mut vv: Vec<i32> = Vec::new();//rpm,power,,torque

            for item in field_vec.iter() {
                let mut buf: Vec<u8> = Vec::new();
                for i in 0..item.bytes {
                    buf.push(buffer[item.offset + i]);
                }
                let val = read_fn_map(**item, buf);
                // let res = match val.parse::<i32>() {
                //     Ok(val) => val,
                //     Err(e) => {
                //         println!("Error parsing value: {} {}",val, e);
                //         0
                //     }
                // };
                vv.push(val);
            }
            if vv[1] > 0.0 {
                build_chart_data(&pcdata, &todata, &vv);

                // pcdata.lock().unwrap().insert(vv[0], [vv[0], vv[1]/ 1000 * 1.34102209 as i32].to_vec());
                // todata.lock().unwrap().insert(vv[0], [vv[0], vv[2] * 0.73756215  as i32].to_vec());
            }

            // println!(
            //     "🪵 [udp.rs:118]~ token ~ \x1b[0;32mdata_vec\x1b[0m = {}",
            //     vv[2]
            // );

            sleep(Duration::from_millis(7)).await;
        }
        // pcdata.lock().unwrap().clear();
        // todata.lock().unwrap().clear();
    });
}

pub fn loop_send_data() {
    let thread_running_flag = THREAD_RUNINNG_FLAG.get_or_init(|| AtomicBool::new(true));
    let _ = tokio::spawn(async move {
        let pcdata = POWER_CHART_DATA.get_or_init(|| Arc::new(Mutex::new(BTreeMap::new())));
        let todata = TORQUE_CHART_DATA.get_or_init(|| Arc::new(Mutex::new(BTreeMap::new())));

        // let pcdata =  POWER_CHART_DATA2.get_or_init(|| Arc::new(Mutex::new(Vec::new())));
        // let todata = TORQUE_CHART_DATA2.get_or_init(|| Arc::new(Mutex::new(Vec::new())));
        let send_data = || -> Result<(), String> {
            // let test: Vec<Vec<_>> = pcdata.lock().unwrap().clone().into_values().collect();
            // let res = reader.send(
            //     UdpDataEvent2::DataIn {
            //         data: &UdpDataItem2 {
            //             power: pcdata.lock().unwrap().clone().into_values().collect(),
            //             torque: todata.lock().unwrap().clone().into_values().collect(),
            //         },
            //     }, // power: pcdata.lock().unwrap().clone()
            //        // , torque: todata.lock().unwrap().clone()} }
            // );
            // match res {
            //     Ok(_) => {
            //         // println!("🪵 [udp.rs:222]~ token ~ \x1b[0;32mok\x1b[0m = {}", "UdpDataEvent2 ok");
            //     }
            //     Err(e) => {
            //         println!("🪵 [udp.rs:222]~ token ~ \x1b[0;32merror\x1b[0m = {}", e);
            //     }
            // }
            Ok(())
        };
        // while thread_running_flag.load(Ordering::SeqCst) {
        //     send_data().unwrap();

        //     sleep(Duration::from_millis(500)).await;
        // }
    });
}

fn save_temp_data(buf: [u8; 1500]) -> Result<(), String> {
    let start_flag = SAING_DATA.get_or_init(|| AtomicBool::new(false));
    if start_flag.load(Ordering::SeqCst) {
        let temp_buf_list = TEMP_SAVING_BUFFER.get_or_init(|| Mutex::new(Vec::new()));
        if let Ok(mut buffer_guard) = temp_buf_list.lock() {
            buffer_guard.push(buf.to_vec());
            println!("Pushed data. Current length: {}", buffer_guard.len());
        } else {
            eprintln!("Failed to acquire lock on TEMP_SAVING_BUFFER");
        }
    }
    Ok(())
}

fn build_chart_data(
    pcdata: &Arc<Mutex<BTreeMap<i32, Vec<f32>>>>,
    todata: &Arc<Mutex<BTreeMap<i32, Vec<f32>>>>,
    vv: &Vec<f32>,
) {
    // 优化 pcdata 的操作
    {
        let mut pcdata_guard = pcdata.lock().unwrap(); // 一次性获取锁
        let entry = pcdata_guard.entry(vv[0] as i32); // 使用 entry API 更高效地处理插入或更新

        // 检查 `vv` 的长度以避免 panic
        if vv.len() < 2 {
            eprintln!("Error: vv must have at least 2 elements for pcdata operation.");
            return; // 或者采取其他错误处理
        }
        let power = vv[1] as f32 / 1000.0 * 1.34102209;
        match entry {
            std::collections::btree_map::Entry::Occupied(mut occupied_entry) => {
                if occupied_entry.get()[1] < power {
                    occupied_entry.insert([vv[0], power].to_vec()); // 注意浮点数乘法
                }
            }
            std::collections::btree_map::Entry::Vacant(vacant_entry) => {
                vacant_entry.insert([vv[0], power].to_vec()); // 注意浮点数乘法
            }
        }
    } // pcdata_guard 在这里离开作用域并释放锁

    // 优化 todata 的操作
    {
        let mut todata_guard = todata.lock().unwrap(); // 一次性获取锁
        let entry = todata_guard.entry(vv[0] as i32); // 使用 entry API

        // 检查 `vv` 的长度以避免 panic
        if vv.len() < 3 {
            eprintln!("Error: vv must have at least 3 elements for todata operation.");
            return; // 或者采取其他错误处理
        }
        let torque = vv[2] as f32 * 0.73756215;
        // println!("🪵 [udp.rs:467]~ token ~ \x1b[0;32mtorque\x1b[0m ={} {}", vv[2],torque);
        match entry {
            std::collections::btree_map::Entry::Occupied(mut occupied_entry) => {
                if occupied_entry.get()[1] < torque {
                    occupied_entry.insert([vv[0], torque].to_vec()); // 注意浮点数乘法
                }
            }
            std::collections::btree_map::Entry::Vacant(vacant_entry) => {
                vacant_entry.insert([vv[0], torque].to_vec()); // 注意浮点数乘法
            }
        }
    } // todata_guard 在这里离开作用域并释放锁
}



pub fn calc_max_area_rpm_zone(rpm_length: i32) {
    // println!("🪵 [udp.rs:517]~ token ~ \x1b[0;32mrpm_length\x1b[0m = {}", rpm_length);
    let pcdata = POWER_CHART_DATA.get_or_init(|| Arc::new(Mutex::new(BTreeMap::new())));

    let _ = tokio::spawn(async move {
        let plist: Vec<Vec<f32>> = pcdata.lock().unwrap().clone().into_values().collect();
        // let mut max_area:Vec<i32> = Vec::new(); //[start_rpm,area]
        // let mut area_list:Vec<Vec<i32>> = Vec::new();//[[start_rpm,area]] 每一小格的面积
        let mut cumulative_areas: Vec<f32> = [0.0].to_vec(); //每一小格累加面积
        let mut best_start = 0;
        let mut best_end = 0;
        let mut min_real_rpm_index: usize = 0;
        let mut max_area: f32 = -1.0;
        // let mut down_count = 0;
        // let max_rpm = plist[plist.len()-1][0];

        for i in 0..plist.len() - 1 {
            let item = &plist[i];
            let item_next = &plist[i + 1];
            if item[0] < 4000.0 {
                cumulative_areas.push(0.0);
                continue;
            }
            if min_real_rpm_index == 0 {
                min_real_rpm_index = i;
            }

            let area: f32 = (item_next[0] - item[0]) * (item_next[1] + item[1]) / 2.0;
            // println!("🪵 [udp.rs:543]~ token ~ \x1b[0;32marea\x1b[0m = {}", area);
            cumulative_areas.push(cumulative_areas[i] + area);
        }
        // println!("🪵 [udp.rs:547]~ token ~ \x1b[0;32mmin_real_rpm_index\x1b[0m = {}", min_real_rpm_index);

        for i in min_real_rpm_index..plist.len() - 1 {
            // 'j' 是窗口的结束点索引
            // 我们要找到第一个 data[j].x 使得 data[j].x - data[i].x >= windowLength
            let mut j = i;
            // println!("🪵 [udp.rs:553]~ token ~ \x1b[0;32mplist[j][0] \x1b[0m = {}  {}", plist[j][0],plist[i][0] );
            while j < plist.len() && (plist[j][0] - plist[i][0]) < rpm_length as f32 {
                // println!("🪵 [udp.rs:562]~ token ~ \x1b[0;32mcur_area\x1b[0m = {} {}", i,j);
                j = j + 1;
            }
            if j < plist.len() && (plist[j][0] - plist[i][0] - rpm_length as f32) > 5.0 {
                //防止超过区间大小太多, 被稀疏数据干扰
                j = j - 1;
            }

            // 确保找到了一个有效的结束点，并且窗口内至少有两个点才能计算面积
            // j 必须是有效的索引，并且 j 必须大于 i
            if j < plist.len() && j > i {
                // 使用预计算的累积面积来快速获取当前窗口的面积
                // cumulativeAreas[j] 包含了从 data[0] 到 data[j] 的面积
                // cumulativeAreas[i] 包含了从 data[0] 到 data[i] 的面积
                // 两者相减就是 data[i] 到 data[j] 之间的面积
                let cur_area = cumulative_areas[j] - cumulative_areas[i];
                // println!("🪵 [udp.rs:562]~ token ~ \x1b[0;32mcur_area\x1b[0m = {} {} {}", i,j,cur_area);

                if cur_area > max_area {
                    max_area = cur_area;
                    best_start = i;
                    best_end = j; // 注意：这里的 endIndex 是包含的，表示 data[j] 是窗口的最后一个点
                }
                // else if cur_area < max_area{//
                //     down_count = down_count + 1;
                //     if down_count > 2{
                //         break;
                //     }
                // }
            }
        }

        // println!("🪵 [udp.rs:545]~ token ~ \x1b[0;32mcumulative_areas\x1b[0m = {} {} {} {}", cumulative_areas[cumulative_areas.len()-1],max_area,best_start,best_end);
    });
}

// fn build_chart_data2(pcdata: &Arc<Mutex<Vec<Vec<i32>>>>, todata: &Arc<Mutex<Vec<Vec<i32>>>>,vv:&Vec<i32>){
//     let torque = (vv[2] as f32 * 0.73756215 ) as i32;
//     let power =  ( vv[1] as f32 / 1000.0 * 1.34102209) as i32;

//     pcdata.lock().unwrap().push([vv[0], power].to_vec());
//     todata.lock().unwrap().push([vv[0], torque].to_vec());
// }
