

// function to add two numbers
fn add() {

    let total_amount_paid = 115+35+25;
    let total_amount_due: i32 = 35+35+31;
    //let a = 32 + 32 + 32;
    //let b = 10;

    let sum = total_amount_paid - total_amount_due;

    println!("Sum of a and b = {}", sum);
}



fn main() {
    println!("Hello, world!");
    add()
}
