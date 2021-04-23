use std::time::SystemTime;

use reel_dummy;
use reel_dummy::{ImageType, ReelDummy};

fn main() {
    let time_millis = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let img = ReelDummy::new(ImageType::Generated)
        .with_size(640, 480)
        .fetch()
        .unwrap();
    img.save(&format!("generated-{}.png", time_millis)).unwrap();

    // let img = ReelDummy::new(ImageType::LoremPicsum)
    //     .with_size(640, 480)
    //     .fetch()
    //     .unwrap();
    // img.save(&format!("picsum-{}.png", time_millis)).unwrap();

    println!("Fin.");
}
