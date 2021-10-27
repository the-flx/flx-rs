/**
 * $File: search.rs $
 * $Date: 2021-10-18 20:03:12 $
 * $Revision: $
 * $Creator: Jen-Chieh Shen $
 * $Notice: See LICENSE.txt for modification and distribution information
 *                   Copyright © 2021 by Shen, Jen-Chieh $
 */

use std::collections::HashMap;

//use unicode_normalization::UnicodeNormalization;

pub const WORD_SEPARATORS: [char; 7] = [' ', '-', '_', ':', '.', '.', '\\'];

const DEFAULT_SCORE: f32 = -35.0;

fn word(char: Option<char>) -> bool {
    if char == None {
        return false;
    }
    let _char : char = char.unwrap();
    for c in WORD_SEPARATORS.iter() {
        if *c == _char {
            return false;
        }
    }
    return true
}

fn capital(char: Option<char>) -> bool {
    let _char : char = char.unwrap();
    return word(char) && _char.is_uppercase()
}

fn boundary(last_char: Option<char>, char: Option<char>) -> bool {
    if last_char == None {
        return true;
    }
    if !capital(last_char) && capital(char) {
        return true;
    }
    if !word(last_char) && word(char) {
        return true;
    }
    return false
}

fn inc_vec(vec: &mut Vec<f32>, inc: Option<f32>, beg: Option<i32>, end: Option<i32>) {
    let _inc = inc.unwrap_or(1.0);
    let mut _beg = beg.unwrap_or(0);
    let _end = end.unwrap_or(vec.len() as i32);
    while _beg < _end {
        vec[_beg as usize] += _inc;
        _beg += 1;
    }
}

fn get_hash_for_string(result: &mut HashMap<Option<u8>, Option<f32>>, str: &str) {
    result.clear();
    let str_len: i32 = str.len() as i32;
    let mut index: i32 = str_len - 1;
    let mut char: Option<char>;
    let mut down_char: Option<char>;

    while 0 <= index {
        char = str.chars().nth(index as usize);

        if capital(char) {
            result.insert(Some(char.unwrap() as u8), None);
            //down_char = &char.to_lowercase();
            down_char = char;
        } else {
            down_char = char;
        }

        result.insert(Some(down_char.unwrap() as u8), None);

        index -= 1;
    }
}

fn get_heatmap_str(scores: &mut Vec<f32>, str: &str, group_separator: Option<char>) {
    let str_len: usize = str.len();
    let mut str_last_index: usize = str_len - 1;
    scores.clear();
    for _n in 0..str_len { scores.push(DEFAULT_SCORE); }
    let penalty_lead: char = '.';
    let mut group_alist: Vec<i32> = vec![-1, 0];

    // final char bonus
    scores[str_last_index] += 1.0;

    // Establish baseline mapping
    let mut last_char: Option<char> = None;
    let mut group_word_count: i32 = 0;
    let mut index: usize = 0;

    for char in str.chars() {
        // before we find any words, all separaters are
        // considered words of length 1.  This is so "foo/__ab"
        // gets penalized compared to "foo/ab".
        let effective_last_char: Option<char> = if group_word_count == 0 { None } else { last_char };

        if boundary(effective_last_char, Some(char)) {
            group_alist.insert(1, index as i32);
        }

        if !word(last_char) && word(Some(char)) {
            group_word_count += 1;
        }

        // ++++ -45 penalize extension
        if last_char.unwrap() == penalty_lead {
            scores[index] += -45.0;
        }
        if group_separator != None && group_separator.unwrap() == char {
            group_word_count = 0;
        }
        if index == str_last_index {

        } else {
            last_char = Some(char);
        }

        index += 1;

        println!("{:?}", group_alist);
    }

    let group_count: f32 = group_alist.len() as f32;
    let separator_count: f32 = group_count - 1.0;

    if separator_count != 0.0 {
        inc_vec(scores, Some(group_count * -2.0), None, None);
    }

    println!("{:?}", scores);
}

fn find_best_match(str_info: HashMap<Option<u8>, Option<f32>>, heatmap: Vec<f32>,
                   greater_than: bool,
                   query: &str, query_length: usize,
                   q_index: usize) {

}

pub fn score(str: &str, query: &str) -> Option<f32> {
    if str.is_empty() || query.is_empty() {
        return None;
    }
    let mut str_info: HashMap<Option<u8>, Option<f32>> = HashMap::new();
    get_hash_for_string(&mut str_info, str);

    let mut heatmap: Vec<f32> = Vec::new();
    get_heatmap_str(&mut heatmap, str, None);

    let query_length: usize = query.len();
    let full_match_boost: bool = (1 < query_length) && (query_length < 5);
    let optimal_match = find_best_match(str_info, heatmap, false, query, query_length, 0);

    if full_match_boost {

    }

    return None
}
