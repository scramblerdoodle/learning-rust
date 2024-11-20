fn main() {
    variables();
    func(10);
    let x = plus_one(15);
    println!("Return from plus_one(15): {x}");
    statements_expressions();
    control_flow();
    loops();

    //// These lines won't compile because
    //// s1's reference is deallocated after the call to `ownership`
    //let s1 = String::from("Hello");
    //ownership(s1);
    //println!("{s1}");
}

//
// OWNERSHIP TEST
//
fn ownership(mut s: String) -> () {
    s.push_str(" World");
    s.push_str("!");
}

//
// FUNCTIONS
//
fn variables() {
    // Types of variables
    let _unsigned_int: u8 = 5;
    let _signed_int: i8 = -5;
    let _float_64: f64 = 10.5;

    // Mathematical operations
    let sum: u64 = 5 + 10;

    //
    // TUPLES
    //

    // Tuples; they have fixed size
    let tup: (i32, f64, u8) = (-100, 6.4, 1);

    // Destructuring tuples
    let (_x, y, _z) = tup;
    println!("The value of y is: {y}");

    // Accessing specific tuple element
    let fst_elem = tup.0;
    println!("The first element in the tuple is: {fst_elem}");

    // Mutable tuples allow for changing elements
    let mut mut_tup: (u32, u32, u32) = (0, 1, 2);
    mut_tup.0 = 2;
    let fst = mut_tup.0;
    println!("Changed first element of tuple: {fst}");

    //
    // ARRAYS
    //
    // Arrays act like C arrays -- fixed size on stack
    let _a = [1, 2, 3, 4, 5];
    let _b = 1..=5;

    // Initialize array filled with zeroes
    let _c: [i32; 5] = [0; 5];

    //
    // SHADOWING
    //
    let x = 5;
    let x = x + 1;

    // Inner scope shadowing
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
    println!("The value of sum is: {sum}");
}

// Function with no return
// "-> ()" can be ommited, but it implicitly returns this
fn func(x: i32) -> () {
    println!("Argument is {x}");
}

// Statements vs. Expressions
fn statements_expressions() -> i32 {
    // Statement
    let _x = 5;

    // Expression as a scope
    let _y = {
        let x = 3;
        x + 1 // Notice it doesn't have a semicolon
    }; // But it has one here

    // Return from final expression
    {
        let z = 0;
        z + 5
    }
}

// Function returning arg
// It doesn't require explicit "return",
// It just returns whatever's on the end of the function
// without a semicolon
fn plus_one(x: i32) -> i32 {
    x + 1
}

//
// CONTROL FLOW
//
fn control_flow() -> () {
    let n = 3;

    // Control flow syntax, basic C stuff
    if n < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // Kinda unreadable but this is accepted, wow
    if {
        let k = 17;
        n + k == 20
    } {
        println!("sum is equal to 20");
    }

    // A better way might be
    let cond = {
        let k = 17;
        n + k == 20
    };

    if cond {
        println!("sum is equal to 20");
    }

    // Also, we can have if-else if statements
    if n % 4 == 0 {
        println!("number is divisible by 4");
    } else if n % 3 == 0 {
        println!("number is divisible by 3");
    } else if n % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3 or 2");
    }

    // We can also use this in assignments
    let cond = true;
    let number = if cond { 1 } else { 2 };
    println!("The value of number is {number}");

    // Both need to be the same value though,
    // otherwise type resolution will fail and will not compile
    //let number = if cond { 1 } else { "two" };
}

//
// LOOPS
//
fn loops() -> () {
    // Will print forever
    //loop {
    //    println!("again!");
    //}

    // Breaking loops and using the return
    let mut counter = 0;
    // Break acts as a return from the loop!
    // Using "return" will return from the entire function
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result from the loop is: {result}");

    // Nested loops can have labels
    let mut counter = 0;

    'counting_up: loop {
        println!("counter = {counter}");
        let mut remaining = 1;

        loop {
            println!("remaining: {remaining}");
            if remaining == 0 {
                break;
            }
            if counter == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        counter += 1;
    }
    println!("End counter = {counter}");

    // Conditional loops: while
    let mut n = 3;
    while n != 0 {
        println!("{n}!");
        n -= 1;
    }

    // Without this, loop would need a more complex structure
    // for manual breaking

    // Loop through a collection: for
    let a = [1, 2, 3, 4, 5];
    for elem in a {
        println!("the value is {elem}");
    }

    // Fun stuff: Ranges have functions
    for elem in (1..6).rev() {
        println!("the value is {elem}");
    }
}
