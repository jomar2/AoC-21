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
    let mut total_flashed : u64 = 0;
    let mut iter_flashed : u64 = 0;

    for x in 0..{
        board.iter_mut().flatten().for_each(|x| *x += 1);
        update_flash(&mut board,(sizex-1,sizey-1),&mut total_flashed,&mut iter_flashed);
        if iter_flashed == 100 {
            println!("part B: Round {}",x);
            break;
        }
    }
    // 234 too low..235 correct , A: 1665  
    println!("Answer1: {:?}\nAnswer2: {}",total_flashed,2);
}

fn update_flash(board: &mut Vec<Vec<u32>>,limits : (usize,usize),total_flashed:&mut u64, iter_flashed:&mut u64)
{
    let mut flashed : Vec<(usize,usize)> = Vec::new();
    let (sizex,sizey) = limits;
    for col in 0..sizex+1 {
        for row in 0..sizey+1 {
            if board[col][row] > 9 && flashed.iter().find(|p| **p==(col,row))  == None {
                flashed.push((col,row));
                flash((col,row),&mut flashed,limits,board);
            }
        }
    }

    board.iter_mut().flatten().for_each(|x| if *x > 9 {*x=0});
    *total_flashed += flashed.len() as u64;
    *iter_flashed = flashed.len() as u64;
    //println!("flashed: {}",flashed.len());
    flashed.clear();
}

fn flash(coord: (usize,usize),flashed: &mut Vec<(usize,usize)>, limits : (usize,usize), board: &mut Vec<Vec<u32>>)
{

    let (x,y) = coord;
    let (sizex,sizey) = limits;
    let mut neighbours : Vec<(usize,usize)> = Vec::new();
    if x>0 {
        neighbours.push((x-1,y));
        if y>0 {neighbours.push((x-1,y-1));}
        if y<sizey {neighbours.push((x-1,y+1));}
    }
    if x < sizex {
        neighbours.push((x+1,y));
        if y>0 {neighbours.push((x+1,y-1));}
        if y<sizey {neighbours.push((x+1,y+1));}
    }
    if y > 0 {
         neighbours.push((x,y-1));
    }
    if y < sizey {
         neighbours.push((x,y+1));
    }

    for n in &neighbours {
        board[n.0][n.1] += 1;
    }

    for n in &neighbours {
        if board[n.0][n.1] > 9 && flashed.iter().find(|p| **p==(n.0,n.1)) == None {
            flashed.push((n.0,n.1));
            flash((n.0,n.1),flashed,limits,board);
        }
    }

}
