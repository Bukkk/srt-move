use srtlib::Subtitle;

extern crate srtlib;

fn main() {
    let input: String = std::env::args().nth(1).expect("No input file given");
    let output: String = std::env::args().nth(2).expect("No output file given");
    let mode: String = std::env::args().nth(3).expect("No mode given");
    let time: String = std::env::args().nth(4).expect("No time given");

    let mut subs =
        srtlib::Subtitles::parse_from_file(input, None).expect("Failed to parse .srt file");
    let time = srtlib::Timestamp::parse(&time).expect("Failed to parse time");

    let f = match mode.as_str() {
        "+" => Subtitle::add,
        "-" => Subtitle::sub,
        _ => panic!("Invalid mode"),
    };

    for mut subtitle in &mut subs {
        f(&mut subtitle, &time);
    }

    subs.write_to_file(output, None).expect("Failed to write to file");
}
