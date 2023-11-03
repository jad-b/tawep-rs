pub mod chapter2 {

    fn substring_sort() {
        /*
         * Setup
         */
        let (l, n) = (1 << 18, 1 << 14);
        // Char array of length l
        // Vector of char* of length n
        {
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
        }
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
