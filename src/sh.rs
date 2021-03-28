#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::mem;

struct Point
{
    x: f64,
    y: f64
}

fn create_point() -> Point
{
    Point{x: 0.0, y: 0.0}
}

pub fn stack_and_heap()
{
    let point1 = create_point();
    let point2 = Box::new(create_point());
    println!("Point1 takes {} bytes of space", mem::size_of_val(&point1));
    println!("Point2 takes {} bytes of space", mem::size_of_val(&point2));
    
    let point3 = *point2;

    println!("Point3 takes {} bytes of space", mem::size_of_val(&point3));

}

pub fn looper()
{
    let mut x = 0;
    loop
    {
        x += 1;

        if x > 10 
        {
            break;
        }

        println!("Lopping {}", x);
    }
}

pub fn for_loop()
{
    let mut x = 0;
    
    for _i in 0..10
    {
        println!("x is {}", x);
        x += 1;
    }

    for (pos, y) in (30..41).enumerate()
    {
        println!("y is in position {} and it's value is {}", pos, y);
    }
}

pub fn matcher()
{
    let country_code = 234;

    let country = match country_code
    {
        44 => "Uk",
        234 => "Nigeria",
        46 => "Sweden",
        1..=999 => "Unknown",
        _ => "Invalid"
    };

    println!("Country is {}", country);
}