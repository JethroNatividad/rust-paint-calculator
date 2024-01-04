use std::io;
use std::io::Write;

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

fn get_input<T: std::str::FromStr>(prompt: &str) -> T {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        match input.trim().parse() {
            Ok(value) => break value,
            Err(_) => println!("Invalid input. Please try again."),
        }
    }
}

fn main() {

    let length: f64 = get_input("What is the length of the room in feet? ")
    let width: f64 = get_input("What is the width of the room in feet? ")

    let ceiling_sqft: f64 = length * width;
    let n_gallons: i64 = calculate_gallons(ceiling_sqft);

    println!("You will need to purchase {} {} of paint to cover {} square feet.", n_gallons, if n_gallons > 1 { "gallons" } else { "gallon" }, ceiling_sqft)

}
