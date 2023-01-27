pub fn poll_kind_id_to_label(id: i32) -> Option<String> {
    match id {
        0 => Some("First Past the Post".to_string()),
        1 => Some("Single Transferable Vote".to_string()),
        2 => Some("Additional Member System".to_string()),
        _ => None,
    }
}
