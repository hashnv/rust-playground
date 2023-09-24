fn mean(nums: &[f64]) -> f64 {
    let sum: f64 = nums.iter().sum();
    sum / nums.len() as f64
}

fn celsius_to_fahrenheit(num: f64) -> f64 {
    (num * 1.8) + 32.0 
}

fn main() {
    let temps = [
        10.2,
        11.3,
        10.7,
        12.0,
        12.0,
        11.2,
        11.9,
    ];
    let average_temp = mean(&temps);
    println!("average_temp_celsius: {:.1} ℃", average_temp);
    println!("average_temp_fahrenheit: {:.0} ℉", celsius_to_fahrenheit(average_temp));
}
