use criterion::{criterion_group, criterion_main};

pub mod ch02 {
    use criterion::{Criterion};
    use rand::thread_rng;
    use tawep::ch02::*;

    pub fn sort_using_builtin_cmp(c: &mut Criterion) {
        let mut rng = thread_rng();
        let (l, n) = (1 << 18, 1 << 14);
        let s = generate_string(&mut rng, l);
        // 1b) Sample values
        let mut vs = sample_strings(&mut rng, &s, n);

        c.bench_function(&format!("Builtin 'str.cmp()'[{} substrings/string {}]", l, n),
                         |b| b.iter(|| vs.sort_by(|a, b| compare_builtin(a, b))));
    }

    pub fn sort_using_char_cmp(c: &mut Criterion) {
        let mut rng = thread_rng();
        let (l, n) = (1 << 18, 1 << 14);
        let s = generate_string(&mut rng, l);
        // 1b) Sample values
        let mut vs = sample_strings(&mut rng, &s, n);
        // next| write custom string comparators

        c.bench_function(&format!("By-char comparison[{} substrings/string {}]", l, n),
                         |b| b.iter(|| vs.sort_by(|a, b| compare_chars(a, b))));
    }
}

pub mod ch03 {
    use criterion::{black_box, Criterion};

    pub fn bm_add(c: &mut Criterion) {
        let f = |a, b| a + b;
        c.bench_function("addition placeholder", |b| b.iter(|| f(black_box(2), black_box(3))));
    }

}

criterion_group!(
    benches,
    ch02::sort_using_builtin_cmp,
    ch02::sort_using_char_cmp,
    ch03::bm_add);
criterion_main!(benches);
