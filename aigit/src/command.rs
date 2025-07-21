use clap::Parser;

/*
 * 基于 clap 实现 aigit 的命令行参数解析
 */

 #[derive(Parser)]
 struct Cli {
     #[command(subcommand)]
     command: Commands,
 }
 
 #[derive(clap::Subcommand, Debug)]
 enum Commands {
     /// 上传文件
     Upload { 
         #[arg(short)] file: String 
     },
     /// 删除文件
     Delete { 
         #[arg(short)] id: u32 
     },
 }

#[derive(Parser, Debug)]
#[command(version, about)] // 自动生成版本/帮助信息
pub struct Args {
    /// 用户名（必填）
    #[arg(short, long)] 
    name: String,

    /// 执行次数（可选，默认值=1）
    #[arg(short, long, default_value_t = 1)]
    count: u8,

    /// 启用调试模式（布尔标志）
    #[arg(short, long)]
    debug: bool,
}

pub fn parse_args() -> Args {
    let args = Args::parse(); // 解析参数
    for _ in 0..args.count {
        println!("Hello, {}!", args.name);
    }
    if args.debug {
        println!("[DEBUG] Execution complete");
    }
    args
}

pub fn parse_subcommand() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Upload { file } => println!("Uploading: {}", file),
        Commands::Delete { id } => println!("Deleting ID: {}", id),
    }
}