//! OlaForge 主入口
//! 
//! 使用方式:
//!   olaforge execute --code "print('hello')" --language python
//!   olaforge serve --port 7860

use clap::{Parser, Subcommand};
use anyhow::Result;

mod executor;
mod sandbox;
mod security;
mod protocol;
mod api;

use executor::Executor;
use protocol::{Command, Response};

#[derive(Parser)]
#[command(name = "olaforge")]
#[command(about = "OlaForge - 智能安全沙箱系统")]
#[command(long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// 详细输出
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// 执行代码
    Execute {
        /// 要执行的代码
        #[arg(short, long)]
        code: String,
        
        /// 编程语言
        #[arg(short, long, default_value = "python")]
        language: String,
        
        /// 安全等级 L0-L3
        #[arg(short, long, default_value = "L2")]
        security: String,
        
        /// 超时时间(秒)
        #[arg(short, long, default_value_t = 60)]
        timeout: u64,
    },

    /// 启动 API 服务
    Serve {
        /// 监听端口
        #[arg(short, long, default_value_t = 7860)]
        port: u16,
    },

    /// 健康检查
    Health,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // 初始化日志
    tracing_subscriber::fmt()
        .with_max_level(if cli.verbose { 
            tracing::Level::DEBUG 
        } else { 
            tracing::Level::INFO 
        })
        .init();

    match cli.command {
        Commands::Execute { code, language, security, timeout } => {
            tracing::info!("执行代码: {} (安全: {})", language, security);
            
            let executor = Executor::new(&security)?;
            let result = executor.execute(&code, &language, timeout).await?;
            
            println!("{}", serde_json::to_string(&result)?);
        }
        
        Commands::Serve { port } => {
            tracing::info!("启动服务: 端口 {}", port);
            api::serve(port).await?;
        }
        
        Commands::Health => {
            println!(r#"{"status":"healthy","version":"1.0.0"}"#);
        }
    }

    Ok(())
}
