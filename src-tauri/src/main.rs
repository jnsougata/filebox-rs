#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::io::{Write, Read};


#[tauri::command]
fn file_exists(path: &str) -> bool {
    std::path::Path::new(path).exists()
}

#[tauri::command]
fn create_app_config(data: &str, path: &str) {
    let mut file = std::fs::File::create(path).unwrap();
    file.write_all(data.as_bytes()).unwrap();
}

#[tauri::command]
fn get_auth_cookies(username: &str, password: &str) -> String{
    let client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none()).build();
    let res = client.unwrap()
        .post("https://alpha.deta.space/api/v0/auth/login")
        .body(format!("{{\"username\": \"{}\",\"password\": \"{}\"}}", username, password))
        .header("Content-Type", "application/json")
        .send()
        .unwrap();
    return res.headers().get("Set-Cookie").unwrap().to_str().unwrap().to_string();
}

#[tauri::command]
fn read_app_config(path: &str) -> String {
    let mut file = std::fs::File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    return contents;
}

#[tauri::command]
fn fetch_instances(bearer: &str) -> String {
    let client = reqwest::blocking::Client::new();
    let res = client.get("https://alpha.deta.space/api/v0/instances")
        .header("Content-Type", "application/json")
        .header("Cookie", format!("_cv0_a={}", bearer))
        .header("User-Agent", "Deta/0.1.0")
        .send()
        .unwrap();
    return res.text().unwrap();
}

#[tauri::command]
fn fetch(url: &str, bearer: &str) -> String {
    let client = reqwest::blocking::Client::new();
    let res = client.get(url)
        .header("Content-Type", "application/json")
        .header("User-Agent", "Deta/0.1.0")
        .header("Cookie", format!("deta_auth_token={}", bearer))
        .send()
        .unwrap();
    return res.text().unwrap();
}

#[tauri::command]
fn post(url: &str, data: &str, bearer: &str) -> String {
    let mut body = String::new();
    body.push_str(data);
    let client = reqwest::blocking::Client::new();
    let res = client.post(url)
        .header("Content-Type", "application/json")
        .header("User-Agent", "Deta/0.1.0")
        .header("Cookie", format!("deta_auth_token={}", bearer))
        .body(body)
        .send()
        .unwrap();
    return res.text().unwrap();
}

#[tauri::command]
fn delete(url: &str, data: &str, bearer: &str) -> String {
    let mut body = String::new();
    body.push_str(data);
    let client = reqwest::blocking::Client::new();
    let res = client.delete(url)
        .header("Content-Type", "application/json")
        .header("User-Agent", "Deta/0.1.0")
        .header("Cookie", format!("deta_auth_token={}", bearer))
        .body(body)
        .send()
        .unwrap();
    return res.text().unwrap();
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            file_exists, 
            create_app_config, 
            get_auth_cookies, 
            read_app_config, 
            fetch_instances, 
            fetch, 
            post,
            delete,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
