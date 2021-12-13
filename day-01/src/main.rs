use std::io::{self, BufRead};

fn main() -> io::Result<()>
{
    let stdin = io::stdin();
    let handle = stdin.lock();

    let mut last_value = i32::MAX;
    let mut increases_count = 0;

    for line in handle.lines() {
        let line_value = line.unwrap().parse::<i32>().unwrap();

        if line_value > last_value {
            increases_count += 1;
        }

        last_value = line_value;
    }

    println!("{}", increases_count);

    Ok(())
}
