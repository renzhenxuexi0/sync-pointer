#[tauri::command]
pub async fn local_ip() -> Result<String, String> {
    local_ip_address::local_ip()
        .map(|addr| addr.to_string())
        .map_err(|e| e.to_string())
}

/// 修改本地化语言
#[tauri::command]
pub async fn change_locale() -> Result<(), String> {
    Ok(())
}
