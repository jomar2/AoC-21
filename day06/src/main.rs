use std::env;

use aoc_reader::reading_file;

fn update_fishies(fishies: &mut Vec<u64>) {
//     println!("Before: {:?}",fishies);
    //for (age,nr) in fishies.iter_mut().enumerate() {
    let mut new_fish = 0;
    for ind in 0..8{
        if ind == 0 {
            //fishies[7] += fishies[0];
            new_fish = fishies[0];
            fishies[ind] = fishies[ind + 1];
        } else {
            fishies[ind] = fishies[ind + 1];
        }
    }
    fishies[8] = new_fish;
    fishies[6] += new_fish;
 //   println!("After: {:?}",fishies);
}

fn main() {
    let input_data = reading_file(env::args());
    // println!("Answer1: {}\nAnswer2: {}", counter(input_data.clone()),counter2(input_data));
    let fishies: Vec<_> = input_data
        .trim_end()
        .split(",")
        .map(|val| val.parse::<i32>().unwrap())
        .collect();
    /*
    for _cycle in 0..18 {
        let mut new_fishies = 0;
        for fish in &mut fishies {
            *fish -= 1;
            if *fish == -1 {
                *fish = reset_value;
                new_fishies += 1;
            }
        }
        let mut new = vec![start_value as i32;new_fishies];
        fishies.append(&mut new);
        println!("{:?}",fishies);
    }
    */
    let mut total_fish: Vec<u64> = vec![0; 9];
    let cycles: i32 = 256;
// Part 1   let cycles: i32 = 18;
    for fish in &fishies {
        total_fish[*fish as usize] += 1;
    }
    for _x in 0..cycles {
        update_fishies(&mut total_fish);
    }

    println!("Result {:?}", total_fish.iter().sum::<u64>());
}
