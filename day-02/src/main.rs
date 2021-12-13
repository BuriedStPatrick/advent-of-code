use std::io::{self};

fn get_lines() -> Vec<String> {
    use std::io::prelude::*;
    let stdin = std::io::stdin();
    return stdin.lock().lines().map(|x| x.unwrap()).collect();
}

fn main() -> io::Result<()>
{
    let mut increases_count = 0;        // the current tally of increases
    let window_size = 3;                // the size of the windows to compare
    let mut last_sum = i32::MAX;        // the last sum we're comparing against

    let lines = get_lines();

    let mut index = 0;
    while index <= (lines.len() - window_size) {
        let mut sum = 0;

        let mut window_index = 0;
        while window_index < window_size {
            sum += lines[index + window_index].parse::<i32>().unwrap();
            window_index += 1;
        }

        if sum > last_sum {
            increases_count += 1;
        }

        last_sum = sum;

        index += 1;
    }

    println!("{}", increases_count);

    Ok(())
}
