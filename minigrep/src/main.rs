use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        // --snip--
        eprintln!("Application error: {e}");

        process::exit(1);
    }
    // dbg!(args);

    //        let query = &args[1];
    //        let file_path = &args[2];
    //
    //        println!("Searching for {}", query);
    //        println!("In file {}", file_path);
    //
    //        let contents = fs::read_to_string(file_path)
    //            .expect("Should have been able to read the file");
//
//        println!("With text:\n{contents}");
    // let config = Config::new(&args);

    //    let config = Config::build(&args).unwrap_or_else(|err| {
    //        println!("Problem parsing arguments: {err}");
//        process::exit(1);
//    });
}

// impl Config {
//     fn new(args: &[String]) -> Config {
//         if args.len() < 3 {
//             // panic!("not enough arguments");
//             return Err("not enough arguments");
//         }
// 
//         let query = args[1].clone();
//         let file_path args[2].clone();
// 
//         Ok(Config { query, file_path })
//     }
// }
// 
// struct Config {
//     query: String,
//     file_path: String,
// }
// 
// fn parse_config(args: &[String]) -> (&str, &str) {
//     // let query = &args[1];
//     // let file_path = &args[2];
// 
//     // (query, file_path)
//     
//     let query = args[1].clone();
//     let file_path = args[2].clone();
// 
//     Config { query, file_path }
// }
