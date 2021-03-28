#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

fn how_many(x: i32) -> &'static str
{
    match x
    {
        0 => "no",
        _ if (x%2 == 0) => "an even number of",
        9..=12 => "lots of",
        _ => "A few"
    }
}
pub fn pattern_matching()
{
    for i in 0..13
    {
        println!("{} I have {} oranges", i, how_many(i));
    }

    let point = (0,5);

    match point
    {
        (0,0) => println!("Origin"),
        (0, y) => println!("x axis, y is {}", y),
        (x, 0) => println!("y axis, x is {}", x),
        (x, y) => println!("x = {}, y = {}", x, y)
        // (_, y) => println!("(?,{})", y),
    }
}

struct Pnt<T,V>
{
    a: T,
    b: V
}

pub fn generics()
{
    let a = Pnt{ a: 3, b: 9 };
    let b:Pnt<u8,f32> = Pnt { a: 9, b: 2.8 };
}