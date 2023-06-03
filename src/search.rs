use std::cmp::min;
/**
 * $File: search.rs $
 * $Date: 2021-10-27 20:23:18 $
 * $Revision: $
 * $Creator: Jen-Chieh Shen $
 * $Notice: See LICENSE.txt for modification and distribution information
 *                   Copyright © 2021 by Shen, Jen-Chieh $
 */
use std::collections::{HashMap, VecDeque};

pub const WORD_SEPARATORS: [u32; 7] = [
    ' ' as u32,
    '-' as u32,
    '_' as u32,
    ':' as u32,
    '.' as u32,
    '/' as u32,
    '\\' as u32,
];

const DEFAULT_SCORE: i32 = -35;

fn word(char: Option<u32>) -> bool {
    char.is_some_and(|c| !WORD_SEPARATORS.contains(&c))
}

fn capital(char: Option<u32>) -> bool {
    match char.map(char::from_u32) {
        Some(Some(ch)) => word(char) && ch.is_uppercase(),
        Some(None) => panic!("{:?} is not a valid char", char),
        None => false,
    }
}

fn boundary(last_char: Option<u32>, char: Option<u32>) -> bool {
    if last_char.is_none() {
        return true;
    }

    if !capital(last_char) && capital(char) {
        return true;
    }

    if !word(last_char) && word(char) {
        return true;
    }

    return false;
}

fn inc_vec(vec: &mut Vec<i32>, inc: i32, beg: Option<i32>, end: Option<i32>) {
    let beg = beg.unwrap_or(0) as usize;
    let end = end.unwrap_or(vec.len() as i32) as usize;
    vec[beg..end].iter_mut().for_each(|e| *e += inc);
}

fn get_hash_for_string(str: &str) -> HashMap<Option<u32>, VecDeque<Option<u32>>> {
    let mut result = HashMap::<_, VecDeque<_>>::new();
    str.char_indices()
        .map(|(pos, ch)| (pos as u32, u32::from(ch)))
        .rev()
        .for_each(|(idx, ch)| {
            let down_char = if capital(Some(ch)) {
                result.entry(Some(ch)).or_default().push_front(Some(idx));

                u32::from(
                    char::from_u32(ch)
                        .and_then(|ch| ch.to_lowercase().next())
                        .unwrap(),
                )
            } else {
                ch
            };

            result
                .entry(Some(down_char))
                .or_default()
                .push_front(Some(idx as u32));
        });

    result
}

pub fn get_heatmap_str(str: &str, group_separator: Option<char>) -> Vec<i32> {
    let str_len = str.len();
    let str_last_index = str_len - 1;
    let mut scores = vec![DEFAULT_SCORE; str_len];
    let penalty_lead = '.' as u32;
    let mut group_alist = vec![vec![-1, 0]];

    // final char bonus
    scores[str_last_index] += 1;

    // Establish baseline mapping
    let mut last_char = None;
    let mut group_word_count = 0;
    let mut index1 = 0;

    for char in str.chars() {
        // before we find any words, all separaters are
        // considered words of length 1.  This is so "foo/__ab"
        // gets penalized compared to "foo/ab".
        let effective_last_char = if group_word_count == 0 {
            None
        } else {
            last_char
        };

        if boundary(effective_last_char, Some(char as u32)) {
            group_alist[0].insert(2, index1 as i32);
        }

        if !word(last_char) && word(Some(char as u32)) {
            group_word_count += 1;
        }

        // ++++ -45 penalize extension
        if last_char.is_some_and(|c| c == penalty_lead) {
            scores[index1] += -45;
        }

        if group_separator.is_some_and(|c| c == char) {
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

    let group_count = group_alist.len() as i32;
    let separator_count = group_count - 1;

    // ++++ slash group-count penalty
    if separator_count != 0 {
        inc_vec(&mut scores, group_count * -2, None, None);
    }

    let mut index2 = separator_count;
    let mut last_group_limit = None;
    let mut basepath_found = false;

    // score each group further
    for group in group_alist {
        let group_start = group[0];
        let word_count = group[1];
        // this is the number of effective word groups
        let words_length = group.len() - 2;
        let mut basepath_p = false;

        if words_length != 0 && !basepath_found {
            basepath_found = true;
            basepath_p = true;
        }

        let num;
        if basepath_p {
            // ++++ basepath separator-count boosts
            let mut boosts = 0;
            if separator_count > 1 {
                boosts = separator_count - 1;
            }
            // ++++ basepath word count penalty
            let penalty = -word_count;
            num = 35 + boosts + penalty;
        }
        // ++++ non-basepath penalties
        else {
            if index2 == 0 {
                num = -3;
            } else {
                num = -5 + ((index2 as i32) - 1);
            }
        }

        inc_vec(&mut scores, num, Some(group_start + 1), last_group_limit);

        let mut cddr_group = group.clone();
        cddr_group.remove(0);
        cddr_group.remove(0);
        let mut word_index = words_length as i32 - 1;
        let mut last_word = last_group_limit.unwrap_or(str_len as i32);

        for word in cddr_group {
            // ++++  beg word bonus AND
            scores[word as usize] += 85;

            let mut index3 = word;
            let mut char_i = 0;
            while index3 < last_word {
                scores[index3 as usize] += (-3 * word_index) -  // ++++ word order penalty
                    char_i; // ++++ char order penalty
                char_i += 1;

                index3 += 1;
            }

            last_word = word;
            word_index -= 1;
        }

        last_group_limit = Some(group_start + 1);
        index2 -= 1;
    }

    scores
}

fn bigger_sublist(
    result: &mut VecDeque<Option<u32>>,
    sorted_list: Option<&VecDeque<Option<u32>>>,
    val: Option<u32>,
) {
    if sorted_list == None {
        return;
    }

    let _sorted_list = sorted_list.unwrap();

    if let Some(val) = val {
        for sub in _sorted_list {
            if sub.unwrap() > val {
                result.push_back(*sub)
            }
        }
    } else {
        for sub in _sorted_list {
            result.push_back(*sub)
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
        Score {
            indices,
            score,
            tail,
        }
    }
}

pub fn find_best_match(
    imatch: &mut Vec<Score>,
    str_info: HashMap<Option<u32>, VecDeque<Option<u32>>>,
    heatmap: Vec<i32>,
    greater_than: Option<u32>,
    query: &str,
    query_length: i32,
    q_index: i32,
    match_cache: &mut HashMap<u32, Vec<Score>>,
) {
    let greater_num = greater_than.unwrap_or(0);
    let hash_key = q_index as u32 + (greater_num * query_length as u32);
    let hash_value = match_cache.get(&hash_key);

    if let Some(hash_value) = hash_value {
        imatch.clear();
        for val in hash_value {
            imatch.push(val.clone());
        }
    } else {
        let uchar = Some(query.chars().nth(q_index as usize).unwrap() as u32);
        let sorted_list = str_info.get(&uchar);
        let mut indexes = VecDeque::new();
        bigger_sublist(&mut indexes, sorted_list, greater_than);
        let mut temp_score;
        let mut best_score = std::f32::NEG_INFINITY as i32;

        if q_index >= query_length - 1 {
            // At the tail end of the recursion, simply generate all possible
            // matches with their scores and return the list to parent.
            for index in indexes {
                let mut indices = Vec::new();
                let _index = index.unwrap() as i32;
                indices.push(_index);
                imatch.push(Score::new(indices, heatmap[_index as usize], 0));
            }
        } else {
            for index in indexes {
                let _index = index.unwrap() as i32;
                let mut elem_group = Vec::new();
                find_best_match(
                    &mut elem_group,
                    str_info.clone(),
                    heatmap.clone(),
                    Some(_index as u32),
                    query,
                    query_length,
                    q_index + 1,
                    match_cache,
                );

                for elem in elem_group {
                    let caar = elem.indices[0];
                    let cadr = elem.score;
                    let cddr = elem.tail;

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
                        let mut indices = elem.indices.clone();
                        indices.insert(0, _index);
                        let mut tail = 0;
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

pub fn score(str: &str, query: &str) -> Option<Score> {
    if str.is_empty() || query.is_empty() {
        return None;
    }

    let str_info = get_hash_for_string(str);
    let heatmap = get_heatmap_str(str, None);

    let query_length = query.len() as i32;
    let full_match_boost = (1 < query_length) && (query_length < 5);
    let mut match_cache = HashMap::new();
    let mut optimal_match = Vec::new();
    find_best_match(
        &mut optimal_match,
        str_info,
        heatmap,
        None,
        query,
        query_length,
        0,
        &mut match_cache,
    );

    let mut result_1 = optimal_match.get(0)?.clone();
    let caar = result_1.indices.len();

    if full_match_boost && caar == str.len() {
        result_1.score += 10000;
    }

    return Some(result_1);
}
