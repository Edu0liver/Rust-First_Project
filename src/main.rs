#![allow(unused)]
#![allow(non_snake_case)]

use std::io;
use rand::Rng;
use std::io::{Write, ErrorKind, BufRead, BufReader};
use std::fs::File;
use std::cmp::Ordering;
use std::ops::Add;
use std::collections::HashMap;

mod restaurant;
use crate::restaurant::orderFood;

fn main() {
    closures();
}

fn nameQuestion() {
    println!("What is your name? ");
    let mut name = String::new();
    let greeting = "Nice fto meet you!";
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

fn tuples() {
    let tuple: ( u8, String, f64 ) = ( 19, "Edu".to_string(), 50_000.00 );

    println!("Name: {}", tuple.1);
    
    let ( v1, v2, v3 ) = tuple;

    println!("Age: {}", v1);
}

fn stringsOverview() {
    let mut string = String::new();

    string.push('A');
    string.push(' ');
    string.push_str("word");

    println!("String: {}", string);

    for word in string.split_whitespace() {
        println!("Word: {}\n", word);
    }

    let string2 = string.replace("A", "Another");

    println!("String 2: {}", string2);

    let string3 = String::from("k n b v k k l m c p e");

    let mut v1: Vec<char> = string3.chars().collect();
    
    v1.sort();
    v1.dedup();

    for char in v1 {
        println!("Char: {}", char);
    }

    let string4: &str = "Random string";
    let mut string5: String = string4.to_string();

    println!("string5: {}", string5);
    
    let byte_arr = string5.as_bytes();
    let string6 = &string[0..6];
    
    println!("String Length: {}", string6.len());

    string5.clear();

    let string7 = String::from("Just some");
    let string8 = String::from(" words");
    let string9 = string7 + &string8;

    for char in string9.bytes() {
        println!("Char: {}", char);
    }

}

fn casting(){
    let int_u8: u8 = 5;
    let int2_u8: u8 = 4;
    let int3_u32: u32 = (int_u8 as u32) + (int2_u8 as u32);

    println!("int3: {}", int3_u32);
}

fn enums(){
    enum Day {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday
    }

    impl Day {
        fn is_weekend(&self) -> bool {
            match self {
                Day::Saturday | Day::Sunday => true,
                _ => false
            }
        }
    }

    let today: Day = Day::Monday;

    match today {
        Day::Monday => println!("Everyone hates Monday!"),
        Day::Tuesday => println!("Donut day!"),
        Day::Wednesday => println!("Hump day!"),
        Day::Thursday => println!("Pay day!"),
        Day::Friday => println!("Almost weekend!"),
        Day::Saturday => println!("Weekend!"),
        Day::Sunday => println!("Weekend!"),
        _ => println!("Not a day")
    }

    println!("Is today a weekend? : {}", today.is_weekend());

}

fn vectors() {
    let vec1: Vec<i32> = Vec::new();
    let mut vec2 = vec![ 1, 2, 3, 4 ];

    vec2.push(5);

    println!("1st: {}", vec2[0]);

    let second: &i32 = &vec2[1];

    match vec2.get(1) {
        Some(second) => println!("2nd: {}", second),
        None => println!("No 2nd value!")
    }

    for i in &mut vec2 {
        *i *= 2;
    }
    
    for i in &mut vec2 {
        println!("{}", i);
    }

    println!("Vec2 length: {}", vec2.len());
    println!("Vec2 value pop: {:?}", vec2.pop());

}

fn generics<T: Add<Output = T>>(x: T, y:T) -> T {
    return x + y;
}

fn ownership() {
    //Applied to Strings, Arrays and Vectors.
    let string1 = String::from("World");
    let string2 = string1;
    let string3 = string2.clone();

    println!("Hello {}, string1 borrow valor", string2);
    println!("Hello {}, string2 cloned valor", string3);
}

fn hashMaps() {
    let mut heroes = HashMap::new();

    heroes.insert("Batman", "Bruce Wayne");
    heroes.insert("The Flash", "Barry Allen");
    heroes.insert("Superman", "Clark Kent");

    for (k, v) in heroes.iter() {
        println!("{} = {}", k, v);
    }

    if heroes.contains_key(&"Batman") {
        let the_batman = heroes.get(&"Batman");

        match the_batman {
            Some(x) => println!("Batman is a hero!"),
            Nome => println!("Batman is not a hero!")
        }

    }
}

fn structs() {

    struct Customer {
        name: String,
        address: String,
        balance: f32
    }

    let mut edu = Customer {
        name: String::from("Eduardo"),
        address: String::from("91 Rua OB"),
        balance: 1432.31
    };

    edu.address = String::from("34 Rua OB");

    struct Rectangle<T, U> {
        length: T,
        height: U
    }

    let rec = Rectangle {
        length: 4,
        height: 10.5
    };

}

fn traits() {
    const PI: f32 = 3.141592;

    trait Shape {
        fn new(length: f32, width: f32) -> Self;
        fn area(&self) -> f32;
    }

    struct Rectangle {
        length: f32,
        width: f32
    }
    struct Circle {
        length: f32,
        width: f32
    }

    impl Shape for Rectangle {
        fn new(length: f32, width: f32) -> Rectangle {
            return Rectangle{ length, width }
        }
        fn area(&self) -> f32 {
            return self.length * self.width;
        }
    };
    
    impl Shape for Circle {
        fn new(length: f32, width: f32) -> Circle {
            return Circle{ length, width }
        }
        fn area(&self) -> f32 {
            return (self.length / 2.0).powf(2.0) * PI;
        }
    };

    let rec: Rectangle = Shape::new(10.0, 10.0);
    let circ: Circle = Shape::new(10.0, 10.0);

    println!("Rectangle Area: {}", rec.area());
    println!("Circle Area: {}", circ.area());

}

fn orderFoodModule() {
    orderFood();
}

fn errorHandling() {
    panic!("Terrible Error");
}

fn fileIo() {
    let path = "lines.txt";
    let output = File::create(path);

    let mut output = match output {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem creating file: {:?}", error);
        }
    };

    write!(output, "Just some\nRandom words")
    .expect("Failed to write to file!");
    
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    
    for line in buffered.lines() {
        println!("{}", line.unwrap());
    }

    let output2 = File::create("rand.txt");

    let output2 = match output2 {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("rand.txt") {
                Ok(fileCreated) => fileCreated,
                Err(error) => panic!("Can't create file: {:?}", error),
            },
            _other_error => panic!("Problem opening file: {:?}", error),
        },
    };

}

fn iterators() {
    let mut arr1 = [1, 2, 3, 4];

    for val in arr1.iter() {
        println!("{}", val);
    }

    let mut iter1 = arr1.iter();

    println!("1st: {:?}", iter1.next())
}

fn closures() {
    fn useFunc<T>(a: i32, b: i32, func: T) -> i32
    where T: Fn(i32, i32) -> i32 {
        func(a,b)
    }

    let sum = |a: i32, b: i32| a + b;
    let multi = |a: i32, b: i32| a * b;

    println!("5 + 4 = {}", useFunc(5, 4, sum));
    println!("5 * 4 = {}", useFunc(5, 4, multi));

}

fn boxConcept() {
    struct TreeNode<T> {
        pub left: Option<Box<TreeNode<T>>>,
        pub right: Option<Box<TreeNode<T>>>,
        pub key: T
    }

    impl<T> TreeNode<T> {
        pub fn new(key: T) -> Self {
            TreeNode { left: None, right: None, key }
        }

        pub fn left(mut self, node: TreeNode<T>) -> Self {
            self.left = Some(Box::new(node));
            return self;
        }

        pub fn right(mut self, node: TreeNode<T>) -> Self {
            self.right = Some(Box::new(node));
            return self;
        }
    }

    let node1 = TreeNode::new(1)
    .left(TreeNode::new(2))
    .right(TreeNode::new(3));

}
