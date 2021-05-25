mod matrix_sort;
mod triangle_with_perimeter;
mod max_num;

fn main() {
    println!("\nСортировка диагонали сатрицы");
    matrix_sort::matrix();

    println!("\nНаиболььшее число");
    max_num::max();

    println!("\nТреугольник с наибольшим периметром");
    triangle_with_perimeter::tri();

    println!("\nПобедитель строк");
    println!("При s1={}; s2={} : {}", "abc", "xya", str2_winner("abc", "xya"));
    println!("При s1={}; s2={} : {}", "abe", "acd", str2_winner("abe", "acd"));

    println!("\nСамый длинный полиндром");
    println!("При s={} : {}", "cbbd", str2_longer_pol("cbbd"));
    println!("При s={} : {}", "babad", str2_longer_pol("babad"));

    println!("\nСамый длинная подстрока 'a + a'");
    println!("При s={} : {}", "abcabcabc", str2_concat_c("abcabcabc"));

    println!("\nСтопки монет");
    println!("При in={:?} : {}", &[2,4,1,2,7,8], bob_alice(&[2,4,1,2,7,8]));
    println!("При in={:?} : {}", &[2,4,5], bob_alice(&[2,4,5]));
    println!("При in={:?} : {}", &[9,8,7,6,5,1,2,3,4], bob_alice(&[9,8,7,6,5,1,2,3,4]));

    println!("\nШарики и стрелы");
    println!("При p={:?} : {}", &[[10,16],[2,8],[1,6],[7,12]], balls(&[[10,16],[2,8],[1,6],[7,12]]));
    println!("При p={:?} : {}", &[[1,2],[3,4],[5,6],[7,8]], balls(&[[1,2],[3,4],[5,6],[7,8]]));
    println!("При p={:?} : {}", &[[1,2],[2,3],[3,4],[4,5]], balls(&[[1,2],[2,3],[3,4],[4,5]]));
    println!("При p={:?} : {}", &[[1,2]], balls(&[[1,2]]));
    println!("При p={:?} : {}", &[[2,3],[2,3]], balls(&[[2,3],[2,3]]));

}

fn str2_winner(str1: &str, str2: &str) -> bool {
    let mut s1: Vec<_> = str1.chars().collect();
    let mut s2: Vec<_> = str2.chars().collect();
    s1.sort();
    s2.sort();
    Iterator::zip(s1.iter(), s2.iter()).all(|(a, b)| *a >= *b)
    || Iterator::zip(s1.iter(), s2.iter()).all(|(a, b)| *a <= *b)
}

fn is_pol(s: &str) -> bool {
    let rev = s.chars().rev().collect::<String>();
    s == rev
}

fn str2_longer_pol(s: &str) -> &str {
    let mut longer = &s[0..1];
    for i in 0..(s.len()-1) {
        for j in (i+1)..s.len() {
            let c = &s[i..j];
            if c.len() > longer.len() && is_pol(c) {
                longer = c;
            }
        }
    }
    longer
}


fn str2_concat_c(s: &str) -> usize {
    let mut count = 0;
    for i in 0..(s.len()-1) {
        for j in (i+1)..s.len() {
            let c = &s[i..j];
            if c.len() % 2 == 0 {
                if c[0..(c.len()/2)] == c[(c.len()/2)..c.len()] {
                    count += 1;
                }
            }
        }
    }
    count
}

fn bob_alice(s: &[i32]) -> i32 {
    let mut sum = 0;
    let mut sorted = Vec::from(s);
    sorted.sort();

    let for_me_and_alice = &sorted[(sorted.len()/3)..];
    for i in 0..(for_me_and_alice.len()/2) {
        sum += for_me_and_alice[i*2];
    }
    sum
}

fn merge_ranges(r1: [i32; 2], r2: [i32; 2]) -> Option<[i32; 2]> {
    let left = r1[0].max(r2[0]);
    let right = r1[1].min(r2[1]);
    if left <= right {
        Some([left, right])
    } else {
        None
    }
}

fn balls(r: &[[i32; 2]]) -> usize {
    let mut source = Vec::from(r);
    let mut changed = true;
    'root:
    while changed {
        changed = false;
        for i in 0..(source.len() - 1) {
            let mut has_pair = false;
            for j in (i + 1)..source.len() {
                let r1 = source[i];
                let r2 = source[j];
                match merge_ranges(r1, r2) {
                    Some(r) => {
                        source.remove(j);
                        source.remove(i);
                        source.push(r);
                        changed = true;
                        has_pair = true;
                        continue 'root;
                    }
                    None => (),
                }
            }
        }
    }
    source.len()
}
