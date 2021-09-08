

#[tauri::command]
pub fn init_demo(name: String) -> String {
    let msg = format!("Capture: {}", &name);
    println!("{}", &msg);
    return msg;
}

#[tauri::command]
pub fn init_capture(name: String) -> String {
    let msg = format!("Capture: {}", &name);
    println!("{}", &msg);
    return msg;
}

// #[tauri::command]
// pub fn add_click(demo: String, url: String) {
//     let msg = format!("Capture: {}", &name);
//     println!("{}", &msg);
//     return msg;
// }