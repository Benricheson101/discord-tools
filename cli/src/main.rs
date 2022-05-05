use std::{error, io};

use clap::{Args, Command, CommandFactory, Parser, Subcommand, ValueHint};
use clap_complete::{generate, Generator, Shell};

use oauth::client_credentials::ClientCredentials;

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
        long = "secret",
        env = "CLIENT_SECRET",
        help = "OAuth application client secret",
        value_hint = ValueHint::Unknown
    )]
    client_secret: String,

    #[clap(
        short = 'i',
        long = "id",
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
            let client_credentials =
                ClientCredentials::request(&oauth.client_id, &oauth.client_secret, "guilds".into())
                    .await?;

            let guilds = client_credentials.get_user_guilds().await?;

            println!("You are in {} servers", guilds.len());
        }

        &Commands::Completions { shell } => {
            let mut cmd = Cli::command();
            print_completions(shell, &mut cmd);
        }
    }

    Ok(())
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}
