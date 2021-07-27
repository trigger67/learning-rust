use std::io; // from the standard library
use rand::Rng; // trait to generate random numbers
use std::cmp::Ordering;

fn main() {
    // Println is a macro (see the !)
    println!("Yo, type a number");

    let _foo = 50; // immutable variable
    let mut user_input; // mutable variable
    user_input = String::new(); // :: for associated function (=static method)

    let random_number = rand::thread_rng().gen_range(1..101);

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    let user_input: u32 = user_input.trim().parse().expect("Please type a number!");

    println!("You wrote: {}, random is {}", user_input, random_number);

    match user_input.cmp(&random_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }

    // Shadowing unmutable variables
    let _spaces = "   ";
    let _spaces = _spaces.len();
    
    data_types(4);
}

fn data_types(_param_one: i32) {
    // Scalar types
    let _integer: u32 = 2;
    let _float = 2.0;
    let _product = 3 * 42;
    let _f: bool = false;
    let _c = 'Z';
    let _heart_eyed_cat = '😻';

    //// Compound types
    // The tuple type
    let _tup: (i32, f64, u8) = (500, 6.4, 1); 
    let _tup = (500, 6.4, 1);
    let (x, y, _z) = _tup; // Destructuring the tuple
    println!("The value of y is: {}", y);
    let _five_hundred = x*_tup.0; //First element of tup

    // The array type
    let _a = [1, 2, 3, 4, 5]; // An array, always has a fixed length (vectors do not)
    let _a: [i32; 5] = [1, 2, 3, 4, 5];
    let _a = [3; 5]; // [3, 3, 3, 3, 3]
    let _first = _a[0];
}

// This function is a statement
fn _statements_and_expression()
{
    // This is a statement
    let _y = 6;
    // let x = (let y * 6) // Do not compile, the statement do not return a value

    // 1 + 2 is an expression
    let _a = 1 + 2;

    // Another expression
    let _b = {
        let x = 3;
        x + 1 // Expressions do not include semicolons (turns it into a statement)
    };
}

fn _five() -> i32 {
    5
}

fn _control_flow() {
    let number = 3;

    if number < 5 {
        println!("condition 1");
    } else if number % 3 == 0 {
        println!("condition 2");
    } else {
        println!("condition 3");
    }

    let _number = if true { 5 } else { 6 };
}

fn _loops () {
    // Loop
    loop {
        println!("again!");
        break;
    }

    let mut counter = 0;

    let _result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    // While
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    // For
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
}

// Exercise ideas
    //Convert temperatures between Fahrenheit and Celsius.
    //Generate the nth Fibonacci number.
    //Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.


    