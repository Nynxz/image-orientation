use assert_cmd::Command as AssertCommand;
use std::process::Output;
use std::str;

#[cfg(test)]
mod tests {
    use super::*;

    fn run_cli(args: &[&str]) -> Output {
        AssertCommand::cargo_bin("imageorientation")
            .unwrap()
            .args(args)
            .output()
            .expect("Failed to execute command")
    }

    #[test]
    fn test_formatting_output() {
        let path = "tests/test";
        let output = run_cli(&[
            "--mode",
            "orientation",
            "--format",
            "{entry} at {path} with {result}",
            path,
        ]);

        let stdout = str::from_utf8(&output.stdout).unwrap();
        let stderr = str::from_utf8(&output.stderr).unwrap();
        println!("stdout: {}", stdout);
        println!("stderr: {}", stderr);

        assert!(stdout.contains("600x400.jpg at tests/test with landscape"));
        assert!(stdout.contains("600x400.png at tests/test with landscape"));

        assert!(stdout.contains("400x600.jpg at tests/test with portrait"));
        assert!(stdout.contains("400x600.png at tests/test with portrait"));
    }
    #[test]
    fn test_mode_output() {
        let path = "tests/test";
        let output = run_cli(&[
            "--mode",
            "resolution",
            "--format",
            "{entry} at {path} with {result}",
            path,
        ]);

        let stdout = str::from_utf8(&output.stdout).unwrap();
        let stderr = str::from_utf8(&output.stderr).unwrap();
        println!("stdout: {}", stdout);
        println!("stderr: {}", stderr);

        assert!(stdout.contains("600x400.jpg at tests/test with 600x400"));
        assert!(stdout.contains("600x400.png at tests/test with 600x400"));

        assert!(stdout.contains("400x600.jpg at tests/test with 400x600"));
        assert!(stdout.contains("400x600.png at tests/test with 400x600"));
    }
}
