fn main() {
    if let Err(e) = project::get_args().and_then(project::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}