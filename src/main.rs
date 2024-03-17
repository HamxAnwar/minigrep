use std::env;
use std::fs;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();
    let argument = Arguments::parse_arguments(&args);
    
    println!("Querry: {:?} - Filename: {:?}", argument.querry, argument.filename);

    if let Err(e) = read_file(argument){
        println!("Error: {}", e)
    }
}

struct Arguments {
    querry: String,
    filename: String
}

impl Arguments {
    fn parse_arguments(args: &[String]) -> Arguments {
        let querry = args[1].clone();
        let filename = args[2].clone();
        Arguments { querry, filename }
    }
}


fn read_file(argument: Arguments) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(argument.filename)?;
    println!("{:?}", contents);
    Ok(())
}
