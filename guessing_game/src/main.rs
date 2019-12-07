use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    let secret_number = rand::thread_rng().gen_range(1,101);
    println!("随机生成:{}",secret_number);

    loop {
        let mut guess = String::new();
        println!("请输入你猜的数字");
        io::stdin().read_line(&mut guess)
            .expect("失败了");

        // 字符串转换为数字
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };   
                       
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小"),
            Ordering::Greater => println!("太大"),
            Ordering::Equal => {
                println!("你赢了");
                break;
            }
        }
    }       
                    
    
    
        

    //println!("你的游戏：{}",guess);
}
