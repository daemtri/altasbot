use clap::Args;

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
        let bot = crate::matrixbot::bot::Bot {
            user_id: self.account.clone(),
            password: self.password.clone(),
        };
        bot.run().await?;
        Ok(())
    }
}
