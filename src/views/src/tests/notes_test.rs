use crate::pages::notes::encode_query_param;

#[test]
fn encode_query_param_leaves_unreserved_chars_untouched() {
    assert_eq!(encode_query_param("rust-async_lib.rs~1"), "rust-async_lib.rs~1");
}

#[test]
fn encode_query_param_escapes_space_as_percent_20() {
    assert_eq!(encode_query_param("rust async"), "rust%20async");
}

#[test]
fn encode_query_param_escapes_reserved_url_chars() {
    assert_eq!(encode_query_param("a&b=c#d?e"), "a%26b%3Dc%23d%3Fe");
}

#[test]
fn encode_query_param_escapes_multibyte_utf8_per_byte() {
    // "café" — 'é' is 2 bytes in UTF-8 (0xC3 0xA9), each escaped separately.
    assert_eq!(encode_query_param("café"), "caf%C3%A9");
}

#[test]
fn encode_query_param_empty_input_is_empty_output() {
    assert_eq!(encode_query_param(""), "");
}
