use std::{net::UdpSocket, thread, time::Duration};

use tokio::time::sleep;

use crate::uitl::{get_local_data_list, load_raw_bytes_from_file};

pub fn init_udp_server() {
    let name_list = get_local_data_list().unwrap();
    let name = match name_list.get(0) {
        Some(first_element) => first_element.to_string(),
        None => "fm.data".to_string(),
    };
    println!("🪵 [test_sender.rs:9]~ token ~ \x1b[0;32mname\x1b[0m = {}", name);
    let data = load_raw_bytes_from_file(&name);
    let data = match data {
        Ok(data) => data,
        Err(e) => {
            println!("Error loading data: {}", e);
            return;
        }
    };
    tokio::spawn(async move {
        // 1. 绑定到本地地址和端口
        // "0.0.0.0:8080" 表示监听所有可用的网络接口的 8080 端口。
        // 如果你只想监听特定接口，例如本地回环地址，可以使用 "127.0.0.1:8080"。
        let socket = UdpSocket::bind("0.0.0.0:18080").expect("Failed to bind to 0.0.0.0:18080");
        println!("UDP Server listening on 0.0.0.0:18080");

        // 设置一个读取超时，这样在没有数据时，recv_from 不会永远阻塞
        socket
            .set_read_timeout(Some(Duration::from_secs(1)))
            .expect("Failed to set read timeout");

        // 克隆 socket，以便在另一个线程中发送数据。
        // UdpSocket 是 Send 和 Sync 的，所以可以安全地在线程间共享或克隆。
        let send_socket = socket.try_clone().expect("Failed to clone the socket");

        // 预设一个外部目标地址和端口，用于主动发送数据
        let external_target_addr = "127.0.0.1:8000"; // 假设有一个客户端在 8081 端口监听

        // 启动一个独立的线程来定期向外部发送数据
        thread::spawn(move || {
            for buffer in data {
                let byte_slice: &[u8] = buffer.as_slice();
                match send_socket.send_to(byte_slice, external_target_addr) {
                    Ok(bytes_sent) => {
                        // println!("sending");
                        // println!("Sent {} bytes proactively to {}: {}", bytes_sent, external_target_addr, message);
                    }
                    Err(e) => {
                        eprintln!(
                            "Failed to send data proactively to {}: {}",
                            external_target_addr, e
                        );
                    }
                }
                thread::sleep(Duration::from_millis(8)); // 每隔3秒发送一次
            }
            println!("🪵 [test_sender.rs:57]~ token ~ \x1b[0;32m8)\x1b[0m = {}", "send over");
            
        });
    });
}
