use std::{env, convert::TryInto};

use aoc_reader::reading_file;
fn get_gam_eps(input : &Vec<&str>)-> (String ,String){
    let bin_size : usize = input[0].chars().count();
    let nr_lines : usize = input.len();
    let mut sum = vec![0;bin_size];
    for line in input {
        let tmp : Vec<char>= line.chars().collect();
        for (i, bit) in tmp.iter().enumerate() {
            sum[i] += bit.to_digit(10).unwrap();
        }
    }
    println!("{:?}",nr_lines);
    let mut gam = "".to_string();
    let mut eps = "".to_string();
    for bit_nr in sum {
        if bit_nr >= ((nr_lines + 1)/2).try_into().unwrap(){
            gam.push_str("1");
            eps.push_str("0");
        }
        else {
            gam.push_str("0");
            eps.push_str("1");

        }
    }
    println!("{:?}",gam);
    println!("{:?}",eps);
    return (gam,eps);
}
fn part_b (input: String) -> i64 {
    let mut input_lines_oxy : Vec<_> = input.lines().collect();
    let mut input_lines_c02 : Vec<_> = input.lines().collect();
    let mut index = 0;
    while input_lines_oxy.len() != 1 as usize{

        let (gam,_eps) = get_gam_eps(&input_lines_oxy);
        input_lines_oxy.retain(|l| l.chars().collect::<Vec<char>>()[index] == gam.chars().collect::<Vec<char>>()[index]);
        index += 1;
        println!("Left: {:?}", &input_lines_oxy );
    }
    println!("final: {:?}", &input_lines_oxy );
    index = 0;
    while input_lines_c02.len() != 1 as usize{

        let (_gam,eps) = get_gam_eps(&input_lines_c02);
        input_lines_c02.retain(|l| l.chars().collect::<Vec<char>>()[index] == eps.chars().collect::<Vec<char>>()[index]);
        index += 1;
        println!("Left: {:?}", &input_lines_c02 );
    }
    println!("final: {:?}", &input_lines_c02 );
    let res = (isize::from_str_radix(&input_lines_oxy[0],2).unwrap() * isize::from_str_radix(&input_lines_c02[0],2).unwrap()).try_into().unwrap();
    return res;
}
fn get_part_a(input : String)-> i64{
    let bin_size : usize = input.lines().next().unwrap().chars().count();
    let nr_lines : usize = input.lines().count();
    let mut sum = vec![0;bin_size];
    for line in input.lines() {
        let tmp : Vec<char>= line.chars().collect();
        for (i, bit) in tmp.iter().enumerate() {
            sum[i] += bit.to_digit(10).unwrap();
        }
    }
    println!("{:?}",nr_lines);
    let mut gam = "".to_string();
    let mut eps = "".to_string();
    for bit_nr in sum {
        if bit_nr > (nr_lines/2).try_into().unwrap() {
            gam.push_str("1");
            eps.push_str("0");
        }
        else{
            gam.push_str("0");
            eps.push_str("1");

        }
    }
    println!("{:?}",gam);
    println!("{:?}",eps);
    let res = (isize::from_str_radix(&gam,2).unwrap() * isize::from_str_radix(&eps,2).unwrap()).try_into().unwrap();
    return res;
}



fn main() {
    let input_data = reading_file(env::args());
    println!("Part 1: {}", get_part_a(input_data.clone()));
    println!("Part 2: {}", part_b(input_data));

}
