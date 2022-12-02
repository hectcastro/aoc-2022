use std::process::Command;

fn main() {
    for day in 1..=2 {
        let day = format!("{:02}", day);
        let cmd = Command::new("cargo")
            .args(["run", "--release", "--bin", &day])
            .output()
            .unwrap();

        println!("Day {}:\n", day);

        let output = String::from_utf8(cmd.stdout).unwrap();
        if output.is_empty() {
            println!("TBD");
        }

        println!("{}", output);
    }
}
