use std::io;

fn main() {
    let mut sum_of_numbers = 0;
    loop {
        let mut input_number = String::new();
        io::stdin().read_line(&mut input_number).unwrap();

        let trimmed_number = input_number.trim();
        if trimmed_number == "-1" {
            break;
        }

        match trimmed_number.parse::<i128>() {
            Ok(num) => {
                if num >= 0 {
                    sum_of_numbers += num;
                } else {
                    println!("NaN");
                    return;
                }
            }
            Err(_) => {
                println!("NaN");
                return;
            }
        }
    }
    println!("{}", sum_of_numbers);
}
