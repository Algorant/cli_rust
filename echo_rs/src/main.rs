use clap::Command;

fn main() {
    let _matches = Command::new("echo_rs")
        .version("0.1.0")
        .author("Algorant <Algorant@protonmail.com>")
        .about("Rust version of echo")
        .get_matches();
}
