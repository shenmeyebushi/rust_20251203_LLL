use std::io;
use rand::Rng;
use std::cmp::Ordering;
use qrcode::QrCode;
use image::Luma;
use std::collections::HashMap;
use std::any::type_name;

/*
fn main() {
    println!("Today is the first day of my learning rust language.");
    let mut s = String::new();
    let mut y = String::new();
    let mut z = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("you got wrong character");
    io::stdin()
        .read_line(&mut y)
        .expect("you got wrong character");
    io::stdin()
        .read_line(&mut z)
        .expect("you got wrong character");
    println!("what you input is \n{}{}{}", s,y,z);
}
fn main(){
    println!("Today is the first day of my learning rust language.This is a guessing game.");
    println!("please input a number:");
    let n = 24;
    let mut m = String::new();
    io::stdin()
        .read_line(&mut m)
        .expect("you got wrong input!");

    let m: i32 = match m.trim().parse() {
        Ok(num) => num,      // 解析成功，返回数字
        Err(_) => {          // 解析失败（用户输入的不是数字）
            println!("Please enter a valid number!");
            return;          // 退出程序
        }
    };

    if (n==m){
        println!("you are right");
    }
    else if (n<m) {
        println!("you are wrong,the number is smaller.");
    }
    else if (n>m) {
        println!("you are wrong, the number is bigger.");
    }
}
fn main(){
    println!("Read_line 方法.");
    let mut iput_01 = String::new();
    io::stdin()
        .read_line(&mut ipput_01)
        .expect("you got wrong input");
    io::stdin()
        .read_line(&mut ipput_01)
        .expect("you got wrong input again");

    println!("your input is {}",ipput_01)
}


fn main(){
    //这是个游戏
    println!("This is a game of guessing number.");

    //生成一个秘密数字，用rand里的thread_rng函数里的gen_range方法，Rng=RandomNumberGenerate
    let mut secret_number = rand::thread_rng().gen_range(1..=100);

    //输入一个数字
    println!("please input a number:");
    let mut input_01 = String::new();
    io::stdin()
        .read_line(&mut input_01)
        .expect("you got wrong input");
    let mut input_01: i32 =input_01.trim().parse().expect("you got wrong input");
    println!("your input number is {}",input_01);

    //比较大小
    match input_01.cmp(&mut secret_number) {
        Ordering::Less => println!("less"),
        Ordering::Greater => println!("greater"),
        Ordering::Equal => println!("you are right"),
    }
    println!("the secret number is {}",secret_number);
}
fn main(){
    println!("this is a game");

    //设定一个随机数字
    let ra_number = rand::thread_rng().gen_range(1..=100);

    //输入一个猜测的数字
    println!("please input a number:");
    let mut guess_number = String::new();

    io::stdin()
        .read_line(&mut guess_number)
        .expect("input illegal");

    let guess_number:i32 = guess_number.trim().parse().expect("you got wrong input");

    println!("the number you guess is {}",guess_number);

    match guess_number.cmp(&ra_number) {
        Ordering::Equal => println!("you are right"),
        Ordering::Less => println!("less"),
        Ordering::Greater => println!("greater"),
    }

    //告诉猜测者答案
    println!("the secret number is {}",ra_number);
}

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

元组的定义、寻找元组的第N个元素
fn main(){
    let tum_try:(u32,f32,i64) =(12,87.02,95);
    let tum_01 =tum_try.1;
    println!("{}",tum_01);
}

fn main(){
    println!("Welcome to RustRover!");
    println!("This is a guessing-number game!");

    //生成一个随机数字
    let mut sec_number = rand::thread_rng().gen_range(1..=100);

    /*

    一个大循环——————————
    1）输入猜测的数字，输入后读出来，并把数字的字符串格式转化成数字格式；
    2）如果失败就continue下一个大循环继续输入；
    3）如果成功就把这个数字返回给OK，然后OK会返回给parse，然后返回给trim，然后赋值给gue_number;
        并且继续执行如下：
    4）比较两个数字：
    如果less就打印“太小”，进入了下一个大循环；
    如果greater就打印“太大”，进入了下一个大循环；
    如果equal就打印“猜对了”，break掉整个大循环，执行下一个步骤；
    5）打印出来全局作用域里的gue_number

    */
    loop {
        let mut gue_number = String::new();
        io::stdin()
            .read_line(&mut gue_number)
            .expect("Your input is illegal!");

        let gue_number:i32 = match gue_number.trim().parse(){
            Ok(nm) => nm,
            Err(_) => {println!("You got wrong input");
                       continue;}
        };

        match gue_number.cmp(&sec_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You are right!");
                break;
            }
        }
    }

    //打印出猜测数字
    println!("The secret number is {}",sec_number);
}


fn main(){
//定义一个tumple
    let a = [5,6,7,8,9];
//建立一个数字字符变量
    let mut input_number = String::new();
//输入这个数字字符并变成usize类型
    io::stdin()
        .read_line(&mut input_number)
        .expect("You got wrong input");
    let input_number:usize = input_number.trim().parse().expect("You got wrong input");
//把数字变成个index，打印相应index的tumple的值
    let ele = a[input_number];
    println!("What you strive for is the element: {}",ele);
}

函数
fn main(){
    another_function(5);
}

fn another_function(x:i32){
    println!("The number is {}",x);
}

fn main(){
    for number in (1..4).rev(){
        println!(" {}", number);
    }
    println!("time out");
}

引用
fn main(){
    let mut s = String::from("hello");
    let s1 = &s;
    let s2 = &s;
    println!("{} and {}",s1,s2);
    println!("{} or {}",s1,s2);
}




一个矩形的面积计算（struct、impl）
struct Rectangle{
    width:i32,
    height:i32,
}

impl Rectangle {
    fn area(&self) -> i32 {
        self.width * self.height
    }
}

fn main(){
    let rect01 = Rectangle{
        width: 30,
        height: 90,
    };
    println!("The area is {}",rect01.area());
}


tuple，和 调用其中的元素
fn main(){
    let mut v = vec! [5,6,9,87,8,99,66];
    let x: &i32 = &v[2];
    println!("The number is {}",x);
    let y: Option<&i32> = v.get(2);
    match y {
        Some(z) => println!("the number is {}",z),
        None => println!("i have no idea!"),
    }
}

//vector 动态数组，也要声明mut，否则不能push
fn main(){
    let mut v = vec![5,6,8,9,78,4,56];
    v.push(4545);
    // for n in v.iter(){
    //     println!("{}",n);
    // }
    // v.sort();
    // for m in v.iter(){
    //     println!("{}",m);
    // }
    for i in &v{
        println!("{}",i);
    }
    v.sort();
    for mut o in &mut v{
        *o += 1;
        let mut u = vec![];
        u.push(o);
        for p in u.iter_mut(){
            println!("element in u is: {}",p);
        }
        // println!("{}",o);
    }
}

字符串string
fn main(){
    // let mut s = String::new();
    let s = String::from("SIR!");
    let data_push = "Can not release!";
    let mut y = data_push.to_string();
    // println!("{}",s);
    y.push_str("\nyes");
    println!("{}\n{}\n{}",s,y,data_push);
}

字符串string,add+,format!;
fn main(){
    let mut s1 = String::new();
    let mut s2 = String::new();
    s1.push_str("hello,");
    s2.push_str("world!");
    // let mut s3 = s1 + &s2;
    // println!("{}",s3);
    // println!("{}",s1);
    let s3 = format!("{}-{}",s1,s2);
    println!("{}:",s3);
}

fn main(){
    let mut s = String::from("hello");
    for mut i in s.chars(){
        println!("{}",i);
    }
}


fn main(){
    let mut scores_h = HashMap::new();
    scores_h.insert(String::from("blue"), 10);
    scores_h.insert(String::from("red"), 20);

    for (m,n) in &scores_h{
        println!("{}:{}",m,n);
    }
}
fn main(){
    let fname = String::from("Color:");
    let cname = String::from("Blue");

    let mut scores_hh = HashMap::new();
    scores_hh.insert(&fname,&cname);
    for (n,m) in scores_hh{
        println!("{}{}",n,m);
    }
    println!("{}",fname);
}

以下是一个哈希map的应用，统计了一句话里的单词出现次数。
fn main(){
    let text_01 = "this is a totally new world";
    let mut map_new = HashMap::new();

    for i in text_01.split_whitespace(){
        let n = map_new.entry(i).or_insert(0);
        *n += 1;
    }
    println!("{map_new:?}");
}


struct PointA<T,O>{
    x:T,
    y:O,
}

impl <L,H> PointA<L,H>{
    fn x(&self) -> &L{
        &self.x
    }
    fn y(&self) -> &H{
        &self.y
    }
}

fn main(){
    let point_a = PointA{
        x:5.0,
        y:6,
    };
    print!("{}\n",point_a.x());
    println!("{}",point_a.y());
}


pub trait Summary{
    fn summarize(&self) -> String;
}
pub struct NewsArticle{
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
pub struct Tweeter {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
impl Summary for NewsArticle{
    fn summarize(&self) -> String {
        format!(
            "{}, by {}, ({})",
            self.headline,
            self.author,
            self.location,
        )
    }
}
impl Summary for Tweeter{
    fn summarize(&self) -> String {
        format!("{}: {}",self.username,self.content)
    }
}


fn notify(x: &impl Summary){
    println!("热点新闻！ {}",x.summarize());
}
fn main(){
    let news = NewsArticle{
        headline: String::from("NEWS"),
        location: String::from("China"),
        content: String::from("China"),
        author: String::from("Liu Jinhua"),
    };
    let tweet = Tweeter{
        username: String::from("Rust"),
        content: String::from("Rust is a good language"),
        reply:false,
        retweet: true,
    };

    notify(&news);
    notify(&tweet);
}

fn main(){
    let string_01 = "abcdef";
    let b = Some(2);
    if b == string_01.find("c"){
        println!("ok");
    }
}

下面是一些错误的代码
fn main(){
    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("you got wrong input");
    first_char(&s);
    fn first_char(s1: &String){
        for char_i in s1.chars(){
            let mut n:i32=1;
            if n == 1{
                print!("The first character is {}.",s1);
            }else if n >=2{
                break;
            }
            n += 1;
        }
    }
}

下面是find和rfind方法的用法
fn main(){
    let s = String::from("hello world you are very nice and strong");
    println!("find 'very' is located at: {}",s.find("very").unwrap());
}

fn main(){
    // 设定一个字符串，以空格为分割依据
    //以http协议的请求行为例
    let mut string_http = "GET /usr/rust/index.html HTTP/1.1.0 \r\n";
    let mut string_http = string_http.trim();
    println!("目前|{}|",&string_http);
    //使用split,以空格为分割依据
    let result_01 = string_http.split(' ');
    println!("result_01的数据类型是{}",name_of_type(&result_01));

    let result_02 = string_http.split(' ').collect::<Vec<&str>>();
    println!("result_02的数据类型是{}",name_of_type(&result_02));
    println!("请求方法是{}",result_02[0]);
    println!("HTTP版本是{}\n到了B这里了",result_02[2]);
    fn name_of_type<T>(_:T) -> &'static str{
        return std::any::type_name::<T>();
    }
    //打印result_01\02里面的element，如果有的话。
    for i in result_01{
        println!("到了C这里了\n{}",i);
    }
    println!("到了D这里了{}\n",result_02[0]);


    let result_03 = &result_02[1].split('/').collect::<Vec<&str>>();
    for j in result_03{
        println!("到元素{}\r",j);
    }
    println!("到了E这里了{}",result_02[2]);
    let mut http_str_new = String::new();
    for k in result_03{
        http_str_new.push_str(" ");
        http_str_new.push_str(k);
    }
    println!("到了F这里了{}",http_str_new);
}



二维码生成
fn main(){
    let content_01 = String::from("\n崇祯五年十二月，余住西湖。
    大雪三日，湖中人鸟声俱绝。\n
    是日更定矣，余拏一小舟，拥毳衣炉火，独往湖心亭看雪。雾凇沆砀，天与云与山与水，上下一白。\n
    湖上影子，惟长堤一痕、湖心亭一点、与余舟一芥、舟中人两三粒而已。\n
    到亭上，有两人铺毡对坐，一童子烧酒炉正沸。见余，大喜曰：'湖中焉得更有此人！'拉余同饮。\n
    余强饮三大白而别。问其姓氏，是金陵人，客此。\n
    及下船，舟子喃喃曰：'莫说相公痴，更有痴似相公者！'\n
    ");
    ;
    let code_01 = QrCode::new(&content_01).unwrap();
    let image_01 = code_01.render::<Luma<u8>>().build();
    image_01.save("zhang_dai.png").unwrap();
    println!("OK!");
}


fn main(){
    let mut house = String::from("This is a big house containing of lots of story.");
    let mut house_00 = & mut house;
    // let mut glass_01 = & mut house;
    let mut glass_01 = & mut house_00;
    *glass_01 = "改动档案".to_string();
    println!("{}",glass_01);
    // println!("glass_01打印结果：{}",glass_01);
    // println!("&glass_01打印结果：{}",&glass_01);
    // println!("*glass_01打印结果：{}",*glass_01);
    // println!("house打印结果：{}",house);
    // println!("&house打印结果：{}",&house);
}

后面有变量引用了某个A变量，这个A变量也是不能随便改动的
fn main(){
    let mut house= String::from("This is a big house containing of lots of story.");
    let mut key = & mut house;
    *key = String::from("df");
    let mut y = & key;
    println!("{}",key);
    println!("{}",house);
    println!("{}",&y);
}


fn main(){
    let mut item_01:(String,f64,f64,i64) = (" ".to_string, 0.0, 0, 0);
    let file_content = std::fs::read_to_string("./text.txt").unwrap();
}



学习枚举类型

声明枚举类型
enum Weekdays {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}
fn main(){
    //接收用户的输入
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("you got wrong input 1");
    let user_input_number: i32 = match user_input.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("parse 之后，发现输入错误！");
            return;}
    };

    if user_input_number >7 || user_input_number <1 {
        println!("you got wrong input 3");
        return;
    }

    let result= match user_input_number {
        1 => Weekdays::Monday,
        2 => Weekdays::Tuesday,
        3 => Weekdays::Wednesday,
        4 => Weekdays::Thursday ,
        5 => Weekdays::Friday,
        6 => Weekdays::Saturday,
        7 => Weekdays::Sunday,
        _ => return,
    };

    match  result{
            Weekdays::Friday => println!("明天是周末了，可以到阳光下去走一走了！"),
            _ => println! ("OK,第二次，什么也不会发生"),
    }
}


参数枚举类型
enum Weekdays {
    Monday(i32),
    Tuesday(i32),
    Wednesday(i32),
    Thursday(i32),
    Friday(i32),
    Saturday(i32),
    Sunday(i32),
}
fn main(){
    println!("请输入一个数字（1-7之间）");
    //接收用户的输入
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("you got wrong input 1");

    //把用户的输入parse成一个i32类型的数字；
    let user_input_number: i32 = match user_input.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("parse 之后，发现输入错误！");
            return;}
    };

    判断一下，用户的输入数字是不是合格的；
    if user_input_number >7 || user_input_number <1 {
        println!("you got wrong input 3");
        return;
    }

    现在开始真正的运用参数枚举
    let hospital_time = Weekdays::Monday(3);
    let school_time = Weekdays::Tuesday(7);

    println!("第一次使用match");

    match hospital_time {
        Weekdays::Monday(x) => println!("医院的上班时间是早上{}点。",x),
        Weekdays::Tuesday(x) => println!("xx上班时间是早上{}点。",x),
        _ => return,
    }

}



类似C语言的枚举类型

enum Weekdays{
        Monday = 1,
        Tuesday = 2,
    }
fn main(){
    let mut a = Weekdays::Monday;
    let mut b = Weekdays::Tuesday;
    // println!("{}",a as f64);
    // println!("{}",&b);
    // println!("{}",&a);
    println!("{}",b as i32);
}
fn name_of_type<T>(_:T){
    println!("的变量类型是{}",type_name::<T>());
}
fn main(){
    let aaa: Option<i32> = Some(23456);
    let bbb: Option<i32> = None;
    // aaa = Option::from(1);
    // bbb = Option::from(None);
    if aaa.is_some() {
        println!("xxxxxx is Some");
    }

    // if bbb.is_some(){
    //     println!("yyyyyy is Some");
    // }

    if bbb.is_none(){
        println!("yyyyy is None");
        name_of_type(bbb)
    }

}


fn main(){
    let a = "1".to_string();
    let b = "1234".to_string();
    // if a == b {
    //     println!("a=b");
    // }
    // else {
    //     println!("wrong");
    // }

    // let c = &a;
    // let d = c;
    // println!("{}",d);
    // let e = b;
    // println!("{}",e);
    //
    // println!("{:p}",&d);
    // println!("{:p}",&e);
    println!("{:p}\n{:p}",&a,&b);
    if b.contains(&a){
        println!("b.contains(&a)");
    }else { println!("b并不contains(&a)"); }

    if a.contains(&b) {
        println!("a.contains(&b)");
    }else { println!("a并不contains(&b)"); }

    let a = Some(123.5);
    let b = 123.5;
    if a.expect("这个时候运行出错了。") == b{
        println!("真的呀，现在真的是a=b，也就是说，a是Some枚举对象，\
        这个对象有个参数值，是Some（）的（）里存放着的那个参数值，在这里是123.5。\
        expect方法使用的时候，Some（）的（）里的值被取出来用了。");
    }


    name_of_type(a);
    fn name_of_type<T>(_:T){
        println!("数据类型为{}",type_name::<T>());
    }
}

时间
use chrono::prelude::*;

fn main(){
    let universal_time = Utc::now();
    println!("当前的UTC时间是{}",universal_time.to_string());
    let local_time = Local::now();
    println!("当前的本地时间是{}",local_time);
}


tuple
Vector
fn main(){
    数组的概念，先生成一个空的数组；
    let array_01:[&str;3] = ["abcd","efgh","xyz"];

    for i in &array_01{
        println!("{}",i);
    }
    println!("{}",array_01.len());


    // for i in array_01{
    //     println!("{}",i);
    // }
    //使用下标的方式，输出某个具体的element；
    let a:&str = array_01[1];
    println!("{}",a);
    println!("++++++++++++++++++++++++++++++++++++++++");

    let mut vec_01 = Vec::new();
    vec_01.push("aaaa");
    vec_01.push("bbbb");
    vec_01.push("dddd");

    for i in &vec_01{
        println!("{}",i);
    }
    println!("=========================================");
    println!("{}",vec_01[1]);
    println!("{}",vec_01.len());
    println!("Program End!");

    /vector的方法——append()
    let mut vec_03:Vec<i32> = (0..5).collect();
    let mut vec_04:Vec<i32> = (11..13).collect();
    collect()是一个专用收集前面的迭代器里的数据的方法；
    （11..15)是一个迭代器；

    let vec_05 = vec_03.append(&mut vec_04);
    for i in vec_05 {
        println!("{}",i);
    }

    vec_03.append(&mut vec_04);
    for i in vec_03 {
        println!("{}",i);
    }
}


//vector的方法——extend()
/*
fn main() {
    let mut vec_06 : Vec<i32> = (0..5).collect();
    vec_06.extend([111,222,333,444].iter().copied());
    println!("Vector类型的变量vec_06当中的全部内容是：");
    for i in &vec_06 {
        println!("i:{}",i);
    }
    let mut vec_07: Vec<f64> = [1.11, 1.22, 1.88, 1.44, 1.55, 1.66].to_vec();
    println!("vec_07的长度是{}", vec_07.len());

    for i in &vec_07 {
        println!("元素是{}",i);
    }
    //     vec_07.remove(2);
    //     for i in &vec_07{
    //         println!("{}",i);
    //     }
    // }
    vec_07.pop();
    for i in &vec_07 {
        println!("{}",i);
    }
    println!("vec_07的新长度是{}",vec_07.len());

    //vector方法——swap()\sort()\sort_by()\shrink_to_fit()
    vec_07.swap(0,4);
    for i in &vec_07{
        println!("{}",i);
    }
    // float 元素的vector排序怎么排？
    vec_07.sort_by(|x,y|x.partial_cmp(&y).unwrap());
    for i in &vec_07{
        println!("{}",i);
    }

*/

//Linked list链表的知识
//原理上来说一个UUID数字极其大，一个6位的37进制数字，约等于2565Million。
//以下是这个计算表示
/*
fn main(){
    let mut n = String::new();
    std::io::stdin()
        .read_line(&mut n)
        .expect("you got wrong input");
    let m: i32= n.trim().parse().expect("something wrong!");
    let mut x:f64 = 37.0_f64.powi(m as i32);
    print!("37(n)={}",x);
}
*/

/*
fn main(){
    let mut n = String::new();
    std::io::stdin()
        .read_line(&mut n)
        .expect("you got wrong input");
    let m: i32= n.trim().parse().expect("something wrong!");
    let mut x:f64 = 37.0_f64.powi(m as i32);
    print!("37(n)={}",x);
}
*/



/*
fn main(){
//自己叫刘言复！
}
 */

use std::process::*;
/*
fn main(){
    let return_value = Command::new("uuidgen")
        .output()
        .expect("you got wrong input");
    let return_value = return_value.stdout;
    // println!("{:?}",return_value);
    let use_return_value = std::str::from_utf8(&return_value).expect("xx");
    println!("{}",use_return_value);

}

 */


//:?怎么用？
/*

fn main(){
    let mut arr_02:[i32;_] = [12,13,15,16,17,14,40,9,30,89];
    println!("{:?}",arr_02);
}



//胖指针

fn main(){
    let str_01 = "hello";
    let arr_01 : [usize;2];

    arr_01 = unsafe{
        std::mem::transmute(str_01)
    };

    let pointer_01 = arr_01[0] as *const u8;

    for i in 0..arr_01[1]{
        unsafe {
            println!("{}",*pointer_01.add(i) as char);
        }
    }
}





fn main(){
    let mut a = String::from("Hello");

    let s_01 = &a;
    let s_02 = &a;
    println!("{},{}",s_01,s_02);
    for i in s_01.chars(){
        println!("{}",i);

    }

    let s_03 = &s_02;
    for i in s_03.chars() {
        println!("{}",i);
    }
}



enum Action{
    Say(String),
    Moveto(i32,i32),
    ChangToColorRGB(u16,u16,u16),
}

fn main(){
    let mut action01:[Action;5] = [
        Action::Say("Hello,world!".to_string()),
        Action::Moveto(18,29),
        Action::ChangToColorRGB(255,255,0),
        Action::Say("你好吗".to_string()),
        Action::Moveto(355555,7123477),
    ];

    for i in action01{
        match i {
            Action::Say(s)=>{
                print!("现在到了i里面，匹配到了Say。它肯定有个s参数，那么就把传入的s参数打印出来：{}\n",s);
            },
            Action::Moveto(m,n)=>{
                println!("现在到了i里面，匹配到了Moveto。它肯定有两个参数，那么我们可以打印一个东西出来：{}\n",(m+n)/3);
            },
            Action::ChangToColorRGB(n,p,_)=>{
                println!("rgb的数字是{},{}\n",n,p);
            }
        }
    }
}

fn main(){
    let age = Some(30);
    println!("匹配前age={:?}",age);
    match &age {
        Some(y) => {println!("What is this age?_{}",y);},
        _ => ()
    }
    println!("匹配后age={:?}",age)
}


fn function_plus_1(x:Option<&i32>)-> Option<i32>{
    match x{
        Some(&y) => Some(y+1),
        None => None,
    }
}
fn main(){
    let mut m = Some(30);
    let result = function_plus_1(m.as_ref());
    println!("{:?}",m);
    println!("{:?}",result);
}



struct Point{
    x:i32,
    y:i32,
}
fn main(){
    let p = Point{x:0,y:9};
    match p {
        Point{x,y:0} => println!("001"),
        // Point{x:0,y} => println!("002"),
        Point{x,y} => println!("003"),
    }
}

//先定义了一个enum类别，里面有两个variants
enum Media {
    Post,
    Weibo,
}

//给每个变体赋予细节内容
#[derive(Debug)]
pub struct Post{
    pub title:String,
    pub author:String,
    pub content:String,
    pub capacity:i32,
}
#[derive(Debug)]
pub struct Weibo{
    pub title:String,
    pub author:String,
    pub content:String,
    pub capacity:i32,
    pub source:String,
    pub username:String,
    pub phone_class:String,
}

//定义trait
pub trait Summary{
    fn summarize(&self) -> String;
    fn desummarize(&self) -> String;
}

//给每个变体赋予相应的trait
impl Summary for Post {
    fn summarize(&self) -> String{
        format!("这个post的标题是：{}",self.title)
    }
    fn desummarize(&self) ->String{
        format!("这个post的具体内容是：{}",self.content)
    }
}

impl Summary for Weibo{
    fn summarize(&self) -> String {
        format!("这个微博的标题是{}，来源于这种手机：{}",self.title,self.phone_class)
    }
    fn desummarize(&self) -> String {
        format!("这个微博的具体内容是：rust真的很严谨")
    }
}

fn main(){
    let post_01 = Post{
        title:String::from("大干快上！"),
        author:String::from("HU Feng"),
        content:String::from("今年是个丰收年，但是不能继续玩。"),
        capacity:128,
    };
    let weibo_01 = Weibo{
        title:String::from("《汽车消费观察》"),
        author:String::from("记者 刘禹锡"),
        content:String::from("美国和加拿大在人均汽车销量上显著高于其他国家，反映了其成熟的汽车消费市场。"),
        capacity: 43,
        source:String::from("Chatgpt"),
        username:String::from("LIU Jinhua"),
        phone_class:String::from("Iphone"),
    };

    println!("{:?}",post_01);
    println!("{:?}",weibo_01);

    let a = post_01.summarize();
    let c = weibo_01.summarize();
    let b = post_01.desummarize();
    let d =weibo_01.desummarize();

    println!("{}\n{}\n{}\n{}\n",a,b,c,d);
}

 */

use::fmt::{Display};
use
trait Draw {
    fn draw(&self)-> String;
}
pub struct Screen<T:Draw>{
    pub components: Vec <T>,
}

impl<T> Screen<T> where T:Draw{
    pub fn run(&self){
        for component in self.components.iter(){
            component.draw()
        }
    }
}

fn main(){
    let screen01 = Screen{
        components:vec![
            Box::new(SelectBox{
                width:75,
                height:125,
                option:vec![
                    String::from(""),
                    String::from(""),
                    String::from(""),
                ],
            }),
            Box::new(SelectBox{
                width:95,
                height:60,
                option:vec![
                    String::from(""),
                    String::from(""),
                    String::from(""),
                ],
            }),
        ],
    };

    screen01.run();
}



#[derive(Debug)]
pub struct ImportantExcerpt<'a>{
    part1:&'a str,
    part2:&'a str,
}

fn main(){
    let i;
    let novel01 = String::from("Call me Ishmael. Some years ago...");

    {
        let first_sentence = novel01.split('.').next().expect("Could not find a '.'.");
        i = ImportantExcerpt{
            part1:first_sentence,
            part2:first_sentence,
        }
    }
    println!("{:?}",i);
}



//引用的生命周期，在一个作用域内，无可变权限的引用和有可变权限的引用不能同时存在。
struct Foo;
impl Foo {
    fn mutate_and_share<'a>(&'a self)-> &'a Self{
        &self
    }
    fn share<'a>(&'a self){}
}

fn main() {
    'b:{
       let mut foo:Foo = Foo;
       foo.mutate_and_share();
       foo.share();
    }
}



#[derive(Debug)]
struct Structure(i32);
#[derive(Debug)]
struct Person<'a>{
    name: &'a str,
    age: u8,
}
fn main(){
    // println!("{number:>100}",number= 1);
    // let number:f64 = 2.0;
    // let width:usize = 5;
    // println!("{number:>width$}");
    // let s = Structure(5);
    // println!("{:?}",s);
    let name = "Peter";
    let age = 25;
    let peter = Person{name, age};
    println!("{:?}",peter);
}




#[derive(Debug)]
struct Structure{
    name : String,
    age : u32,
}

impl fmt::Display for Structure{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}", self.name, self.age)
    }
}

fn main() {
    let peter = Structure{
        name : String::from("peter"),
        age : 28,
    };
    println!("{}", peter);
}



#[derive(Debug)]
struct Complex{
    real:f64,
    imag:f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {:+}i)", self.real, self.imag)
    }
}
fn main(){
    let p1 = Complex{
        real: 3.3,
        imag: 7.2,
    };
    let p2 = Complex{
        real: 4.7,
        imag: -2.3,
    };
    println!("Display: {}", p1);
    println!("Debug: {:?}\n", p1);
    println!("Display: {}", p2);
    println!("Debug: {:?}\n", p2);

}



//打印并display一个list(vec<i32>)
struct List(Vec<i32>);

impl fmt::Display for List{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;

        write!(f, "[")?;

        for (i,v) in vec.iter().enumerate() {
            if i != 0 {write!(f, ",")?;}//?的作用是告诉rustc在出错时返回；
            write!(f, "{}:{}", i,v)?;
        }
        write!(f, "]")

    }
}
fn main() {
    let v = List(vec![1,2,3,4,5]);
    println!("{}", v);
}


use std::fmt::{self,Formatter,Display};

struct City{
    name:&'static str,
    lat:f32,
    lon:f32,
}

impl Display for City{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 {"N"} else {"S"};
        let lon_c = if self.lon >= 0.0 {"E"} else {"W"};
        write!(f,"{}:    {:.3} {}_{:.3}{}",
        self.name,self.lat.abs(),lat_c,self.lon.abs(),lon_c)
    }
}
#[derive(Debug)]
struct  Color{
    red:u8,
    green:u8,
    blue:u8,
}

impl Display for Color{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,"(red:{:0>3},green:{:0>3},blue:{:0>3})",
               self.red,self.green,self.blue)
    }
}

fn main() {
    for city in [
        City{name:"Dublin",lat:53.0347778,lon:-6.259722},
        City{name:"Oslo",lat:59.95,lon:10.75},
        City{name:"Vancouver",lat:49.25,lon:-123.1}
    ]{
        println!("{}", city);
    }

    for color in [
        Color{red:128, green:255, blue:90},
        Color{red:0, green:3, blue:254},
        Color{red:0, green:0, blue:0},
    ]{
        println!("RGB:{}",color);

    }
}

use::std::fmt::{self,Formatter,Display};
#[derive(Debug)]
struct Matrix1(f32, f32, f32,f32);

fn main() {
    let long_tuple = (1u8,2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
    0.1f32,0.2f64,'a',true);

    println!("长元组的第一个值：{}",long_tuple.0);
    println!("长元组的第二个值：{}",long_tuple.1);

    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
    println!("{:?}",tuple_of_tuples);

    let pair02 = (1,true);
    println!("{:?}",pair02);
    println!("{:?}",reverse(pair02));
    let matrix01=Matrix1(1.1,2.2,3.3,4.4);
    println!("{:?}",matrix01);


}

fn reverse(p: (i32,bool)) -> (bool,i32) {
    let (x,y) = p;
    (y,x)
}


//定义一个矩形，为它安装个计算面积的工具箱；

use::std::fmt::{self,Formatter,Display};

#[allow(dead_code)]

#[derive(Debug)]
struct Person{
    name:String,
    age:u32,
}
// #[derive(Debug)]
// struct Unit;
#[derive(Debug)]
struct Point{
    x:f32,
    y:f32,
}
#[derive(Debug)]
struct Rectangle{
    top_left:Point,
    bottom_right:Point,
}
trait CalculateArea{
    fn area(&self) -> f32{
        self.area()
    }
}
impl CalculateArea for Rectangle{
    fn  area(&self) -> f32{
        let length = self.bottom_right.x - self.top_left.x;
        let width = self.bottom_right.y - self.top_left.y;
        let area = length*width;
        if area <= 0.0 {
            -area
        }else {
            area
        }
    }
}
fn main() {
    let name = String::from("Peter");
    let age: u32 = rand::thread_rng().gen_range(1..=100);
    let peter_hall = Person { name, age };
    println!("peter_hall = {:?}", peter_hall);

    let point1 = Point { x: 5.2, y: 0.4 };
    let point2 = Point { x: 10.3, y: 0.2 };
    let rectangle = Rectangle { top_left: point1, bottom_right: point2 };
    println!("area of rectangle = {}", rectangle.area());
}




//定义一个矩形，为它安装个计算面积的工具箱；
use std::fmt::{self,Display,Formatter};
//先定义点；
#[derive(Debug)]
struct Point{
    x: f64,
    y: f64,
}

//再定义N，矩形，矩形就是在平面上的两个点构成的图形；
struct Rectangle{
    top_left: Point,
    bottom_right: Point,
}

//再定义这个世界上有一种V，“工具/能力”，trait，可以为任何一个事物计算面积
trait CalculateArea{
    fn area(&self) -> f64;
}

//最后把这种能力安排给Rectangle

impl CalculateArea for Rectangle{
    fn area(&self) -> f64{
        let width = self.bottom_right.x - self.top_left.x;
        let height = self.bottom_right.y - self.top_left.y;
        let area = if width * height>0.0 {width*height} else {0.0-width*height};
        return area;
    }
}

fn main() {
    let rec_01 = Rectangle{
        top_left: Point{x: 12.0, y: 3.0},
        bottom_right: Point{x: 9.0, y: 15.0},
    };
    println!("S={}",rec_01.area());
}


//枚举
enum WebEventVeryManyLotsOfThingsInTheNet{
    PageLoad,
    PageUnload,
    KeyPress(char),
    KeyRelease(char),
    Paste(String),
    Click{x:i64, y:i64},
}
type WebEvent = WebEventVeryManyLotsOfThingsInTheNet;
fn inspect(event: WebEvent){
    match event {
        WebEvent::PageLoad => { println!("page loaded"); },
        WebEvent::PageUnload => { println!("page unloaded"); },
        WebEvent::KeyPress(x) => { println!("key pressed: {}", x); },
        WebEvent::KeyRelease(y) => { println!("key released: {}", y); },
        WebEvent::Paste(x) => {println!("您刚才粘贴了————{}", x);},
        WebEvent::Click{x, y} => { println!("您点击的坐标是—————({}, {})", x, y); },
    }
}

fn main() {
    let press01 = WebEvent::KeyPress('l');
    let pasted01 = WebEvent::Paste("你的文本".to_string());
    let click01 = WebEvent::Click{x:45,y:3489};
    let unload01 = WebEvent::PageUnload;

    inspect(press01);
    inspect(pasted01);
    inspect(click01);
    inspect(unload01);
}



//关于type、trait（注意，trait的声明里只有fn的签名，没有function的body；
enum VeryVeryLongVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

type Very = VeryVeryLongVerboseEnumOfThingsToDoWithNumbers;

trait FangFa {
    fn run(&self,x:i32,y:i32)->&str;
    fn lift(&self) ->&str;
}

impl FangFa for Very {
    fn run(&self,x:i32,y:i32)->&str{
        match self {
            Self::Add => {println!("x+y={}",x+y);
                "add OK"},
            Self::Subtract => {println!("x-y={}",x-y);
                "subtract OK"},
        }
    }
    fn lift(&self)->&str {
        match self {
            Self::Add => {"您传入的是add"},
            Self::Subtract=> {"您传入的是subtract"},
        }
    }
}

fn main() {
    let very=Very::Add;
    let n = very.run(2,3);
    println!("{}",n);
    let m = very.lift();
    println!("{}",m);
}



//手动作用域限定——也就是把use写在main里面，这样可以方便很多，
//不需要写那么多路径
enum Stage{
    Beginner,
    Advanced,
}
enum Role{
    Student,
    Teacher,
}

fn main() {
    use crate::Stage::*;
    use crate::Role::*;

    let fu_yi_fei = Student;
    let zhang_ying = Teacher;
    let stage_01 = Beginner;
    let stage_02 = Advanced;

    match fu_yi_fei {
        Student => println!("Fuyifei is a student."),
        Teacher => println!("Fuyifei is a teacher."),
    }

    match zhang_ying {
        Student => println!("ZhangYing is a student"),
        Teacher => println!("ZhangYing is a teacher"),
    }

    match stage_01 {
        Beginner => {println!("This is a beginner.");},
        Advanced => println!("This is a advanced."),
    }
    match stage_02 {
        Beginner => println!("This is a beginner"),
        Advanced => println!("This is a advanced."),
    }
}

//冻结
fn main(){
    let mut _mutable_int = 78i32;
    {
        let _mutable_int = _mutable_int;
        // _mutable_int = 40;
    }
    _mutable_int = 3;
}


//if else
fn main(){
    let n = 5;
    if n<0{
        println!("{}是负数。",n);
    }else if n>0{
        println!("{}是正数。",n);
    }else {
        println!("{}是零。",n);
    }

    let big_n =
    if n<10 && n>-10{
        println!("big_n是一个小数字。");
        n*10
    }else {
        println!("big_n是一个大数字。");
        n/2
    };
    println!("{}", big_n);
}

//loop
fn main() {
    let mut count = 0i32;
    loop {
        count += 1;

        if count == 5{
            continue
        }else {
            println!("{}", count);
        }

        if count == 19{
            break;
        }

    }
}


fn main() {
    let mut counter = 0;
    let result = loop{
        counter += 1;
        if counter == 10 {
            break counter*2;
        }
    };
    assert_eq!(result,20);
}

fn main() {
    for mut i in 0..=100{
        println!("i={}",i);
        i+=1;
    }
}


enum Foo{
    Bar,Baz,Qux(u32)
}
fn main() {
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    if let Foo::Bar = a {
        println!("Hello, world! a 是Foo的一种变体，它是Bar类型");
    }//如果匹配失败，则nothing发生
}


#[derive(Debug)]
enum Foo{Bar}

fn main(){
    let a = Foo::Bar;
    if let Foo::Bar = a{
        println!("{:?}",a);
    }
}


fn main() {
    let mut u = Some(3);
    loop {
        match u {
            Some(i) =>
                if i>=19{
                    println!("Some的盒子里装着的数值是大于8的整数。");
                    break
                }else {
                    println!("没什么事情发生，一切顺利。");
                    u = Some(i+3);
                },
            None => break,
            _=>break,
        }
    }
}


fn main() {
    let mut u = Some(19);
    loop {
        match u {
            Some(i)=>{
                if i >=29{
                    println!("u已经大于29了。");
                    // u=Some(i+1);
                }else {
                    println!("");
                    break
                }
            }
            _=> println!{"OK"}
        }
    }
    println!("最终，u={}",u.expect("未能成功！"));
}


fn main(){
    let mut n = Some(9);
    loop {
        match n {
            Some(i) => { println!("{}",n.expect("Err!"));
                if i>=19 {
                    println!("i大于19！");
                    break
                }
                else {
                    n = Some(i+1);
                }
            },

            None =>{ println!("None");
            },

            _=>{ println!("0x0x0x");
            },
        }
    }
}

fn main(){
    let mut n = 0;
    loop {
        match n {
            0 => {
            println!("0");
            n+=1;
            if n == 10 {
                break;
            }
        }
    }
}

    println!("{}",n);
}

fn main(){
    let mut n = 2;

    let u =|m: i32| m+n;
    println!("{}",u(3));
}


fn main(){
    let mut u = 23;
    fn fuh(&x:i32)->i32{

    }
    let i = fuh(3);
    println!("{}",i);
}




struct Point{
    x: f64,
    y: f64,
}

impl Point{
    fn new(x: f64, y: f64) -> Point{
        Point{x: x, y: y}
    }

    fn original() -> Point{
        Point{x: 0.0, y: 0.0}
    }


}


struct Rectangle{
    p1:Point,
    p2:Point,
}

impl Rectangle{
    fn area(&self) -> f64{
        let Point{x: x1, y: y1} = &self.p1;
        let Point{x: x2, y: y2} = &self.p2;
        ((x1-x2)*(y1-y2)).abs()
    }

    fn perimeter(&self) -> f64{
        let Point{x: x1, y: y1} = &self.p1;
        let Point{x: x2, y: y2} = &self.p2;
        2.0*((x1-x2).abs()+(y1-y2).abs())
    }

    fn translate(&mut self, x: f64, y: f64){
        self.p1.x += x;
        self.p2.x += x+3.0;
        self.p1.y += y+4.0;
        self.p2.y += y+9.0;
    }
}
#[derive(Debug)]
struct Pair(Box<i32>, Box<i32>);
impl Pair{
    fn destroy(self) {
        let Pair(first, second) = self;
        println!("first: {}, second: {}", first, second);
    }
}

fn main() {
    let mut rectangle01 = Rectangle{
        p1:Point::original(),
        p2:Point{x: 9.0, y: 6.0},
    };
    println!("Rectangle area: {}", rectangle01.area());
    println!("Rectangle perimeter: {}", rectangle01.perimeter());
    let mut new_rectangle = Rectangle {
        p1:Point::new(rectangle01.p1.x,rectangle01.p1.y),
        p2:Point::new(rectangle01.p2.x,rectangle01.p2.y),
    };
    new_rectangle.translate(5.0,4.5);
    println!("New Rectangle area: {}", new_rectangle.area());
    println!("New Rectangle perimeter: {}", new_rectangle.perimeter());
    let pair01 = Pair(Box::new(12),Box::new(6));
    pair01.destroy();

}


fn main() {
    use std::mem;

    let color = String::from("red");
    let print = ||println!("{}", color);
    print();
    // let _redborrow=&color;
    // print();
    let mut count = 0;
    let mut inc = ||{
        loop{
            if count<=18{
                count+=1;
                println!("'count':{}", count);
            }else {
                break
            }
        }
        println!("再一次打印'count':{}",count);
    };
    inc();
}




 //这是Fn、FnMut、FnOnce三种闭包，fn（）可以自动转换成Fn（）。
fn apply<T>(f:T)where
    T:FnOnce(){
    f()
}

fn call_me<F:Fn()>(f:F){
    f()
}

fn function(){
    println!("函数内部");
}

fn main(){
    let closure = || println!("闭包内部");
    call_me(function);
    call_me(closure);
}
 */

mod my_mod_01{

    //struct一个OpenBox
    pub struct OpenBox<T>{
        pub content: T,
    }

    //struct一个CloseBox
    pub struct CloseBox<T>{
        content: T,
    }

    impl<T> CloseBox<T>{
        pub fn new(c: T)->Self{
            CloseBox{
                content:c,
            }
        }
    }
}

fn main() {
    let mut openbox_01 =my_mod_01::OpenBox{
        content:"this is openbox NO.01!".to_string(),
    };

    let mut closebox_01 = my_mod_01::CloseBox::new(openbox_01);
    println!("{}", closebox_01.content);
}




