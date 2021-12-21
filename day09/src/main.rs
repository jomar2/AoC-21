use std::env;

use aoc_reader::reading_file;

fn main() {
    let input_data = reading_file(env::args());
    let lines:Vec<&str> = input_data.lines().collect();
    let sizey: usize= lines.len();
    let sizex = lines.first().unwrap().chars().count();
    let mut board : Vec<Vec<u32>> = Vec::new();//vec![vec![0;sizey];sizex];
    for line in lines {
        let _ = &board.push(line.chars().map(|x| x.to_digit(10).unwrap()).collect());
    }
    let mut min_points : Vec<(usize,usize)> = Vec::new();
    let mut ans : u32 = 0;
    for col in 0..sizex {
        for row in 0..sizey {
            if check_neighbours((row,col), &board){
               ans += board[row][col]+1;
                min_points.push((row,col));
            }
        }
    }
    let mut ans_vec : Vec<u32> = Vec::new();
    for p in min_points {
        ans_vec.push(check_baisin_size(p, &mut board));
    }

    ans_vec.sort();
    ans_vec.reverse();
//    let part2 = ans_vec
// 1640 too high // 600 OK!
    println!("Answer1: {:?}\nAnswer2: {}",&ans_vec[..3].iter().product::<u32>(),board[2][3]);
}

fn in_baisins(input: (usize,usize),parents: &mut Vec<(usize,usize)>, limits : (usize,usize), board: &mut Vec<Vec<u32>>, ans: &mut u32){
    let (x,y) = input;
    let (sizex,sizey) = limits;
    *ans+=1;
    parents.push(input);

    // println!("inb: {:?},{:?},ans: {}",input,limits,ans);
    if y > 0 {
        if board[x][y-1] != 9 && !parents.iter().any(|&p| p == (x,y-1)) {
            in_baisins((x,y-1),parents,limits,board,ans);
        }
    }
    if x > 0 {
        if board[x-1][y] != 9 && !parents.iter().any(|&p| p == (x-1,y)){
            in_baisins((x-1,y),parents,limits,board,ans);

        }
    }
    if y < sizey {
        if board[x][y+1] != 9 && !parents.iter().any(|&p| p == (x,y+1)){
            in_baisins((x,y+1),parents,limits,board,ans);
        }
    }
    if x < sizex {
        if board[x+1][y] != 9 && !parents.iter().any(|&p| p == (x+1,y)){
            in_baisins((x+1,y),parents,limits,board,ans);
        }
    }

    board[x][y] = 9;
}
fn check_baisin_size(input: (usize,usize),board: &mut Vec<Vec<u32>>) -> u32 {

    let (x,y) = input;
    // println!("Neigh: {:?}",input);
    let sizex = board.len()-1;
    let sizey = board[0].len()-1;
    let (mut _x,mut _y) = (x,y);
    let mut total_size = 0;
    let mut parents : Vec<(usize,usize)> = Vec::new();
    in_baisins(input,&mut parents,(sizex,sizey),board,&mut total_size);
    println!("baisin size: {:?}:{}",input,total_size);
    return total_size;
}
fn check_neighbours(input: (usize,usize),board: &Vec<Vec<u32>>) -> bool {

    let (x,y) = input;
    // println!("Neigh: {:?}",input);
    let min_value : u32 = board[x][y];
    let sizex = board.len()-1;
    let sizey = board[0].len()-1;
    let mut neighbours : Vec<(usize,usize)> = Vec::new();
    if x>0 {
        neighbours.push((x-1,y));
    }
    if x < sizex {
        neighbours.push((x+1,y));
    }
    if y > 0 {
         neighbours.push((x,y-1));
    }
    if y < sizey {
         neighbours.push((x,y+1));

    }

    for point in neighbours {
        if min_value>=board[point.0][point.1] {
            return false;

        }
    }
    // println!("minimum!,{:?}:{}",input,min_value);
    return true;
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
    }
}
