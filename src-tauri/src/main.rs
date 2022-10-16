
#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use std::process;

#[tauri::command]
fn login(username: &str,password: &str) -> String {
  if username == "admin" && password=="admin"
  {
    return format!("success") ;
  
  }
  else {
    return format!("unsuccessful") ;
  }
   
}
#[tauri::command]
fn initializesystemwindowowow()
{
  // window.set_size(Size::Logical(LogicalSize { width: 100.0, height: 100.0 })).unwrap();
// or
// window.set_size(Size::Physical(PhysicalSize { width: 100, height: 100 })).unwrap();
}
#[tauri::command]
fn endapp()
{
  process::exit(1);
}
fn main() {
   tauri::Builder::default()
    // Add this line
    .invoke_handler(tauri::generate_handler![login,initializesystemwindowowow,endapp])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
