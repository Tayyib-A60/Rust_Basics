#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

struct Human
{
    // name: &str,
    age: u8
}

enum Color
{
    Green,
    Red,
    Blue,
    RGBColor(u8,u8,u8),
    CYMKColor{ cyan: u8, yellow: u8, magenta: u8, black: u8 }
}

pub fn run()
{
    let toyeeb = Human { age: 25 };
    println!("I am {}", toyeeb.age);

    let c = Color::RGBColor(0,0,1);

    match c
    {
        Color::Blue => println!("Color is blue"),
        Color::Red => println!("Color is red"),
        Color::RGBColor(0,0,0) => println!("Black"),
        Color::RGBColor(r,g,b) => println!("Color is RGB({} {} {})",r,g,b),
        Color::CYMKColor{ black:255,.. } => println!("Color is black"),
        _ => println!("I no sabi")
    }
}

pub fn option()
{
    let x = 3.0;
    let y = 1.0;

    let result:Option<f64> = if y != 0.0 { Some(x/y) } else { None };

    match result 
    {
        Some(z) => println!("{}/{} is {}", x,y,z),
        None => ()
    }

    if let Some(z) = result { println!("z is {}", z); }
}