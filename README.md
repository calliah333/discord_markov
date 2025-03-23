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
cargo run --release  (scrape|markov) <input_file> <username>
```

For analysis, it'll sort all words used by occurrence and write the top 1000 to an output.txt file.

For markov chain generation, it'll generate 5 iterations of 50 words.

If a username is specified, it will only generate for that specific user.

I will consider expanding the cli later. But right now I don't to add more dependencies and im lazy so just edit the code and rerun it if you care.


