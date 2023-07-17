use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use clap::Parser;

use self::ids::IdMap;

mod ids;

#[derive(Debug, Parser)]
struct Arguments {
    pub query: String,
    pub files: Vec<String>,
}

fn main() {
    let args = Arguments::parse();
    let mut ids = IdMap::default();
    let mut per_file_terms: HashMap<String, _> = HashMap::default();
    let mut total_term_occurences: HashMap<u32, u32> = HashMap::default();

    for filename in &args.files {
        match parse_file(filename, &mut ids) {
            Ok(map) => {
                for word_ids in map.keys() {
                    let count = total_term_occurences.entry(*word_ids).or_default();
                    *count += 1;
                }
                per_file_terms.insert(filename.clone(), map);
            }
            Err(err) => eprintln!("Unable to read file {}: {:?}", filename, err),
        }
    }
    let document_count = per_file_terms.len();
    let mut scores: Vec<_> = per_file_terms.iter().map(|(filename, term_frequency)| {
        (
            filename,
            compute_file_score(
                &args.query,
                term_frequency,
                &total_term_occurences,
                document_count as u32,
                &ids,
            ),
        )
    }).collect();
    scores.sort_unstable_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());

    for (filename, score) in scores.iter().take(20) {
        println!("{}: {}", filename, score);
    }
}

fn compute_file_score(
    query: &str,
    term_frequency: &HashMap<u32, u32>,
    document_frequence: &HashMap<u32, u32>,
    document_count: u32,
    ids: &IdMap,
) -> f64 {
    let total_term_count: u32 = term_frequency.values().sum();
    if total_term_count == 0 {
        return 0.0;
    }
    let mut total_score = 0.0;
    for query_word in query
        .split(|c: char| !c.is_alphanumeric())
        .filter(|split| !split.is_empty())
        .map(|split| split.to_lowercase())
    {
        if let Some(query_word_id) = ids.get(&query_word) {
            let tf = term_frequency.get(&query_word_id).copied().unwrap_or(0) as f64
                / total_term_count as f64;

            if tf == 0.0 {
                continue;
            }

            let term_document_count = document_frequence.get(&query_word_id).copied().expect(
                "Should be in at least one document : the one we are computing the score for",
            ) as f64;
            let idf = f64::log2(document_count as f64 / term_document_count);

            total_score += tf * idf;
        }
    }

    total_score
}

fn parse_file(file: &str, ids: &mut IdMap) -> io::Result<HashMap<u32, u32>> {
    let mut map = HashMap::default();
    let mut reader = BufReader::new(File::open(file)?);
    let mut buf = String::default();

    // While there is still characters to read
    while reader.read_line(&mut buf)? > 0 {
        for word in buf
            .split(|c: char| !c.is_alphanumeric())
            .filter(|split| !split.is_empty())
            .map(|split| split.to_lowercase())
        {
            let id = ids.get_or_register(word);
            let count = map.entry(id).or_default();
            *count += 1;
        }
        buf.clear();
    }
    Ok(map)
}
