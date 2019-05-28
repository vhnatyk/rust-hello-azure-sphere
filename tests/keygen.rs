#[macro_use]
extern crate criterion;
extern crate emerald_city;

mod common;

#[test]
fn test_keygen_t1_n2() {
    common::keygen_t_n_parties(1, 2);
}

#[test]
fn test_keygen_t2_n3() {
    common::keygen_t_n_parties(2, 3);
}

#[test]
fn test_keygen_t2_n4() {
    common::keygen_t_n_parties(2, 4);
}

pub mod bench {
    use criterion::Criterion;

    pub fn bench_full_keygen_party_one_two(c: &mut Criterion) {
        c.bench_function("keygen t=1 n=2", move |b| {
            b.iter(|| {
                super::common::keygen_t_n_parties(1, 2);
            })
        });
    }

    pub fn bench_full_keygen_party_two_three(c: &mut Criterion) {
        c.bench_function("keygen t=2 n=3", move |b| {
            b.iter(|| {
                super::common::keygen_t_n_parties(2, 3);
            })
        });
    }

    criterion_group! {
    name = keygen;
    config = Criterion::default().sample_size(super::common::BENCH_SAMPLE_SIZE);
    targets =
    self::bench_full_keygen_party_one_two,
    self::bench_full_keygen_party_two_three
    }
}
criterion_main!(bench::keygen);
