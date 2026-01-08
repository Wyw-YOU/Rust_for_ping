use clap::Parser;
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(
    name = "command_ping",
    version,
    about = "A ping utility written in Rust",
    author,
    after_help = "Example:\n  command_ping www.baidu.com -c 4 -t 1"
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

impl Args {
    pub fn timeout_duration(&self) -> Duration {
        Duration::from_secs_f32(self.timeout)
    }

    pub fn interval_duration(&self) -> Duration {
        Duration::from_secs_f32(self.interval)
    }

    // 👉 我们自己做“合法性检查”
    pub fn validate(&self) -> Result<(), String> {
        if self.count == 0 {
            return Err("count must be greater than 0".into());
        }
        if self.timeout <= 0.0 {
            return Err("timeout must be greater than 0".into());
        }
        if self.interval <= 0.0 {
            return Err("interval must be greater than 0".into());
        }
        if self.size == 0 || self.size > 1500 {
            return Err("size must be between 1 and 1500".into());
        }
        Ok(())
    }
}
