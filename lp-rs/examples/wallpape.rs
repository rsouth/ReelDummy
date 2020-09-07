use lp_rs;

fn main() {
    let hmac = lp_rs::download(640, 480, "test.jpg");
    println!("HMAC for the image is: {:?}", hmac);
}
