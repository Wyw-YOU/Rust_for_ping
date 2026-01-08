use clap::Parser;
use colored::*;

mod cli;
mod ping;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = cli::Args::parse();
    
    if let Err(msg) = args.validate() {
        eprintln!("Invalid arguments: {}", msg);
        std::process::exit(2);
    }
    
    
    println!("{}", format!("Command_ping v{}", env!("CARGO_PKG_VERSION")).cyan().bold());
    println!("Pinging {}...\n", args.host.green());
    
    match ping::ping_host(&args).await {
        Ok(stats) => {
            println!("\n--- Ping statistics for {} ---", args.host);
            println!("Packets: Sent = {}, Received = {}, Lost = {} ({:.1}% loss)",
                     stats.sent, stats.received, stats.lost, stats.loss_percentage);
            
            if stats.received > 0 {
                println!("Round-trip times (ms):");
                println!("  Min = {:.1}, Max = {:.1}, Avg = {:.1}", 
                         stats.min_rtt, stats.max_rtt, stats.avg_rtt);
            }
            Ok(())
        }
        Err(e) => {
            eprintln!("{}: {}", "Error".red().bold(), e);
            std::process::exit(1);
        }
    }
}