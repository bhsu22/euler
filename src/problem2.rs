fn fibonacci(n: i32) -> i32 {
    if n <= 1 {
        return 1;
    }
    return fibonacci(n-1) + fibonacci(n-2);
}

fn main() {
    let upper = 4000000;
    let mut summation = 0;
    let mut number = 0;
    let mut n = 1;

    while number < upper {
        number = fibonacci(n);
        if number > upper {
            break;
        }
        n+=1;

        if number %2 == 0 {
            summation += number;
        }
    }
    println!("Sum is {0} and last Fibonacci number is {1}", summation, number);
}