

// function to add two numbers
fn add() {
    let total_amount_paid = 115+25+35;
    let total_amount_due: i32 = 32+32+32;

    let sum = total_amount_paid - total_amount_due;
    println!("test addition function = {}", sum);   
}



struct Item {
    name: String,    
    position: (i32, i32),
}

fn make_grid(){
    let rows = 20;
    let cols = 20;

    // create a 2D grid of integers with a size of 3x3
    let grid = vec![vec![0; cols]; rows];

    // print the grid
    for row in &grid {
        println!("{:?}", row);    }

}

fn make_item(){
    let item = Item {
        name: String::from("First Item"),
        position: (2, 2),
    };
    println!("item name = {}", item.name);
    println!("item position = {:?}", item.position);
}


fn main() {
    println!("main function!");
    add();
    make_grid();
    make_item();
}
