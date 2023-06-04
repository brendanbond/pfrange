use pfrange::parser::{Hand, Parser};
use std::io;
use std::io::Write;

fn main() {
    println!("PFRange v0.0.1");
    println!("Press Ctrl+c to exit\n");

    loop {
        print!("Enter ranges: ");
        io::stdout().flush().unwrap();
        let mut ranges = String::new();

        io::stdin()
            .read_line(&mut ranges)
            .expect("Error getting range");
        let mut range_vector = ranges.split(",").collect::<Vec<&str>>();
        let mut results: Vec<Vec<Hand>> = Vec::new();
        for i in 0..range_vector.len() {
            range_vector[i] = range_vector[i].trim();
            let mut parser: Parser = Parser::new(range_vector[i]);
            match parser.parse_range() {
                Ok(hands) => {
                    results.push(hands);
                }
                Err(e) => {
                    panic!("{}", e);
                }
            };
        }
        println!("{:?}", results.into_iter().flatten().collect::<Vec<_>>());
    }
}
