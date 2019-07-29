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
    let result = re.replace_all(CONTENTS, "");
    println!("{}", result);
}
