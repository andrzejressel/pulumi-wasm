mod common;

#[test]
fn replace_files_should_be_set_to_false() {
    assert!(!common::replace_files());
}
