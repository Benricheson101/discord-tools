<h1>DiscordTools 🛠</h1>

A collection of CLI tools for the Discord power user

## Features:
- guild count
- get bearer token for scope via client credentials
- permission calculator

## Option Priority:
CLI flags > environment variables > config file

## Config Format:
### Location:
**Linux**: `/home/$USER/.config/discordtools/config.toml` \
**MacOS**: `/Users/$USER/Library/Application Support/discordtools/config.toml` \
**Windows**: `C:\Users\$USER\AppData\Roaming\discordtools\config.toml`

```toml
[oauth]
client_id = ""
client_secret = ""

[bot]
token = ""
```

## TODO:
- [ ] list all user and guild experiments
- [ ] list servers in experiments
- [ ] user lookup using bot token
- [ ] current versions
- [ ] status page status
- [ ] intent calculator
- [ ] slash command create/delete/list
