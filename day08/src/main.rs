use std::env;

use aoc_reader::reading_file;
use stringsort::vecsort;

fn main() {
    let input_data = reading_file(env::args());
    let mut segments: Vec<Vec<_>>= Vec::new();
    for line in input_data.lines(){
        segments.push(line.trim().split("|").collect());
    };
    let mut n_numbers = 0;
    let mut code_total = 0;
    for x in segments {
        n_numbers += counter_num_a(x.clone());
        code_total += ident_num_b(x);
    }

    println!("Answer1: {:?}\nAnswer2: {}", n_numbers, code_total);
}

fn counter_num_a(input : Vec<&str>) -> u32 {
    let mut ans : Vec<_> = input[1]
        .split_whitespace()
        .map(str::len)
        .collect::<Vec<usize>>();
    ans.retain(|&elem| elem==2 || elem==3||elem==4 ||elem==7);
    //println!("a: {:?}, {}", ans, ans.len());
    return ans.len() as u32;
}
fn get_dig (input : &str) -> Result<String,String> {
    println!("dig: {:?}",&input);
   match input {
        "abcdefg" => Ok("8".to_string()),
        "bcdef" => Ok("5".to_string()),
        "acdfg" => Ok("2".to_string()),
        "abcdf" => Ok("3".to_string()),
        "abd" => Ok("7".to_string()),
        "abcdef" => Ok("9".to_string()),
        "bcdefg" => Ok("6".to_string()),
        "abef" => Ok("4".to_string()),
        "abcdeg" => Ok("0".to_string()),
        "ab" => Ok("1".to_string()),
        _ => return Err("not a number".to_string())
    }
}
fn get_code(input: Vec<&str>) -> u32{
    let mut ans : String = "".to_string();
    for dig in input {
        let digit = get_dig(&vecsort(dig));
        if digit.is_ok()
        {
            ans.push_str(&digit.unwrap());
        }
    }
    println!("code: {:?}",ans);
    if ans.is_empty() {return 0};
return ans.parse::<u32>().unwrap();
}

fn ident_num_b(input : Vec<&str>) -> u32 {
    let output : Vec<_> = input[1]
        .split_whitespace()
        .collect();
   
    return get_code(output);

}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let indata : Vec<&str> = vec!["be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb","fdgacbe cefdb cefbgd gcbe"];
        assert_eq!(2,counter_num_a(indata.clone()),"input:",);
    }
    #[test]
    fn test2() {
        let indata : Vec<&str> = vec!["be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb","fdgacbe cefdb cefbgd gcbe"];
        assert_eq!(2,ident_num_b(indata.clone()),"input:",);
    }
    #[test]
    fn test3() {
        let indata : Vec<&str> = vec!["be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb","fdgacbe cefdb cefbgd gcbe"];
        assert_eq!(3,ident_num_b(indata.clone()),"input:",);
    }
}
