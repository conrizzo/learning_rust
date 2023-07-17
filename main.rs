

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
    let rows = 8;
    let cols = 8;

    // create a 2D grid of characters with a size of 8x8
    let mut grid = vec![vec![' '; cols]; rows];

    // add the letter "p" to all cells in the second row

    let first_row = ['R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R'];
    let last_row = ['R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R'];
    for col in 0..cols {
        grid[0][col] = first_row[col];
        grid[1][col] = 'p';
        grid[6][col] = 'p';
        grid[7][col] = last_row[col];
    }

// print the grid
for row in &grid {
    println!("{:?}", row);
}
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
