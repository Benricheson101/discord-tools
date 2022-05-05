pub mod client_credentials;
pub mod error;

use serde::{Deserialize, Serialize, Serializer};
use strum::{Display, EnumIter, EnumMessage, EnumString};

// TODO: separate enum for whitelisted scopes

#[derive(Debug, Deserialize, Serialize, Display, EnumIter, EnumMessage, EnumString)]
pub enum Scope {
    /// allows your app to fetch data from a user's 'Now Playing/Recently Played' list - requires Discord approval
    #[strum(serialize = "activities.read")]
    ActivitiesRead,
    /// allows your app to update a user's activity - requires Discord approval (NOT REQUIRED FOR GAMESDK ACTIVITY MANAGER)
    #[strum(serialize = "activities.write")]
    ActivitiesWrite,
    /// allows your app to read build data for a user's applications
    #[strum(serialize = "applications.builds.read")]
    ApplicationsBuildsRead,
    /// allows your app to upload/update builds for a user's applications - requires Discord approval
    #[strum(serialize = "applications.builds.upload")]
    ApplicationsBuildsUpload,
    /// allows your app to use commands in a guild
    #[strum(serialize = "applications.commands")]
    ApplicationsCommands,
    /// allows your app to update its commands using a Bearer token - client credentials grant only
    #[strum(serialize = "applications.commands.update")]
    ApplicationsCommandsUpdate,
    /// allows your app to update permissions for its commands in a guild a user has permissions to
    #[strum(serialize = "applications.commands.permissions.update")]
    ApplicationsCommandsPermissionsUpdate,
    /// allows your app to read entitlements for a user's applications
    #[strum(serialize = "applications.entitlements")]
    ApplicationsEntitlements,
    /// allows your app to read and update store data (SKUs, store listings, achievements, etc.) for a user's applications
    #[strum(serialize = "applications.store.update")]
    ApplicationsStoreUpdate,
    /// for oauth2 bots, this puts the bot in the user's selected guild by default
    #[strum(serialize = "bot")]
    Bot,
    /// allows /users/@me/connections to return linked third-party accounts
    #[strum(serialize = "connections")]
    Connections,
    /// enables /users/@me to return an email
    #[strum(serialize = "email")]
    Email,
    /// allows your app to join users to a group dm
    #[strum(serialize = "gdm.join")]
    GdmJoin,
    /// allows /users/@me/guilds to return basic information about all of a user's guilds
    #[strum(serialize = "guilds")]
    Guilds,
    /// allows /guilds/{guild.id}/members/{user.id} to be used for joining users to a guild
    #[strum(serialize = "guilds.join")]
    GuildsJoin,
    /// allows /users/@me/guilds/{guild.id}/member to return a user's member information in a guild
    #[strum(serialize = "guilds.members.read")]
    GuildsMembersRead,
    /// allows /users/@me without email
    #[strum(serialize = "identify")]
    Identify,
    /// for local rpc server api access, this allows you to read messages from all client channels (otherwise restricted to channels/guilds your app creates)
    #[strum(serialize = "messages.read")]
    MessagesRead,
    /// allows your app to know a user's friends and implicit relationships - requires Discord approval
    #[strum(serialize = "relationships.read")]
    RelationshipsRead,
    /// for local rpc server access, this allows you to control a user's local Discord client - requires Discord approval
    #[strum(serialize = "rpc")]
    Rpc,
    /// for local rpc server access, this allows you to update a user's activity - requires Discord approval
    #[strum(serialize = "rpc.activities.write")]
    RpcActivitiesWrite,
    /// for local rpc server access, this allows you to receive notifications pushed out to the user - requires Discord approval
    #[strum(serialize = "rpc.notifications.read")]
    RpcNotificationsRead,
    /// for local rpc server access, this allows you to read a user's voice settings and listen for voice events - requires Discord approval
    #[strum(serialize = "rpc.voice.read")]
    RpcVoiceRead,
    /// for local rpc server access, this allows you to update a user's voice settings - requires Discord approval
    #[strum(serialize = "rpc.voice.write")]
    RpcVoiceWrite,
    /// this generates a webhook that is returned in the oauth token response for authorization code grants
    #[strum(serialize = "webhook.incoming")]
    WebhookIncoming,
}

pub(crate) fn de_vec_scope<'de, D>(deserializer: D) -> Result<Vec<Scope>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    let scopes = s
        .split(' ')
        .filter_map(|s| Scope::try_from(s).ok())
        .collect::<Vec<Scope>>();
    Ok(scopes)
}

pub(crate) fn se_vec_scope<S>(scopes: &Vec<Scope>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let scope_string = scopes
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    serializer.serialize_str(&scope_string)
}
