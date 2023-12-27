// Calculate gallons of paint needed to paint the ceiling of a
// room. Prompt for the length and width, and assume one
// gallon covers 350 square feet. Display the number of gallons
// needed to paint the ceiling as a whole number

// Remember, you can’t buy a partial gallon of paint. You must
// round up to the next whole gallon.

// n of gallons to paint the ceiling of lxw room, where a gallon covers 350 sqft, round up the gallon.

// Inputs: Length, Width
// Process: ceiling sqft = lxw, gallons = round_up(ceiling sqft / gallon sqft).
// Output: You will need to purchase {n} gallon/s of paint to cover {ceiling sqft} square feet.

fn calculate_gallons(ceiling_sqft: f64) -> i64 {
    const GALLON_SQFT: f64 = 350.0;
    (ceiling_sqft / GALLON_SQFT).ceil() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_gallons(){
        assert_eq!(calculate_gallons(350.0), 1);
        assert_eq!(calculate_gallons(84.0), 1);
        assert_eq!(calculate_gallons(385.0), 2);
        assert_eq!(calculate_gallons(3276.0), 10);
    }
}

use std::io;
use std::io::Write;

fn main() {
    print!("What is the length of the room in feet? ");
    let mut length = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut length).expect("Failed to read input");
    let length: f64 = length.trim().parse().expect("Please enter a valid number");

    print!("What is the width of the room in feet? ");
    let mut width = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut width).expect("Failed to read input");
    let width: f64 = width.trim().parse().expect("Please enter a valid number");

    let ceiling_sqft: f64 = length * width;
    let gallons: i64 = calculate_gallons(ceiling_sqft);

}
