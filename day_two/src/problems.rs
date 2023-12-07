#![allow(dead_code)]
use regex::Regex;

fn split_string(splitter: &str, string: &str) -> Vec<String>{
    let iterator = string.split(splitter);
    let mut substrings: Vec<String> = Vec::new();
    for split_string in iterator{
        substrings.push(split_string.to_string());
    }
    return substrings;
}

fn validate_color(color: Vec<String>) -> bool{
    let max_red: i32 = 12;
    let max_green: i32 = 13;
    let max_blue: i32 = 14;
    if(color[1] == "red") {return color[0].parse::<i32>().unwrap() > max_red;}
    else if(color[1] == "green"){return color[0].parse::<i32>().unwrap() > max_green;}
    else if(color[1] == "blue"){return color[0].parse::<i32>().unwrap() > max_blue;}
    else{return true}
}

pub fn problem_one(input: &str) {
    let mut valid_sum: usize = 0; 
    let regex: Regex = Regex::new(r"(Game).*").unwrap();
    for (index, game_str) in regex.find_iter(input).enumerate() {
        let mut game_id_to_add = index + 1;
        let mut game: Vec<String> = split_string(":", game_str.as_str());
        let game_data_as_str = game[1].to_string();
        let mut remove_empty_space_iterator = game_data_as_str.chars();
        remove_empty_space_iterator.next();
        game[1] = remove_empty_space_iterator.as_str().to_string();
        let game_data: &str = game[1].as_str();
        for round_data in game_data.split(';'){
            let colors: Vec<String> = split_string(",", round_data);
            for color in colors.iter(){
                let mut final_split = split_string(" ", color.as_str());
                final_split.retain(|x| x != "");
                if(validate_color(final_split)){
                    game_id_to_add = 0;
                }
            }
        }
        valid_sum += game_id_to_add;
    }
    println!("{:?}", valid_sum);
}