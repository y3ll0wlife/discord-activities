use crate::discord::*;
use crate::verify::verify;
use reqwest;
use serde_json;
use worker::*;

mod discord;
mod utils;
mod verify;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or("unknown region".into())
    );
}

async fn validate_request(mut req: Request, public_key: String) -> Option<String> {
    let x_sig_ed = req.headers().get("X-Signature-Ed25519").unwrap();
    let x_sig_time = req.headers().get("X-Signature-Timestamp").unwrap();

    if x_sig_ed.is_none() || x_sig_time.is_none() {
        return None;
    }

    let body = req.text().await.unwrap();

    if verify(&x_sig_ed.unwrap(), &x_sig_time.unwrap(), &body, public_key) {
        return Some(body);
    }
    None
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);

    utils::set_panic_hook();
    let router = Router::new();

    router
        .get("/", |_, _| Response::ok("Hello from Workers!"))
        .post_async("/", |req, ctx| async move {
            let body = validate_request(req, ctx.secret("PUBLIC_KEY").unwrap().to_string()).await;

            if body.is_none() {
                console_log!("Failed to validate request");
                return Response::error("Bad Request", 400);
            }

            let i = serde_json::from_str::<Interaction>(&body.unwrap());
            if i.is_err() {
                console_log!("Failed to get the json from Discord {:#?}", i.err());
                return Response::error("Bad Request", 400);
            }
            let interaction = i.unwrap();

            if interaction.interaction_type == InteractionType::Ping {
                console_log!("Ping ----> Pong");
                return Response::from_json(&InteractionResponse {
                    data: None,
                    interaction_type: InteractionResponseType::Pong,
                });
            }

            if interaction.interaction_type == InteractionType::ApplicationCommand {
                let interaction_data = match interaction.data {
                    Some(data) => data,
                    None => return Response::error("Bad Request", 400),
                };

                if interaction_data.name.starts_with("activities") {
                    let options = interaction_data.options.unwrap();

                    let activity_id = options[0].value.as_ref().unwrap();
                    let channel_id = options[1].value.as_ref().unwrap();

                    let invite = ChannelInviteRequest {
                        max_age: 0,
                        max_uses: 0,
                        temporary: false,
                        unique: false,
                        target_type: 2,
                        target_application_id: activity_id.to_string(),
                    };

                    let url = format!(
                        "https://discord.com/api/v10/channels/{}/invites",
                        channel_id
                    );

                    let client = reqwest::Client::new();
                    let resp = client
                        .post(&url)
                        .json(&invite)
                        .header(
                            "Authorization",
                            format!("Bot {}", &ctx.secret("DISCORD_TOKEN").unwrap().to_string()),
                        )
                        .send()
                        .await;
                    let resp_json = resp.unwrap().json::<ChannelInviteResponse>().await.unwrap();

                    return Response::from_json(&InteractionResponse {
                        data: Some(InteractionResponseData {
                            content: format!(
                                "[Click to open **{}** in <#{}>](https://discord.gg/{})",
                                resp_json.target_application.name, channel_id, resp_json.code
                            ),
                        }),
                        interaction_type: InteractionResponseType::ChannelMessageWithSource,
                    });
                }
            }

            Response::from_json(&InteractionResponse {
                data: None,
                interaction_type: InteractionResponseType::Pong,
            })
        })
        .run(req, env)
        .await
}
