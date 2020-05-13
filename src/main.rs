use std::fs;
use std::time::Instant;

fn main() {
    let i = vec!["Lorem", "ipsum", "dolor", "sit", "amet", "consetetur", "sadipscing", "elitr", "sed", "diam", "nonumy", "eirmod", "tempor", "invidunt", "ut", "labore", "et", "dolore", "magna", "aliquyam"].iter().map(|x| x.to_string()).collect();
    let result = counting_sort(i);
    println!("{:?}", result);

    let filename = "../text.txt";
    let content = fs::read_to_string(filename).unwrap_or(String::from("Hello World"));
    let words: Vec<String> = content.split_whitespace().map(|x| x.to_string()).collect();

    let now = Instant::now();
    let result = counting_sort(words);
    let duration = now.elapsed();
    println!("took {:?}", duration);
}

/// Runs countingsort over the word length (max length 49)
fn counting_sort(vec: Vec<String>) -> Vec<String> {
    let mut counts = vec![0; 50];
    
    for word in &vec {
        let word_len = word.len();
        counts[word_len] = counts[word_len] + 1;
    }

    let mut end_index = 0;

    for x in 0..counts.len() {
        end_index += counts[x];
        counts[x] = end_index;
    }

    assert_eq!(end_index, vec.len());

    let mut output = vec!["".to_string(); vec.len()];

    for word in vec.iter().rev() {
        output[counts[word.len()] - 1] = word.to_string();
        counts[word.len()] = counts[word.len()] - 1;
    }

    output
}