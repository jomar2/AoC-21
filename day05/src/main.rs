use aoc_reader::reading_file;
use std::cmp;
use std::env;

struct MyLine {
    start: (i32, i32),
    end: (i32, i32),
}
impl MyLine {
    pub fn distance(&self) -> i32 {
        cmp::max(
            (self.start.0 - self.end.0).abs(),
            (self.start.1 - self.end.1).abs(),
        ) 
    }

    pub fn direction(&self) -> (i32, i32) {
        let mut x_dir = 0;
        let mut y_dir = 0;

        if self.start.0 < self.end.0 {
            x_dir = 1;
        } else if self.start.0 > self.end.0 {
            x_dir = -1;
        }
        if self.start.1 < self.end.1 {
            y_dir = 1;
        } else if self.start.1 > self.end.1 {
            y_dir = -1;
        }
        return (x_dir, y_dir);
    }

    pub fn is_diagonal(&self) -> bool {
        return !(self.start.0 == self.end.0 || self.start.1 == self.end.1);
    }
}

struct Board {
   board : Vec<Vec<i32>>,
}


impl Board {
    pub fn add_line(&mut self, line: MyLine)
    {
        let steps = line.distance();
        let direction = line.direction();
  //      println!("st: {:?}, Dir {:?}",steps,direction);
        for stp in 0..steps+1 {
            self.board[(line.start.0+(direction.0*stp)) as usize][(line.start.1+(direction.1*stp)) as usize] += 1;
  //          println!("step++");
        }
    }

    pub fn get_interceptions(&self) -> i32
    {
        self.board.iter().flatten().filter(|&&c| c >= 2).count() as i32
    }
}
fn parse_lines(lines: String) -> Vec<MyLine> {
    let mut all_lines: Vec<MyLine> = Vec::new();
    for line in lines.lines() {
        let pts: Vec<String> = line
            .split("->")
            .map(|p| p.split_whitespace().collect::<String>())
            .collect();

        let p1 = pts[0]
            .split(",")
            .map(|c| c.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let p2 = pts[1]
            .split(",")
            .map(|c| c.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let line_entry = MyLine {
            start: (p1[0], p1[1]),
            end: (p2[0], p2[1]),
        };
        // println!("{:?},{:?}",line_entry.start, line_entry.end);
        all_lines.push(line_entry);

    }
    return all_lines;
}
fn max_size(lines: String) -> i32 {
    let modded_line: String = lines.replace(",", " ").replace("->", " ");
    let all_numbers: Vec<i32> = modded_line.split_whitespace().map(|n| n.parse::<i32>().unwrap()).collect();

    *all_numbers.iter().max().unwrap() 
}

fn main() {
    let input_data = reading_file(env::args());
    let board_size = max_size(input_data.clone())+1;
//    let board_template  =  vec![vec![0; board_size as usize]; board_size as usize];
    let mut my_board = Board {board: vec![vec![0; board_size as usize]; board_size as usize],};
    let a = parse_lines(input_data);
    for l in a {
    //   if !l.is_diagonal() { // Part A
           my_board.add_line(l);
 //           println!("{:?}\n",my_board.board); 
      // }

    }
    println!("final: {}",my_board.get_interceptions());

}
