use std::io;

fn main() {
    println!("Today is the first day of my learning rust language.");
    let mut s = String::new();
    let mut y = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("you got wrong character");
    io::stdin()
        .read_line(&mut y)
        .expect("you got wrong character");
    println!("what you input is \n{}{}", s ,y);
}
