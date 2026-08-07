use super::*;

#[test]
fn hash_part_is_deterministic() {
    let a = hash_part("styling-sat-set-pake-utility-tailwind-css-tanpa-bikin-lo-mikir-selector");
    let b = hash_part("styling-sat-set-pake-utility-tailwind-css-tanpa-bikin-lo-mikir-selector");
    assert_eq!(a, b);
}

#[test]
fn hash_part_differs_for_different_input() {
    assert_ne!(hash_part("first-post"), hash_part("second-post"));
}

#[test]
fn hash_part_shrinks_long_input_to_fixed_length() {
    let long_slug = "a".repeat(200);
    assert_eq!(hash_part(&long_slug).len(), 16);
    assert_eq!(hash_part("short").len(), 16);
}

#[test]
fn versioned_key_uses_hashed_slug_part() {
    let hashed = hash_part("some-slug");
    let key = versioned_key("notes", 0, &["slug", &hashed]);
    assert_eq!(key, format!("notes:v0:slug:{hashed}"));
}
