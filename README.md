<h1 align="center">Mini Me</h1>
<div align="center">
  <img width="414" height="483" src="./assets/mini-me.jpg">
</div>
<div align="center">
 <strong>
   Looks can be deceiving |
   <a href="https://discord.com/api/oauth2/authorize?client_id=854961435388936242&permissions=2147814464&scope=bot">
     Invite to Your Server
   </a>
 </strong>
</div>

---

## Usage

Commands:

- `?set-prefix <prefix>` - Set a default prefix for the bot in this server. `?` is the default if left unconfigured, and `??` is always available even if a custom prefix is configured.
- `?set-moderator-role @role` - Set the minimum discord role required to add/remove commands.
- `?command add <command> <response...>` - Add a dyanmic command! Will respond with the rest of the arguments.
- `?command remove <command>` - Remove a dynamic command.
- `?command get <command>` - Same as running the command itself
- `?<dynamic command>` - Print the response of a dynamic command

## Command Nuances

- Using `%or` separates the response of a command into multiple equally randomly chosen responses.

## Notes

Required permissions: 2147814464

- View Channels
- Send Messages
- Read Message History
- Use External Emoji
- Add Reactions
- Use Slash Commands

## Building

1. Install rust/cargo/etc., docker + docker-compose
2. `cargo install sqlx-cli`
3. `docker-compose -f dev.docker-compose.yml up -d`
4. `cp .env.example .env`
5. Add a discord bot token to .env where it says
6. `sqlx database create`
7. `sqlx migrate run`
8. `cargo build` / `cargo run`

## Contributors

- eecks
- jp db
- zoo los
- amin (security consultant)
