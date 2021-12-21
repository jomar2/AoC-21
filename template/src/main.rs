use std::env;

use aoc_reader::reading_file;

fn main() {
    let input_data = reading_file(env::args());
    // println!("Answer1: {}\nAnswer2: {}", counter(input_data.clone()),counter2(input_data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let indata = "(())".to_string();
        assert_eq!(0,counter(indata.clone()),"input: {}", indata);
    }
    #[test]
    fn test2(){
        let indata = "()()".to_string();
        assert_eq!(0,counter(indata.clone()),"input: {}", indata);
    }
    #[test]
    fn test3(){
        let indata = "(((".to_string();
        assert_eq!(3,counter(indata.clone()),"input: {}", indata);
    }
    #[test]
    fn test4(){
        let indata = "(()(()(".to_string();
        assert_eq!(3,counter(indata.clone()),"input: {}", indata);
    }
    #[test]
    fn test5(){
        let indata = "))(((((".to_string();
        assert_eq!(3,counter(indata.clone()),"input: {}", indata);
    }
    #[test]
    fn test6(){
        let indata = "())".to_string();
        assert_eq!(-1,counter(indata.clone()),"input: {}", indata);
    }
    #[test]
    fn test7(){
        let indata = "))(".to_string();
        assert_eq!(-1,counter(indata.clone()),"input: {}", indata);
    }
    #[test]
    fn test8(){
        let indata = ")))".to_string();
        assert_eq!(-3,counter(indata.clone()),"input: {}", indata);
    }
    #[test]
    fn test9(){
        let indata = ")())())".to_string();
        assert_eq!(-3,counter(indata.clone()),"input: {}", indata);
    }
    #[test]
    fn test10(){
        let indata = ")".to_string();
        assert_eq!(1,counter2(indata.clone()),"input: {}", indata);
    }
    #[test]
    fn test11(){
        let indata = "()())".to_string();
        assert_eq!(5,counter2(indata.clone()),"input: {}", indata);
    }










}
