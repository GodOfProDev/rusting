pub fn fibonacci_sequence() {
    println!("Fibonacci Sequence in 50 iterations");

    let mut numbers: Vec<usize> = Vec::new();

    for i in 0..50 {
        if i == 0 {
            numbers.push(0);
            println!("Iteration: 1\tValue: 0");
        } else if i == 1 || i == 2 {
            numbers.push(1);
            println!("Iteration: {}\tValue: 1", i + 1);
        } else {
            let value = numbers[i-2] + numbers[i-1];
            numbers.push(value);

            println!("Iteration: {}\tValue: {}", i + 1, value);
        }
    }
}