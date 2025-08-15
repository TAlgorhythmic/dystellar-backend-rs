mod user;
mod http;
mod microsoft;
pub mod mailing;
pub mod punishments;
pub mod permissions;
pub mod fs_json;
pub mod routing;
use std::error::Error;

pub use user::User;
pub use microsoft::SigninState;
pub use microsoft::UserCredentials;
pub use microsoft::MinecraftData;
pub use microsoft::XboxLiveTokensData;
pub use microsoft::MicrosoftTokens;
pub use http::BackendError;

/**
* Trait that allows to easily (de)serialize from database/storage.
*/
pub trait Serializable {
    fn load(key: &str) -> Result<Option<Self>, Box<dyn Error + Send + Sync>> where Self: Sized;
    fn save(&self) -> Result<(), Box<dyn Error + Send + Sync>>;
}

pub struct Mod {
    pub name: Box<str>,
    pub version: Box<str>,
    pub filename: Box<str>
}
