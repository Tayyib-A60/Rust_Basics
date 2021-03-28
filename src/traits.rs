#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]


trait Animal
{
    fn create(name: &'static str) -> Self;

    fn name(&self) -> &'static str;

    fn talk(&self)
    {
        println!("{} cannot talk", self.name());
    }
}

trait Summable<T>
{
    fn sum(&self) -> T;
}

struct Human
{
    name: &'static str
}

struct Cat
{
    name: &'static str
}

impl Animal for Human
{
    fn create(name: &'static str) -> Human
    {
        Human { name: name }
    }

    fn name(&self) -> &'static str
    {
        self.name
    }

    fn talk(&self)
    {
        println!("{} says hi", self.name());
    }
}

impl Animal for Cat
{
    fn create(name: &'static str) -> Cat
    {
        Cat { name: name }
    }

    fn name(&self) -> &'static str
    {
        self.name
    }

    fn talk(&self)
    {
        println!("{} says meow", self.name());
    }
}

impl Summable<i32> for Vec<i32>
{
    fn sum(&self) -> i32
    {
        let mut vec_sum:i32 = 0;
        for i in self
        {
            vec_sum += i;
        }

        return vec_sum;
    }

}

pub fn traits()
{
    // let me = Human { name: "Toyeeb" };
    let me = Human::create("Toyeeb");
    me.talk();

    let mia:Cat = Animal::create("Mia");
    mia.talk();

    let a = vec![2,3,5];
    // a.sum();
    println!("Sum of a is {}", a.sum());

    let b = a;
}

pub fn borrowing()
{
    let print_vector = |x:&Vec<i32>| //-> Vec<i32>
    {
        println!("{:?}", x);
        // x
    };

    let vec1 = vec![3,2,4];
    print_vector(&vec1);

    let mut a = 12;

    let b = &mut a;
    *b += 2;

    println!("A is now {}", b);
    println!("A is {}", a);

    a = 19;

    println!("a is now {}", a);

    let mut z = vec![4,6,7];

    for i in &z
    {
        println!("i is {}", i);
        // z.push(9);
    }
}