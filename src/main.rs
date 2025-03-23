use std::collections::HashMap;
use std::env;
use std::io::{BufReader, Write};
use std::fs::File;
use std::time::Instant;
use rand::rngs::ThreadRng;
use serde::Deserialize;
use markov_str::*;
use regex::Regex;

#[derive(Deserialize)]
struct Message {
    author: String,
    content: String,
    embed: bool
}

#[derive(Deserialize)]
struct Messages {
    messages: Vec<Message>
}


fn parse_buffered(m: &mut Messages, file: &String){
    let file = File::open(file).unwrap();
    let reader = BufReader::new(file);
    *m = serde_json::from_reader(reader).unwrap();
    println!("parsed {} messages", m.messages.len());
}

#[allow(dead_code)]
fn analysis(m: &Messages){
    let mut hm: HashMap<String, u32> = HashMap::new();
    for msg in &m.messages {
        if msg.embed {
            continue;
        }
        for word in msg.content.split_whitespace() {
            *hm.entry(word.to_string().to_lowercase()).or_insert(0) += 1;
        }
    }

    let mut vec: Vec<(String, u32)> = hm.into_iter().collect();
    vec.sort_by(|a, b| {
        if a.1 != b.1 {
            // sort by occurrence
            b.1.cmp(&a.1)
        } else {
            // sort lexicographically
            a.0.cmp(&b.0)
        }
    });
    let mut output = File::create("output.txt").unwrap();
    for (w,n) in vec.iter().take(5000) {
        writeln!(&mut output, "{n} : {w}").unwrap();
    }
    output.flush().unwrap();
    
}


// None for all users, Some("uname") for specific user
fn create_markov(messages: &Messages, markov: &mut RawMarkovChain<4>, user: Option<&String>){
    for msg in &messages.messages {
        if msg.embed{
            continue;
        }
        match user {
            None => markov.add_text(&msg.content),
            Some(uname) => {
                if *uname == msg.author {
                    markov.add_text(&msg.content);
                }
            }
        }
    }
}


//TODO: Compare word usage to a normal corpus of text. Try using internet text

/// Usage:
///     
/// Scraping:
///     cargo run --release scrape <input_file> <uname (optional)>
/// 
/// Analysis:
///     cargo run --release analysis <input_file>    
fn main() {
    let args: Vec<String> = env::args().collect(); // [location, command, input, username]

    if args.len() < 3 { 
        println!("Usage: (markov|analysis) <input_file> <username>");
        return;
    }

    let input_file = args.get(2).unwrap();
    let mut now = Instant::now();
    let mut v: Messages = Messages {  messages: Vec::new() } ; 
    parse_buffered(&mut v, input_file);
    println!("Parsed input in {}ms", now.elapsed().as_millis());
    now = Instant::now();

    let command = args.get(1).unwrap();
    if command == "analysis" {
        analysis(&v);
        println!("Analyzed word usages in {}ms", now.elapsed().as_millis());
        println!("Wrote to output.txt");
        return;
    }
    
    let mut user = None;
    if args.len() > 3 {
        user = Some(args.get(3).unwrap());
    }

    let mut m: RawMarkovChain<4> = markov_str::MarkovChain::new(2, Regex::new(WORD_REGEX).unwrap());
    create_markov(&v, &mut m, user);
    println!("Created markov in {}ms", now.elapsed().as_millis());

    let mut rng = ThreadRng::default();
    for _ in 0..5{
        println!("{}", m.generate(50, &mut rng).unwrap());
        print!("\n");
    }
}

