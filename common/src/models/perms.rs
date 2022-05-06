use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumMessage, EnumString, IntoEnumIterator};

// TODO: separate enum for whitelisted scopes

#[derive(Debug)]
pub struct Permissions(pub u64);

impl Permissions {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn add(&mut self, p: Permission) {
        self.0 |= p as u64;
    }

    pub fn remove(&mut self, p: Permission) {
        self.0 &= p as u64;
    }

    pub fn has(&self, p: Permission) -> bool {
        self.0 & p as u64 == p as u64
    }

    pub fn to_permissions_vec(&self) -> Vec<Permission> {
        Permission::iter().filter(|p| self.has(*p)).collect()
    }
}

impl Default for Permissions {
    fn default() -> Self {
        Self::new()
    }
}

impl From<&Vec<Permission>> for Permissions {
    fn from(ps: &Vec<Permission>) -> Self {
        let i = ps.iter().fold(0, |acc, a| acc | *a as u64);
        Self(i)
    }
}

#[repr(u64)]
#[derive(
    Debug, Copy, Clone, Deserialize, Serialize, Display, EnumIter, EnumMessage, EnumString,
)]
pub enum Permission {
    /// Allows creation of instant invites
    #[strum(serialize = "CREATE_INSTANT_INVITE")]
    CreateInstantInvite = 1 << 0,
    /// Allows kicking members
    #[strum(serialize = "KICK_MEMBERS")]
    KickMembers = 1 << 1,
    /// Allows banning members
    #[strum(serialize = "BAN_MEMBERS")]
    BanMembers = 1 << 2,
    /// Allows all permissions and bypasses channel permission overwrites
    #[strum(serialize = "ADMINISTRATOR")]
    Administrator = 1 << 3,
    /// Allows management and editing of channels
    #[strum(serialize = "MANAGE_CHANNELS")]
    ManageChannels = 1 << 4,
    /// Allows management and editing of the guild
    #[strum(serialize = "MANAGE_GUILD")]
    ManageGuild = 1 << 5,
    /// Allows for the addition of reactions to messages
    #[strum(serialize = "ADD_REACTIONS")]
    AddReactions = 1 << 6,
    /// Allows for viewing of audit logs
    #[strum(serialize = "VIEW_AUDIT_LOG")]
    ViewAuditLog = 1 << 7,
    /// Allows for using priority speaker in a voice channel
    #[strum(serialize = "PRIORITY_SPEAKER")]
    PrioritySpeaker = 1 << 8,
    /// Allows the user to go live
    #[strum(serialize = "STREAM")]
    Stream = 1 << 9,
    /// Allows guild members to view a channel, which includes reading messages in text channels and joining voice channels
    #[strum(serialize = "VIEW_CHANNEL")]
    ViewChannel = 1 << 10,
    /// Allows for sending messages in a channel and creating threads in a forum (does not allow sending messages in threads)
    #[strum(serialize = "SEND_MESSAGES")]
    SendMessages = 1 << 11,
    /// Allows for sending of /tts messages
    #[strum(serialize = "SEND_TTS_MESSAGES")]
    SendTtsMessages = 1 << 12,
    /// Allows for deletion of other users messages
    #[strum(serialize = "MANAGE_MESSAGES")]
    ManageMessages = 1 << 13,
    /// Links sent by users with this permission will be auto-embedded
    #[strum(serialize = "EMBED_LINKS")]
    EmbedLinks = 1 << 14,
    /// Allows for uploading images and files
    #[strum(serialize = "ATTACH_FILES")]
    AttachFiles = 1 << 15,
    /// Allows for reading of message history
    #[strum(serialize = "READ_MESSAGE_HISTORY")]
    ReadMessageHistory = 1 << 16,
    /// Allows for using the @everyone tag to notify all users in a channel, and the @here tag to notify all online users in a channel
    #[strum(serialize = "MENTION_EVERYONE")]
    MentionEveryone = 1 << 17,
    /// Allows the usage of custom emojis from other servers
    #[strum(serialize = "USE_EXTERNAL_EMOJIS")]
    UseExternalEmojis = 1 << 18,
    /// Allows for viewing guild insights
    #[strum(serialize = "VIEW_GUILD_INSIGHTS")]
    ViewGuildInsights = 1 << 19,
    /// Allows for joining of a voice channel
    #[strum(serialize = "CONNECT")]
    Connect = 1 << 20,
    /// Allows for speaking in a voice channel
    #[strum(serialize = "SPEAK")]
    Speak = 1 << 21,
    /// Allows for muting members in a voice channel
    #[strum(serialize = "MUTE_MEMBERS")]
    MuteMembers = 1 << 22,
    /// Allows for deafening of members in a voice channel
    #[strum(serialize = "DEAFEN_MEMBERS")]
    DeafenMembers = 1 << 23,
    /// Allows for moving of members between voice channels
    #[strum(serialize = "MOVE_MEMBERS")]
    MoveMembers = 1 << 24,
    /// Allows for using voice-activity-detection in a voice channel
    #[strum(serialize = "USE_VAD")]
    UseVad = 1 << 25,
    /// Allows for modification of own nickname
    #[strum(serialize = "CHANGE_NICKNAME")]
    ChangeNickname = 1 << 26,
    /// Allows for modification of other users nicknames
    #[strum(serialize = "MANAGE_NICKNAMES")]
    ManageNicknames = 1 << 27,
    /// Allows management and editing of roles
    #[strum(serialize = "MANAGE_ROLES")]
    ManageRoles = 1 << 28,
    /// Allows management and editing of webhooks
    #[strum(serialize = "MANAGE_WEBHOOKS")]
    ManageWebhooks = 1 << 29,
    /// Allows management and editing of emojis and stickers
    #[strum(serialize = "MANAGE_EMOJIS_AND_STICKERS")]
    ManageEmojisAndStickers = 1 << 30,
    /// Allows members to use application commands, including slash commands and context menu commands.
    #[strum(serialize = "USE_APPLICATION_COMMANDS")]
    UseApplicationCommands = 1 << 31,
    /// Allows for requesting to speak in stage channels. (This permission is under active development and may be changed or removed.)
    #[strum(serialize = "REQUEST_TO_SPEAK")]
    RequestToSpeak = 1 << 32,
    /// Allows for creating, editing, and deleting scheduled events
    #[strum(serialize = "MANAGE_EVENTS")]
    ManageEvents = 1 << 33,
    /// Allows for deleting and archiving threads, and viewing all private threads
    #[strum(serialize = "MANAGE_THREADS")]
    ManageThreads = 1 << 34,
    /// Allows for creating public and announcement threads
    #[strum(serialize = "CREATE_PUBLIC_THREADS")]
    CreatePublicThreads = 1 << 35,
    /// Allows for creating private threads
    #[strum(serialize = "CREATE_PRIVATE_THREADS")]
    CreatePrivateThreads = 1 << 36,
    /// Allows the usage of custom stickers from other servers
    #[strum(serialize = "USE_EXTERNAL_STICKERS")]
    UseExternalStickers = 1 << 37,
    /// Allows for sending messages in threads
    #[strum(serialize = "SEND_MESSAGES_IN_THREADS")]
    SendMessagesInThreads = 1 << 38,
    /// Allows for using Activities (applications with the EMBEDDED flag) in a voice channel
    #[strum(serialize = "USE_EMBEDDED_ACTIVITIES")]
    UseEmbeddedActivities = 1 << 39,
    /// Allows for timing out users to prevent them from sending or reacting to messages in chat and threads, and from speaking in voice and stage channels
    #[strum(serialize = "MODERATE_MEMBERS")]
    ModerateMembers = 1 << 40,
}
