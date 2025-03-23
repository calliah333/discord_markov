# Markov chain generator from discord messages

## Parsing

Use <https://github.com/Tyrrrz/DiscordChatExporter> to export the full json from a channel

Parse the data into a more digestible format with this jq command. It will not work otherwise because I don't want to make it work with the original. 

For me on an M1 Pro and a 3.5GB json, it took approx 2 minutes.

WARNING: THIS WILL USE A LOT OF MEMORY FOR LARGE FILE. BE CAREFUL

```sh
jq '[.messages | map({content: .content, author: .author.name, embed: (.embeds | length > 0)})]' input.json > input_compact.json
```

## Command usage

```sh
cargo run --release <input.json> <username (optional)>
```

If a username is specified, it will only generate for that specific user.

I added a function for word usage analysis but it's currently commented. You can uncomment and run it if you wish. If i make a better cli, this readme will be updated.


