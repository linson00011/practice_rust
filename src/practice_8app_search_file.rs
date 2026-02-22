use practice_rust::printlns_simple;
use std::process;

pub fn practice_app_search_file_main() {
    let config = practice_rust::get_program_args().unwrap_or_else(|e| {
        eprintln!("{:?}", e);
        process::exit(1);
    });
    printlns_simple!(config);

    let res = practice_rust::run(&config);

    if let Err(e) = res {
        eprintln!("{:?}", e);
        process::exit(1);
    }
}
