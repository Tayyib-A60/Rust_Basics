#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

pub fn str_ops()
{
    let msg:&'static str = "Hello there!";

    for _c in msg.chars().rev()
    {
        // println!("{}", c);
    }

    if let Some(first_char) = msg.chars().nth(0)
    {
        println!("First char is {}", first_char);
    }

    let mut letters = String::new();
    let mut na = String::new();
    na.push_str("hello");
    let mut a = 'a' as u8;

    while a <= ('z' as u8) {
        letters.push(a as char);
        letters.push_str(",");

        a += 1;
    }

    letters.pop();

    println!("Letters: {:?}", letters);

    // &str <> String
    // let conv:&str = &letters;

    // Concatenation
    // let z = letters + "abc";

    // let z = letters + &na;

    let mut hello = String::from("hello");
    // let mut hello2 = "hello".to_string();
    hello.remove(0);
    hello.push_str("@(28");

    println!("{}", hello);
}

fn sum_and_prod(x:i32, y:i32) -> (i32, i32)
{
    (x+y, x*y)
}


pub fn tuple_ops()
{
    let x = 3;
    let y = 4;
    let sp = sum_and_prod(x,y);

    println!("Sum and prod of {} and {} is {} and {} ", x, y, sp.0, sp.1);

    let (sum, prod) = sp;

    println!("Sum is {} and prod  is {}", sum, prod);

    let sp2 = sum_and_prod(2,32);

    let combined = (sp, sp2);

    let ((c,d), (e,f)) = combined;
}