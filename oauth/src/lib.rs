use serde::{Deserialize, Serialize};

pub mod client_credentials;

#[derive(Debug, Deserialize, Serialize)]
pub enum Scope {
    /// allows your app to fetch data from a user's "Now Playing/Recently Played" list - requires Discord approval
    ActivitiesRead,
    /// allows your app to update a user's activity - requires Discord approval (NOT REQUIRED FOR GAMESDK ACTIVITY MANAGER)
    ActivitiesWrite,
    /// allows your app to read build data for a user's applications
    ApplicationsBuildsRead,
    /// allows your app to upload/update builds for a user's applications - requires Discord approval
    ApplicationsBuildsUpload,
    /// allows your app to use commands in a guild
    ApplicationsCommands,
    /// allows your app to update its commands using a Bearer token - client credentials grant only
    ApplicationsCommandsUpdate,
    /// allows your app to update permissions for its commands in a guild a user has permissions to
    ApplicationsCommandsPermissionsUpdate,
    /// allows your app to read entitlements for a user's applications
    ApplicationsEntitlements,
    /// allows your app to read and update store data (SKUs, store listings, achievements, etc.) for a user's applications
    ApplicationsStoreUpdate,
    /// for oauth2 bots, this puts the bot in the user's selected guild by default
    Bot,
    /// allows /users/@me/connections to return linked third-party accounts
    Connections,
    /// enables /users/@me to return an email
    Email,
    /// allows your app to join users to a group dm
    GdmJoin,
    /// allows /users/@me/guilds to return basic information about all of a user's guilds
    Guilds,
    /// allows /guilds/{guild.id}/members/{user.id} to be used for joining users to a guild
    GuildsJoin,
    /// allows /users/@me/guilds/{guild.id}/member to return a user's member information in a guild
    GuildsMembersRead,
    /// allows /users/@me without email
    Identify,
    /// for local rpc server api access, this allows you to read messages from all client channels (otherwise restricted to channels/guilds your app creates)
    MessagesRead,
    /// allows your app to know a user's friends and implicit relationships - requires Discord approval
    RelationshipsRead,
    /// for local rpc server access, this allows you to control a user's local Discord client - requires Discord approval
    Rpc,
    /// for local rpc server access, this allows you to update a user's activity - requires Discord approval
    RpcActivitiesWrite,
    /// for local rpc server access, this allows you to receive notifications pushed out to the user - requires Discord approval
    RpcNotificationsRead,
    /// for local rpc server access, this allows you to read a user's voice settings and listen for voice events - requires Discord approval
    RpcVoiceRead,
    /// for local rpc server access, this allows you to update a user's voice settings - requires Discord approval
    RpcVoiceWrite,
    /// this generates a webhook that is returned in the oauth token response for authorization code grants
    WebhookIncoming,
}

impl Into<String> for Scope {
    fn into(self) -> String {
        match self {
            Scope::ActivitiesRead => "activities.read",
            Scope::ActivitiesWrite => "activities.write",
            Scope::ApplicationsBuildsRead => "applications.builds.read",
            Scope::ApplicationsBuildsUpload => "applications.builds.upload",
            Scope::ApplicationsCommands => "applications.commands",
            Scope::ApplicationsCommandsUpdate => "applications.commands.update",
            Scope::ApplicationsCommandsPermissionsUpdate => {
                "applications.commands.permissions.update" // rustfmt why :angr:
            }
            Scope::ApplicationsEntitlements => "applications.entitlements",
            Scope::ApplicationsStoreUpdate => "applications.store.update",
            Scope::Bot => "bot",
            Scope::Connections => "connections",
            Scope::Email => "email",
            Scope::GdmJoin => "gdm.join",
            Scope::Guilds => "guilds",
            Scope::GuildsJoin => "guilds.join",
            Scope::GuildsMembersRead => "guilds.members.read",
            Scope::Identify => "identify",
            Scope::MessagesRead => "messages.read",
            Scope::RelationshipsRead => "relationships.read",
            Scope::Rpc => "rpc",
            Scope::RpcActivitiesWrite => "rpc.activities.write",
            Scope::RpcNotificationsRead => "rpc.notifications.read",
            Scope::RpcVoiceRead => "rpc.voice.read",
            Scope::RpcVoiceWrite => "rpc.voice.write",
            Scope::WebhookIncoming => "webhook.incoming",
        }
        .to_string()
    }
}

impl Into<Scope> for &str {
    fn into(self) -> Scope {
        match self {
            "activities.read" => Scope::ActivitiesRead,
            "activities.write" => Scope::ActivitiesWrite,
            "applications.builds.read" => Scope::ApplicationsBuildsRead,
            "applications.builds.upload" => Scope::ApplicationsBuildsUpload,
            "applications.commands" => Scope::ApplicationsCommands,
            "applications.commands.update" => Scope::ApplicationsCommandsUpdate,
            "applications.commands.permissions.update" => {
                Scope::ApplicationsCommandsPermissionsUpdate
            }
            "applications.entitlements" => Scope::ApplicationsEntitlements,
            "applications.store.update" => Scope::ApplicationsStoreUpdate,
            "bot" => Scope::Bot,
            "connections" => Scope::Connections,
            "email" => Scope::Email,
            "gdm.join" => Scope::GdmJoin,
            "guilds" => Scope::Guilds,
            "guilds.join" => Scope::GuildsJoin,
            "guilds.members.read" => Scope::GuildsMembersRead,
            "identify" => Scope::Identify,
            "messages.read" => Scope::MessagesRead,
            "relationships.read" => Scope::RelationshipsRead,
            "rpc" => Scope::Rpc,
            "rpc.activities.write" => Scope::RpcActivitiesWrite,
            "rpc.notifications.read" => Scope::RpcNotificationsRead,
            "rpc.voice.read" => Scope::RpcVoiceRead,
            "rpc.voice.write" => Scope::RpcVoiceWrite,
            "webhook.incoming" => Scope::WebhookIncoming,
            &_ => panic!("invalid scope"),
        }
    }
}

fn de_vec_scope<'de, D>(deserializer: D) -> Result<Vec<Scope>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    let scopes = s.split(' ').map(|s| s.into()).collect::<Vec<Scope>>();
    Ok(scopes)
}

// fn se_vec_scope<S>(serializer: S) -> Result<String, S::Error>
// where
//     S: serde::Serialize,
// {
// }
