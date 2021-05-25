use rand::Rng;
use std::collections::HashMap;
use std::ops::Add;

const MIN_STR_LEN: usize = 0;
const MAX_STR_LEN: usize = 10000;
const ALPHABET: &'static [char] = &[
    '1', '2', '3', '4', '5', '6', '7', '8', '9', 'q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p',
    'a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'z', 'x', 'c', 'v', 'b', 'n', 'm', 'Q', 'W', 'E',
    'R', 'T', 'Y', 'U', 'I', 'O', 'P', 'A', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L', 'Z', 'X', 'C',
    'V', 'B', 'N', 'M', ' ', '-', '.', ',', '?', '!', '/',
];

pub fn gen_rand_str_with_substr<R: Rng>(
    count: usize,
    dest: &mut Vec<(String, String)>,
    mut rng: R,
) {
    for _i in 0..count {
        let mut buf = String::new();
        for _i in 0..(rng.gen_range(MIN_STR_LEN..=MAX_STR_LEN)) {
            let idx = rng.gen_range(0..ALPHABET.len());
            let ch = ALPHABET[idx];
            buf.push(ch);
        }

        let sub_str = if buf.len() > 0 {
            let source = buf.chars().collect::<Vec<_>>();
            let start_idx = rng.gen_range(0..source.len());
            let end_idx = rng.gen_range(start_idx..=source.len());
            source[start_idx..end_idx].iter().collect()
        } else {
            "".to_string()
        };

        dest.push((buf, sub_str));
    }
}

pub fn kmp_prefix_function(s: &str) -> Vec<usize> {
    let chars = s.as_bytes();
    let mut prefix_func = Vec::with_capacity(chars.len());
    prefix_func.push(0);

    for current in 1..chars.len() {
        let mut matched_prefix = current - 1;
        let mut candidate = prefix_func[matched_prefix];
        while candidate != 0 && chars[current] != chars[candidate] {
            matched_prefix = prefix_func[matched_prefix] - 1;
            candidate = prefix_func[matched_prefix];
        }
        if candidate == 0 {
            let to_push = if chars[current] == chars[0] { 1 } else { 0 };
            prefix_func.push(to_push);
        } else {
            prefix_func.push(candidate + 1);
        }
    }

    prefix_func
}

pub fn kmp_find(source: &str, pattern: &str) -> Option<usize> {
    let pattern_len = pattern.len();
    if pattern.len() > source.len() {
        None
    } else {
        let union = pattern.to_string().add("$").add(source);
        let prefixes = kmp_prefix_function(&union);
        prefixes.iter().enumerate().find_map(|(idx, c)| {
            //println!("> {}", *c);
            if *c == pattern_len {
                Some(idx - 2 * pattern_len)
            } else {
                None
            }
        })
    }
}

pub fn bm_find(source: &str, template: &str) -> Option<usize> {
    if template.len() == 0 {
        return Some(0);
    }
    if template.len() > source.len() {
        return None;
    }

    let source = source.as_bytes();
    let template = template.as_bytes();
    let source_len = source.len();
    let template_len = template.len();

    let mut offset_table = Box::new([template_len; 256]);

    for (i, char) in template.iter().take(template_len - 1).enumerate() {
        offset_table[*char as usize] = template_len - i - 1;
    }

    let mut i = template_len - 1;
    while i < source_len {
        let start = i + 1 - template_len;
        let matched = (0..template_len).all(|j| source[start + j] == template[j]);
        if matched {
            return Some(i + 1 - template_len);
        }
        //println!("i = {}", i);
        i += offset_table[source[i] as usize];
    }
    return None;
}
