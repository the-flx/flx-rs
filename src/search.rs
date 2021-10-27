/**
 * $File: search.rs $
 * $Date: 2021-10-27 20:23:18 $
 * $Revision: $
 * $Creator: Jen-Chieh Shen $
 * $Notice: See LICENSE.txt for modification and distribution information
 *                   Copyright © 2021 by Shen, Jen-Chieh $
 */

use std::collections::HashMap;

pub const WORD_SEPARATORS: [char; 7] = [' ', '-', '_', ':', '.', '.', '\\'];

const DEFAULT_SCORE: i32 = -35;

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

fn inc_vec(vec: &mut Vec<i32>, inc: Option<i32>, beg: Option<i32>, end: Option<i32>) {
    let _inc = inc.unwrap_or(1);
    let mut _beg = beg.unwrap_or(0);
    let _end = end.unwrap_or(vec.len() as i32);
    while _beg < _end {
        vec[_beg as usize] += _inc;
        _beg += 1;
    }
}

fn get_hash_for_string(result: &mut HashMap<Option<u8>, Option<i32>>, str: &str) {
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

fn get_heatmap_str(scores: &mut Vec<i32>, str: &str, group_separator: Option<char>) {
    let str_len: usize = str.len();
    let str_last_index: usize = str_len - 1;
    scores.clear();
    for _n in 0..str_len { scores.push(DEFAULT_SCORE); }
    let penalty_lead: char = '.';
    let mut group_alist: Vec<Vec<i32>> = vec![vec![-1, 0]];

    // final char bonus
    scores[str_last_index] += 1;

    // Establish baseline mapping
    let mut last_char: Option<char> = None;
    let mut group_word_count: i32 = 0;
    let mut index1: usize = 0;

    for char in str.chars() {
        // before we find any words, all separaters are
        // considered words of length 1.  This is so "foo/__ab"
        // gets penalized compared to "foo/ab".
        let effective_last_char: Option<char> = if group_word_count == 0 { None } else { last_char };

        if boundary(effective_last_char, Some(char)) {
            group_alist[0].insert(2, index1 as i32);
        }

        if !word(last_char) && word(Some(char)) {
            group_word_count += 1;
        }

        // ++++ -45 penalize extension
        if last_char != None && last_char.unwrap() == penalty_lead {
            scores[index1] += -45;
        }

        if group_separator != None && group_separator.unwrap() == char {
            group_alist[0][1] = group_word_count;
            group_word_count = 0;
            group_alist.insert(0, vec![index1 as i32, group_word_count]);
        }

        if index1 == str_last_index {
            group_alist[0][1] = group_word_count;
        } else {
            last_char = Some(char);
        }

        index1 += 1;
    }

    let group_count: i32 = group_alist.len() as i32;
    let separator_count: i32 = group_count - 1;

    if separator_count != 0 {
        inc_vec(scores, Some(group_count * -2), None, None);
    }

    let mut index2: i32 = separator_count;
    let mut last_group_limit: Option<i32> = None;
    let mut basepath_found: bool = false;

    for group in group_alist {
        let group_start: i32 = group[0];
        let word_count: i32 = group[1];
        let words_length: usize = group.len() - 2;
        let mut basepath_p: bool = false;

        if words_length != 0 && !basepath_found {
            basepath_found = true;
            basepath_p = true;
        }

        let num: i32;
        if basepath_p {
            // ++++ basepath separator-count boosts
            let mut boosts: i32 = 0;
            if separator_count > 1 {
                boosts = separator_count - 1;
            }
            // ++++ basepath word count penalty
            let penalty: i32 = -word_count;
            num = 35 + boosts + penalty;
        }
        // ++++ non-basepath penalties
        else {
            if index2 == 0 {
                num = -3;
            } else {
                num = -5 + ((index2 as i32) -1);
            }
        }

        inc_vec(scores, Some(num), Some(group_start + 1), last_group_limit);

        let word: i32 = group[2];
        let mut word_index: i32 = (words_length - 1) as i32;
        let mut last_word: i32 = if last_group_limit != None { last_group_limit.unwrap() } else { str_len as i32 };

        while 0 <= word_index {
            // ++++  beg word bonus AND
            scores[word as usize] += 85;

            let mut index3: i32 = word;
            let mut char_i: i32 = 0;
            while index3 < last_word {

                scores[index3 as usize] +=
                    (-3 * word_index) -  // ++++ word order penalty
                    char_i;  // ++++ char order penalty
                char_i += 1;

                index3 +=  1;
            }

            last_word = word;
            word_index -= 1;
        }

        last_group_limit = Some(group_start + 1);
        index2 -= 1;
    }
}

fn find_best_match(imatch: &mut Vec<i32>,
                   str_info: HashMap<Option<u8>, Option<i32>>, heatmap: Vec<i32>,
                   greater_than: bool,
                   query: &str, query_length: usize,
                   q_index: usize) {

}

pub fn score(str: &str, query: &str) -> Option<f32> {
    if str.is_empty() || query.is_empty() {
        return None;
    }
    let mut str_info: HashMap<Option<u8>, Option<i32>> = HashMap::new();
    get_hash_for_string(&mut str_info, str);

    let mut heatmap: Vec<i32> = Vec::new();
    get_heatmap_str(&mut heatmap, str, None);

    let query_length: usize = query.len();
    let full_match_boost: bool = (1 < query_length) && (query_length < 5);
    let mut optimal_match: Vec<i32> = Vec::new();
    find_best_match(&mut optimal_match, str_info, heatmap, false, query, query_length, 0);

    if full_match_boost {

    }

    return None
}
