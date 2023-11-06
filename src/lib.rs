pub mod chapter2 {
    use rand::Rng;

    pub fn substring_sort() {
        // Char array of length l
        let l = 1 << 18;
        // Vector of char* of length n
        let _n = 1 << 14;

        // 1) Generate vector of string pointers
        let _vs = generate_string(l);
        // 2) Execute comparison
        // https://doc.rust-lang.org/std/primitive.slice.html#method.sort
        // 3) Output timing

    }

    fn generate_string(string_size: usize) -> String {
        let s: Vec<u8> = Vec::with_capacity(string_size);
        // NEXT|
        // Fill vec with 'a' (utf8)
        // Swap _some_ of the characters with random bytes
        return String::from_utf8(s).unwrap();
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
