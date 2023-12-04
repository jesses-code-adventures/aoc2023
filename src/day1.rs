use serde_json;

static NUMBER_SUBSTRINGS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

enum WordsToNumbers {
    ONE,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
    TEN,
    ELEVEN,
    TWELVE,
    THIRTEEN,
    FOURTEEN,
    FIFTEEN,
    SIXTEEN,
    SEVENTEEN,
    EIGHTEEN,
    NINETEEN,
    NONE,
}

impl From<&str> for WordsToNumbers {
    fn from(string: &str) -> WordsToNumbers {
        match string {
            "one" => WordsToNumbers::ONE,
            "two" => WordsToNumbers::TWO,
            "three" => WordsToNumbers::THREE,
            "four" => WordsToNumbers::FOUR,
            "five" => WordsToNumbers::FIVE,
            "six" => WordsToNumbers::SIX,
            "seven" => WordsToNumbers::SEVEN,
            "eight" => WordsToNumbers::EIGHT,
            "nine" => WordsToNumbers::NINE,
            "ten" => WordsToNumbers::TEN,
            "eleven" => WordsToNumbers::ELEVEN,
            "twelve" => WordsToNumbers::TWELVE,
            "thirteen" => WordsToNumbers::THIRTEEN,
            "fourteen" => WordsToNumbers::FOURTEEN,
            "fifteen" => WordsToNumbers::FIFTEEN,
            "sixteen" => WordsToNumbers::SIXTEEN,
            "seventeen" => WordsToNumbers::SEVENTEEN,
            "eighteen" => WordsToNumbers::EIGHTEEN,
            "nineteen" => WordsToNumbers::NINETEEN,
            _ => WordsToNumbers::NONE,
        }
    }
}

impl Into<usize> for WordsToNumbers {
    fn into(self) -> usize {
        match self {
            WordsToNumbers::ONE => 1,
            WordsToNumbers::TWO => 2,
            WordsToNumbers::THREE => 3,
            WordsToNumbers::FOUR => 4,
            WordsToNumbers::FIVE => 5,
            WordsToNumbers::SIX => 6,
            WordsToNumbers::SEVEN => 7,
            WordsToNumbers::EIGHT => 8,
            WordsToNumbers::NINE => 9,
            WordsToNumbers::TEN => 10,
            WordsToNumbers::ELEVEN => 11,
            WordsToNumbers::TWELVE => 12,
            WordsToNumbers::THIRTEEN => 13,
            WordsToNumbers::FOURTEEN => 14,
            WordsToNumbers::FIFTEEN => 15,
            WordsToNumbers::SIXTEEN => 16,
            WordsToNumbers::SEVENTEEN => 17,
            WordsToNumbers::EIGHTEEN => 18,
            WordsToNumbers::NINETEEN => 19,
            WordsToNumbers::NONE => 0,
        }
    }
}

impl Into<&str> for WordsToNumbers {
    fn into(self) -> &'static str {
        match self {
            WordsToNumbers::ONE => "1",
            WordsToNumbers::TWO => "2",
            WordsToNumbers::THREE => "3",
            WordsToNumbers::FOUR => "4",
            WordsToNumbers::FIVE => "5",
            WordsToNumbers::SIX => "6",
            WordsToNumbers::SEVEN => "7",
            WordsToNumbers::EIGHT => "8",
            WordsToNumbers::NINE => "9",
            WordsToNumbers::TEN => "10",
            WordsToNumbers::ELEVEN => "11",
            WordsToNumbers::TWELVE => "12",
            WordsToNumbers::THIRTEEN => "13",
            WordsToNumbers::FOURTEEN => "14",
            WordsToNumbers::FIFTEEN => "15",
            WordsToNumbers::SIXTEEN => "16",
            WordsToNumbers::SEVENTEEN => "17",
            WordsToNumbers::EIGHTEEN => "18",
            WordsToNumbers::NINETEEN => "19",
            WordsToNumbers::NONE => "",
        }
    }
}

fn get_data() -> Vec<String> {
    let data = include_str!("../data/day1.json");
    let json: serde_json::Value = serde_json::from_str(data).unwrap();
    json.as_array()
        .unwrap()
        .to_vec()
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
}

fn get_number_from_first_and_last_digit(word: &str) -> usize {
    let digits: Vec<char> = word.chars().filter(|c| c.is_numeric()).collect();
    let first = digits[0];
    let last = digits[digits.len() - 1];
    let combined = format!("{}{}", first, last);
    let resp = combined.parse::<usize>().unwrap();
    resp
}

fn replace_words_with_digits(word: &str) -> String {
    let mut resp = word.to_string();
    let mut indexes: Vec<(usize, &str)> = Vec::new();
    for substring in NUMBER_SUBSTRINGS {
        let left_idx = resp.find(substring);
        if let Some(idx) = left_idx {
            indexes.push((idx, substring));
        }
        let right_idx = resp.rfind(substring);
        if let Some(idx) = right_idx {
            if idx == left_idx.unwrap() {
                continue;
            }
            indexes.push((idx, substring));
        }
    }
    if indexes.is_empty() {
        return resp;
    }
    indexes.sort_by(|a, b| a.0.cmp(&b.0));
    let (min_idx, min_value) = indexes[0];
    let (max_idx, max_value) = indexes[indexes.len() - 1];
    resp.insert_str(max_idx, WordsToNumbers::from(max_value).into());
    resp.insert_str(min_idx, WordsToNumbers::from(min_value).into());
    resp
}

pub fn day1() {
    let input_data = get_data();
    let output_numbers: Vec<usize> = input_data
        .iter()
        .map(|string| {
            let modified = replace_words_with_digits(&string);
            let number = get_number_from_first_and_last_digit(modified.as_str());
            number
        })
        .collect();
    let mut total = 0;
    for number in output_numbers {
        total += number;
    }
    println!("{:?}", total);
}
