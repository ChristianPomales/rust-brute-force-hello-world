extern crate rand;
use rand::Rng;
use std::{thread, time};

fn look_cool() {
    let mut count = 100;

    while count >= 0 {
        for character in 32..126 {
            let actual_character = character as u8;
            print!("{}", actual_character as char);
            print!("{}", (8u8 as char));
        }

        count -= 1;
    }
}

fn main() {
    let hello_world = "Hello, World!";
    let mut rng = rand::thread_rng();
    let wait_time = time::Duration::from_millis(1);
    let mut guess: char;

    for character in hello_world.chars() {
        thread::sleep(wait_time);
        loop {
            look_cool();
            guess = rng.gen_range(32 as u8, 126 as u8) as char;
            print!("{}", guess);
            thread::sleep(wait_time);

            if guess == character {
                break;
            } else {
                print!("{}", (8u8 as char));
            }
        }
    }

    println!("");
}
