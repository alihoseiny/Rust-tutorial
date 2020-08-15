fn main() {
    let num_of_rows = 9;    // number of rows we want
    let mut current_row = 1; // number of the row that we are printing right now.
    for column_counter in 0..num_of_rows { // We want 9 rows, so we should repeat the loop 9 times. column_counter value starts from 0 to 8
        for _ in 0..current_row { // Before the middle, we should print n start in nth row (We change the row number for other rows below)
            // Using the underscore(_) for the variable in the loop because Because i will not use it in the loop.
            // Using underscore in this case prevents compiler warning about unused variable.
            print!("*"); // Print * without new line after it.
        }
        print!("\n"); // Just a new line after each column

        if column_counter < num_of_rows / 2{
            current_row += 1;  // We did not reach the middle of the pattern yet. So rows should increase the row number.
        }
        else {
            current_row -= 1;  // We passed the middle and now rows should get shorter each time. So we decrease the row number.
        }
    }
}

