#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#[tauri::command]
fn login(username: &str,password: &str) -> String {
  if username == "a" && password=="1"
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
fn main() {
   tauri::Builder::default()
    // Add this line
    .invoke_handler(tauri::generate_handler![login,initializesystemwindowowow])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
