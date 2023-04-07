mod cmd;
mod matrixbot;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version,about,long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "启动服务", long_about = "xxx启动服务xxx")]
    Run(cmd::run::RunArgs),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Run(args) => {
            args.run().await?;
        }
    }
    Ok(())
}
