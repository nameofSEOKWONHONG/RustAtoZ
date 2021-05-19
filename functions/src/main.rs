fn main() {
    println!("Hello, world!");
    function1();
    parameter_function(43);
    println!("{}", lamda_function(4));

    let number = 5;

    if number <= 0 {
        println!("number is zero({})", number);
    }
    else {
        println!("number is not zero({})", number);
    }

    let condition = true;
    let number = if condition {
        5
    } else {
        //"six" //error
        6
    };

    println!("The value of number is: {}", number);

    array_print();
    array_reserve_print();
}

fn function1() {
    println!("i'm function1");
}

fn parameter_function(x:i32) {
    println!("number is {}", x);
}

fn lamda_function(x:i32) -> i32 {
    5 + x
}

fn array_print() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is :{}", element);
    }
}

fn array_reserve_print() {
    for element in (0..5).rev() {
        println!("the value is :{}", element);
    }
}