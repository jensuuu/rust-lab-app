use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Exercise {
    pub prompt: &'static str,
    pub starter: &'static str,
    pub expected_output: &'static str,
}

#[derive(Clone, Serialize)]
pub struct Lesson {
    pub id: u32,
    pub title: &'static str,
    pub summary: &'static str,
    pub paragraphs: &'static [&'static str],
    pub example: &'static str,
    pub exercise: Exercise,
}

pub fn all() -> &'static [Lesson] {
    LESSONS
}

pub fn find(id: u32) -> Option<&'static Lesson> {
    LESSONS.iter().find(|lesson| lesson.id == id)
}

static LESSONS: &[Lesson] = &[
    Lesson {
        id: 1,
        title: "Hello, World",
        summary: "Write and run your first Rust program.",
        paragraphs: &[
            "Every Rust program starts running at a function called `main`.",
            "`println!` prints text followed by a new line. The exclamation mark tells you it is a macro and not a regular function.",
            "Text in double quotes is called a string literal. Statements end with a semicolon.",
        ],
        example: r#"fn main() {
    println!("Hello, Rust!");
}
"#,
        exercise: Exercise {
            prompt: "Print two lines: Hello, Rustacean! and then Learning Rust is fun.",
            starter: r#"fn main() {
}
"#,
            expected_output: "Hello, Rustacean!\nLearning Rust is fun.",
        },
    },
    Lesson {
        id: 2,
        title: "Variables and Mutability",
        summary: "Store values with let and change them with mut.",
        paragraphs: &[
            "Variables are created with `let`. By default they are immutable, which means their value cannot change after it is set.",
            "Add `mut` to make a variable mutable. Rust makes you opt in to change so that you always know which values can move.",
            "Shadowing lets you reuse a name by writing `let` again. The new variable hides the old one and can even have a different type.",
            "Inside `println!`, curly braces insert a variable: `{name}` prints the value of `name`.",
        ],
        example: r#"fn main() {
    let x = 5;
    let mut y = 10;
    y += x;
    println!("x = {x}, y = {y}");

    let x = x * 2;
    println!("shadowed x = {x}");
}
"#,
        exercise: Exercise {
            prompt: "Make counter mutable and add 1 to it three times so the program prints Counter: 3.",
            starter: r#"fn main() {
    let counter = 0;
    println!("Counter: {counter}");
}
"#,
            expected_output: "Counter: 3",
        },
    },
    Lesson {
        id: 3,
        title: "Data Types",
        summary: "Integers, floats, booleans, characters, tuples and arrays.",
        paragraphs: &[
            "Rust is statically typed: every value has a type known at compile time. The compiler can usually infer it, and you can write it yourself after a colon, like `let age: u32 = 30;`.",
            "Integers come in signed (`i8` to `i128`) and unsigned (`u8` to `u128`) sizes. Floats are `f32` and `f64`. Other basic types are `bool` and `char`.",
            "A tuple groups values of different types, and you read its parts with `.0`, `.1` and so on. An array holds several values of one type and has a fixed length.",
        ],
        example: r#"fn main() {
    let age: u32 = 30;
    let price = 9.99_f64;
    let is_ready = true;
    let letter = 'R';

    let point = (3, 7);
    let scores = [10, 20, 30];

    println!("{age} {price} {is_ready} {letter}");
    println!("x = {}, y = {}", point.0, point.1);
    println!("first score = {}", scores[0]);
}
"#,
        exercise: Exercise {
            prompt: "Multiply the two numbers inside dimensions and print the result as Area: 42.",
            starter: r#"fn main() {
    let dimensions: (u32, u32) = (6, 7);
}
"#,
            expected_output: "Area: 42",
        },
    },
    Lesson {
        id: 4,
        title: "Functions",
        summary: "Group code into reusable pieces that take and return values.",
        paragraphs: &[
            "Functions are declared with `fn`. Parameters need explicit types, and the return type comes after an arrow `->`.",
            "The last expression in a function is its return value. Leave off the semicolon on that line, because adding one turns the expression into a statement that returns nothing.",
        ],
        example: r#"fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let total = add(2, 3);
    println!("2 + 3 = {total}");
}
"#,
        exercise: Exercise {
            prompt: "Write a function named square that takes an i32 and returns it multiplied by itself. The program should print 81.",
            starter: r#"fn main() {
    println!("{}", square(9));
}
"#,
            expected_output: "81",
        },
    },
    Lesson {
        id: 5,
        title: "Control Flow",
        summary: "Make decisions with if and repeat work with loops.",
        paragraphs: &[
            "`if` checks a condition that must be a `bool`. Because `if` is an expression, it can produce a value.",
            "Rust has three loops: `loop` repeats forever until you `break`, `while` repeats while a condition holds, and `for` walks through a range or collection.",
            "The range `1..=5` includes both ends, while `1..5` stops before 5.",
        ],
        example: r#"fn main() {
    let number = 7;
    let kind = if number % 2 == 0 { "even" } else { "odd" };
    println!("{number} is {kind}");

    for step in 1..=3 {
        println!("step {step}");
    }
}
"#,
        exercise: Exercise {
            prompt: "For every number from 1 to 5, print a line such as 1 is odd or 2 is even.",
            starter: r#"fn main() {
}
"#,
            expected_output: "1 is odd\n2 is even\n3 is odd\n4 is even\n5 is odd",
        },
    },
    Lesson {
        id: 6,
        title: "Ownership",
        summary: "The idea that makes Rust safe without a garbage collector.",
        paragraphs: &[
            "Every value in Rust has exactly one owner. When the owner goes out of scope, the value is dropped and its memory is freed automatically.",
            "Assigning a heap value such as a `String` to another variable moves ownership. The old variable can no longer be used.",
            "If you need two independent copies, call `.clone()`. Simple values like integers implement `Copy`, so they are duplicated automatically instead of moved.",
        ],
        example: r#"fn main() {
    let original = String::from("Rust");
    let moved = original;
    println!("{moved}");

    let copy = moved.clone();
    println!("{moved} and {copy}");
}
"#,
        exercise: Exercise {
            prompt: "This program does not compile because first is moved. Fix it so it prints Rust Rust.",
            starter: r#"fn main() {
    let first = String::from("Rust");
    let second = first;
    println!("{first} {second}");
}
"#,
            expected_output: "Rust Rust",
        },
    },
    Lesson {
        id: 7,
        title: "References and Borrowing",
        summary: "Use a value without taking ownership of it.",
        paragraphs: &[
            "A reference, written `&value`, lets a function look at a value without owning it. This is called borrowing.",
            "A mutable reference, written `&mut value`, lets the function change the value. You may have many shared references or exactly one mutable reference at a time, never both.",
            "This rule lets the compiler prevent data races and dangling pointers before your program ever runs.",
        ],
        example: r#"fn length(text: &String) -> usize {
    text.len()
}

fn shout(text: &mut String) {
    text.push_str("!!!");
}

fn main() {
    let mut message = String::from("hello");
    println!("length: {}", length(&message));
    shout(&mut message);
    println!("{message}");
}
"#,
        exercise: Exercise {
            prompt: "Write the function add_exclamation that appends a ! to a String it borrows mutably. The program should print Hello!.",
            starter: r#"fn main() {
    let mut greeting = String::from("Hello");
    add_exclamation(&mut greeting);
    println!("{greeting}");
}
"#,
            expected_output: "Hello!",
        },
    },
    Lesson {
        id: 8,
        title: "Structs and Methods",
        summary: "Create your own types and attach behavior to them.",
        paragraphs: &[
            "A struct groups related named fields into one type.",
            "Methods live inside an `impl` block. The first parameter `&self` gives the method access to the value it is called on.",
            "Functions in an `impl` block without `self`, such as `new`, are called associated functions and are used with `Type::name()`.",
        ],
        example: r#"struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rectangle = Rectangle { width: 3, height: 4 };
    println!("area: {}", rectangle.area());
}
"#,
        exercise: Exercise {
            prompt: "Add a perimeter method to Rectangle. For a 3 by 4 rectangle the program should print 14.",
            starter: r#"struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rectangle = Rectangle { width: 3, height: 4 };
    println!("{}", rectangle.perimeter());
}
"#,
            expected_output: "14",
        },
    },
    Lesson {
        id: 9,
        title: "Enums and Match",
        summary: "Model choices and handle every case with pattern matching.",
        paragraphs: &[
            "An enum is a type that can be exactly one of several variants, and variants can carry data.",
            "`match` compares a value against patterns. The compiler checks that you covered every possibility, so you cannot forget a case.",
            "Rust has no null. Instead `Option<T>` is either `Some(value)` or `None`, which forces you to handle missing values.",
        ],
        example: r#"enum Direction {
    North,
    South,
}

fn name(direction: Direction) -> &'static str {
    match direction {
        Direction::North => "north",
        Direction::South => "south",
    }
}

fn main() {
    println!("{}", name(Direction::North));

    let maybe: Option<i32> = Some(5);
    match maybe {
        Some(value) => println!("got {value}"),
        None => println!("nothing"),
    }
}
"#,
        exercise: Exercise {
            prompt: "Write describe(n: i32) -> &'static str using match so it returns negative, zero or positive. Print the result for -4, 0 and 12, one per line.",
            starter: r#"fn main() {
}
"#,
            expected_output: "negative\nzero\npositive",
        },
    },
    Lesson {
        id: 10,
        title: "Vectors and Iterators",
        summary: "Store many values and transform them with iterator chains.",
        paragraphs: &[
            "A `Vec<T>` is a growable list. Create one with `vec![1, 2, 3]` and add items with `push`.",
            "Iterators process collections step by step. `iter()` walks the items, `map` transforms each one, `filter` keeps some of them, and `sum` or `collect` finish the chain.",
            "Closures are small anonymous functions written like `|x| x * 2`.",
        ],
        example: r#"fn main() {
    let mut numbers = vec![1, 2, 3];
    numbers.push(4);

    let doubled: Vec<i32> = numbers.iter().map(|n| n * 2).collect();
    let total: i32 = doubled.iter().sum();

    println!("{doubled:?}");
    println!("total: {total}");
}
"#,
        exercise: Exercise {
            prompt: "Use an iterator chain to add up the squares of the numbers. The program should print Sum of squares: 55.",
            starter: r#"fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let total = 0;
    println!("Sum of squares: {total}");
}
"#,
            expected_output: "Sum of squares: 55",
        },
    },
    Lesson {
        id: 11,
        title: "Error Handling",
        summary: "Deal with failure using Result and the ? operator.",
        paragraphs: &[
            "Operations that can fail return `Result<T, E>`: either `Ok(value)` when things worked or `Err(error)` when they did not.",
            "Use `match` to handle both outcomes. The `?` operator is a shortcut that returns the error to the caller immediately, or unwraps the success value.",
            "Avoid `unwrap()` in real programs, because it crashes when the result is an error.",
        ],
        example: r#"use std::num::ParseIntError;

fn double(text: &str) -> Result<i32, ParseIntError> {
    let number: i32 = text.parse()?;
    Ok(number * 2)
}

fn main() {
    println!("{:?}", double("21"));
    println!("{:?}", double("abc"));
}
"#,
        exercise: Exercise {
            prompt: "Write parse_age(text: &str) -> Result<u32, std::num::ParseIntError>. The program should print Age: 42 and then Invalid age.",
            starter: r#"fn main() {
    for text in ["42", "abc"] {
        match parse_age(text) {
            Ok(age) => println!("Age: {age}"),
            Err(_) => println!("Invalid age"),
        }
    }
}
"#,
            expected_output: "Age: 42\nInvalid age",
        },
    },
    Lesson {
        id: 12,
        title: "Traits",
        summary: "Share behavior between different types.",
        paragraphs: &[
            "A trait describes behavior that types can implement, similar to an interface in other languages.",
            "You implement a trait for a type with `impl Trait for Type`. Any code can then use the shared method on every type that implements it.",
            "Traits are also how generics work: a function can accept any type as long as it implements the traits you require.",
        ],
        example: r#"trait Speak {
    fn speak(&self) -> String;
}

struct Dog;

impl Speak for Dog {
    fn speak(&self) -> String {
        String::from("Woof")
    }
}

fn main() {
    println!("{}", Dog.speak());
}
"#,
        exercise: Exercise {
            prompt: "Implement Speak for Cat so it returns Meow. The program should print Woof and then Meow.",
            starter: r#"trait Speak {
    fn speak(&self) -> String;
}

struct Dog;
struct Cat;

impl Speak for Dog {
    fn speak(&self) -> String {
        String::from("Woof")
    }
}

fn main() {
    println!("{}", Dog.speak());
    println!("{}", Cat.speak());
}
"#,
            expected_output: "Woof\nMeow",
        },
    },
];
