use dotenv::dotenv;

pub const PUBLIC_FILE_PATH_ENV_VAR: &str = "PUBLIC_FILE_PATH";
pub const PORT_ENV_VAR: &str = "PORT";
pub const POST_EMOJI: &str = "POST_EMOJI";
pub const CHANNELS_ENDPOINT: &str = "CHANNELS_ENDPOINT";
pub const EMOTE_ENDPOINT: &str = "EMOTE_ENDPOINT";
pub const AUTH_TOKEN: &str = "AUTH_TOKEN";

#[derive(Debug)]
pub enum EnvVarError {
    PortError,
    PublicFilePathError,
    PostEmojiError,
    ChannelsEndpointError,
    EmoteEndpointError,
    AuthTokenError,
}

pub fn initialize_env() -> Result<(), EnvVarError> {
    dotenv().ok();
    let port = std::env::var(PORT_ENV_VAR);
    let public_file_path = std::env::var(PUBLIC_FILE_PATH_ENV_VAR);
    let post_emoji = std::env::var(POST_EMOJI);
    let channels_endpoint = std::env::var(CHANNELS_ENDPOINT);
    let emote_endpoint = std::env::var(EMOTE_ENDPOINT);
    let auth_token = std::env::var(AUTH_TOKEN);

    if let Err(_) = port {
        return Err(EnvVarError::PortError);
    }
    if let Err(_) = public_file_path {
        return Err(EnvVarError::PublicFilePathError);
    }
    if let Err(_) = post_emoji {
        return Err(EnvVarError::PostEmojiError);
    }
    if let Err(_) = channels_endpoint {
        return Err(EnvVarError::ChannelsEndpointError);
    }
    if let Err(_) = emote_endpoint {
        return Err(EnvVarError::EmoteEndpointError);
    }
    if let Err(_) = auth_token {
        return Err(EnvVarError::AuthTokenError);
    }

    return Ok(());
}
