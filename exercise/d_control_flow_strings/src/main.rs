// Silence some warnings so they don't distract from the exercise.
#![allow(dead_code, unused_mut, unused_variables)]

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    for arg in args {
        if arg == "sum" {
            sum();
        } else if arg == "double" {
            double();
        }
        else {
            count(arg);
        }
    }
}

fn sum() {
    let mut sum = 0;
    for  x  in 7..=23  {
        sum += x 
    }
    println!("The sum is {}", sum);
}

fn double() {
    let mut count = 0;
    let mut x = 1;
    loop {
        x = x * 2;
        count += 1;
        if x > 500 {
            break
        }
    }
    println!("You can double x {} times until x is larger than 500", count);
}

fn count(arg: String) {
    let mut counter: i8 = 0;
    loop {
        print!("{} ", arg);
        counter += 1;
        if counter > 7 {
            break
        }
    }
    println!();
}
