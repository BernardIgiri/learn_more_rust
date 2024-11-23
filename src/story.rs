use super::math::*;
use hashbrown::HashMap;
use std::cmp;
use std::cmp::Ordering;
use unescape::unescape;

#[allow(dead_code)]
#[derive(Debug)]
struct StringCount(String, u32);

impl cmp::Ord for StringCount {
    fn cmp(&self, other: &Self) -> Ordering {
        other.1.cmp(&self.1)
    }
}

impl cmp::PartialOrd for StringCount {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl cmp::PartialEq for StringCount {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

impl cmp::Eq for StringCount {}

fn top_n_entries(n: usize, map: &HashMap<String, u32>) -> Vec<StringCount> {
    let mut top = Vec::new();
    top.reserve_exact(n);
    for (index, entry) in map.iter().enumerate() {
        if index < n {
            top.push(StringCount(entry.0.to_string(), *entry.1));
        } else {
            if index == n {
                top.sort();
            }
            if entry.1 > &top.first().unwrap().1 {
                top.pop();
                top.insert(0, StringCount(entry.0.to_string(), *entry.1));
            }
        }
    }
    top
}

pub async fn read_story(data_source: String) -> Result<String, Box<dyn std::error::Error>> {
    let resp = reqwest::get(&data_source)
        .await?
        .text_with_charset("utf-8")
        .await?;
    let story = unescape(&resp).unwrap();
    let words = word_freq(&story);
    let letters = letter_freq(&story);
    let words = top_n_entries(5, &words);
    let letters = top_n_entries(5, &letters);
    let out = format!("{}\n{:#?}\n{:#?}", story, words, letters);
    Ok(out)
}
