use propparse::fetch_file;

fn main() {
    println!("Hello");
    println!("Hello");
    match fetch_file("res/demo.properties") {
        Ok(p) => println!("{:?}", p),
        Err(_) => println!("Error"),
    };
}
