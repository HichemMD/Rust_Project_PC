use std::{io, string};

fn main() {

    println!("Enter your weight Please: ");
    Let mut input = String:: new();






    io::stdin().read_line(&mut input).unwrap();
    let weigth: f32= input.trim().purse().unwrap();
    let mars_weight = calculate_weight_on_mars(weight);
    println!("Weight on Mars: {}kg", mars_weight);


}


fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}





















