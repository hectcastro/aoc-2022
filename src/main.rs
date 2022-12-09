use std::process::Command;

fn main() {
    for day in 1..=8 {
        let day = format!("{day:02}");
        let cmd = Command::new("cargo")
            .args(["run", "--release", "--bin", &day])
            .output()
            .unwrap();

        println!("Day {day}:\n");

        let output = String::from_utf8(cmd.stdout).unwrap();
        if output.is_empty() {
            println!("TBD");
        }

        println!("{output}");
    }
}
