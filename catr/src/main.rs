
fn main() {
    if let Err(e) = cat-clone::get_args().and_then(cat-clone::run) {
        eprintln!("{}", e);
        std::process:exit(1);
    }
}
