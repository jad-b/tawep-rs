use rand::{thread_rng, Rng};
use rand::distributions::{Distribution, Uniform};
use std::ascii::Char::{SmallA, SmallZ};
use std::cmp::Ordering;
use std::time::Instant;

pub fn substring_sort(string_size :usize, num_substrings : usize) {
    // 1a) Generate possible values
    let mut rng = thread_rng();
    let s = generate_string(&mut rng, string_size);
    // 1b) Sample values
    let mut vs = sample_strings(&mut rng, &s, num_substrings);

    // 2) Sort
    // https://doc.rust-lang.org/std/primitive.slice.html#method.sort
    let t1 = Instant::now();
    let mut count = 0;

    vs.sort_by(|a, b| {
        count += 1;
        return compare_builtin(a, b);
    });

    println!("Sort time: {}ms ({} comparisons)", t1.elapsed().as_millis(), count);
}

pub fn compare_builtin(s1 : &str, s2 : &str) -> Ordering {
    return s1.cmp(s2);
}

pub fn sample_strings <'a, R: Rng + Sized> (rng : &mut R, s : &'a str, n : usize) -> Vec<&'a str> {
    // Allocate a big enough Vector
    let mut vs : Vec<&str> = Vec::with_capacity(n);
    for _i in 0..n {
        let s_idx = rng.gen::<usize>() % (s.len() - 1);
        vs.push(&s[s_idx..]);
        // println!("vs[{}]|{}| = @s[{}]", _i, vs[_i].len(), s_idx)
    }
    println!("|vs|={}", vs.len());
    return vs
}

pub fn generate_string<R: Rng + Sized>(rng : &mut R, l : usize) -> String {
    // Start with an Vector of 'a' bytes
    let mut s = vec![SmallA.to_u8(); l];
    // Create a random lowercase ASCII generator
    let ascii_rng = Uniform::new_inclusive(SmallA.to_u8(), SmallZ.to_u8());

    for _i in 0..l>>10 {
        let idx = rng.gen::<usize>() % (l - 1);
        let c = ascii_rng.sample(rng);
        s[idx] = c;
        // println!("[{}] = {}", idx, Char::from_u8(c).unwrap());
    }

    println!("|s|={}", l);
    return String::from_utf8(s).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_substrings() {
        substring_sort(1 << 18, 1 << 14);
    }

    #[test]
    fn generates_string_of_size_l() {
        let l = 1 << 18;
        let mut rng = thread_rng();
        let result = generate_string(&mut rng, l);
        // println!("First 256 characters: {}", &result[..256]);
        assert_eq!(l, result.len(), "Generated a String of size {}", result.len());
    }

    #[test]
    fn samples_n_strings() {
        let mut rng = thread_rng();
        let s = generate_string(&mut rng, 1 << 18);
        let result = sample_strings(&mut rng, &s, 1 << 14);
        assert_eq!(1 << 14, result.len(), "Sample {} strings", result.len());
    }
}
