use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::iter::Map;
use std::net::{TcpListener, TcpStream};

fn main() {
    println!("Server ip is: http://127.0.0.1:9090");
    let listener = TcpListener::bind("127.0.0.1:9090").unwrap();
    for stream in listener.incoming() {
        let stream: TcpStream = stream.unwrap();

        // handle connection
        // println!("New connection: {}", stream.peer_addr().unwrap());
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    // println!("Request: {:#?}", http_request);

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("index.html").unwrap();
    let length = contents.len();

    let page = ret_page(http_request);
    let length = page.len();
    // println!("platform is {}", );

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{page}");
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    println!("Response sent!");
}

fn ret_page(request: Vec<String>) -> String {
    let map_data = vec2map(request);

    let mut platform_real = String::new();
    // 查找键为 "sec-ch-ua-platform" 的值
    if let Some(map) = map_data.iter().find(|m| m.contains_key("sec-ch-ua-platform")) {
        if let Some(platform) = map.get("sec-ch-ua-platform") {
            platform_real = platform.clone(); // 赋值找到的值
        }
    } else {
        platform_real = "not found".to_string(); // 如果没找到，赋值为错误消息
    }

    // get brower
    let browser = String::new();

    let index_html = format!(
        r#"<!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta http-equiv="X-UA-Compatible" content="IE=edge">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Document</title>
        </head>

        <body>
            <h1>Hello, World!</h1>
            <p>Platform is: {}</p>
        </body>
        </html>"#,
        platform_real // 插入 platform 的值
    );

    index_html
}

fn vec2map(request: Vec<String>) -> Vec<HashMap<String, String>> {
    // 将请求连接成一个字符串，然后以 "\", " 分割
    let data: Vec<String> = request.join(",").split(",").map(|s| s.to_string()).collect();

    let mut map_data = Vec::new(); // 用于存储 HashMap

    // 遍历分割后的数据
    for item in data {
        let parts: Vec<String> = item.split(':').map(|s| s.trim().to_string()).collect();

        // 检查分割后的结果是否有 key 和 value
        if parts.len() == 2 {
            let mut map = HashMap::new();
            let key = parts[0].clone();
            let value = parts[1].clone();
            map.insert(key, value);
            map_data.push(map);
        }
    }

    // 打印 map_data
    println!("map_data is {:#?}", map_data);

    map_data
}
