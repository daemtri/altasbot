use clap::{Parser, Subcommand};
mod easy;

#[derive(Parser)]
#[command(author, version ,about,long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "启动服务", long_about = "xxx启动服务xxx")]
    Run {
        name: Option<String>,
        #[arg(short, help = "Pass `-h` and you'll see me!", env = "MYAPP_BAR")]
        bar: String,
    },
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Run { name, bar } => {
            println!("myapp add was used, name is: {:?}, {bar}", name)
        }
    }
}
