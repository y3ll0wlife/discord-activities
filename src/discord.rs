use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

// https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-interaction-type
#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq)]
#[repr(u8)]
pub enum InteractionType {
    Ping = 1,
    ApplicationCommand = 2,
    MessageComponent = 3,
    ApplicationCommandAutoComplete = 4,
    ModalSubmit = 5,
}

// https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-types
#[derive(Serialize_repr, Deserialize_repr, Debug)]
#[repr(u8)]
pub enum InteractionCommandType {
    ChatInput = 1,
    User = 2,
    Message = 3,
}

// https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-types
#[derive(Serialize_repr, Deserialize_repr, Debug)]
#[repr(u8)]
pub enum InteractionCommandOptionType {
    SubCommand = 1,
    SubCommandGroup = 2,
    String = 3,
    Integer = 4,
    Boolean = 5,
    User = 6,
    Channel = 7,
    Role = 8,
    Mentionable = 9,
    Number = 10,
    Attachment = 11,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq)]
#[repr(u8)]
pub enum InteractionResponseType {
    Pong = 1,
    ChannelMessageWithSource = 4,
    DeferredChannelMessageWithSource = 5,
    DeferredUpdateMessage = 6,
    UpdateMessage = 7,
    ApplicationCommandAutocompleteResult = 8,
    Modal = 9,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InteractionResponse {
    #[serde(rename = "type")]
    pub interaction_type: InteractionResponseType,

    pub data: Option<InteractionResponseData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InteractionResponseData {
    pub content: String,
}

// https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-interaction-data
#[derive(Serialize, Deserialize, Debug)]
pub struct InteractionData {
    pub id: String,

    pub name: String,

    #[serde(rename = "type")]
    pub interaction_type: InteractionCommandType,

    // add if needed
    pub resolved: Option<InteractionResolvedData>,

    pub options: Option<Vec<InteractionCommandOption>>,
}

// https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-application-command-interaction-data-option-structure
#[derive(Serialize, Deserialize, Debug)]
pub struct InteractionCommandOption {
    pub name: String,

    #[serde(rename = "type")]
    pub interaction_type: InteractionCommandOptionType,

    pub value: Option<String>, // string, integer, double, or boolean

                               // options array of application command interaction data option

                               // focused boolean
}

// https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-resolved-data-structure
#[derive(Serialize, Deserialize, Debug)]
pub struct InteractionResolvedData {
    //users: Option<Vec<User>>,
    // members: Option<Vec<GuildMember>>,
    // roles: Option<Vec<Role>>,
    //channels: Option<Vec<Channel>>,
    // messages: Option<Vec<Message>>,
    // attachments: Option<Vec<Attachement>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GuildMember {}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Interaction {
    pub id: String,

    #[serde(rename = "type")]
    pub interaction_type: InteractionType,

    pub application_id: String,

    pub data: Option<InteractionData>,

    guild_id: Option<String>,

    channel_id: Option<String>,

    member: Option<GuildMember>,

    user: Option<User>,

    pub token: String,

    pub version: i32,

    message: Option<Message>,

    app_permissions: Option<String>,

    locale: Option<String>, // This is available on all interaction types except PING

    guild_locale: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChannelInviteRequest {
    pub max_age: i32,
    pub max_uses: i32,
    pub temporary: bool,
    pub unique: bool,
    pub target_type: i32,
    pub target_application_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChannelInviteResponse {
    pub code: String,
    pub target_application: Application,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Application {
    pub name: String,
}
