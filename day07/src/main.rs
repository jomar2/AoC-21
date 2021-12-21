use std::env;

use aoc_reader::reading_file;
fn calc_total_fuel_a(position_v: &Vec<i32>, possible_position: i32) -> i32 {
    let mut fuel_cost = 0;
    for pos in position_v {
        fuel_cost += (pos - possible_position).abs();
    }
    return fuel_cost;
}
fn calc_total_fuel_b(position_v: &Vec<i32>, possible_position: i32) -> i32 {
    let mut fuel_cost = 0;
    for pos in position_v {
        fuel_cost += (0..(pos-possible_position).abs()+1).sum::<i32>();
    }
    return fuel_cost;
}

fn main() {
    let input_data = reading_file(env::args());
    let crab_pos: Vec<_> = input_data
        .trim_end()
        .split(",")
        .map(|val| val.parse::<i32>().unwrap())
        .collect();
    let max_pos = crab_pos.iter().max().unwrap();
    let min_pos = crab_pos.iter().min().unwrap();
    let mut min_cost_a: (i32, i32) = (i32::MAX, i32::MAX);
    let mut min_cost_b: (i32, i32) = (i32::MAX, i32::MAX);
    for pos in *min_pos..*max_pos {
        let cost_a = calc_total_fuel_a(&crab_pos, pos);
        let cost_b = calc_total_fuel_b(&crab_pos, pos);
 //       println!("cost: {},currentMin:{:?}",cost_a,min_cost_a.0);
        if cost_a < min_cost_a.0 {
            min_cost_a = (cost_a, pos);
        }
        if cost_b < min_cost_b.0 {
            min_cost_b = (cost_b, pos);
        }
    }

     println!("Answer1: {:?}\nAnswer2: {:?}",min_cost_a,min_cost_b);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let indata = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(
            37,
            calc_total_fuel_a(&indata.clone(), 2),
            "input: {:?}",
            indata
        );
    }
    #[test]
    fn test2() {
        let indata = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(
            41,
            calc_total_fuel_a(&indata.clone(), 1),
            "input: {:?}",
            indata
        );
    }
    #[test]
    fn test3() {
        let indata = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(
            39,
            calc_total_fuel_a(&indata.clone(), 3),
            "input: {:?}",
            indata
        );
    }
    #[test]
    fn test4() {
        let indata = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(
            71,
            calc_total_fuel_a(&indata.clone(), 10),
            "input: {:?}",
            indata
        );
    }
    #[test]
    fn test5() {
        let indata = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(
            168,
            calc_total_fuel_b(&indata.clone(), 5),
            "input: {:?}",
            indata
        );
    }
    #[test]
    fn test6() {
        let indata = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(
            206,
            calc_total_fuel_b(&indata.clone(), 2),
            "input: {:?}",
            indata
        );
    }
}
