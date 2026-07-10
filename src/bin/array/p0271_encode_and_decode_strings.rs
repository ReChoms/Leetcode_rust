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

        if strs == decoded {
            println!("Result:   PASSED\n");
        } else {
            println!("Result:   FAILED\n");
        }
    }
}
