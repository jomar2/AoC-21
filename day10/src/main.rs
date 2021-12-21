use std::{env, collections::vec_deque};

use aoc_reader::reading_file;
use std::collections::HashMap;
fn main() {
    let input_data = reading_file(env::args());
    let mut input: Vec<_> = input_data.lines().collect();
    let mut brackets : Vec<char> = Vec::new();
    let bracket_pair = HashMap::from([('{','}'),('[',']'),('(',')'),('<','>')]);
    let bracket_points = HashMap::from([('}',1197),(']',57),(')',3),('>',25137)]);
    let bracket_completion_points = HashMap::from([('{',3),('[',2),('(',1),('<',4)]);
    let mut total_points = 0;
    let mut total_vec : Vec<u64> = Vec::new();
    println!("line1 : {:?}\nAnswer2: {}",input.len(),0 );

    input.retain(|&l| !is_corrupt(l));

    println!("{:?}",input);

    for line in input{
        for c in line.chars(){
            match c {
                '{'| '(' | '['| '<'=> {brackets.push(c);}
                '}'| ')' | ']'| '>'  => {
                    if bracket_pair.get(brackets.last().unwrap()).unwrap() == &c {
                    // println!("match.. {:?}:{}",bracket_pair.get(brackets.last().unwrap()).unwrap(),c);
                    brackets.pop();
                    }
                },
                _ => println!("dunno.."),
            }
        }
        while !brackets.is_empty() {
            total_points*=5;
            total_points+=bracket_completion_points.get(&brackets.pop().unwrap()).unwrap();
        }
        total_vec.push(total_points);
        total_points = 0;
    }
    total_vec.sort();
    let ans_pos = 0;
    println!("Score: {:?}, ans: {}",total_vec[total_vec.len()/2],0);
    // Too High 5517452839,
    //          5517452839
    //             7175783
}

fn is_corrupt(line:&str) -> bool
{
    let mut brackets : Vec<char> = Vec::new();
    let bracket_pair = HashMap::from([('{','}'),('[',']'),('(',')'),('<','>')]);
    for c in line.chars(){
        match c {
            '{'| '(' | '['| '<'=> {brackets.push(c);}
            '}'| ')' | ']'| '>'  => {
                if bracket_pair.get(brackets.last().unwrap()).unwrap() == &c {
                    brackets.pop();
                }
                else {
                    // println!("Corrupt!!, {:?}:{}",line,&c);
                    return true;
                }
            },
            _ => println!("dunno.."),
        }
    }
    return false;
}

