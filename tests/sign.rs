#[macro_use]
extern crate criterion;

mod common;

#[test]
fn test_sign_n3_t1_ttag2() {
    common::sign(1, 3, 2, vec![0, 2]);
}

#[test]
fn test_sign_n3_t2_ttag3() {
    common::sign(2, 3, 3, vec![0, 1, 2]);
}

#[test]
fn test_sign_n5_t2_ttag4() {
    common::sign(2, 5, 4, vec![0, 2, 3, 4])
}

#[test]
fn test_sign_n8_t4_ttag6() {
    common::sign(4, 8, 6, vec![0, 1, 2, 4, 6, 7])
}

pub mod bench {
    use criterion::Criterion;

    pub fn bench_sign_n3_t1_ttag2(c: &mut Criterion) {
        c.bench_function("sign n=3 t=1 ttag=2", move |b| {
            b.iter(|| {
                super::common::sign(1, 3, 2, vec![0, 2]);
            })
        });
    }

    pub fn bench_sign_n3_t2_ttag3(c: &mut Criterion) {
        c.bench_function("sign n=3 t=2 ttag=3", move |b| {
            b.iter(|| {
                super::common::sign(2, 3, 3, vec![0, 1, 2]);
            })
        });
    }

    criterion_group! {
    name = sign;
    config = Criterion::default().sample_size(super::common::BENCH_SAMPLE_SIZE);
    targets =
    self::bench_sign_n3_t1_ttag2,
    self::bench_sign_n3_t2_ttag3
    }
}
criterion_main!(bench::sign);
