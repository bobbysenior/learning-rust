fn main() {
    println!("Hello, world!");
    foo(5);
    println!("5  est égal à {}", five());
}

fn foo(x: i32) {
    println!("Une autre fonction {x}");
}

fn five() -> i32 {
    5 // La dernière expression d'une fonction est retournée implicitement
}
