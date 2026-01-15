// What not knowing about ORMs did to someone. Preserved for posterity.
// Some stuff here really should live in server, but orphan rules prevent that.
use std::fmt::Display;
use std::num::ParseIntError;
use std::str::FromStr;

use chrono::{DateTime, NaiveDateTime, Utc};
use rand::{distr::Alphanumeric, prelude::*};
use serde::{Deserialize, Serialize};

/// How much information should be omitted when sending `UserRecord` to the client.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UserMaskLevel {
    /// Allow everything through, **including the hashed password!**
    SelfUse,
    /// Allow everything except the hashed password.
    HidePass,
    /// Allow everything except the hashed password and email.
    HidePassEmail,
    /// Allow everything except hashed password, email, friends, and groups.
    HidePassEmailMembership,
}

/// Public-facing version of `UserRecord`. Can be safely sent to the client.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PublicUserRecord {
    pub uid: UserId,
    pub email: Option<String>,
    pub pubkey: Pubkey,
    pub hashed_pass: Option<HashedPassword>,
    pub alias: Option<String>,
    pub friends: Option<Vec<UserId>>,
    pub groups: Option<Vec<GroupId>>,
    pub motd: Option<String>,
    pub online: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PublicUserMessage {
    pub umid: UserMessageId,
    pub from: UserId,
    pub to: UserId,
    pub time_posted: DateTime<Utc>,
    pub content: ClientMessage,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupRecord {
    pub gid: GroupId,
    pub motd: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum UserStatus {
    Online,
    #[default]
    Offline,
    Invisible,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum UserVisibility {
    Private,
    #[default]
    FriendsOnly,
    Public,
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq, Copy)]
pub struct UserId(u32);

impl FromStr for UserId {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tmp = u32::from_str(s)?;
        Ok(Self(tmp))
    }
}

impl Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u32> for UserId {
    fn from(i: u32) -> Self {
        UserId(i)
    }
}

impl From<UserId> for u32 {
    fn from(val: UserId) -> Self {
        val.0
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupId(u32);

impl Display for GroupId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u32> for GroupId {
    fn from(i: u32) -> Self {
        GroupId(i)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserMessageId(u64);

impl Display for UserMessageId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u64> for UserMessageId {
    fn from(i: u64) -> Self {
        UserMessageId(i)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupMessageId(u64);

impl Display for GroupMessageId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u64> for GroupMessageId {
    fn from(i: u64) -> Self {
        GroupMessageId(i)
    }
}

/// Public key for message signing. Server performs no validation!
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Pubkey(String);

impl From<String> for Pubkey {
    fn from(s: String) -> Self {
        Pubkey(s)
    }
}

/// Hashed password. Server performs no validation!
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HashedPassword(String);

impl From<String> for HashedPassword {
    fn from(s: String) -> Self {
        HashedPassword(s)
    }
}

pub const LT_LEN: usize = 40;

pub fn alphanumeric_len(value: &str, len: usize) -> bool {
    value
        .chars()
        .all(|c| char::is_alphanumeric(c) && char::is_ascii(&c) && !char::is_whitespace(c))
        && value.chars().count() == len
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Debug)]
pub struct LoginToken {
    pub tk: String,
}

impl LoginToken {
    pub fn new() -> LoginToken {
        LoginToken {
            tk: rand::rng()
                .sample_iter(Alphanumeric)
                .take(LT_LEN)
                .map(char::from)
                .collect(),
        }
    }
}

impl Default for LoginToken {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for LoginToken {
    type Err = usize;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if alphanumeric_len(s, LT_LEN) {
            Ok(LoginToken { tk: s.to_owned() })
        } else {
            Err(s.chars().count())
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum MessageActor {
    Dm(UserId),
    Group(GroupId),
}

#[derive(Serialize, Deserialize)]
pub enum HistoryQuery {
    Unseen,
    Interval {
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    },
    Since(DateTime<Utc>),
}

impl<T> From<T> for WsClientboundPayload
where
    T: ClientboundPayload,
{
    fn from(i: T) -> Self {
        i.make_payload()
    }
}

/// Message that will be sent over ws to the client.
/// The server shouldn't interfere with this,
/// nor should it be processed in any way.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum WsClientboundPayload {
    NewMessage(PublicUserMessage),
    NewMessages(Vec<PublicUserMessage>),
    MessageSent(UserMessageId),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterRequest {
    pub email: String,
    pub password_hash: String,
    pub pubkey: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginRequest {
    pub email: String,
    pub password_hash: String,
}

pub trait ClientboundPayload
where
    Self: Sized,
{
    fn make_payload(self) -> WsClientboundPayload;
}

impl ClientboundPayload for PublicUserMessage {
    fn make_payload(self) -> WsClientboundPayload {
        WsClientboundPayload::NewMessage(self)
    }
}

impl ClientboundPayload for Vec<PublicUserMessage> {
    fn make_payload(self) -> WsClientboundPayload {
        WsClientboundPayload::NewMessages(self)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum WsServerboundPayload {
    NewUserMessage { to: UserId, content: ClientMessage },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientMessage(String);

impl From<String> for ClientMessage {
    fn from(s: String) -> Self {
        ClientMessage(s)
    }
}

impl Display for ClientMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
/// Tuple type for `UserRecord`. Only used in conversion from raw sql row.
pub type SqlUserRecord = (
    u32,            // uid
    String,         // email
    String,         // pubkey
    String,         // hashed_pass
    Option<String>, // alias
    String,         // friends
    String,         // groups
    Option<String>, // motd
    String,         // status
    String,         // visibility
);

/// Tuple type for `UserMessage`.
pub type SqlUserMessage = (
    u64,           // umid
    u32,           // sender_id
    u32,           // receiver_id
    String,        // msg_content
    NaiveDateTime, // time_posted stored as UTC
    bool,          // r
);

#[cfg(feature = "server")]
pub mod server_ext {
    use chrono::{DateTime, Utc};

    use crate::{
        ClientMessage, GroupId, GroupMessageId, HashedPassword, Pubkey, PublicUserMessage,
        PublicUserRecord, SqlUserMessage, SqlUserRecord, UserId, UserMaskLevel, UserMessageId,
        UserStatus, UserVisibility,
    };

    /// **Internal server use:** User record. Intentionally made not serializable, so it doesn't accidentally get sent to the client.
    #[derive(Debug, Clone)]
    pub struct UserRecord {
        pub uid: UserId,
        pub email: String,
        pub pubkey: Pubkey,
        pub hashed_pass: HashedPassword,
        pub alias: Option<String>,
        pub friends: Vec<UserId>,
        pub groups: Vec<GroupId>,
        pub motd: Option<String>,
        pub status: UserStatus,
        pub visibility: UserVisibility,
    }

    impl UserRecord {
        /// Convert to a public-facing form with optional hiding of information.
        pub fn mask(self, mask: UserMaskLevel) -> PublicUserRecord {
            PublicUserRecord {
                uid: self.uid,
                email: match mask {
                    UserMaskLevel::HidePassEmail | UserMaskLevel::HidePassEmailMembership => None,
                    _ => Some(self.email),
                },
                pubkey: self.pubkey,
                hashed_pass: match mask {
                    UserMaskLevel::SelfUse => Some(self.hashed_pass),
                    _ => None,
                },
                alias: self.alias,
                friends: match mask {
                    UserMaskLevel::HidePassEmailMembership => None,
                    _ => Some(self.friends),
                },
                groups: match mask {
                    UserMaskLevel::HidePassEmailMembership => None,
                    _ => Some(self.groups),
                },
                motd: self.motd,
                online: match (mask, self.status) {
                    (UserMaskLevel::SelfUse, UserStatus::Offline) => false,
                    (UserMaskLevel::SelfUse, _) => true,
                    (_, UserStatus::Online) => true,
                    (_, _) => false,
                },
            }
        }
    }

    pub trait FromSqlTup<T>
    where
        Self: Sized,
    {
        fn from_sql_tup(tup: T) -> Option<Self>;
    }

    impl FromSqlTup<SqlUserRecord> for UserRecord {
        /// Try to convert the raw sql row representation to a Rust struct.
        fn from_sql_tup(tup: SqlUserRecord) -> Option<Self> {
            let uid = UserId::from(tup.0);
            let email = tup.1;
            let pubkey = Pubkey::from(tup.2.clone());
            let hashed_pass = HashedPassword::from(tup.3.clone());
            let alias = tup.4;
            let friends = serde_json::from_str(&tup.5).ok()?;
            let groups = serde_json::from_str(&tup.6).ok()?;
            let motd = tup.7;
            let status = serde_json::from_str(&tup.8).ok()?;
            let visibility = serde_json::from_str(&tup.9).ok()?;
            Some(Self {
                uid,
                email,
                pubkey,
                hashed_pass,
                alias,
                friends,
                groups,
                motd,
                status,
                visibility,
            })
        }
    }

    pub trait IntoSqlValue
    where
        Self: Sized + Into<mysql::Value>,
    {
        fn into_sql(self) -> mysql::Value {
            self.into()
        }
    }

    impl<T> IntoSqlValue for T where T: Sized + Into<mysql::Value> {}
    impl FromSqlTup<SqlUserMessage> for PublicUserMessage {
        fn from_sql_tup(tup: SqlUserMessage) -> Option<Self> {
            Some(Self {
                umid: UserMessageId::from(tup.0),
                from: UserId::from(tup.1),
                to: UserId::from(tup.2),
                content: ClientMessage::from(tup.3),
                time_posted: DateTime::from_utc(tup.4, Utc),
            })
        }
    }
    impl Into<mysql::Value> for GroupId {
        fn into(self) -> mysql::Value {
            self.0.into()
        }
    }
    impl Into<mysql::Value> for UserMessageId {
        fn into(self) -> mysql::Value {
            self.0.into()
        }
    }
    impl Into<mysql::Value> for GroupMessageId {
        fn into(self) -> mysql::Value {
            self.0.into()
        }
    }
    impl Into<mysql::Value> for UserId {
        fn into(self) -> mysql::Value {
            self.0.into()
        }
    }
}
#[cfg(feature = "client")]
pub mod client_ext {
    use log::debug;
    use openssl::rsa::{Padding, Rsa};

    use crate::{ClientMessage, Pubkey, PublicUserRecord, WsServerboundPayload};

    impl ToString for Pubkey {
        fn to_string(&self) -> String {
            self.0.to_owned()
        }
    }
    impl PublicUserRecord {
        /// Input message was signed with **private key of origin.**
        pub fn decrypt(&self, msg: ClientMessage) -> Option<String> {
            let in_bytes = msg.to_string();
            let pubkey = Rsa::public_key_from_pem(self.pubkey.to_string().as_bytes()).ok()?;
            let mut res = vec![0; 10000];
            let unhexed = hex::decode(in_bytes.as_bytes()).ok()?;
            debug!("unhex ok");
            let bytes_written = pubkey
                .public_decrypt(&unhexed, &mut res, Padding::PKCS1)
                .ok()?;
            debug!("content decode ok");
            String::from_utf8(res).ok()
        }
    }
    impl From<WsServerboundPayload> for tungstenite::Message {
        fn from(value: WsServerboundPayload) -> Self {
            tungstenite::Message::from(serde_json::to_string(&value).unwrap())
        }
    }
}
