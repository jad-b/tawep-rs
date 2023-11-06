pub mod chapter2 {

    pub fn substring_sort() {
        // NEXT|
        //   Define logic using type signatures and stubs

        // Char array of length l
        let l = 1 << 18;
        // Vector of char* of length n
        let n = 1 << 14;

        // 1) Generate vector of string pointers
        let vs = generate_string(l);
        // 2) Execute comparison
        // 3) Output timing

    }

    fn generate_string(string_size: usize) -> String {
        // rng = RNG()
        /* for(
         *   Initial condition:
         *   - char* p = get the head value from the char array
         *   - end = set to (init. mem pos. + 2^18)
         *   Terminal condition:
         *   - p reaches end
         *   Update condition:
         *   - p += num. of randgen'd bytes
         * ) {
         *   rand_data = rng.rando()
         *   Overwrite array[p,p+sizeof(rand_data)] with rand_data
         * }
         */
        let s: Vec<u8> = Vec::with_capacity(string_size);
        return String::from_utf8(s).unwrap();
    }

}

#[cfg(test)]
mod tests {
    // use super::*;

    pub fn add(left: usize, right: usize) -> usize {
        left + right
    }

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
