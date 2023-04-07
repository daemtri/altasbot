use anyhow;
use matrix_sdk::ruma::UserId;
use matrix_sdk::Client;

pub struct Bot {
    pub user_id: String,
    pub password: String,
}

impl Bot {
    pub async fn run(&self) -> anyhow::Result<()> {
        let user = UserId::parse(&self.user_id).unwrap();

        let client: Client = Client::builder()
            .server_name(user.server_name())
            .build()
            .await?;

        client.login_username(&user, &self.password).send().await?;

        Ok(())
    }
}
