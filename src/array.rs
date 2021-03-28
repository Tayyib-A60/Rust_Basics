#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::mem;

pub fn arrays()
{
    let arr = [1u8; 10];
    println!("{:?}", arr);
    println!("Size of arr is {}", mem::size_of_val(&arr));

    let mtx_arr:[[u8;3];2] = [
        [2,3,4],
        [4,5,9]
    ];

    println!("{:?}", mtx_arr);
}

fn use_slice(slice: &mut [i8])
{
    println!("{:?}", slice);
    let res = slice.get(0);

    match res 
    {
        Some(_x) => slice[0] = 98,
        None => ()
    }

}

pub fn slice()
{
    let mut data:[i8;5] = [1,2,3,4,5];

    use_slice(&mut data[1..4]);

    println!("{:?}", data);
}

pub fn vectors()
{
    let mut a = Vec::new();
    a.push(6);
    a.push(9);
    a.push(12);

    println!("{:?}", a);
    
    a.push(44);
    
    println!("{:?}", a);

    let idx:usize = 3;

    a[idx] = 982;

    let res = a.get(2);

    match res 
    {
        Some(val) => println!("{}", val),
        None => ()
    }

    // for x in &a { println!("{}", x);}

    while let Some(x) = a.pop()
    {
        println!("{}", x);
    }

}