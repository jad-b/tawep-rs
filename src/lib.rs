#![feature(ascii_char)]
#![feature(ascii_char_variants)]

pub mod chapter2 {
    use rand::{thread_rng, Rng};
    use rand::distributions::{Distribution, Uniform};
    use std::ascii::Char::{SmallA, SmallZ};

    // Char array of length l
    const L : usize = 1 << 18;

    pub fn substring_sort() {
        // Vector of char* of length n
        let _n = 1 << 14;

        // 1) Generate vector of string pointers
        let _vs = generate_string();
        // 2) Execute comparison
        // https://doc.rust-lang.org/std/primitive.slice.html#method.sort
        // 3) Output timing

    }

    fn generate_string() -> [u8; L] {
        // Start with an array filled with 'a'
        let s = [SmallA.to_u8(); L];
        // Create a random lowercase ASCII generator
        let ascii_rng = Uniform::from(SmallA.to_u8()..(SmallZ.to_u8()+1));
        let mut rng = thread_rng();
        for i in 0..L>>10 {
            let c = ascii_rng.sample(&mut rng);
            // NEXT| Fix u8 v. usize
            let idx : u8 = rng.gen() % (L - 1);
            s[idx] = c;
        }

        return s;
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn string_generation_works() {
            let result = generate_string(4);
            assert_eq!(4, result.len(), "Generated a String of size {}", result.len());
        }
    }
}
