use std::env;

mod element;
mod periodic_table;

fn main() {
    // load the periodic table elements
    let periodic_table = periodic_table::PeriodicTable::init("./resources/periodic_table".to_string());
    
    // read command line arguments (if any)
    let args: Vec<String> = env::args().collect();
    if args.len() > 0 {
        if args[1] == "quiz".to_string() {
            let score = periodic_table.quiz(5u8);
            println!("Quiz results: {} / {} ", score, 5u8);
        }
    }
    else {
        println!("Number of elements in periodic table file: {}", periodic_table.total_elements());
    }
}
