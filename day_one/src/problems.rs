use regex::Regex;

pub fn problem_one(s: [&str; 1000]) {
    let mut total_sum: i32 = 0;
    let regex = Regex::new(r"([1-9])").unwrap();
    for i in s {
        let mut matches: Vec<&str> = Vec::new();
        for match_str in regex.find_iter(i) {
            matches.push(match_str.as_str());
        }
        let first: &str = matches[0];
        let last: &str = matches[matches.len() - 1];
        let mut number: i32 = 0;
        let num_matches = matches.len();
        if num_matches > 0 {
            let number_as_str: String = format!("{first}{last}");
            number = number_as_str.parse::<i32>().unwrap();
        }
        total_sum += number;
    }

    println!("{:#?}", total_sum);
}