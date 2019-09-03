fn main() {
    println!("ch3_3_functions");
    another_function(5, -5);
    let x5 = five();
    println!("value of fn five() is: {}", x5);
    let x6 = plus_one(5);
    println!("value of fn plus_one(5) is: {}", x6);
    if_else(6);
    let_if(true);
    loop_break(10);
}

//fn don't return anything
fn another_function( x: i32, y: i32) {
    println!("values from another_function: x: {}, y: {}", x, y);
}

//fn with return value - without ';'
fn five() -> i32 {
    5
}

//fn with input and return value
fn plus_one(x: i32) -> i32{
    x+1
}

fn if_else(x: i32) {
    if x % 4 == 0 {
        println!("number {} is devisible by 4", x);
    } else if x % 3 == 0{
        println!("number {} is devisible by 3", x);
    } else if x % 2 == 0{
        println!("number {} is devisible by 2", x);
    }
}

fn let_if(condition: bool) {
    let number = if condition{
        5
    } else {
        6
    };

    println!("let if value is {}", number);
}

fn loop_break(x :i32) {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == x {
            break counter * 2;
        }
    };
    println !("loop {} x 2 = {}", x, result);
}
