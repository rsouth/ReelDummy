use lp_rs;

fn main() {
    let thing = lp_rs::download(640, 480, "test.jpg");
    println!("I made a thing: {:?}", thing);
}
