mod state;
mod to_do;
mod processes;

use std::env;
use state::read_file;
use serde_json::value::Value;
use serde_json::Map;
use to_do::to_do_factory;
use processes::process_input;



fn main() {
 let args: Vec<String> = env::args().collect();
 if args.len() < 3 {
    eprintln!("Usage: {} <status> <title>", args[0]);
    std::process::exit(1);
}
    let command: &String = &args[1];
    let title: &String = &args[2];
    let state: Map<String, Value> =
    read_file("./state.json");


    let status = match state.get(title) {
        Some(result) => result.as_str().unwrap_or("pending").to_string(),
        None => "pending".to_string(),
    };

    let item = to_do_factory(&status,
    title).expect(&status);
    process_input(item, command.to_string(), &state);
}