use std::char;

pub fn wstr_to_string(wstr: &[u16]) -> String {
    char::decode_utf16(wstr.iter().cloned())
        .map(|r| r.unwrap_or(char::REPLACEMENT_CHARACTER))
        .filter(|&ch| ch != char::from(0))
        .collect()
}

pub fn str_to_wstring(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(Some(0)).collect()
}
