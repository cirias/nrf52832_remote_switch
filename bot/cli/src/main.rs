use tgbot;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bot = tgbot::TelegramBot::new("").expect("could not create bot");

    let mut get_update_params =  tgbot::types::GetUpdatesParams {
        limit: Option::Some(10),
        allowed_updates: Option::None,
        ..Default::default()
    };
    loop {
        let updates = bot.get_updates(&get_update_params).await.expect("could not get updates from bot");
        for update in &updates {
            handle_update(&bot, update).await.expect("could not handle update");
            get_update_params.offset = Option::Some(update.update_id + 1);
        }
    } 
}

async fn handle_update(bot: &tgbot::TelegramBot, update: &tgbot::types::Update) -> Result<(), Box<dyn std::error::Error>> {
    let message = match &update.message {
        None => return Ok(()),
        Some(message) => message,
    };

    let text = match &message.text {
        None => return Ok(()),
        Some(message) => message,
    };

    let send_message_params = tgbot::types::SendMessageParams {
        chat_id: message.chat.id,
        text: text.into(),
        ..Default::default()
    };
    bot.send_message(&send_message_params).await?;
    Ok(())
}
