

#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]


/*
#[tauri::command]
fn execCommand(invoke_message: String, arg: String)
{
  let output = Command::new(invoke_message)
  .arg(arg)
  .output().unwrap_or_else(|e| {
      panic!("failed to execute process: {}", e)
});

if output.status.success() {
  let s = String::from_utf8_lossy(&output.stdout);

  print!("rustc succeeded and stdout was:\n{}", s);
} else {
  let s = String::from_utf8_lossy(&output.stderr);

  print!("rustc failed and stderr was:\n{}", s);
}
}

*/

#[tauri::command]
fn execCommand(invoke_message: String, arg: String)
{
  let mut child = Command::new(invoke_message)
  .arg(arg)
  .spawn()
  .expect("failed to execute child");

let ecode = child.wait()
.expect("failed to wait on child");

assert!(ecode.success());
}


#[tauri::command]
fn execCommandExtra(invoke_message: String, arg: String,arg2: String)
{
  let output = Command::new(invoke_message)
  .arg(arg)
  .arg("-d")
  .arg(arg2)
  .output().unwrap_or_else(|e| {
      panic!("failed to execute process: {}", e)
});

if output.status.success() {
  let s = String::from_utf8_lossy(&output.stdout);

  print!("rustc succeeded and stdout was:\n{}", s);
} else {
  let s = String::from_utf8_lossy(&output.stderr);

  print!("rustc failed and stderr was:\n{}", s);
}
}



use std::process::Command;
fn main() {
  let context = tauri::generate_context!();
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![execCommand,execCommandExtra])
    .run(context)
    .expect("error while running tauri application");
}
