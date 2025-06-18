use std::io;
use std::process::Command;

fn shell_exists(shell: &str) -> bool {
    Command::new("which")
    .arg(shell)
    .status()
    .map(|s| s.success())
    .unwrap_or(false)
}

fn main() -> io::Result<()> {
    let shell = if shell_exists("bash") {
        "bash"
    } else {
        "sh"
    };

    let output = Command::new(shell)
        .arg("-c")
        .arg("pgrep -f 'kate -b' | head -1")
        .output()?;

    // Sweet PID for you
    if output.status.success() {
        let pid = String::from_utf8_lossy(&output.stdout);
        println!("Найден PID: {}", pid.trim());
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        println!("Ошибка: {}", error);
    }
    Ok(())
}
