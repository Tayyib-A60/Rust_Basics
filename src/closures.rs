#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

pub fn closures()
{
    let plus_one = |x:i32| -> i32 { x + 1 };

    let a = 6;
    println!("{}", plus_one(a));

    let mut two = 2;
    {
        let plus_two = |x| 
        {
            let mut z = x;
            z += two;
            z
        };
    
        println!("{}", plus_two(9));
    }

    let borrow_two = &mut two;

    println!("{}", borrow_two);
}

fn is_even(x: u32) -> bool
{
    x % 2 == 0
}


pub fn higher_ord_fns()
{
    let limit = 500;

    let mut sum = 0;

    for i in 0..
    {
        let isq = i * i;

        if is_even(isq)
        {
            sum += isq;
        } 
        else if isq > limit 
        {
            break;
        }
    }

    println!("Sum is {}", sum);

    let sum2 = (0..).map(|x| x*x).take_while(|&x| x < 500).filter(|x| is_even(*x)).fold(0, |sum,x| sum + x);

    println!("Sum2 is {}", sum2);
}

