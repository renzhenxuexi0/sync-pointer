use uuid::Uuid;

#[tauri::command]
pub async fn generate_uuid() -> Result<String, String> {
    Ok(Uuid::new_v4().to_string())
}
