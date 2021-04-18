use lp_rs;
use std::time::SystemTime;

fn main() {
    let time_millis = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let file_path = &format!("example-{}.jpg", time_millis);

    lp_rs::LoremPicsum::default()
        .download(640, 480)
        .expect("failed to download")
        .save(file_path)
        .expect("failed to save");
}
