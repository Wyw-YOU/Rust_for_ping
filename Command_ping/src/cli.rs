use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "Command_ping",
    version,
    about = "A ping utility written in Rust",
    author
)]
pub struct Args {
    /// Target hostname or IP address
    pub host: String,
    
    /// Number of echo requests to send
    #[arg(short = 'c', long, default_value_t = 4)]
    pub count: u16,
    
    /// Timeout in seconds
    #[arg(short = 't', long, default_value_t = 1.0)]
    pub timeout: f32,
    
    /// Interval between requests in seconds
    #[arg(short = 'i', long, default_value_t = 1.0)]
    pub interval: f32,
    
    /// Size of payload in bytes
    #[arg(short = 's', long, default_value_t = 56)]
    pub size: usize,
}