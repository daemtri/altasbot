use anyhow::{self, Context};
use matrix_sdk::{
    config::SyncSettings,
    room::Room,
    ruma::{
        events::{
            room::message::{
                MessageType, OriginalSyncRoomMessageEvent, Relation, RoomMessageEvent,
                RoomMessageEventContent, SyncRoomMessageEvent,
            },
            AnySyncTimelineEvent,
        },
        UserId,
    },
    Client,
};
use regex::Regex;

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

        client.add_event_handler(
            |ev: OriginalSyncRoomMessageEvent, room: Room, client: Client| async move {
                let Room::Joined(room) = room else {return ;};
                let MessageType::Text(text_content) = ev.content.msgtype else {return ;};
                println!("Received message: {}", text_content.body);
                //    "formatted_body": "Hello <a href='https://matrix.to/#/@alice:example.org'>Alice</a>!"
                let Some(formatted) = text_content.formatted else {return ;};
                let mention_regexp =
                    Regex::new(r#"<a\s+href='https://matrix.to/#/(?P<account>[^']+)'>[^<]+</a>"#)
                        .unwrap();
                if let Some(mentions) = mention_regexp.captures(formatted.body.as_str()) {
                    // 遍历判断是否提到自己
                    let user = client.user_id().unwrap();
                    for mention in mentions.iter() {
                        if let Some(mention) = mention {
                            let mention = mention.as_str();
                            if mention == user.to_string() {
                                println!("Mentioned by {}", ev.sender);
                                room.typing_notice(true).await.unwrap();
                                break;
                            }
                        }
                    }
                }
                let timeline_event = room.event(&ev.event_id).await.unwrap();
                let event_content = timeline_event
                    .event
                    .deserialize_as::<RoomMessageEvent>()
                    .unwrap();
                let original_message = event_content.as_original().unwrap();

                let content = RoomMessageEventContent::text_plain("Hello World!")
                    .make_reply_to(original_message);
                room.send(content, None).await.unwrap();
                println!("message sent");
            },
        );

        let result = client.sync(SyncSettings::default()).await.unwrap();
        Ok(result)
    }
}
