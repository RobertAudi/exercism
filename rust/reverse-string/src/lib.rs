extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    // input
    //     .chars()
    //     .rev()
    //     .map(|c| c.to_string())
    //     .collect::<Vec<String>>()
    //     .join("")

    // UnicodeSegmentation::graphemes(input, true)
    //     .collect::<Vec<&str>>()
    //     .iter()
    //     .rev()
    //     .map(|word| word.to_string())
    //     .collect::<Vec<String>>()
    //     .join("")

    input.graphemes(true).rev().collect()
}
