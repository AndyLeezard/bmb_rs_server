use std::io;

fn main() {
    println!("Enter your weight in kilograms:");

    let mut weight = String::new();
    io::stdin().read_line(&mut weight).unwrap();
    let weight = weight.trim().parse::<f32>().unwrap();

    println!("Enter your height in centimeters:");

    let mut height = String::new();
    io::stdin().read_line(&mut height).unwrap();
    let height = height.trim().parse::<f32>().unwrap();

    let height_in_meters = height / 100.0;
    let bmi = weight / (height_in_meters * height_in_meters);
    println!("Your BMI is: {:.1}", bmi);

    if bmi < 16.0 {
        println!("Severely underweight");
    } else if bmi >= 16.0 && bmi < 18.5 {
        println!("Underweight");
    } else if bmi >= 18.5 && bmi < 25.0 {
        println!("Healthy weight");
    } else if bmi >= 25.0 && bmi < 30.0 {
        println!("Overweight");
    } else if bmi >= 30.0 && bmi < 35.0 {
        println!("Class I obesity");
    } else if bmi >= 35.0 && bmi < 40.0 {
        println!("Class II obesity");
    } else {
        println!("Class III obesity");
    }
}
