use lp_rs;
use std::time::SystemTime;

fn main() {
    let time_millis = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let hmac = lp_rs::download(640, 480, &format!("test-{}.jpg", time_millis));
    println!("HMAC for the image is: {:?}", hmac);
}
