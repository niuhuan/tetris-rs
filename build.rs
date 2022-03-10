use std::path::Path;

fn main() {
    let profile = std::env::var("PROFILE").unwrap();
    std::fs::copy(
        Path::new("font.ttf"),
        Path::new("target").join(&profile).join("font.ttf"),
    )
    .unwrap();
}
