use std::fs;
use std::time::Instant;

fn main() {
    let i: Vec<String> = vec!["Lorem", "ipsum", "dolor", "sit", "amet", "consetetur", "sadipscing", "elitr", "sed", "diam", "nonumy", "eirmod", "tempor", "invidunt", "ut", "labore", "et", "dolore", "magna", "aliquyam"].iter().map(|x| x.to_string()).collect();
    let result = counting_sort(&i, |word| word.len(), 20);
    println!("{:?}", result);

    let filename = "../text.txt";
    let content = fs::read_to_string(filename).unwrap_or(String::from("Hello World"));
    let words: Vec<String> = content.split_whitespace().map(|x| x.to_string()).collect();

    let now = Instant::now();
    let result = counting_sort(&words, |word| word.len(), 20);
    let duration = now.elapsed();
    println!("took {:?}", duration);
}

/// Runs countingsort
fn counting_sort<E, F: Fn(&E) -> usize>(vec: &[E], value_func: F, classes: usize) -> Vec<&E> {
    let mut counts = vec![0; classes];
    
    for element in vec {
        let word_len = value_func(element);
        counts[word_len] = counts[word_len] + 1;
    }

    let mut end_index = 0;

    for x in 0..counts.len() {
        end_index += counts[x];
        counts[x] = end_index;
    }

    assert_eq!(end_index, vec.len());

    let mut output = vec![None; vec.len()];

    for element in vec.iter().rev() {
        let element_value = value_func(element);
        output[counts[element_value] - 1] = Some(element);
        counts[element_value] = counts[element_value] - 1;
    }

    output.iter().map(|x| x.unwrap()).collect()
}