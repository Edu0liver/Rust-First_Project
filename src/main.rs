#![allow(unused)]
#![allow(non_snake_case)]

use std::io;
use rand::Rng;
use std::io::{Write, ErrorKind, BufRead, BufReader};
use std::fs::File;
use std::cmp::Ordering;

fn main()
{
    dataNumberTypes()
}

fn nameQuestion()
{
    println!("What is your name? ");
    let mut name = String::new();
    let greeting = "Nice to meet you!";
    io::stdin().read_line(&mut name)
    .expect("Didn't receive input");
    
    println!("Hello {}!, {}", name.trim_end(), greeting);
}

fn changeVariableType()
{
    const ONE_MILLION: u32 = 1_000_000;
    const PI: f32 = 3.141592;
    let age = "19";
    let mut age: u32 = age.trim().parse()
    .expect("Age wasn't assigned a number!");

    age = age + 1;

    println!("I'm {} and I have ${}", age, ONE_MILLION);
}

fn dataNumberTypes()
{
    println!("Max u32: {}", u32::MAX);
    println!("Max u64: {}", u64::MAX);
    println!("Max usize: {}", usize::MAX);
    println!("Max u128: {}", u128::MAX);
    println!("Max f32: {}", f32::MAX);
    println!("Max f64: {}", f64::MAX);
}