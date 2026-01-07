//! rbuster - Blazingly fast directory/DNS/vhost buster written in Rust
//!
//! A high-performance alternative to gobuster, written in Rust for maximum speed.
//!
//! Usage:
//!   rbuster dir -u https://example.com -w wordlist.txt
//!   rbuster dns -d example.com -w subdomains.txt
//!   rbuster vhost -u https://example.com -w vhosts.txt
//!   rbuster fuzz -u https://example.com/FUZZ -w wordlist.txt

use clap::Parser;
use colored::Colorize;
use std::time::Instant;

mod cli;
mod core;
mod error;
mod modes;
mod output;

use cli::{print_banner, print_config, print_finished, Cli, Commands};

#[tokio::main]
async fn main() {
    // Parse command line arguments
    let cli = Cli::parse();

    // Run the appropriate mode
    let result = run(cli).await;

    if let Err(e) = result {
        eprintln!("{} {}", "[ERROR]".bright_red(), e);
        std::process::exit(1);
    }
}

async fn run(cli: Cli) -> error::Result<()> {
    let start = Instant::now();

    // Determine if we should show the banner
    let quiet = match &cli.command {
        Commands::Dir(args) => args.global.quiet,
        Commands::Dns(args) => args.global.quiet,
        Commands::Vhost(args) => args.global.quiet,
        Commands::Fuzz(args) => args.global.quiet,
        Commands::S3(args) => args.global.quiet,
        Commands::Gcs(args) => args.global.quiet,
        Commands::Tftp(args) => args.global.quiet,
    };

    // Print banner
    if !quiet {
        print_banner();
    }

    // Get found count for final stats
    let found_count = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));

    // Execute the command
    match cli.command {
        Commands::Dir(args) => {
            if !quiet {
                print_config(
                    "directory enumeration",
                    &[
                        ("Url", args.url.clone()),
                        ("Method", args.http.method.clone()),
                        ("Threads", args.global.threads.to_string()),
                        ("Wordlist", args.global.wordlist.display().to_string()),
                        ("User Agent", args.http.user_agent.clone()),
                        ("Timeout", format!("{}s", args.http.timeout)),
                    ],
                );
            }
            modes::dir::run(args).await?;
        }
        Commands::Dns(args) => {
            if !quiet {
                print_config(
                    "DNS subdomain enumeration",
                    &[
                        ("Domain", args.domain.clone()),
                        ("Threads", args.global.threads.to_string()),
                        ("Wordlist", args.global.wordlist.display().to_string()),
                        (
                            "Resolver",
                            args.resolver
                                .clone()
                                .unwrap_or_else(|| "system".to_string()),
                        ),
                        ("Timeout", format!("{}s", args.timeout)),
                    ],
                );
            }
            modes::dns::run(args).await?;
        }
        Commands::Vhost(args) => {
            if !quiet {
                print_config(
                    "virtual host enumeration",
                    &[
                        ("Url", args.url.clone()),
                        ("Threads", args.global.threads.to_string()),
                        ("Wordlist", args.global.wordlist.display().to_string()),
                        ("Append Domain", args.append_domain.to_string()),
                    ],
                );
            }
            modes::vhost::run(args).await?;
        }
        Commands::Fuzz(args) => {
            if !quiet {
                print_config(
                    "fuzzing",
                    &[
                        ("Url", args.url.clone()),
                        ("Method", args.http.method.clone()),
                        ("Threads", args.global.threads.to_string()),
                        ("Wordlist", args.global.wordlist.display().to_string()),
                    ],
                );
            }
            modes::fuzz::run(args).await?;
        }
        Commands::S3(args) => {
            if !quiet {
                print_config(
                    "S3 bucket enumeration",
                    &[
                        ("Threads", args.global.threads.to_string()),
                        ("Wordlist", args.global.wordlist.display().to_string()),
                        ("Max Files", args.max_files.to_string()),
                    ],
                );
            }
            modes::s3::run(args).await?;
        }
        Commands::Gcs(args) => {
            if !quiet {
                print_config(
                    "GCS bucket enumeration",
                    &[
                        ("Threads", args.global.threads.to_string()),
                        ("Wordlist", args.global.wordlist.display().to_string()),
                        ("Max Files", args.max_files.to_string()),
                    ],
                );
            }
            modes::gcs::run(args).await?;
        }
        Commands::Tftp(args) => {
            if !quiet {
                print_config(
                    "TFTP file enumeration",
                    &[
                        ("Server", args.server.clone()),
                        ("Threads", args.global.threads.to_string()),
                        ("Wordlist", args.global.wordlist.display().to_string()),
                        ("Timeout", format!("{}s", args.timeout)),
                    ],
                );
            }
            modes::tftp::run(args).await?;
        }
    }

    // Print completion stats
    let duration = start.elapsed();
    let found = found_count.load(std::sync::atomic::Ordering::Relaxed);
    if !quiet {
        print_finished(found, 0, duration);
    }

    Ok(())
}
