use std::io;

fn main() {
    loop {
        let mut input = String::new();

        io::stdin().read_line( &mut input).expect("Invalid input!");

        if input.contains("quit") {
            break;
        }

        let new_input = match input.trim().parse::<u32>() {
            Ok(len) => len,
            Err(_) => continue,
        };

        generate_fibbonaci_numbers(new_input);
    }
}

fn generate_fibbonaci_numbers(len: u32) {
    let mut result: Vec<u128> = vec![0, 1];
    let mut laster = *result.first().unwrap();
    let mut last= *result.last().unwrap();
    let new_len = match len {
        0 => 0,
        _ => len - 1,
    };
    for _i in 0..new_len {
        let current = laster + last;
        result.push(current);
        laster = last;
        last = current;
    }
    println!("The Fibonacci number where n = {} is: {}", len, result.get(len as usize).unwrap());
}