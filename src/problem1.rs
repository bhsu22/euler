fn main() {
    let mut summation = 0;
    for number in 1..1000 {
        if number % 3 == 0 || number %5 == 0 {
            summation += number
        }
    }
    println!("The sum is: {}", summation)
}