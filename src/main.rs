#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

extern crate rand;
extern crate phrases;

mod sh;
mod ds;
mod array;
mod strings;
mod pm;
mod fns;
mod closures;
mod traits;

use phrases::greetings::french;

fn main() {
    // let btw = 1 | 2;

    // println!("Pi is {}", std::f64::consts::PI);
    // sh::stack_and_heap();
    // sh::looper();
    // sh::for_loop();
    // sh::matcher();
    // ds::run();
    // ds::option();
    // array::vectors();
    // array::slice();

    // strings::str_ops();
    // strings::tuple_ops();

    // pm::pattern_matching();

    // fns::functions();

    // closures::closures();
    // closures::higher_ord_fns();

    // traits::traits();

    // traits::borrowing();
    crates();
}

trait Trait {
    fn f(&self);
}

impl Trait for i32 
{
    fn f(&self) {
        println!("1");
    }
}

struct Guard;

impl Drop for Guard
{
    fn drop(&mut self)
    {
        println!("1");
    }
}

struct S 
{
    f: fn()
}

impl S{
    fn f(&self)
    {
        println!("1");
    }
}

fn crates()
{
    let a: = [3;5];
    let mut three = String::from("example");

    let s1 = three;
    println!("{}", s1);

    three = "sjsh".to_string();

    println!("{}", three);
    // let st = String::new();

    // let print2 = || println!("2");

    // S{ f: print2 }.f();

    // let g = Guard;
    // println!("3");

    // let _ = Guard;
    // println!("2");
    // const y:i32 = 12;
    // let x = &0;

    // x.f();

    // let mut rng = rand::thread_rng();

    // // let b:bool = rng.gen();

    // println!("English: {}", phrases::greetings::english::hello());
    // println!("French: {}", french::hello());
}
