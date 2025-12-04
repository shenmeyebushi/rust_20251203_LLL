use std::io;
use rand::Rng;
use std::cmp::Ordering;

// fn main() {
//     println!("Today is the first day of my learning rust language.");
//     let mut s = String::new();
//     let mut y = String::new();
//     let mut z = String::new();
//     io::stdin()
//         .read_line(&mut s)
//         .expect("you got wrong character");
//     io::stdin()
//         .read_line(&mut y)
//         .expect("you got wrong character");
//     io::stdin()
//         .read_line(&mut z)
//         .expect("you got wrong character");
//     println!("what you input is \n{}{}{}", s,y,z);
// }
// fn main(){
//     println!("Today is the first day of my learning rust language.This is a guessing game.");
//     println!("please input a number:");
//     let n = 24;
//     let mut m = String::new();
//     io::stdin()
//         .read_line(&mut m)
//         .expect("you got wrong input!");
//
//     let m: i32 = match m.trim().parse() {
//         Ok(num) => num,      // 解析成功，返回数字
//         Err(_) => {          // 解析失败（用户输入的不是数字）
//             println!("Please enter a valid number!");
//             return;          // 退出程序
//         }
//     };
//
//     if (n==m){
//         println!("you are right");
//     }
//     else if (n<m) {
//         println!("you are wrong,the number is smaller.");
//     }
//     else if (n>m) {
//         println!("you are wrong, the number is bigger.");
//     }
// }
// fn main(){
//     println!("Read_line 方法.");
//     let mut ipput_01 = String::new();
//     io::stdin()
//         .read_line(&mut ipput_01)
//         .expect("you got wrong input");
//     io::stdin()
//         .read_line(&mut ipput_01)
//         .expect("you got wrong input again");
//
//     println!("your input is {}",ipput_01)
// }


// fn main(){
//     //这是个游戏
//     println!("This is a game of guessing number.");
//
//     //生成一个秘密数字，用rand里的thread_rng函数里的gen_range方法，Rng=RandomNumberGenerate
//     let mut secret_number = rand::thread_rng().gen_range(1..=100);
//
//     //输入一个数字
//     println!("please input a number:");
//     let mut input_01 = String::new();
//     io::stdin()
//         .read_line(&mut input_01)
//         .expect("you got wrong input");
//     let mut input_01: i32 =input_01.trim().parse().expect("you got wrong input");
//     println!("your input number is {}",input_01);
//
//     //比较大小
//     match input_01.cmp(&mut secret_number) {
//         Ordering::Less => println!("less"),
//         Ordering::Greater => println!("greater"),
//         Ordering::Equal => println!("you are right"),
//     }
//     println!("the secret number is {}",secret_number);
// }
// fn main(){
//     println!("this is a game");
//
//     //设定一个随机数字
//     let ra_number = rand::thread_rng().gen_range(1..=100);
//
//     //输入一个猜测的数字
//     println!("please input a number:");
//     let mut guess_number = String::new();
//
//     io::stdin()
//         .read_line(&mut guess_number)
//         .expect("input illegal");
//
//     let guess_number:i32 = guess_number.trim().parse().expect("you got wrong input");
//
//     println!("the number you guess is {}",guess_number);
//
//     match guess_number.cmp(&ra_number) {
//         Ordering::Equal => println!("you are right"),
//         Ordering::Less => println!("less"),
//         Ordering::Greater => println!("greater"),
//     }
//
//     //告诉猜测者答案
//     println!("the secret number is {}",ra_number);
// }

fn main(){
    println!("Welcome to RustyBeer!");
    println!("this is a guessing game!");

    //设定一个答案
    let sec_number = rand::thread_rng().gen_range(1..=100);

    //输入一个猜测数字
    loop {
        println!("please input a number:");
        let mut guess_number = String::new();
        io::stdin()
            .read_line(&mut guess_number)
            .expect("Failed to read line");
        let guess_number:u32 = guess_number.trim().parse().expect("Please type a number!");

        // 输出猜测结果
        match guess_number.cmp(&sec_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You are right!");
                break;
            }
        }
    }


    println!("the secret number is {}", sec_number);
}