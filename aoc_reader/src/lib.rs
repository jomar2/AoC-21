use std::env;
use std::fs;

pub fn reading_file(args: env::Args) -> String {
    let args: Vec<String> = args.collect();
    if args.len() < 2 {
        panic!("Not enough inputs..");
    }
    fs::read_to_string(&args[1]).expect("Missing file!")
}




#[cfg(test)]

mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
