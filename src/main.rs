pub mod models;
pub mod oauth;

use std::{error, io};

use clap::{Args, Command, CommandFactory, Parser, PossibleValue, Subcommand, ValueHint};
use clap_complete::{generate, Generator, Shell};

use models::{
    perms::{Permission, Permissions},
    scope::Scope,
};
use oauth::client_credentials::ClientCredentials;
use strum::{EnumMessage, IntoEnumIterator};

macro_rules! pv {
    ($type:ty) => {{
        <$type>::iter()
            .map(|s| {
                // FIXME: this is BAD is there a better way without leaking memory
                let name = Box::leak(s.to_string().into_boxed_str());

                let mut pv = PossibleValue::new(name);

                if let Some(doc) = s.get_documentation() {
                    pv = pv.help(doc);
                }

                pv
            })
            .collect::<Vec<PossibleValue<'static>>>()
    }};
}

#[derive(Debug, Parser)]
#[clap(
    name = "discord-tools",
    about = "A collection of tools for the Discord power user",
    long_about = None
)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[clap(about = "Get a bearer token for selected OAuth scopes")]
    ClientCredentials {
        #[clap(flatten)]
        oauth: OAuthArgs,
        #[clap(
            name = "SCOPE",
            long = "scope",
            required = true,
            multiple_occurrences = true,
            possible_values = pv!(Scope),
        )]
        scopes: Vec<Scope>,
    },

    #[clap(about = "Output shell completion functions")]
    Completions {
        #[clap(arg_enum)]
        shell: Shell,
    },

    #[clap(about = "Count the number of guilds you're in.")]
    GuildCount {
        #[clap(flatten)]
        oauth: OAuthArgs,
    },

    #[clap(about = "Calculate bitwise permissions", aliases = &["permission-calculator", "perm-calc"])]
    Perms {
        #[clap(flatten)]
        perms: PermissionArgs,
    },
}

#[derive(Debug, Clone, Args)]
struct OAuthArgs {
    #[clap(
        short = 's',
        long = "client-secret",
        env = "CLIENT_SECRET",
        help = "OAuth application client secret",
        value_hint = ValueHint::Unknown
    )]
    client_secret: String,

    #[clap(
        short = 'i',
        long = "client-id",
        env = "CLIENT_ID",
        help = "OAuth application client ID",
        value_hint = ValueHint::Unknown
    )]
    client_id: String,
}

#[derive(Debug, Args)]
struct PermissionArgs {
    #[clap(
        help = "Discord permission name",
        possible_values = pv!(Permission),
    )]
    permissions: Vec<Permission>,
}

#[derive(Debug, Args)]
struct BotAuthArgs {
    #[clap(short = 't', long = "token", env = "DISCORD_TOKEN")]
    token: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let args = Cli::parse();

    match &args.command {
        Commands::GuildCount { oauth } => {
            let scope = Scope::Guilds;

            let client_credentials =
                ClientCredentials::request(&oauth.client_id, &oauth.client_secret, &[scope])
                    .await?;

            let guilds = client_credentials.get_user_guilds().await?;

            println!("{}", guilds.len());
        }

        &Commands::Completions { shell } => {
            let mut cmd = Cli::command();
            print_completions(shell, &mut cmd);
        }

        Commands::ClientCredentials { oauth, scopes } => {
            let client_credentials =
                ClientCredentials::request(&oauth.client_id, &oauth.client_secret, scopes).await?;

            let json = serde_json::to_string(&client_credentials)?;

            print!("{}", json);
        }

        Commands::Perms { perms } => {
            let p = Permissions::from(&perms.permissions);
            print!("{}", p.0);
        }
    }

    Ok(())
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}
