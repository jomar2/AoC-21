use std::env;
use aoc_reader::reading_file;
fn add_mark (board: &mut Vec<i32>,new_value: i32) 
{
    let index = match board.iter().position(|val| *val == new_value){
        Some(idx) => idx,
        None => {
            println!("{} is not on the board",new_value);
            return;
        },
    };
    board[index] = -1;
 //   println!("index values: {:?},{:?}",index, new_value);
}

fn count_remainder(board: Vec<i32>) -> i32 {
    let total = board.iter().filter(|&&elem| elem != -1).sum();
    println!("Total: {}", total);
    return total;
}

fn check_board (board: Vec<i32>) -> (bool,i32)
{
    let mut sum = 0;
    let mut bingo = false;
    let current_board_rows: Vec<Vec<i32>> = board.chunks(5).map(|s| s.into()).collect();
    let mut current_board_columns : Vec<Vec<i32>> = Vec::new();
    for index in 0..4 {
        let tmp_vector : Vec<i32>= board.clone().into_iter().enumerate().filter(|(i,_)| i % 5 == index).map(|(_,e)| e).collect();
        current_board_columns.push(tmp_vector.clone());
    }

    println!("boardrows: {:?}", current_board_rows);
    println!("boardcolumns: {:?}", current_board_columns);
    

    for row in current_board_rows {
        if row.iter().filter(|&&elem| elem==-1).count() == 5 {
            bingo = true;
            sum = count_remainder(board);
            return(bingo,sum);
        }
    }
    for col in current_board_columns {
            if col.iter().filter(|&&elem| elem==-1).count() == 5 {
            bingo = true;
            sum = count_remainder(board);
            return(bingo,sum);
        }
    }
//    let mut board_iter = board.into_iter().peekable();
//    while board_iter.peek().is_some(){
//        let row: Vec<_> = board_iter.by_ref().take(10);     

   return (bingo,sum)
}
fn main() {
    let input_data = reading_file(env::args());
    let (seq, brds) = input_data.split_once("\n\n").unwrap();
    let sequence : Vec<_> = seq.split(",").map(|s| s.parse::<i32>().unwrap()).collect();
    let boards :Vec<_> = brds.split("\n\n").collect(); 
    let mut boardz: Vec<Vec<i32>> = boards.iter().map(|brd| brd.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect()).collect();
    let nr_boards = boardz.len();
    let mut nr_bingos = 0;
    //    let board1 :Vec<i32> = boards[0].split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();


    println!("{:?}\n\n",sequence);
    for number in sequence {
        for mut board in &mut boardz {
            add_mark(&mut board,number);
            println!("{:?}: size: {}\n\n",board,board.len());
            let (bingo, sum) = check_board(board.to_vec());
            if bingo
            {
                nr_bingos += 1;
                if nr_bingos == nr_boards
                {
                    println!("Last to bingo! ans: {}",sum*number);
                    return;
                }
                else
                {
                    board.clear();
                }
            }
        }
    }
// Wrong answers:  12924 too Low  
// A: 34506
// B: 7686
}
