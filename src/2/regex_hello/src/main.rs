/*
 * Rustで正規表現（regex）。
 * CreatedAt: 2019-07-29
 */
#[macro_use]
extern crate lazy_static;
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
    println!("{}", some_helper_function(CONTENTS));
    println!("{}", some_helper_function("A2019-07B"));
    println!("{}", some_helper_function("A2019-07-29B"));
}
fn some_helper_function(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})").unwrap();
    }
    RE.is_match(text)
}
