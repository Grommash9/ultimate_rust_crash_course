// Silence some warnings so they don't distract from the exercise.
#![allow(unused_mut, unused_variables)]

fn main() {
    // This fancy stuff either gets the first argument as a String, or prints
    // usage and exits if an argument was not supplied to the program.
    let mut arg: String = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Please supply an argument to this program.");
        std::process::exit(-1);
    });


    inspect(&arg);

    change(&mut arg);
    println!("I have many {}", arg);

    if eat(arg) {
       println!("Might be bananas");
    } else {
       println!("Not bananas");
    }

    // Challenge: Write a function "bedazzle" that takes a mutable reference to a String and
    // ignores what is in the string and replaces the contents of the string with the String
    // "sparkly". Then uncomment the code below.
    //
    // Hint: You will need to dereference the mutable reference in order to assign it a
    // new value.
    //
    let mut material = "mud".to_string();
    println!("This material is just `{}`.", material);
    bedazzle(&mut material);
    println!("Wow! Now the material is `{}`!", material);
}


fn eat (string: String) -> bool {
    if string.starts_with("b") && string.contains("a") {
        true
    } else {
        false
    }
}

fn change (string: &mut String) {
    if !string.ends_with("s") {
        string.push_str("s");
    }
}

fn bedazzle(string: &mut String) {
    (*string) = "sparkly".to_string();
}
    

fn inspect (string: &String) {
    if string.ends_with("s") {
        println!("da");
    } else {
        println!("net");
    }
}