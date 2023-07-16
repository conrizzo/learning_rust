

// function to add two numbers
fn add() {

    let total_amount_paid = 115+35+25;
    let total_amount_due: i32 = 35+35+31;
    //let a = 32 + 32 + 32;
    //let b = 10;

    let sum = total_amount_paid - total_amount_due;

    println!("test addition function = {}", sum);
}



fn main() {
    println!("main function!");
    add()
}
