#[path = "../../../tests/row7_bindings/cases.rs"]
mod cases;

fn main() {
    match cases::report() {
        Ok(report) => println!("{report}"),
        Err(error) => { eprintln!("{error}"); std::process::exit(1); }
    }
}
