use std::process;

use reqwest::{Client, Method, Url};
use serde::{Deserialize, Serialize};

use crate::{de_vec_scope, se_vec_scope, Scope};

const OAUTH_TOKEN_URL: &str = "https://discord.com/api/v10/oauth2/token";
const USER_GUILDS_URL: &str = "https://discord.com/api/v10/users/@me/guilds";

#[derive(Debug, Deserialize, Serialize)]
pub struct ClientCredentials {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u32,
    #[serde(
        rename = "scope",
        deserialize_with = "de_vec_scope",
        serialize_with = "se_vec_scope"
    )]
    pub scopes: Vec<Scope>,
}

impl ClientCredentials {
    pub async fn request(
        client_id: &str,
        client_secret: &str,
        scopes: &[Scope],
    ) -> Result<Self, reqwest::Error> {
        let url = Url::parse(OAUTH_TOKEN_URL).unwrap();

        let scope = scopes
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join(" ");

        let body = ClientCredentialsRequest {
            scope: &scope,
            client_id,
            client_secret,
            grant_type: "client_credentials",
        };

        let data = serde_urlencoded::to_string(&body).unwrap();

        let client = Client::new();
        let req = client
            .request(Method::POST, url)
            .header(
                reqwest::header::CONTENT_TYPE,
                "application/x-www-form-urlencoded",
            )
            .body(data);

        let resp = req.send().await?;
        if resp.status().is_success() {
            return resp.json::<Self>().await;
        } else {
            // TODO: return Error type
            let bdy = resp.text().await?;
            eprintln!("{}", bdy);
            process::exit(1);
        }
    }

    pub async fn get_user_guilds(&self) -> Result<Vec<Guild>, reqwest::Error> {
        let url = Url::parse(USER_GUILDS_URL).unwrap();
        let req = Client::new()
            .request(Method::GET, url)
            .bearer_auth(&self.access_token);

        req.send().await?.json::<Vec<Guild>>().await
    }
}

#[derive(Debug, Serialize)]
struct ClientCredentialsRequest<'a> {
    grant_type: &'a str,
    scope: &'a str,
    client_secret: &'a str,
    client_id: &'a str,
}

#[derive(Debug, Deserialize)]
pub struct Guild {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
    pub owner: bool,
    pub permissions: String,
    pub features: Vec<String>,
}
