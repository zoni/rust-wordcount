fn main() {
    if let Err(err) = wordcount::run() {
        eprintln!("Error: {:?}", err);
        std::process::exit(1);
    }
}
