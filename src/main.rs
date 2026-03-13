use std::fs;
use regex::Regex;

fn get_password(rotations: String) -> i32 {
    let mut current_position = 50;
    let re = Regex::new(r"^([A-Za-z])(.+)$").unwrap();
    let mut num_times_at_zero = 0;
    
    for line in rotations.lines() {
        let Some(caps) = re.captures(line) else {
            println!("Bad");
            return 0;
        };
        
        let num = &caps[2].parse::<i32>().unwrap();
        let delta = match &caps[1] {
            "L" => -*num,
            "R" => *num,
            _ => return -1,
        };
        let max = 99;
        current_position = ((current_position + delta) % (max + 1) + (max + 1)) % (max + 1);
        if current_position == 0 {
            num_times_at_zero += 1;
        }
    }
    num_times_at_zero
}

fn main() -> Result<(), std::io::Error> {
    let file_contents = fs::read_to_string("input.txt")?;
    println!("{}", get_password(file_contents));
    Ok(())
}
