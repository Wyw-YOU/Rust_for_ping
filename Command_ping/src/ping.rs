use std::net::IpAddr;
use surge_ping::{Client, Config, PingIdentifier, PingSequence, IcmpPacket};
use tokio::time;
use colored::*;

#[derive(Debug)]
pub struct PingStatistics {
    pub sent: u16,
    pub received: u16,
    pub lost: u16,
    pub loss_percentage: f32,
    pub min_rtt: f32,
    pub max_rtt: f32,
    pub avg_rtt: f32,
}

pub async fn ping_host(args: &crate::cli::Args) -> anyhow::Result<PingStatistics> {
    // 解析主机地址（支持域名）
    let addr = resolve_host(&args.host).await?;
    
    // 创建客户端
    let config = Config::default();
    let client = Client::new(&config)?;
    
    // 准备payload
    let payload = vec![0; args.size];
    let identifier = PingIdentifier(123);
    
    let mut stats = PingStatistics {
        sent: 0,
        received: 0,
        lost: 0,
        loss_percentage: 0.0,
        min_rtt: f32::MAX,
        max_rtt: 0.0,
        avg_rtt: 0.0,
    };
    
    let mut rtts: Vec<f32> = Vec::new();
    
    println!("Pinging {} [{}] with {} bytes of data:", args.host, addr, args.size);
    
    // 创建Pinger
    let mut pinger = client.pinger(addr, identifier).await;
    
    for seq in 0..args.count {
        stats.sent += 1;
        
        // 等待间隔时间
        if seq > 0 {
            time::sleep(args.interval_duration()).await;
        }
        
        // 使用tokio的timeout来设置超时
        match time::timeout(
            args.timeout_duration(), 
            pinger.ping(PingSequence(seq as u16), &payload)
        ).await {
            Ok(Ok((packet, duration))) => {
                stats.received += 1;
                let rtt_ms = duration.as_secs_f32() * 1000.0;
                rtts.push(rtt_ms);
                
                // 根据返回的包类型显示不同的信息
                match packet {
                    IcmpPacket::V4(v4_packet) => {
                        // 处理可能的None值
                        let ttl = v4_packet.get_ttl().unwrap_or(0);
                        println!("Reply from {}: bytes={} time={:.1}ms TTL={}",
                                 v4_packet.get_source(),
                                 v4_packet.get_size(),
                                 rtt_ms,
                                 ttl);
                    }
                    IcmpPacket::V6(v6_packet) => {
                        println!("Reply from {}: bytes={} time={:.1}ms",
                                 v6_packet.get_source(),
                                 v6_packet.get_size(),
                                 rtt_ms);
                    }
                }
            }
            Ok(Err(e)) => {
                stats.lost += 1;
                println!("{}: {}", "Error".red(), e);
            }
            Err(_) => {
                stats.lost += 1;
                println!("{}: Request timed out.", "Timeout".yellow());
            }
        }
    }
    
    // 计算统计信息
    if !rtts.is_empty() {
        stats.min_rtt = rtts.iter().fold(f32::MAX, |a: f32, &b| a.min(b));
        stats.max_rtt = rtts.iter().fold(0.0_f32, |a: f32, &b| a.max(b));
        stats.avg_rtt = rtts.iter().sum::<f32>() / rtts.len() as f32;
    }
    
    stats.loss_percentage = if stats.sent > 0 {
        (stats.lost as f32 / stats.sent as f32) * 100.0
    } else {
        0.0
    };
    
    Ok(stats)
}

// DNS解析函数
async fn resolve_host(host: &str) -> anyhow::Result<IpAddr> {
    // 先尝试直接解析为IP地址
    if let Ok(ip) = host.parse::<IpAddr>() {
        return Ok(ip);
    }
    
    // 如果是域名，进行DNS解析
    println!("Resolving {}...", host);
    
    // 使用tokio的异步DNS解析
    match tokio::net::lookup_host(format!("{}:0", host)).await {
        Ok(mut addresses) => {
            if let Some(addr) = addresses.next() {
                let ip = addr.ip();
                println!("Resolved {} to {}", host, ip);
                return Ok(ip);
            }
            anyhow::bail!("Could not resolve host: {}", host)
        }
        Err(e) => {
            anyhow::bail!("DNS resolution failed for {}: {}", host, e)
        }
    }
}