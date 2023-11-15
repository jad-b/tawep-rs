#![feature(ascii_char)]
#![feature(ascii_char_variants)]

pub mod chapter2 {
    use rand::{thread_rng, Rng};
    use rand::distributions::{Distribution, Uniform};
    use std::ascii::Char::{SmallA, SmallZ};


    pub fn substring_sort() {
        // Size of the string we'll sample substrings from
        let l = 1 << 18;
        // Number of substring samples
        let n = 1 << 14;

        // 1a) Generate l possible values
        let mut rng = thread_rng();
        let s = generate_string(&mut rng, l);
        // 1b) Sample n values
        let _vs = sample_strings(&mut rng, &s, n);

        // 2) Execute comparison
        // https://doc.rust-lang.org/std/primitive.slice.html#method.sort
        // let t1 = // Start timer

        // 3) Output timing

    }

    fn sample_strings <'a, R: Rng + Sized> (rng : &mut R, s : &'a str, n : usize) -> Vec<&'a str> {
        // Allocate a big enough Vector
        let mut vs : Vec<&str> = Vec::with_capacity(n);
        for _i in 0..n {
            let s_idx = rng.gen::<usize>() % (s.len() - 1);
            vs.push(&s[s_idx..]);
            println!("vs[{}]|{}| = @s[{}]", _i, vs[_i].len(), s_idx)
        }
        return vs
    }

    fn generate_string<R: Rng + Sized>(rng : &mut R, l : usize) -> String {
        // Idea| Create a String, filled with 'a', and teach Uniform to generate Chars
        // Start with an Vector of 'a'
        let mut s = vec![SmallA.to_u8(); l];
        println!("Made buffer of size {}", l);
        // Create a random lowercase ASCII generator
        let ascii_rng = Uniform::new_inclusive(SmallA.to_u8(), SmallZ.to_u8());

        for _i in 0..l>>10 {
            let idx = rng.gen::<usize>() % (l - 1);
            let c = ascii_rng.sample(rng);
            s[idx] = c;
            // println!("[{}] = {}", idx, Char::from_u8(c).unwrap());
        }

        return String::from_utf8(s).unwrap();
    }

    #[cfg(test)]
    mod tests {
        use super::*;

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
}
