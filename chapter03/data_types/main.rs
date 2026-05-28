fn main() {
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    let a = [2; 5];
    println!("Deuxième valeur : {}", y);
    println!("Première valeur : {}", tup.0);
    println!("Array valeur 0 : {}", a[6]);
}
