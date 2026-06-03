
impl String {
    fn coucou(&self) {
        println!("{}", format!("{}", self));
    }
}

fn main() {

    let a = String::from("coucou les ptits loups!");
    println!("Hello, world!");
    let b = format!("{}", a);
    println!("{}", b);
}
