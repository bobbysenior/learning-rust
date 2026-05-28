fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter *2
        }
    };

    println!("The result is {result}");
    countdown();
}

fn countdown() {
    for i in (1..5).rev() {
        println!("{i}");
    }
    println!("LIFTOFF !!!");
}
