use std::io::{self,BufRead};

fn main() -> io::Result<()>
{
    let stdin = io::stdin();
    let handle = stdin.lock();

    let mut pos_y = 0;
    let mut pos_x = 0;
    let mut aim = 0;

    for line in handle.lines() {
        let line_str = line.unwrap();
        let line_value:Vec<&str> = line_str.split_whitespace().collect();
        let command = line_value[0];
        let value = line_value[1].parse::<i32>().unwrap();

        match command {
            "forward" => {
                pos_x += value;
                pos_y -= aim * value;
            },
            "down" => aim += value,
            "up" => aim -= value,
            _ => {}
        }
    }

    println!("{}", pos_x * (-1 * pos_y));

    Ok(())
}
