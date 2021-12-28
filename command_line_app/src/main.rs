use std::io;

fn main() {

    println!("Enter your weight Please (Onlly number): ");
    let mut input = String::new();






    io::stdin().read_line(&mut input).unwrap();
    let weight: f32= input.trim().parse().unwrap();
    let mars_weight = calculate_weight_on_mars(weight);
    println!("Weight on Mars: {}kg", mars_weight);

}


fn calculate_weight_on_mars(_weight: f32) -> f32 {
    (_weight / 9.810) * 3.711
}





















