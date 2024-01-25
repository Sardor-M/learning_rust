//  In order to find the perimneter 2 * (length + width) is used;

fn main() {
    
    let length: f64 = 50.5;
    let width: f64 = 3.2;

    // calculate the area 
    let area: f64 = length * width;
    let perimeter: f64 = 2.0 * (length + width);

    println!("The area of the rectangle is: {}", area);
    println!("The perimeter of the rectangle is: {}", perimeter);
}