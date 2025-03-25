use std::collections::HashMap;
use std::env;
use std::io::{Write};
use std::fs::File;
use std::time::Instant;
use rand::rngs::ThreadRng;
use markov_str::*;
use regex::Regex;
use rusqlite::{Connection, Result};

fn analysis(conn : &Connection) -> Result<()> {
    let mut statement = conn.prepare("SELECT content, embed FROM messages")?;
    let mut rows = statement.query([])?;
    let mut hm: HashMap<String, u32> = HashMap::new();
    while let Some(row) = rows.next()? {
        let msg:String = row.get(0)?;
        let embed: bool = row.get(1)?;
        if embed {
            continue;
        }
        for word in msg.split_whitespace() {
            *hm.entry(word.to_string()).or_default() += 1;
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
    Ok(())
}

fn create_markov_from_db(conn: &Connection, markov: &mut RawMarkovChain<4>, user: Option<&String>) -> Result<()> {
    let sql = if user.is_some() {
        "SELECT content FROM messages WHERE author = ?1"
    } else {
        "SELECT content FROM messages"
    };

    let mut statement = conn.prepare(sql)?;

    let mut rows = if let Some(user_value) = user {
        statement.query([user_value])?
    } else {
        statement.query([])?
    };

    while let Some(row) = rows.next()? {
        let msg:String = row.get(0)?;
        markov.add_text(&msg);
    }
    Ok(())
}



//TODO: Compare word usage to a normal corpus of text. Try using internet text

/// Usage:
///     
/// Scraping:
///     cargo run --release markov <uname (optional)>
/// 
/// Analysis:
///     cargo run --release analysis   
fn main() -> Result<()>{
    let args: Vec<String> = env::args().collect(); // [location, command, input, username]

    if args.len() < 2 { 
        println!("Usage: (markov|analysis) <username>");
        return Ok(());
    }
    
    let conn = Connection::open("messages.db")?;
    println!("Opened connection to database");
    let start = Instant::now();
    let command = args.get(1).unwrap();
    if command == "analysis" {
        analysis(&conn)?;
        println!("Analyzed and wrote in {}ms", start.elapsed().as_millis());
        return Ok(());
    }
    
    let mut user = None;
    if args.len() > 2 {
        user = Some(args.get(2).unwrap());
    }

    let mut m: RawMarkovChain<4> = MarkovChain::new(2, Regex::new(WORD_REGEX).unwrap());
    create_markov_from_db(&conn, &mut m, user)?;
    println!("Read from db and created markov in {}ms", start.elapsed().as_millis());
    
    let mut rng = ThreadRng::default();
    for _ in 0..5{
        println!("{}\n", m.generate(50, &mut rng).unwrap());
    }

    Ok(())
}

