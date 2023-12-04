use regex::Regex;

fn convert_string_to_number(value: &str) -> &str{
    match value{
        "one"=>return "1",
        "two"=>return "2",
        "three"=>return "3",
        "four"=>return "4",
        "five"=>return "5",
        "six"=>return "6",
        "seven"=>return "7",
        "eight"=>return "8",
        "nine"=>return "9",
        _=>return value
    }
}

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

    println!("Problem one: {:#?}", total_sum);
}

pub fn problem_two(s: [&str; 1000]) {
    let mut total_sum: i32 = 0;
    let regex = Regex::new(r"([1-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    for index in s {
        let mut matches: Vec<String> = Vec::new();
        for cap in index.char_indices().filter_map(|(i,_)| regex.captures(&index[i..]))  {
            matches.push(cap[1].to_string());
        }
        //println!("Problem two: {:#?}", total_sum);
        let first: &str = convert_string_to_number(matches[0].as_str());
        let last: &str = convert_string_to_number(matches[matches.len() - 1].as_str());
        let mut number: i32 = 0;
        let num_matches = matches.len();
        if num_matches > 0 {
            let number_as_str: String = format!("{first}{last}");
            number = number_as_str.parse::<i32>().unwrap();
        }
        total_sum += number;
    }

    println!("Problem two: {:#?}", total_sum);
}