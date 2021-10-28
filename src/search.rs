/**
 * $File: search.rs $
 * $Date: 2021-10-27 20:23:18 $
 * $Revision: $
 * $Creator: Jen-Chieh Shen $
 * $Notice: See LICENSE.txt for modification and distribution information
 *                   Copyright © 2021 by Shen, Jen-Chieh $
 */

use std::collections::{HashMap, VecDeque};

pub const WORD_SEPARATORS: [u8; 7] = [
    ' ' as u8,
    '-' as u8,
    '_' as u8,
    ':' as u8,
    '.' as u8,
    '.' as u8,
    '\\' as u8,
];

const DEFAULT_SCORE: i32 = -35;

fn word(char: Option<u8>) -> bool {
    if char == None {
        return false;
    }
    for c in WORD_SEPARATORS.iter() {
        if *c == char.unwrap() {
            return false;
        }
    }
    return true
}

fn capital(char: Option<u8>) -> bool {
    return word(char) && (char.unwrap() as char).is_uppercase()
}

fn boundary(last_char: Option<u8>, char: Option<u8>) -> bool {
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

fn get_hash_for_string(result: &mut HashMap<Option<u8>, VecDeque<Option<u8>>>, str: &str) {
    result.clear();
    let str_len: i32 = str.len() as i32;
    let mut index: i32 = str_len - 1;
    let mut char: Option<u8>;
    let mut down_char: Option<u8>;

    while 0 <= index {
        char = Some(str.chars().nth(index as usize).unwrap() as u8);

        if capital(char) {
            result.entry(char).or_insert_with(VecDeque::new).push_front(Some(index as u8));
            down_char = Some((char.unwrap() as char).to_lowercase().next().unwrap() as u8);
        } else {
            down_char = char;
        }

        result.entry(down_char).or_insert_with(VecDeque::new).push_front(Some(index as u8));

        index -= 1;
    }
}

pub fn get_heatmap_str(scores: &mut Vec<i32>, str: &str, group_separator: Option<char>) {
    let str_len: usize = str.len();
    let str_last_index: usize = str_len - 1;
    scores.clear();
    for _n in 0..str_len { scores.push(DEFAULT_SCORE); }
    let penalty_lead: u8 = '.' as u8;
    let mut group_alist: Vec<Vec<i32>> = vec![vec![-1, 0]];

    // final char bonus
    scores[str_last_index] += 1;

    // Establish baseline mapping
    let mut last_char: Option<u8> = None;
    let mut group_word_count: i32 = 0;
    let mut index1: usize = 0;

    for char in str.chars() {
        // before we find any words, all separaters are
        // considered words of length 1.  This is so "foo/__ab"
        // gets penalized compared to "foo/ab".
        let effective_last_char: Option<u8> = if group_word_count == 0 { None } else { last_char };

        if boundary(effective_last_char, Some(char as u8)) {
            group_alist[0].insert(2, index1 as i32);
        }

        if !word(last_char) && word(Some(char as u8)) {
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
            last_char = Some(char as u8);
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

fn bigger_sublist(result: &mut VecDeque<Option<u8>>,
                  sorted_list: &VecDeque<Option<u8>>,
                  val: Option<u8>) {
    if val != None {
        let _val: u8 = val.unwrap();
        for sub in sorted_list {
            if sub.unwrap() > _val {
                result.push_back(Some(sub.unwrap()));
            }
        }
    } else {
        for sub in sorted_list {
            result.push_back(Some(sub.unwrap()));
        }
    }
}

pub fn find_best_match(imatch: &mut Vec<i32>,
                       str_info: HashMap<Option<u8>, VecDeque<Option<u8>>>,
                       heatmap: Vec<i32>,
                       greater_than: Option<u8>,
                       query: &str, query_length: i32,
                       q_index: i32,
                       match_cache: &mut HashMap<u8, Vec<i32>>) {
    let greater_num: u8 = if greater_than != None { greater_than.unwrap() } else { 0 };
    let hash_key: u8 = q_index as u8 + (greater_num * query_length as u8);
    let mut hash_value: Option<&Vec<i32>> = match_cache.get(&hash_key);

    if hash_value != None {
        for val in hash_value.unwrap() {
            imatch.push(*val as i32);
        }
    } else {
        let uchar: Option<u8> = Some(query.chars().nth(q_index as usize).unwrap() as u8);
        let sorted_list: &VecDeque<Option<u8>> = str_info.get(&uchar).unwrap();
        let mut indexes: VecDeque<Option<u8>> = VecDeque::new();
        bigger_sublist(&mut indexes, sorted_list, greater_than);
        imatch.clear();
        let mut temp_score: i32;
        let mut best_score: i32 = std::f32::NEG_INFINITY as i32;
        println!("sorted_list: {:?}", sorted_list);
        println!("indexes: {:?}", indexes);

        if q_index >= query_length - 1 {
            // At the tail end of the recursion, simply generate all possible
            // matches with their scores and return the list to parent.
            for index in indexes {
                //omatch.push();
            }
        } else {
            for index in indexes {
                let mut elem: Vec<i32> = Vec::new();
                find_best_match(&mut elem, str_info.clone(), heatmap.clone(), Some(index.unwrap() as u8), query, query_length, q_index + 1, match_cache);

                temp_score = 0;

                // We only care about the optimal match, so only forward the match
                // with the best score to parent
                if temp_score > best_score {
                    best_score = temp_score;

                }
            }
        }

        // Calls are cached to avoid exponential time complexity
        match_cache.insert(hash_key, imatch.to_vec());
    }
}

pub fn score(str: &str, query: &str) -> Option<(i32, Vec<i32>)> {
    if str.is_empty() || query.is_empty() {
        return None;
    }
    let mut str_info: HashMap<Option<u8>, VecDeque<Option<u8>>> = HashMap::new();
    get_hash_for_string(&mut str_info, str);

    let mut heatmap: Vec<i32> = Vec::new();
    get_heatmap_str(&mut heatmap, str, None);

    let query_length: i32 = query.len() as i32;
    let full_match_boost: bool = (1 < query_length) && (query_length < 5);
    let mut match_cache: HashMap<u8, Vec<i32>> = HashMap::new();
    let mut optimal_match: Vec<i32> = Vec::new();
    find_best_match(&mut optimal_match, str_info, heatmap, None, query, query_length, 0, &mut match_cache);

    if full_match_boost {

    }

    return None
}
