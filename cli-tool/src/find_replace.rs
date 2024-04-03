use text_colorizer::*;
use std::env;
use std::fs;
#[derive(Debug)]
#[allow(dead_code)]
struct Argument{
    pattern:String,
    replace:String,
    input_file:String,
    output_file:String
}

fn print_help(){
    eprintln!("{}-replace a string with a new string","Find and replace".green().bold());
    eprintln!("Usage <Target String> <Replacement string> <input file> <output file>");
}
fn read_and_write(args:Argument){
    let data=match fs::read_to_string(&args.input_file){
        Ok(f)=>f,
        Err(e)=>{
            eprintln!("{} failed to read from file {}: {:?}","Error".red().bold(),args.input_file,e );
            std::process::exit(1);
        }
    };
    match  fs::write(&args.output_file,&data){
        Ok(_)=>{},
        Err(e)=>{
            eprintln!("{} failed to read from file {}: {:?}","Error".red().bold(),args.output_file,e );
            std::process::exit(1);
        }
        
    }

}
fn parse_args()->Argument{
    let args:Vec<String>=env::args().skip(1).collect();
    if args.len()!=4{
        print_help();
        eprintln!("{} wrong number of arguments given.Expected 4 but got {:?}", "Error".red().bold(),args.len());
        std::process::exit(1);
    }
    Argument{
        pattern:args[0].clone(),
        replace:args[1].clone(),
        input_file:args[2].clone(),
        output_file:args[3].clone()
    }
}

pub fn run(){
    let args = parse_args();
    read_and_write(args);
    
   
}