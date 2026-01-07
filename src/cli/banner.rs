//! Banner display for rbuster

use colored::*;

pub fn print_banner() {
    let banner = r#"
    ____  __               __           
   / __ \/ /_  __  _______/ /____  _____
  / /_/ / __ \/ / / / ___/ __/ _ \/ ___/
 / _, _/ /_/ / /_/ (__  ) /_/  __/ /    
/_/ |_/_.___/\__,_/____/\__/\___/_/     
                                         "#;

    println!("{}", banner.bright_cyan().bold());
    println!("{}", "═".repeat(60).bright_blue());
    println!("{} {}", "rbuster".bright_green().bold(), "v1.0.0".white());
    println!(
        "{}",
        "Blazingly fast directory/DNS buster written in Rust".white()
    );
    println!("{}", "═".repeat(60).bright_blue());
}

pub fn print_config(mode: &str, config: &[(&str, String)]) {
    println!(
        "[{}] Mode:                     {}",
        "+".bright_green(),
        mode.bright_yellow()
    );
    for (key, value) in config {
        println!(
            "[{}] {:25} {}",
            "+".bright_green(),
            format!("{}:", key),
            value.bright_white()
        );
    }
    println!("{}", "═".repeat(60).bright_blue());
    println!("Starting rbuster in {} mode", mode.bright_yellow().bold());
    println!("{}", "═".repeat(60).bright_blue());
}

pub fn print_finished(found: usize, total: usize, duration: std::time::Duration) {
    println!();
    println!("{}", "═".repeat(60).bright_blue());
    println!("{}", "Finished".bright_green().bold());
    println!("{}", "═".repeat(60).bright_blue());
    println!(
        "[{}] Found: {} | Tested: {} | Duration: {:.2}s",
        "*".bright_cyan(),
        found.to_string().bright_green(),
        total.to_string().bright_white(),
        duration.as_secs_f64()
    );
}
