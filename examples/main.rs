use std::time::SystemTime;

use reel_dummy;
use reel_dummy::{ImageType, ReelDummy};
use ImageType::{Generated, LoremPicsum};

fn main() {
    let time_millis = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let img = ReelDummy::generate_image(Generated, 640, 480).unwrap();
    img.save(&format!("generated-{}.png", time_millis)).unwrap();

    let img = ReelDummy::generate_image(LoremPicsum, 640, 480).unwrap();
    img.save(&format!("picsum-{}.png", time_millis)).unwrap();

    println!("Fin.");
}
