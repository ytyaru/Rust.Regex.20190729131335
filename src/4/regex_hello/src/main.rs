/*
 * Rustで正規表現（regex）。
 * CreatedAt: 2019-07-29
 */
use regex::Regex;

fn main() {
    const CONTENTS: &'static str = "MZ-80K2E";
    let re = Regex::new(r"\d+").unwrap();
    let result = re.replace_all(CONTENTS, |caps: &regex::Captures| {
        let num: u32 = (&caps[0]).parse().unwrap();
        format!("{:04}", num)
    });
    println!("{}", result);
}
