use std::{env,str::FromStr, string::ParseError};

use aoc_reader::reading_file;
#[derive(Debug)]
struct Point {
    x: u64,
    y: u64,
}
#[derive(Debug)]
enum Fold {
    X(u64),
    Y(u64),
}

impl FromStr for Point {
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> Result<Self,Self::Err> {
        let (x,y) = s.split_once(",").unwrap();
        return Ok(Point{
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        })
    }
}

impl FromStr for Fold {
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> Result<Self,Self::Err> {
        let (axis,value) = s.split_once("fold along ").unwrap().1.split_once("=").unwrap();
    return match axis {
            "x" => Ok(Fold::X(value.parse().unwrap())),
            "y" => Ok(Fold::Y(value.parse().unwrap())),
            _ => unreachable!()
        }
    }
}
fn main() {
    let input_data = reading_file(env::args());
    let (points,folds) = input_data.trim().split_once("\n\n").unwrap();



    let all_points : Vec<Point> = points.lines().map(str::parse).map(Result::unwrap).collect();
    let all_folds : Vec<Fold> = folds.lines().map(str::parse).map(Result::unwrap).collect();



    println!("Answer1: {:?}\nAnswer2: {:?}", all_points,all_folds);

}
