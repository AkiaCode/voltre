use revolt::{client::Client, client::EventHandler, context::Context, model::{message::Message, user::FetchUser}};

#[tokio::main]
async fn main() {
    let mut client = Client::new("TOKEN".to_owned()).await;
    client.run(Handler).await;
}

#[derive(Copy, Clone)]
struct Handler;

#[async_trait::async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context) {
        println!("ready, username: {}({})", ctx.bot.username, ctx.bot._id);
    }

    #[allow(dead_code)]
    async fn authenticated(&self) {}

    async fn on_message(&self, ctx: Context, message: Message) {
        if message.author == ctx.bot._id {return;}

        match message.content.as_str() {
            "/ping" => {
                ctx.begin_typing(&message.channel).await;
                ctx.send_message("pong", None).await;
            },
            "/neko" => {
                ctx.begin_typing(&message.channel).await;
                ctx.reply(&nekoslife::get(nekoslife::SfwCategory::Neko).await.unwrap()).await;
            },
            "/help" => {
                ctx.begin_typing(&message.channel).await;
                ctx.reply(["# voltre Bot",
                    "> ## Command List",
                    ">> ### /help",
                    ">> `This command give help command to you`",
                    ">> ### /neko",
                    ">> `This command give a random neko girl (sfw)`",
                    ">> ### /invite",
                    ">> `You can invite me to your server :3`",
                    ">> ### /server",
                    ">> `Show Server Command`",
                    ">> ### /github",
                    ">> `Show Github Command`",
                    ">> ### /profile",
                    ">> `Show Profile Command`",
                    ">> ### /user {mention: Optional}",
                    ">> `Show User Command`",
                    "",
                    "> #### Special thanks",
                    "> <@01FCXFBQPYCBZWX40NSBYXYAWW>"].join("\n").as_str()).await;
            },
            "/invite" => {
                ctx.begin_typing(&message.channel).await;
                ctx.reply("> [Invite Bot](https://app.revolt.chat/bot/01FEAC4GEHGS0NVHBZPWDFVT24)").await;
            },
            "/server" => {
                ctx.begin_typing(&message.channel).await;
                ctx.reply("> [Server](https://app.revolt.chat/invite/k1shcdeh)").await;
            },
            "/github" => {
                ctx.begin_typing(&message.channel).await;
                ctx.reply("> [Github](https://github.com/AkiaCode/revolt.rs)").await;
            },
            "/profile" => {
                ctx.begin_typing(&message.channel).await;
                let profile = ctx.fetch_profile(&message.author).await;
                ctx.reply([
                    "## User Profile",
                    "> ### Content",
                    format!("> {}", &profile.content).as_str(),
                ].join("\n").as_str()).await;
            },
            cmd => {
                if cmd.starts_with("/user") {
                    ctx.begin_typing(&message.channel).await;
                    let user: FetchUser;
                    if let Some(mentions) = message.mentions {
                        user = ctx.fetch_user(&mentions[0]).await;
                    } else {
                        user = ctx.fetch_user(&message.author).await;
                    }

                    if let Some(status) = user.status {
                        ctx.reply(["## User Info",
                            "> ### Username",
                            format!("> {}", user.username).as_str(),
                            "> ### ID",
                            format!("> {}", user._id).as_str(),
                            "> ### Badges",
                            format!("> {}", user.badges).as_str(),
                            "> ### Flags",
                            format!("> {}", user.flags.unwrap_or(0)).as_str(),
                            "> ### Online",
                            format!("> {}", user.online).as_str(),
                            "> ### Relationship",
                            format!("> {}", user.relationship).as_str(),
                            "> ### Status Text",
                            format!("> {}", status.text.unwrap_or("None".to_string())).as_str(),
                            "> ### Status Presence",
                            format!("> {}", status.presence.unwrap_or("None".to_string())).as_str(),
                        ].join("\n").as_str()).await;
                    } else {
                        ctx.reply(["## User Info",
                            "> ### Username",
                            format!("> {}", user.username).as_str(),
                            "> ### ID",
                            format!("> {}", user._id).as_str(),
                            "> ### Badges",
                            format!("> {}", user.badges).as_str(),
                            "> ### Flags",
                            format!("> {}", user.flags.unwrap_or(0)).as_str(),
                            "> ### Online",
                            format!("> {}", user.online).as_str(),
                            "> ### Relationship",
                            format!("> {}", user.relationship).as_str(),
                        ].join("\n").as_str()).await;
                    }
                }
            },
        }

        ctx.end_typing(&message.channel).await;
    }
}