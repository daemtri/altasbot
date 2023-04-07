use clap::Args;
use tokio::signal;

#[derive(Args)]
pub struct RunArgs {
    /// 服务名称
    #[arg(value_name = "NAME")]
    name: Option<String>,

    /// 设置matrix账号，形如: @xxx:matrix.org
    #[arg(short, long, env = "ALTAS_ACCOUNT")]
    account: String,

    /// 设置matrix密码
    #[arg(short, long, env = "ALTAS_PASSWORD")]
    password: String,
}

impl RunArgs {
    pub async fn run(&self) -> anyhow::Result<()> {
        let default_panic = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            default_panic(info);
            std::process::exit(1);
        }));

        let bot = crate::matrixbot::bot::Bot {
            user_id: self.account.clone(),
            password: self.password.clone(),
        };
        tokio::spawn(async move {
            let result = bot.run().await;
            if let Err(err) = result {
                panic!("Unable to run bot: {}", err);
            }
        });
        match signal::ctrl_c().await {
            Ok(()) => Ok(()),
            Err(err) => {
                eprintln!("Unable to listen for shutdown signal: {}", err);
                Err(err.into())
            }
        }
    }
}
