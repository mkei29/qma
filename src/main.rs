
extern crate qma;
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    config_path: String,    
    log_path: Option<String>
}


fn parse_args() -> Args{
    Args::parse()
}

fn main () {
    let args = parse_args();
    let filename :Option<&str> = args.log_path.as_deref();
    qma::run(&args.config_path, filename);
}
