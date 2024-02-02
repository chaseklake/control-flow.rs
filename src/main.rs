fn main() {
    ifs_example();
    loops_example();
}

// Demonstrates how ifs work in Rust
fn ifs_example() {
    let condition = true;
    let condition2 = false;

    let mut x = 5;

    println!("The starting value of x is: {x}");

    // Example of if statement. In this case, the first arm (x = 6) will always trigger unless 'condition' is changed.
    if condition {
        x = 6;
    } else if condition2 {
        x = 7;
    } else {
        x = 8;
    }

    // Since ifs are expressions, we can use them to resolve statements.
    let x = if condition {"six"} else {"five"};

    println!("The value of x is : {x}");

    // ERROR: Rust needs to know what x will be at compile time, so the following will throw an error. All arms must return the same type.
    // let x = if condition {"six"} else {5};
}

// Demonstrates loops, while loops, and for loops
fn loops_example() {
    let mut x = 0;
    let y = loop {
        println!("This will keep printing forever unless we [break]");
        if x >= 3 {
            break x; // end the loop (and return x)
        } else {
            x +=1;
            continue; // skip the rest of the code and start the next loop
        }
        // The following gives a warning: unreachable code. This is because we will ALWAYS break or continue before this point.
        //println!("Oops, something when horribly wrong. How did I get here?");
    };

    println!("Phew! Now the value of y is: {y}");

    // You can also have loops within loops and end outer loops with inner loops using labels!
    let mut count = 0;
    'outer: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("Remaining = {remaining}");
            if remaining == 9 {
                break; // This breaks the current, inner loop
            }
            if count == 2 {
                break 'outer; // this breaks the outer loop!
            }
            remaining -=1;
        }
        count += 1;
    }
    println!("End count = {count}");

    // While loops evaluate a condition each loop, automatically calling 'break' when the condition is false.
    // This is often clearer and easier to write than 'loop' with counters and breaks.
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }
    println!("LIFTOFF!!!");

    // When looping through a collection, such as an array, it's best to use a 'for' loop
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("The value is: {element}");
    }

    // This uses a Range object, which is in the standard library, and calls the 'rev()' function on it before running!
    for number in (1..10).rev() {
        println!("{number}...");
    }
    println!("LIFTOFF!!! (Again!)");

}
