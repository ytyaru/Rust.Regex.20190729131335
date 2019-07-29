/*
 * Rustで正規表現（regex）。
 * CreatedAt: 2019-07-29
 */
use regex::Regex;

fn main() {
    const CONTENTS: &'static str = r#"AAA
        2019-07-29 BBB
        CCC 2019-07-30 DDD"#;
    let re = Regex::new(r"(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})").unwrap();
    for caps in re.captures_iter(CONTENTS) {
        println!("year: {}, month: {}, day: {}",
            caps.get(1).unwrap().as_str(),
            caps.get(2).unwrap().as_str(),
            caps.get(3).unwrap().as_str());
    }
}
