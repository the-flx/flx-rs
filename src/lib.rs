/**
 * $File: lib.rs $
 * $Date: 2021-10-17 20:22:21 $
 * $Revision: $
 * $Creator: Jen-Chieh Shen $
 * $Notice: See LICENSE.txt for modification and distribution information
 *                   Copyright © 2021 by Shen, Jen-Chieh $
 */
mod search;

pub use search::{find_best_match, get_heatmap_str, score, Result};

#[cfg(test)]
mod tests {
    use score;

    #[test]
    fn test_score_switch_to_buffer() {
        let result = score("switch-to-buffer", "stb");
        assert_eq!(result.unwrap().score, 237);
    }

    #[test]
    fn test_score_tsfe() {
        let result = score("TestSomeFunctionExterme", "met");
        assert_eq!(result.unwrap().score, 57);
    }

    #[test]
    fn test_score_mxv() {
        let result = score("MetaX_Version", "met");
        assert_eq!(result.unwrap().score, 211);
    }
}
