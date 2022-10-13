#![allow(unused)]
#![allow(non_snake_case)]

use std::io;
use rand::Rng;
use std::io::{Write, ErrorKind, BufRead, BufReader};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    arrayWhile()
}

fn nameQuestion() {
    println!("What is your name? ");
    let mut name = String::new();
    let greeting = "Nice to meet you!";
    io::stdin().read_line(&mut name)
    .expect("Didn't receive input");
    
    println!("Hello {}!, {}", name.trim_end(), greeting);
}

fn changeVariableType() {
    const ONE_MILLION: u32 = 1_000_000;
    const PI: f32 = 3.141592;
    let age = "19";
    let mut age: u32 = age.trim().parse()
    .expect("Age wasn't assigned a number!");

    age = age + 1;

    println!("I'm {} and I have ${}", age, ONE_MILLION);
}

fn dataNumberTypes() {
    println!("Max u32: {}", u32::MAX);
    println!("Max u64: {}", u64::MAX);
    println!("Max usize: {}", usize::MAX);
    println!("Max u128: {}", u128::MAX);
    println!("Max f32: {}", f32::MAX);
    println!("Max f64: {}", f64::MAX);
}

fn randomNum() {
    let randomNum = rand::thread_rng().gen_range(1..101);

    println!("Random: {}", randomNum);
}

fn conditionals() {
    let mut input = String::new();

    println!("What is your age?");
    io::stdin().read_line(&mut input)
    .expect("Didn't receive input");

    let age: usize = input.trim().parse()
    .expect("Age wasn't assigned a number!");

    if (age >= 1) && (age <= 18) {
        println!("Important Bithday");
    } else if (age == 21) || (age == 50) {
        println!("Important Bithday");
    } else if age >= 65 {
        println!("Important Bithday");
    } else {
        println!("Not Important Bithday");
    }
}

fn ternaryOperator() {
    let mut age = String::new();

    println!("What is your age?");
    io::stdin().read_line(&mut age)
    .expect("Didn't receive input");

    let age: usize = age.trim().parse()
    .expect("Age wasn't assigned a number!");
    
    let canVote = if age >= 16 {
        true
    } else {
        false
    };

    if canVote {
        println!("You can vote!");
    } else {
        println!("You can't vote!");
    }

}

fn matchStatement() {
    let mut input = String::new();

    println!("What is your age?");
    io::stdin().read_line(&mut input)
    .expect("Didn't receive input");

    let age: i32 = input.trim().parse()
    .expect("Age wasn't assigned a number!");

    match age {
        1..=18 => println!("Important Bithday"),
        21 | 50 => println!("Important Bithday"),
        65..=i32::MAX => println!("Important Bithday"),
        _ => println!("Not Important Bithday")
    };
}

fn matchStatementCmp() {
    let mut input = String::new();
    let votingAge = 16;

    println!("What is your age?");
    io::stdin().read_line(&mut input)
    .expect("Didn't receive input");

    let age: usize = input.trim().parse()
    .expect("Age wasn't assigned a number!");

    match age.cmp(&votingAge) {
        Ordering::Less => println!("Can't Vote"),
        Ordering::Equal => println!("You Gained the Right to Vote"),
        Ordering::Greater => println!("Can Vote"),
    };
}

fn arrayLoop() {
    let arr = [ 1, 2, 3, 4, 5, 6, 7, 8, 9 ];
    let mut loopIndex = 0;

    loop {
        if arr[loopIndex] % 2 == 0 {
            loopIndex += 1;
            continue;
        }
        
        if arr[loopIndex] == 9 {
            break;
        }

        println!("Valor: {}", arr[loopIndex]);
        loopIndex += 1;
    }
}

fn arrayWhile() {
    let arr = [ 1, 2, 3, 4, 5, 6, 7, 8, 9 ];
    let mut loopIndex = 0;

    while loopIndex < arr.len() {
        println!("Array: {}", arr[loopIndex]);
        loopIndex += 1;
    }

}

fn arrayForLoop() {
    let arr = [ 1, 2, 3, 4, 5, 6, 7, 8, 9 ];
    let mut loopIndex = 0;

    for val in arr.iter() {
        println!("Valor: {}", val);
    }
}
