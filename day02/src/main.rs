use std::env;

use aoc_reader::reading_file;
fn part_a(input: String) -> i32 {
    let mut h_pos: i32 = 0;
    let mut depth: i32 = 0;

    for line in input.lines() {
        let inst: Vec<_> = line.split_whitespace().collect();
        match inst[0] {
            "forward" => h_pos += inst[1].parse::<i32>().unwrap(),
            "up" => depth -= inst[1].parse::<i32>().unwrap(),
            "down" => depth += inst[1].parse::<i32>().unwrap(),
            _ => println!("unknown instruction"),
        }

//         println!("hpos: {}, depth {}", h_pos, depth);
    }

    return h_pos*depth;
}

fn part_b(input: String) -> i32 {
    let mut h_pos: i32 = 0;
    let mut depth: i32 = 0;
    let mut aim: i32 = 0;

    for line in input.lines() {
        let inst: Vec<_> = line.split_whitespace().collect();
        match inst[0] {
            "forward" => {
                h_pos += inst[1].parse::<i32>().unwrap();
                depth += aim*inst[1].parse::<i32>().unwrap(); 
            }
            "up" => {
                //depth -= inst[1].parse::<i32>().unwrap();
                aim -= inst[1].parse::<i32>().unwrap();
            }
            "down" => {
                //depth += inst[1].parse::<i32>().unwrap();
                aim += inst[1].parse::<i32>().unwrap(); 
            }
            _ => println!("unknown instruction"),
        }

         // println!("hpos: {}, depth {}, aim {}", h_pos, depth, aim);
    }

    return h_pos*depth;
}

fn main() {
    let input_data = reading_file(env::args());

    let part_a_ans = part_a(input_data.clone());
    let part_b_ans = part_b(input_data);

    println!("Answer1: {}\nAnswer2: {}", part_a_ans,part_b_ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let indata = "(())".to_string();
        assert_eq!(0, counter(indata.clone()), "input: {}", indata);
    }
}
