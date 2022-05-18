/**
 * $File: search.rs $
 * $Date: 2021-10-27 20:23:18 $
 * $Revision: $
 * $Creator: Jen-Chieh Shen $
 * $Notice: See LICENSE.txt for modification and distribution information
 *                   Copyright © 2021 by Shen, Jen-Chieh $
 */

use std::collections::{HashMap, VecDeque};
use std::cmp::min;

pub const WORD_SEPARATORS: [u32; 7] = [
    ' ' as u32,
    '-' as u32,
    '_' as u32,
    ':' as u32,
    '.' as u32,
    '.' as u32,
    '\\' as u32,
];

const DEFAULT_SCORE: i32 = -35;

fn word(char: Option<u32>) -> bool {
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

fn capital(char: Option<u32>) -> bool {
    let valid: Option<char> = char::from_u32(char.unwrap());
    if valid == None {
        return false;
    }
    return word(char) && valid.unwrap().is_uppercase()
}

fn boundary(last_char: Option<u32>, char: Option<u32>) -> bool {
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

fn get_hash_for_string(result: &mut HashMap<Option<u32>, VecDeque<Option<u32>>>, str: &str) {
    result.clear();
    let str_len: i32 = str.len() as i32;
    let mut index: i32 = str_len - 1;
    let mut char: Option<u32>;
    let mut down_char: Option<u32>;

    while 0 <= index {
        char = Some(str.chars().nth(index as usize).unwrap() as u32);

        if capital(char) {
            result.entry(char).or_insert_with(VecDeque::new).push_front(Some(index as u32));
            let valid: Option<char> = char::from_u32(char.unwrap());
            down_char = Some(valid.unwrap().to_lowercase().next().unwrap() as u32);
        } else {
            down_char = char;
        }

        result.entry(down_char).or_insert_with(VecDeque::new).push_front(Some(index as u32));

        index -= 1;
    }
}

fn process_cache(result: &mut HashMap<Option<u32>, VecDeque<Option<u32>>>,
                 str: &str,
                 cache: Option<HashMap<&str, HashMap<Option<u32>, VecDeque<Option<u32>>>>>) {
    if cache.is_none() {
        get_hash_for_string(result, str);
        return;
    }

    let mut _cache: HashMap<&str, HashMap<Option<u32>, VecDeque<Option<u32>>>> = cache.unwrap();

    if _cache.contains_key(str) {
        let data: HashMap<Option<u32>, VecDeque<Option<u32>>> = _cache.get(str).unwrap().clone();
        for (key, value) in data {
            result.insert(key, value);
        }
    } else {
        get_hash_for_string(result, str);
        _cache.insert(str, result.clone());
    }
}

pub fn get_heatmap_str(scores: &mut Vec<i32>, str: &str, group_separator: Option<char>) {
    let str_len: usize = str.len();
    let str_last_index: usize = str_len - 1;
    scores.clear();
    for _n in 0..str_len { scores.push(DEFAULT_SCORE); }
    let penalty_lead: u32 = '.' as u32;
    let mut group_alist: Vec<Vec<i32>> = vec![vec![-1, 0]];

    // final char bonus
    scores[str_last_index] += 1;

    // Establish baseline mapping
    let mut last_char: Option<u32> = None;
    let mut group_word_count: i32 = 0;
    let mut index1: usize = 0;

    for char in str.chars() {
        // before we find any words, all separaters are
        // considered words of length 1.  This is so "foo/__ab"
        // gets penalized compared to "foo/ab".
        let effective_last_char: Option<u32> = if group_word_count == 0 { None } else { last_char };

        if boundary(effective_last_char, Some(char as u32)) {
            group_alist[0].insert(2, index1 as i32);
        }

        if !word(last_char) && word(Some(char as u32)) {
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
            last_char = Some(char as u32);
        }

        index1 += 1;
    }

    let group_count: i32 = group_alist.len() as i32;
    let separator_count: i32 = group_count - 1;

    // ++++ slash group-count penalty
    if separator_count != 0 {
        inc_vec(scores, Some(group_count * -2), None, None);
    }

    let mut index2: i32 = separator_count;
    let mut last_group_limit: Option<i32> = None;
    let mut basepath_found: bool = false;

    // score each group further
    for group in group_alist {
        let group_start: i32 = group[0];
        let word_count: i32 = group[1];
        // this is the number of effective word groups
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

        let mut cddr_group: Vec<i32> = group.clone();
        cddr_group.remove(0);
        cddr_group.remove(0);
        let mut word_index: i32 = (words_length - 1) as i32;
        let mut last_word: i32 = if last_group_limit != None { last_group_limit.unwrap() } else { str_len as i32 };

        for word in cddr_group {
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

fn bigger_sublist(result: &mut VecDeque<Option<u32>>,
                  sorted_list: Option<&VecDeque<Option<u32>>>,
                  val: Option<u32>) {
    if sorted_list == None {
        return;
    }
    let _sorted_list: &VecDeque<Option<u32>> = sorted_list.unwrap();
    if val != None {
        let _val: u32 = val.unwrap();
        for sub in _sorted_list {
            if sub.unwrap() > _val {
                result.push_back(Some(sub.unwrap()));
            }
        }
    } else {
        for sub in _sorted_list {
            result.push_back(Some(sub.unwrap()));
        }
    }
}

#[derive(Debug, Clone)]
pub struct Score {
    pub indices: Vec<i32>,
    pub score: i32,
    pub tail: i32,
}

impl Score {
    pub fn new(indices: Vec<i32>, score: i32, tail: i32) -> Score {
        Score { indices, score, tail }
    }
}

pub fn find_best_match(imatch: &mut Vec<Score>,
                       str_info: HashMap<Option<u32>, VecDeque<Option<u32>>>,
                       heatmap: Vec<i32>,
                       greater_than: Option<u32>,
                       query: &str, query_length: i32,
                       q_index: i32,
                       match_cache: &mut HashMap<u32, Vec<Score>>) {
    let greater_num: u32 = if greater_than != None { greater_than.unwrap() } else { 0 };
    let hash_key: u32 = q_index as u32 + (greater_num * query_length as u32);
    let hash_value: Option<&Vec<Score>> = match_cache.get(&hash_key);

    if !hash_value.is_none() {  // Process match_cache here
        imatch.clear();
        for val in hash_value.unwrap() {
            imatch.push(val.clone());
        }
    } else {
        let uchar: Option<u32> = Some(query.chars().nth(q_index as usize).unwrap() as u32);
        let sorted_list: Option<&VecDeque<Option<u32>>> = str_info.get(&uchar);
        let mut indexes: VecDeque<Option<u32>> = VecDeque::new();
        bigger_sublist(&mut indexes, sorted_list, greater_than);
        let mut temp_score: i32;
        let mut best_score: i32 = std::f32::NEG_INFINITY as i32;

        if q_index >= query_length - 1 {
            // At the tail end of the recursion, simply generate all possible
            // matches with their scores and return the list to parent.
            for index in indexes {
                let mut indices: Vec<i32> = Vec::new();
                let _index: i32 = index.unwrap() as i32;
                indices.push(_index);
                imatch.push(Score::new(indices, heatmap[_index as usize], 0));
            }
        } else {
            for index in indexes {
                let _index: i32 = index.unwrap() as i32;
                let mut elem_group: Vec<Score> = Vec::new();
                find_best_match(&mut elem_group, str_info.clone(), heatmap.clone(), Some(_index as u32), query, query_length, q_index + 1, match_cache);

                for elem in elem_group {
                    let caar: i32 = elem.indices[0];
                    let cadr: i32 = elem.score;
                    let cddr: i32 = elem.tail;

                    if (caar - 1) == _index {
                        temp_score = cadr + heatmap[_index as usize] +
                            (min(cddr, 3) * 15) +  // boost contiguous matches
                            60;
                    } else {
                        temp_score = cadr + heatmap[_index as usize];
                    }

                    // We only care about the optimal match, so only forward the match
                    // with the best score to parent
                    if temp_score > best_score {
                        best_score = temp_score;

                        imatch.clear();
                        let mut indices: Vec<i32> = elem.indices.clone();
                        indices.insert(0, _index);
                        let mut tail: i32 = 0;
                        if (caar - 1) == _index {
                            tail = cddr + 1;
                        }
                        imatch.push(Score::new(indices, temp_score, tail));
                    }
                }
            }
        }

        // Calls are cached to avoid exponential time complexity
        match_cache.insert(hash_key, imatch.clone());
    }
}

pub fn score(str: &str, query: &str,
             cache: Option<HashMap<&str, HashMap<Option<u32>, VecDeque<Option<u32>>>>>)
             -> Option<Score> {
    if str.is_empty() || query.is_empty() {
        return None;
    }
    let mut str_info: HashMap<Option<u32>, VecDeque<Option<u32>>> = HashMap::new();
    process_cache(&mut str_info, str, cache);

    let mut heatmap: Vec<i32> = Vec::new();
    get_heatmap_str(&mut heatmap, str, None);

    let query_length: i32 = query.len() as i32;
    let full_match_boost: bool = (1 < query_length) && (query_length < 5);
    let mut match_cache: HashMap<u32, Vec<Score>> = HashMap::new();
    let mut optimal_match: Vec<Score> = Vec::new();
    find_best_match(&mut optimal_match, str_info, heatmap, None, query, query_length, 0, &mut match_cache);

    if optimal_match.is_empty() {
        return None;
    }

    let mut result_1: Score = optimal_match[0].clone();
    let caar: usize = result_1.indices.len();

    if full_match_boost && caar == str.len() {
        result_1.score += 10000;
    }

    return Some(result_1);
}
