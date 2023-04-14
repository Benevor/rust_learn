use std::cmp::Ordering;
use std::io;
// prelude
use rand::Rng; // trait

const MAX_POINT: u32 = 10_0000;

fn main() {
    loop_expr();
}

fn guess_num() {
    // 生成一个随机数
    let random_num = rand::thread_rng().gen_range(1..101);  // 生成随机数
    println!("random number {}", random_num);

    loop {
        // 输入一个猜测的数字
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("读取时发生了错误");   // 输入
        // shadow 字符串转为整型，u32指示了parse()的转换类型
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        // 比较两个数字并返回比较结果
        match guess.cmp(&random_num) {
            Ordering::Less => println!("less"),
            Ordering::Greater => println!("greater"),
            Ordering::Equal => {
                println!("equal and you win");
                break;
            }
        }
    }
}

fn shadow_1() {
    const MIN_POINT: u32 = 1;
    let x = 5;
    let x = x + 1;
    println!("{}", x);

    let y = 1;
    let y = "hello";
    println!("{}", y);
}

fn tuple_1() {
    let tup1: (i32, f64) = (100, 3.2);  // 长度不可变，类似地，没有mut声明的话，初始化后，tuple内容不可更新
    println!("{},{}", tup1.0, tup1.1);
    let (x, y) = tup1;
    println!("{},{}", x, y);
}

fn array_1() {
    // 数据存放在栈上面，而不是堆上面
    // 数组的长度不可变
    // 访问超过数组下标会报错，不允许越界访问
    let month: [&str; 3] = ["1", "2", "3"];
    let a = [5; 3];
}

fn para_1(x: i32, y: f64) {
    println!("{}", x);
    println!("{}", y);
}

fn expr_1() {
    // expression 与 statement，要区分清楚
    let y = {
        let x = 1;
        x + 3
    };  //  中括号的整体表示一个 expression，而不是一个 statement
    println!("{}", y);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn func_return_value() -> i32 {
    // rust中，函数返回值就是函数体中最后一个表达式的值
    // 如果要提前返回，则需要使用 return 关键字
    let tmp = plus_one(6);
    if tmp == 3 || tmp == 2 {
        return tmp - 1;
    }
    if tmp > 0 {
        tmp + 2
    } else {
        tmp + 10
    }
}

fn if_expr() {
    let condition = true;
    let number = if condition {
        println!("test");
        5
    } else { 6 }; // if 和 else 的最后的表达式的类型必须相同
}

fn loop_expr() {
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;  //  loop 表达式的值
        }
    };
    println!("{}", result);
}

fn for_loop_1() {
    let tmp = [1, 2, 3, 4, 5];
    for item in tmp.iter() {
        println!("{}", item);
    }

    for item in (1..4).rev() {
        println!("{}", item);
    }
}