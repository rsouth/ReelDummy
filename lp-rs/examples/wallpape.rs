use lp_rs;

fn main() {
    let thing = lp_rs::download("test.jpg");
    println!("I made a thing: {:?}", thing);
}
