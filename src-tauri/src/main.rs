#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]


#[tauri::command]
fn update_repo(path: &str) {
    use git2::Repository;
    print!("Test");
    let url = "https://github.com/Mineorbit/DungeonsAndDungeons";
    let repo = match Repository::clone(url, path) {
    Ok(repo) => repo,
    Err(e) => panic!("failed to clone: {}", e),
    };
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![update_repo])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
