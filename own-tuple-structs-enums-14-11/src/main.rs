fn take_ownership(bar: String) {
    let words = bar.split(" ");
    for word in words {
        println!("{}", word);
    }
}

fn main() {
    let foo = String::from("Hello World");
    take_ownership(foo);
    println!("{}", foo);
}
