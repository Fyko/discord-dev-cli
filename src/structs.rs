use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Application {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
    pub description: String,
    pub summary: String,
    pub hook: bool,
    pub bot_public: Option<bool>,
    pub bot_require_code_grant: Option<bool>,
    pub verify_key: String,
    pub owner: ApplicationOwner,
    pub flags: i32,
    pub secret: String,
    pub redirect_uris: Vec<String>,
    pub rpc_application_state: i8,
    pub store_application_state: i8,
    pub verification_state: i8,
    pub bot: Option<Bot>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationOwner {
    pub id: String,
    pub username: String,
    pub avatar: Option<String>,
    pub discriminator: String,
    pub public_flags: u32,
    pub flags: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bot {
    pub id: String,
    pub username: String,
    pub avatar: Option<String>,
    pub discriminator: String,
    pub public_flags: i32,
    pub bot: bool,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Team {
    pub id: String,
    pub icon: Option<String>,
    pub name: String,
    pub owner_user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TeamMember {
    pub user: ApplicationOwner,
    pub team_id: String,
    pub membership_state: u8,
    pub permissions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Me {
    pub id: String,
    pub username: String,
    pub avatar: String,
    pub discriminator: String,
    pub public_flags: u32,
    pub flags: u32,
    pub email: String,
    pub verified: bool,
    pub locale: String,
    pub nsfw_allowed: bool,
    pub mfa_enabled: bool,
    pub phone: Option<String>,
    pub premium_type: u8,
}
