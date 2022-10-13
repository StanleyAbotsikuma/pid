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

fn main() {
   tauri::Builder::default()
    // Add this line
    .invoke_handler(tauri::generate_handler![login])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
