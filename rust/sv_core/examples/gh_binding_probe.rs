#[path = "../../../tests/row7_gh/probe.rs"]
mod probe;
fn main() {
    match probe::report() {
        Ok(report) => println!("{report}"),
        Err(error) => { eprintln!("{error}"); std::process::exit(1); }
    }
}
