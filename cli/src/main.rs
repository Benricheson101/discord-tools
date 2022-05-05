use std::{error, io, str::FromStr};

use clap::{Args, Command, CommandFactory, Parser, PossibleValue, Subcommand, ValueHint};
use clap_complete::{generate, Generator, Shell};

use oauth::{client_credentials::ClientCredentials, Scope};
use strum::{EnumMessage, IntoEnumIterator};

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
            possible_values = scope_to_possible_value()
        )]
        scopes: Vec<oauth::Scope>,
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
struct BotAuthArgs {
    #[clap(short = 't', long = "token", env = "DISCORD_TOKEN")]
    token: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let args = Cli::parse();

    match &args.command {
        Commands::GuildCount { oauth } => {
            let scope = Scope::from_str("guilds").unwrap();

            let client_credentials =
                ClientCredentials::request(&oauth.client_id, &oauth.client_secret, &vec![scope])
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
    }

    Ok(())
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

fn scope_to_possible_value() -> Vec<PossibleValue<'static>> {
    oauth::Scope::iter()
        .map(|s| {
            // FIXME: this is BAD is there a better way without leaking memory
            let mut pv = PossibleValue::new(Box::leak(s.to_string().into_boxed_str()));

            if let Some(doc) = s.get_documentation() {
                pv = pv.help(doc);
            }

            pv
        })
        .collect::<Vec<_>>()
}
