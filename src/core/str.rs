pub fn wstr_to_string(wstr: &[u16]) -> String {
    String::from_utf16(wstr).unwrap()
}

pub fn str_to_wstring(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}
