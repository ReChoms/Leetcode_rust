/*
Design an algorithm to encode a list of strings to a string. The encoded string is then sent over the network and is decoded back to the original list of strings.

Machine 1 (sender) has the function:

String encode(List<String> strs) {
    // ... your code
    return encoded_string;
}
Machine 2 (receiver) has the function:

List<String> decode(String encoded_string) {
    // ... your code
    return decoded_strs;
}
So Machine 1 does:

String encoded_string = encode(strs);
and Machine 2 does:

List<String> decoded_strs = decode(encoded_string);
decoded_strs in Machine 2 should be the same as the input strs in Machine 1.

Implement the encode and decode methods.

Example 1:
Input: strs = ["Hello","World"]
Output: ["Hello","World"]

Example 2:
Input: strs = [""]
Output: [""]

Constraints:
0 <= strs.length < 100
0 <= strs[i].length < 200
strs[i] contains any possible characters out of 256 valid ASCII characters.
*/

struct Solution;

impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        let mut ret = String::new();
        for str in strs {
            ret.push_str(&str.len().to_string());
            ret.push('#');
            ret.push_str(&str.to_string());
        }
        ret
    }

    pub fn decode(s: String) -> Vec<String> {
        let mut ret: Vec<String> = Vec::new();
        let chars: Vec<char> = s.chars().collect();
        let mut st1 = String::new();
        let mut st2 = String::new();
        let mut idx = 0;

        for c in chars {
            if idx == 0 && c != '#' {
                st1.push(c);
            } else if c == '#' && idx == 0 {
                idx = st1.parse().unwrap();
                st1.clear();
                if idx == 0 {
                    ret.push(String::new());
                }
            } else if idx != 0 {
                st2.push(c);
                idx -= 1;
                if idx == 0 {
                    ret.push(st2.clone());
                    st2.clear();
                }
            }
        }

        ret
    }

    pub fn decode_v2(s: String) -> Vec<String> {
        let mut ret = Vec::new();
        // Improvement: Since we know the constraints guarantee ASCII, we can safely and efficiently
        // iterate over the raw bytes of the string instead of allocating an intermediate Vec<char>.
        // Slicing bytes directly is much faster than pushing characters one-by-one.
        let bytes = s.as_bytes();
        let mut i = 0;

        while i < bytes.len() {
            let mut j = i;
            
            // 1. Advance j until we hit the delimiter '#' (ASCII byte b'#')
            while bytes[j] != b'#' {
                j += 1;
            }

            // 2. Extract the length number string from the bytes we passed over
            let length_str = std::str::from_utf8(&bytes[i..j]).unwrap();
            let length: usize = length_str.parse().unwrap();

            // 3. The actual word starts exactly 1 index after the '#'
            let start = j + 1;
            let end = start + length;

            // 4. Extract the word using the calculated slice indices
            let word = std::str::from_utf8(&bytes[start..end]).unwrap().to_string();
            ret.push(word);

            // 5. Instantly jump our main pointer to the start of the next number
            i = end;
        }

        ret
    }
}

fn main() {
    let test_cases = vec![
        vec!["Hello".to_string(), "World".to_string()],
        vec!["".to_string()],
        vec!["abc".to_string(), "def".to_string(), "ghi".to_string()],
        vec!["encode".to_string(), "".to_string(), "decode".to_string()],
        vec!["!@#$%^&*()".to_string(), "1234567890".to_string()],
    ];

    for (i, strs) in test_cases.into_iter().enumerate() {
        println!("Test case {}:", i + 1);
        println!("Original: {:?}", strs);

        let encoded = Solution::encode(strs.clone());
        println!("Encoded:  {:?}", encoded);

        let decoded = Solution::decode(encoded.clone());
        println!("Decoded:  {:?}", decoded);

        let decoded_v2 = Solution::decode_v2(encoded.clone());
        println!("Decoded v2: {:?}", decoded_v2);

        if strs == decoded && strs == decoded_v2 {
            println!("Result:   PASSED\n");
        } else {
            println!("Result:   FAILED\n");
        }
    }

    // --- Performance Benchmarking ---
    println!("--- Benchmarking Performance (100,000 iterations) ---");
    let bench_data = vec![
        "SuperLongWord123".to_string(), 
        "AnotherWord#$$".to_string(), 
        "Short".to_string(), 
        "".to_string()
    ];
    let encoded = Solution::encode(bench_data);
    let iterations = 100_000;

    let start_v1 = std::time::Instant::now();
    for _ in 0..iterations {
        let _ = Solution::decode(encoded.clone());
    }
    let duration_v1 = start_v1.elapsed();

    let start_v2 = std::time::Instant::now();
    for _ in 0..iterations {
        let _ = Solution::decode_v2(encoded.clone());
    }
    let duration_v2 = start_v2.elapsed();

    println!("v1 (For Loop State Machine): {:?}", duration_v1);
    println!("v2 (While Loop Byte Slice):  {:?}", duration_v2);
    
    if duration_v2 < duration_v1 {
        let multiplier = duration_v1.as_secs_f64() / duration_v2.as_secs_f64();
        println!("v2 is {:.2}x faster!", multiplier);
    } else {
        let multiplier = duration_v2.as_secs_f64() / duration_v1.as_secs_f64();
        println!("v1 is {:.2}x faster!", multiplier);
    }
}
