#![allow(dead_code)]
extern crate phrases;
mod sh;
mod pm;
mod traits;
use std::mem;
use phrases::greetings::english;

const MEANING_OF_LIFE: u8 = 42;

fn scope_and_shadowing() {
    let a: i8 = 123;

    {
        let a = 1;
        println!("a is {}", a);
    }
    println!("a is {}", a);
}

fn operators() {
    let mut b = 2 + 3 * 4;
    println!("b {}", b);
    b -= 2;
    println!("b {}", b);
}

fn arrays() {
    let mut a: [i32; 5] = [1, 2, 3, 4, 5];
    a = [1, 2, 3, 4, 5];
    println!("{:#?}", a);

    let b = [1u16, 10];
    for i in 0..b.len() {
        println!("{}", b[i])
    }

    println!("b size {} bytes", mem::size_of_val(&b)); //u16
}

fn vectors() {
    let mut c = Vec::new();
    c.push(1);
    c.push(44);

    let idx: usize = 0;
    println!("c = {}", c[idx]);

    match c.get(6) {
        Some(x) => println!("a[6] = {}", x),
        None => println!("error, no such element"),
    }

    for i in &c {
        println!("{}", i);
    }
}


fn strings() {
    let s:&'static str = "hello there!"; // &str = string slice
    
    for c in s.chars().rev() {
        println!("{}", c);
    }

    if let Some(first_char) = s.chars().nth(0) {
        println!("first char is {}", first_char);
    }

    // heap
    // string
    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8) {
        letters.push(a as char);
        letters.push_str(", ");
        a += 1;
    }

    println!("{}", letters);
}

fn sum_and_product(x:i32, y:i32) -> (i32, i32){
    (x + y, x * y)
}
fn tuples(){
    let x = 3;
    let y = 4;
    let sp = sum_and_product(x, y);
    println!("sp = {:?}", sp);
    println!("{0} + {1} = {2}, {0} * {1} = {3}", x, y, sp.0, sp.1);

    // destructuring
    let (a, b) = sp;
    println!("a = {}, b = {}", a, b);
}

struct Point<T> {
    x: T,
    y: T,
}

struct Line{
    start: Point<f64>,
    end: Point<f64>,
}

impl Line {
    fn len(&self) -> f64 {
        let dx = self.start.x - self.end.x;
        let dy = self.start.y - self.end.y;
        (dx*dx+dy*dy).sqrt()
    }
}

fn generics(){
    let a: Point<i32> = Point{x: 0, y: 0};
    let b: Point<f64> = Point{x: 1.2, y: 3.4};
}

fn methods(){
    let p = Point{x: 3.0, y: 4.0};
    let p2 = Point{x: 5.0, y: 10.0};
    let my_line = Line{ start: p, end: p2};
    println!("length = {}", my_line.len());
}

fn say_hello() { println!("Hello") }
fn closures(){
    let say = say_hello;
    say();

    let plus_one = |x:i32| -> i32 { x + 1 };
    let a = 6;
    println!("{} + 1 = {}", a, plus_one(a));

    let plus_two = |x:i32| {
        let mut z = x;
        z += 2;
        z
    };
    println!("{} + 2 = {}", 3, plus_two(3));

    let plus_three = |x:&mut i32| *x += 3;

    let mut f = 12;
    plus_three(&mut f);
    println!("12 + 3 = {}", f);
}

fn higher_order_functions(){
    let limit = 500;
    let is_even = |x| x % 2 == 0;

    let sum = (0..).map(|x| x*x)
    .take_while(|&x| x < limit)
    .filter(|x| is_even(*x))
    .fold(0, |sum, x| sum+x);

    println!("sum = {}", sum);
}
// fn is_even(x: u32) -> bool { x % 2 == 0 }



fn main() {
    // println!("Hello, world! {}", MEANING_OF_LIFE);
    // scope_and_shadowing();
    // operators();
    // arrays();
    // vectors();
    // strings();
    // tuples();
    // sh::stack_and_heap();
    // pm::pattern_matching();
    // generics();
    // methods();
    // closures();
    // higher_order_functions();
    // traits::run();

    println!("English: {}, {}",
        english::hello(),
        english::goodbye()
    );
}
