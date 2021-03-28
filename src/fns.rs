#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

pub fn functions()
{
    let mut x = 10;

    increase(&mut x);

    println!("{}", x);

    let line: Line = Line { start: Point{ x: 3f64, y: 43f64 }, end: Point { x: 33f64, y: 35f64 } };
    let length = line.len();
    println!("Length of line is {}", length);
}

struct Point 
{
    x: f64,
    y: f64
}

struct Line 
{
    start: Point,
    end: Point
}

impl Line 
{
    fn len(&self) -> f64
    {
        let dx = self.start.x - self.end.x;
        let dy = self.start.y - self.end.y;
        (dx*dx + dy*dy).sqrt()
    }
}

fn increase(x: &mut i32) 
{
    *x += *x;
}