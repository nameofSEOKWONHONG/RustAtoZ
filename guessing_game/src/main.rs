extern crate rand;

//declare standard io lib
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    //print
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        // let foo = 5; //불변
        // let mut bar = 5; //변
        //foo is immutable variable, error statement
        //foo = bar;

        //declare mutable string variable
        let mut guess = String::new();

        //readline allocate to guess
        io::stdin().read_line(&mut guess)
            //fail message
            .expect("Faild to read line");

        //string to int (Shadowing - declare same name variable, allow to different type)
        // let guess: u32 = guess.trim().parse()
        //     .expect("Please type a number!");
        // don't stop exception, check exception process.
        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        //let으로 정의된 변수는 동일 변수명이라도 할당할 수 있다.
        //no error
        let x = 5;
        let x = x + 1;
        let x = x + 2;

        // no error
        // let mut x = 5;
        // x = x + 1;

        //error
        // let x = 5;
        // x = x + 5;

        let x = 2.0;
        let y:f32 = 3.0;
    }

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("h={}", heart_eyed_cat);

    let tuple = (500, 6.4, 1);
    let (x, y, z) = tuple;
    println!("The value of y is: {}", y);
}
