use thiserror::Error;

#[derive(Error, Debug)]
pub enum PingError {
    #[error("Network error: {0}")]
    NetworkError(#[from] std::io::Error),
    
    #[error("Invalid host: {0}")]
    InvalidHost(String),
    
    #[error("Timeout after {0}s")]
    Timeout(f32),
    
    #[error("ICMP error: {0}")]
    IcmpError(String),
}