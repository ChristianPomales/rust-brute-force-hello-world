extern crate rand;
use rand::Rng;
use std::{thread, time};

fn look_cool(input: &String, count: usize) {
    for _ in 0..count {
        sleep(3);
        print!("{}", input);
        clear_term(input.len());
    }
}

fn get_guess() -> char {
    let mut rng = rand::thread_rng();
    rng.gen_range(32 as u8, 126 as u8) as char
}

fn clear_term(clear_length: usize) {
    for _ in 0..clear_length {
        print!("{}", (8u8 as char));
    }
}

fn sleep(time_in_ms: u64) {
    let wait_time = time::Duration::from_millis(time_in_ms);
    thread::sleep(wait_time);
}

fn main() {
    let hello_world = vec![
        'H', 'e', 'l', 'l', 'o', ',', ' ', 'W', 'o', 'r', 'l', 'd', '!',
    ];
    let mut guess = vec!['o'; hello_world.len()];

    while guess != hello_world {
        clear_term(hello_world.len());

        for item in guess.iter_mut().enumerate() {
            let index = item.0;
            let character = item.1;

            if *character != hello_world[index] {
                let guess_char = get_guess();
                *character = guess_char.to_owned();
            }
        }

        let final_string: String = guess.iter().cloned().collect();
        look_cool(&final_string, 10);
        print!("{}", final_string);

        sleep(5);
    }

    print!("\n");
}
