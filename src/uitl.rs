// use std::str::FromStr;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use winapi::um::winuser::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};
use std::io::{Read, Seek, SeekFrom};
use std::os::raw::c_ulong;
use std::path::Path;
use winapi::um::winbase::GetUserNameA;
use screenshots::Screen;
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use std::process::{Command, Output};
use std::error::Error;
use std::{
    io::{ BufWriter, Write},
    net::TcpListener,
};


pub fn format_duration_extended(milliseconds: u64) -> String {
    let total_seconds = milliseconds / 1000;
    // let days = total_seconds / 86400;
    let hours = (total_seconds % 86400) / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    // let remaining_ms = milliseconds % 1000;

    // format!(
    //     "{}天 {:02}:{:02}:{:02}.{:03}",
    //     days, hours, minutes, seconds, remaining_ms
    // )
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

pub fn transform_wuyang_time_ts(arr: &[&str]) -> i64 {
    // 解析日期和时间字符串
    let current_year = "2025"; //统一假设为2025, 跨年判断在服务器做
    let str = format!("{} {}", current_year, arr[0]);
    let time_str = arr[1].trim_end();
    let date = NaiveDate::parse_from_str(&str, "%Y %m-%d").expect("日期解析失败");
    let time = NaiveTime::parse_from_str(time_str, "%H_%M_%S").expect("时间解析失败");
    // 合并日期和时间
    let datetime = NaiveDateTime::new(date, time);
    // 转换为时间戳
    datetime.and_utc().timestamp()
}

pub fn read_last_lines<P: AsRef<Path>>(path: P, num: usize) -> std::io::Result<Vec<String>> {
    let mut file = File::open(path)?;
    let file_size = file.metadata()?.len();
    println!(
        "🪵 [uitl.rs:38]~ token ~ \x1b[0;32mfile_size\x1b[0m = {}",
        file_size
    );

    let mut buffer = Vec::new();
    let mut pos = file_size; // 从末尾开始
    let mut line_count = 0;

    // 反向逐字节读取
    while pos > 0 && line_count < num {
        pos -= 1;
        file.seek(SeekFrom::Start(pos))?;
        let mut byte = [0u8; 1];
        file.read_exact(&mut byte)?;

        if byte[0] == b'\n' {
            line_count += 1;
            buffer.push(b'@');
        }

        buffer.push(byte[0]);
    }

    // 处理最后一行无换行符的情况
    if line_count < num && pos == 0 {
        // line_count += 1;
    }

    buffer.reverse(); // 恢复正向顺序
    let content = String::from_utf8_lossy(&buffer).into_owned();
    let lines: Vec<&str> = content.lines().collect();

    // 提取最后两行
    let start = if lines.len() >= num {
        lines.len() - num
    } else {
        0
    };
    Ok(lines[start..].iter().map(|s| s.to_string()).collect())
}

pub fn read_first_lines<P: AsRef<Path>>(path: P, num: usize) -> std::io::Result<Vec<String>> {
    let mut file = File::open(path)?;
    let file_size = file.metadata()?.len();
    // println!("🪵 [uitl.rs:38]~ token ~ \x1b[0;32mfile_size\x1b[0m = {}", file_size);

    let mut buffer = Vec::new();
    let mut pos = 0;
    let mut line_count = 0;

    // 正向逐字节读取
    while pos < file_size-1 && line_count < num {
        pos += 1;
        file.seek(SeekFrom::Start(pos))?;
        let mut byte = [0u8; 1];
        file.read_exact(&mut byte)?;

        if byte[0] == b'\n' {
            line_count += 1;
            buffer.push(b'@');
        }

        buffer.push(byte[0]);
    }

    // 处理最后一行无换行符的情况
    if line_count < num && pos == file_size {
        // line_count += 1;
    }

    // buffer.reverse(); // 恢复正向顺序
    let content = String::from_utf8_lossy(&buffer).into_owned();
    let lines: Vec<&str> = content.lines().collect();
    // println!("🪵 [uitl.rs:108]~ token ~ \x1b[0;32mlines.len()\x1b[0m = {} {}", lines.len(),content);

    // 提取最后n行
    let start = if lines.len() >= num {
        lines.len() - num
    } else {
        0
    };
    Ok(lines[start..].iter().map(|s| s.to_string()).collect())
}

pub fn read_lines<P: AsRef<Path>>(
    path: P,
    start_num: usize,
    num: usize,
) -> std::io::Result<String> {
    let mut file = File::open(path)?;
    let file_size = file.metadata()?.len();
    let end_num = start_num + num;
    // println!("🪵 [uitl.rs:38]~ token ~ \x1b[0;32mfile_size\x1b[0m = {}", file_size);

    let mut buffer = Vec::new();
    let mut pos = 0;
    let mut line_count = 0;

    // 正向逐字节读取
    while pos < file_size-1 && line_count < end_num {
        pos += 1;
        file.seek(SeekFrom::Start(pos))?;
        let mut byte = [0u8; 1];
        file.read_exact(&mut byte)?;

        if byte[0] == b'\n' {
            line_count += 1;
            if line_count >= start_num {
                buffer.push(b'@');
            }
        }
        if line_count >= start_num {
            buffer.push(byte[0]);
        }
    }

    // 处理最后一行无换行符的情况
    if line_count < num && pos == file_size {
        // line_count += 1;
    }

    // buffer.reverse(); // 恢复正向顺序
    let content = String::from_utf8_lossy(&buffer).into_owned();
    let lines: Vec<&str> = content.lines().collect();
  

    // 提取最后n行
    // let start = if lines.len() >= num {
    //     lines.len() - num
    // } else {
    //     0
    // };
    let start = 0;
    // let res:Vec<String> = lines.iter().map(|s| s.to_string()).collect();
    // println!("🪵 [uitl.rs:174]~ token ~ \x1b[0;32mres\x1b[0m = {}",&res.join("!!"));
    Ok(content)
}

pub fn get_sys_username() -> String {
    let mut username = [0u8; 256];
    let mut size: c_ulong = username.len() as c_ulong;
    unsafe {
        GetUserNameA(username.as_mut_ptr() as *mut i8, &mut size);
    }
    String::from_utf8_lossy(&username).to_string()
}

/// 查找字符串坐标（行号从1开始，列号从0开始）
pub fn find_string_coordinates<P: AsRef<Path>>(
    file_path: P,
    target: &str,
) -> Result<Vec<(usize, usize)>, std::io::Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut positions = Vec::new();
    let target_bytes = target.as_bytes();

    for (line_num, line_result) in reader.lines().enumerate() {
        let line = line_result?;
        // line.find(target);
        let line_bytes = line.as_bytes();

        // 遍历每一字节查找匹配
        let mut pos = 0;
        while pos <= line_bytes.len().saturating_sub(target_bytes.len()) {
            if line_bytes[pos..].starts_with(target_bytes) {
                positions.push((line_num + 1, pos + 1)); // 列号从1开始
                pos += target_bytes.len();
            } else {
                pos += 1;
            }
        }
    }

    Ok(positions)
}

pub fn screen_shot() -> () {
    let start = Instant::now();
    let screens = Screen::all().unwrap();
    let ts= SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    for screen in screens {
        println!("Capturer {:?}", screen);
        let image = screen.capture().unwrap();
        image
            .save(format!("D:\\MCode\\Rust/{}-{}.png", screen.display_info.id,ts))
            .unwrap();

        // image = screen.capture_area(300, 300, 300, 300).unwrap();
        // image
        //     .save(format!("target/{}-2.png", screen.display_info.id))
        //     .unwrap();
    }


    println!("运行耗时: {:?}", start.elapsed());
    // Ok("截图成功".to_string())
}


pub fn ping(target: &str) -> Result<Output, Box<dyn Error>> {
    let output = if cfg!(target_os = "windows") {
        Command::new("ping")
            .arg(target)
            .output()?
    } else {
        Command::new("ping")
            .arg("-c") // 指定发送的包数量 (Linux/macOS)
            .arg("4")
            .arg(target)
            .output()?
    };

    Ok(output)
}



use crate::config::TelemetryDataField;

pub fn is_port_available(ip: &str, port: u16) -> bool {
    match TcpListener::bind((ip, port)) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn save_raw_bytes_to_file(data: &Vec<Vec<u8>>, filename: &str) -> std::io::Result<()> {
    let cfg = bincode::config::standard();
    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file);

    //use bincode encode data then write data to file
    let encoded: Vec<u8> = bincode::encode_to_vec(&data, cfg).unwrap();
    
    writer.write_all(&encoded)?;
   

    writer.flush()?;
    Ok(())
}

pub fn load_raw_bytes_from_file(filename: &str) -> std::io::Result<Vec<Vec<u8>>> {
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);
    // let mut result_vec: Vec<Vec<u8>> = Vec::new();

    //use bincode decode data from file
    let decoded: Vec<Vec<u8>> = bincode::decode_from_reader(&mut reader, bincode::config::standard()).unwrap();
    // let result_vec = decoded;

    // // 首先读取 Vec 的长度
    // let mut num_arrays_bytes = [0u8; 8]; // u64 是 8 字节
    // reader.read_exact(&mut num_arrays_bytes)?;
    // let num_arrays = u64::from_le_bytes(num_arrays_bytes) as usize;

    // // 然后根据长度读取每个数组
    // for _ in 0..num_arrays {
    //     let mut buffer = Vec::new();
    //     reader.read_exact(&mut buffer)?;
    //     result_vec.push(buffer);
    // }

    // （可选）检查是否已读取到文件末尾，确保没有额外数据
    // let mut one_byte = [0u8;1];
    // match reader.read(&mut one_byte) {
    //     Ok(0) => {}, // EOF, good
    //     Ok(_) => return Err(std::io::Error::new(ErrorKind::InvalidData, "Extra data found at the end of the file")),
    //     Err(ref e) if e.kind() == ErrorKind::UnexpectedEof => {}, // Also fine if read_exact consumed everything
    //     Err(e) => return Err(e),
    // }

    Ok(decoded)
}


pub fn read_fn_map(item: TelemetryDataField, buf: Vec<u8>) -> f32 {
  if item.type_name == "F32" {
      return f32::from_le_bytes(buf.try_into().unwrap()) as f32;
  } else if item.type_name == "S32" {
      return  i32::from_le_bytes(buf.try_into().unwrap()) as f32;
  } else if item.type_name == "U32" {
      return u32::from_le_bytes(buf.try_into().unwrap()) as f32;
  } else if item.type_name == "U16" {
      return u16::from_le_bytes(buf.try_into().unwrap())  as f32;
  } else if item.type_name == "U8" {
      return u8::from_le_bytes(buf.try_into().unwrap())   as f32;
  } else if item.type_name == "S8" {
      return i8::from_le_bytes(buf.try_into().unwrap())   as f32;
  } else {
      return 0 as f32;
  }
}


pub fn get_local_data_list() -> Result<Vec<String>, io::Error> {
    let mut name_list: Vec<String> = Vec::new();
    for entry in fs::read_dir(".")? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(file_name) = path.file_name() {
                if let Some(name_str) = file_name.to_str() {
                    if name_str.contains("fm") && name_str.contains(".data") {
                        name_list.push(name_str.to_string());
                    }
                }
            }
        }
    }
    Ok(name_list)
}

pub fn format_milliseconds_to_mmssms(total_ms: u32) -> String {
    let minutes = total_ms / (1000 * 60); // 1分钟 = 60秒 = 60000毫秒
    let remaining_ms_after_minutes = total_ms % (1000 * 60);

    let seconds = remaining_ms_after_minutes / 1000; // 1秒 = 1000毫秒
    let milliseconds = remaining_ms_after_minutes % 1000;

    // 使用格式化宏来确保两位数的分钟和秒，三位数的毫秒（如果需要前导零）
    format!("{:02}:{:02}:{:03}", minutes, seconds, milliseconds)
}

pub fn get_sreen_info()->(f32,f32){
    let screen_width = unsafe { GetSystemMetrics(SM_CXSCREEN) } as f32;
    let screen_height = unsafe { GetSystemMetrics(SM_CYSCREEN) } as f32;
    println!("主屏幕分辨率 (物理像素): {}x{}", screen_width, screen_height);
    (screen_width,screen_height)
}

pub fn get_now_ts() -> f64 {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    since_the_epoch.as_secs_f64()
}