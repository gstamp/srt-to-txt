use regex::Regex;
use srtlib::Subtitles;
use std::fs::File;
use std::io::Write;
use std::{env, process};

fn main() {
    if env::args().len() != 3 {
        println!("Usage: srt-to-txt {{srt-file}} {{output-file}}");
        process::exit(1);
    }
    let args: Vec<String> = env::args().collect();
    let input_path = &args[1];
    let output_path = &args[2];

    let mut subs = Subtitles::parse_from_file(input_path, None).unwrap();

    let mut output_file =
        File::create(output_path).expect("Should have been able to create the output file");

    for s in &mut subs {
        let braces_regex = Regex::new(r"（.*?）").unwrap();
        let braces_regex2 = Regex::new(r"\(.*?\)").unwrap();

        let text = &s.text;
        let cleaned_text = braces_regex.replace_all(text, "").to_string();
        let cleaned_text = braces_regex2.replace_all(&cleaned_text, "").to_string();
        if !cleaned_text.trim().is_empty() {
            writeln!(output_file, "{}", cleaned_text.trim())
                .expect("Should have been able to write to the output file");
        }
    }
}
