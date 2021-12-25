use std::io;

fn get_lines() -> Vec<String> {
    use std::io::prelude::*;
    let stdin = std::io::stdin();

    return stdin.lock().lines().map(|x| x.unwrap()).collect();
}

fn main() -> io::Result<()>
{
    let lines = get_lines();
    let len = lines[0].len(); // The length of a single line. We assume all lines have identical lengths so that we can take the first line length as valid for all.

    let mut num_ones = vec![0; len];   // index: column, value: number of ones for the column
    let mut num_zeroes = vec![0; len]; // index: column, value: number of zeroes for the column

    for line in lines {
        for (i, c) in line.chars().enumerate() {
            match c {
                '0' => num_zeroes[i] += 1,
                '1' => num_ones[i] += 1,
                _ => unreachable!()
            }
        }
    }

    // Print the gamma
    let mut gamma: Vec<char> = vec!['0'; len];
    let mut epsilon: Vec<char> = vec!['0'; len];
    for i in 0..len {
        let zeroes = num_zeroes[i];
        let ones = num_ones[i];

        if zeroes > ones {
            gamma[i] = '0';
            epsilon[i] = '1';
        } else {
            gamma[i] = '1';
            epsilon[i] = '0';
        }
    }

    let gamma = get_decimal_value(gamma);
    let epsilon = get_decimal_value(epsilon);

    println!("{}", gamma * epsilon);

    Ok(())
}

fn get_decimal_value(binary_chars: Vec<char>) -> isize {
    let binary_chars: String = binary_chars.iter().collect();
    let binary_chars = isize::from_str_radix(&binary_chars, 2).unwrap();

    return binary_chars;
}
