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

fn calculate_gallons(length: f64, width: f64) -> i64 {
    const GALLON_SQFT: f64 = 350.0;
    let ceiling_sqft: f64 = length * width;
    (ceiling_sqft / GALLON_SQFT).ceil() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_gallons(){
        assert_eq!(calculate_gallons(35.0, 10.0), 1);
        assert_eq!(calculate_gallons(14.0, 6.0), 1);
        assert_eq!(calculate_gallons(35.0, 11.0), 2);
        assert_eq!(calculate_gallons(52.0, 63.0), 10);
    }
}


fn main() {
    println!("Hello, world!");
}
