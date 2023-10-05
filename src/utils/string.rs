// String to persistent str
pub fn persistent_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

// Optional string to persistent str
pub fn persistent_str_optional(s: Option<String>) -> &'static str {
    if let Some(s) = s {
        persistent_str(s)
    } else {
        ""
    }
}