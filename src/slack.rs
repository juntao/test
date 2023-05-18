use slack_flows::{listen_to_channel, send_message_to_channel};
use openai_flows::{chat_completion, ChatOptions};
use std::env;

#[no_mangle]
pub fn run() {
    let workspace = env::var("workspace").unwrap_or_else(|_| "secondstate".to_string());
    let channel = env::var("channel").unwrap_or_else(|_| "collaborative-chat".to_string());
    let openai_key_name = env::var("openai_key_name").unwrap_or_else(|_| "gpt4".to_string());

    listen_to_channel(&workspace, &channel, |sm| {
        let chat_id = workspace.clone() + &channel;
        let c = chat_completion(&openai_key_name, &chat_id, &sm.text, &ChatOptions::default());
        if let Some(c) = c {
            if c.restarted {
                send_message_to_channel(&workspace, &channel, "Let's start a new conversation!".to_string());
            }
            send_message_to_channel(&workspace, &channel, c.choice);
        }
    });
}
