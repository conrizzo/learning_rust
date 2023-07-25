
use std::io;

use rand::Rng;

use std::process::Command;
/* 
fn add(x: i32, y: i32) -> i32 {
    return x + y;
}
*/

fn get_python_file() -> String {
    let output = Command::new("python")
    //.arg("C:\\Users\\conri\\conrad_python\\run_python_from_rust.py")
    .arg("C:\\Users\\conri\\conrad_rust\\rust_learning\\src\\python_file_to_run.py")
    .output() 
    .expect("Failed to run Python file");
    
    //println!("{}", String::from_utf8_lossy(&output.stdout));
    return String::from_utf8_lossy(&output.stdout).to_string();
}

fn generate_number() -> i32 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(1..=10);
}

fn pick_number() -> i32 {
    println!("Pick a number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let number: i32 = input.trim().parse().expect("Invalid number");
    
    if generate_number() == number {
        println!("You win!");
        return 1;

    } else {
        println!("You lose!");
        return 0;
  }
}

fn main() {
    // initialize game state
    let mut score = 0;
    let mut game_over = false;

    let output = get_python_file();
    println!("{}", output);
    // game loop

     
    while !game_over {
        // update game state
        

        if pick_number() == 1 {
            score += 10;
        }
        // render game
        println!("Score: {}", score);

        // check for game over condition
        if score >= 10 {
            game_over = true;
        }

        // wait for a short time before the next frame
        std::thread::sleep(std::time::Duration::from_millis(16));
    }

    // game over
    println!("Game over!");
}