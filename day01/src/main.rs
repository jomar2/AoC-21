use std::{env};

use aoc_reader::reading_file;
fn part_a (data : Vec<i32>) -> i32 {

    let mut old_val = data[0];
    let mut nr_increase = 0;
    for number in data {
        if old_val < number {
            nr_increase += 1;
        }
        old_val = number;
    }
    return nr_increase;
}

fn part_b (data : Vec<i32>) -> i32 {
    let mut nr_increase = 0;
    for win in data.windows(4) {
        let prev = win[0] + win[1] + win[2];
        let current = win[1] + win[2] + win[3];
        if current > prev {
            nr_increase += 1;
        }
    }
    nr_increase
}


fn main() {
    let input_data = reading_file(env::args());
    let data : Vec<i32> = input_data.lines().map(|s| s.parse().expect("err..")).collect();


    println!("Answer1: {}",part_a(data.clone()));
    println!("Answer2: {}",part_b(data));
//     println!("Answer1: {}\nAnswer2:split("\n").map(|s| s.parse::<i32>().expect("err..")).collect()); {}", counter(input_data.clone()),counter2(input_data));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let indata = "(())".to_string();
//        assert_eq!(0,counter(indata.clone()),"input: {}", indata);
    }
    #[test]
    fn test2(){
        let indata = "()()".to_string();
  //      assert_eq!(0,counter(indata.clone()),"input: {}", indata);
    }
}
