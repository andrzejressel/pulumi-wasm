use std::path::Path;

fn main() {
    changelog_lib::generate_changelog("test", Path::new(".")).unwrap();
}