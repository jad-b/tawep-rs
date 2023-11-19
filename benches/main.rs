#![feature(test)]

extern crate test;

#[cfg(test)]
mod ch02 {
    use rand::thread_rng;
    use tawep::ch02::{generate_string, sample_strings, compare_builtin};
    use test::Bencher;

    #[bench]
    fn bench_sort_substrings(b: &mut Bencher) {
        let mut rng = thread_rng();
        let s = generate_string(&mut rng, 1 << 18);
        // 1b) Sample values
        let mut vs = sample_strings(&mut rng, &s, 1 << 14);
        // next| write custom string comparators

        b.iter(|| vs.sort_by(|a, b| return compare_a(a, b)));
    }
}
