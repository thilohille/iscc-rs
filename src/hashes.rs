//! Feature Hashing
use std::hash::Hasher;

use bit_vec::BitVec;
use twox_hash::{XxHash32, XxHash64};

use crate::constants::MINHASH_PERMUTATIONS;

const MERSENNE_PRIME: u64 = 2_305_843_009_213_693_951;

/// The `minimum_hash` function takes an arbitrary sized set of 32-bit integer
/// features and reduces it to a fixed size vector of 64 features such that it
/// preserves similarity with other sets. It is based on the MinHash
/// implementation of the [datasketch](https://ekzhu.github.io/datasketch/)
/// library by [Eric Zhu](https://github.com/ekzhu).
pub fn minimum_hash(features: Vec<u32>) -> Vec<u32> {
    let mut min_features: Vec<u32> = Vec::new();
    for [a, b] in MINHASH_PERMUTATIONS.iter() {
        let min = features
            .iter()
            .map(|f| ((a.wrapping_mul((*f).into())).wrapping_add(*b) % MERSENNE_PRIME) as u32)
            .min()
            .unwrap();
        min_features.push(min);
    }
    min_features
}

pub fn sliding_window(seq: &str, width: usize) -> Vec<String> {
    assert!(width >= 2, "Sliding window width must be 2 or bigger.");
    let characters: Vec<char> = seq.chars().collect();
    let characters_len = characters.len();
    if characters_len <= width {
        return vec![seq.to_string()];
    }
    let max_idx = characters_len - width;
    let mut result: Vec<String> = Vec::new();
    for i in 0..=max_idx {
        result.push(characters[i..i + width].iter().collect())
    }
    result
}

/// The `similarity_hash` function takes a sequence of hash digests which
/// represent a set of features. Each of the digests MUST be of equal size. The
/// function returns a new hash digest (raw 8-bit bytes) of the same size. For
/// each bit in the input hashes calculate the number of hashes with that bit set
/// and subtract the count of hashes where it is not set. For the output hash
/// set the same bit position to `0` if the count is negative or `1` if it is
/// zero or positive. The resulting hash digest will retain similarity for
/// similar sets of input hashes. See also
/// [Charikar2002](http://dx.doi.org/10.1145/509907.509965).
pub fn similarity_hash(hash_digests: Vec<u64>) -> Vec<u8> {
    assert!(!hash_digests.is_empty());
    let n_digests = hash_digests.len();

    let mut bitcounts: Vec<u64> = vec![0; 64];
    for digest in hash_digests {
        #[allow(clippy::needless_range_loop)]
        for i in 0..64 {
            let bit_i = (digest >> i) & 1;
            bitcounts[i] += bit_i;
        }
    }
    let minfeatures = (n_digests / 2 + n_digests % 2) as u64;
    let shash: BitVec<u64> = bitcounts
        .into_iter()
        .rev()
        .map(|bitcount| bitcount >= minfeatures)
        .collect();
    shash.to_bytes().to_vec()
}

pub fn xxhash32(data: &[u8]) -> u32 {
    let mut hasher = XxHash32::with_seed(0);
    hasher.write(data);
    hasher.finish() as u32
}

pub fn xxhash64(data: &[u8]) -> u64 {
    let mut hasher = XxHash64::with_seed(0);
    hasher.write(data);
    hasher.finish()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(clippy::unreadable_literal)]
    #[test]
    fn test_minimum_hash() {
        let features = vec![
            2307709831, 4057803343, 1189896175, 998490104, 1957593182, 985638384, 1499267049,
            3716940741, 3418313233, 2481613561,
        ];
        let outputs = vec![
            60408839, 417500306, 248076695, 439165054, 16435796, 663273601, 479764472, 349786614,
            101920380, 648920756, 339730954, 615880630, 228027170, 90214669, 561869889, 160815691,
            234003495, 692821200, 197097035, 319162332, 668467202, 99033705, 804386631, 19156741,
            78172280, 209296906, 796384485, 977070588, 106355403, 263520651, 1168853690, 323965204,
            490884707, 16173960, 553061992, 21573926, 258728281, 596549298, 178319044, 21074688,
            34885302, 311991890, 257487873, 255911998, 40150096, 48546619, 113497506, 1907446217,
            7964589, 1197745461, 307475108, 1372978708, 204261673, 53785375, 163685074, 171806364,
            557744027, 137940826, 112234379, 37838865, 511303187, 924908431, 1191964073,
            1069371393,
        ];

        assert_eq!(minimum_hash(features), outputs);
    }
    #[test]
    fn test_sliding_window() {
        assert_eq!(sliding_window("", 4), vec!["".to_string()]);
        assert_eq!(sliding_window("A", 4), vec!["A".to_string()]);
        assert_eq!(
            sliding_window("Hello", 4),
            vec!["Hell".to_string(), "ello".to_string()]
        );
    }
    #[test]
    fn test_similarity_hash() {
        let hash_digests: Vec<u64> = vec![0; 16];
        let expected: Vec<u8> = vec![0; 8];
        assert_eq!(similarity_hash(hash_digests), expected);

        //TODO: More tests
    }

}
