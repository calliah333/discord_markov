# Markov chain generator from discord messages

## Parsing

Use <https://github.com/Tyrrrz/DiscordChatExporter> to export the full json from a channel

Parse the data into a more digestible format with this jq command. It will not work otherwise because I don't want to make it work with the original. 

For me on an M1 Pro and a 3.5GB json, it took approx 2 minutes.

WARNING: THIS WILL USE A LOT OF MEMORY FOR LARGE FILE. BE CAREFUL

```sh
jq -r '.messages[] | [.content, .author.name, .author.id, (.embeds | length > 0)] | @csv' messages.json > messages.csv
```

After this use whatever importer you want to import the csv into a sqlite db and edit the code to connect it. 

## Command usage

```sh
cargo run --release  (analysis|markov) <username>
```

For analysis, it'll sort all words used by occurrence and write the top 5000 to an output.txt file.

For markov chain generation, it'll generate 5 iterations of 50 words.

If a username is specified, it will only generate for that specific user.

I will consider expanding the cli later. But right now I don't to add more dependencies and im lazy so just edit the code and rerun it if you care.


