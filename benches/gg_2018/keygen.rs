#[macro_use]
extern crate criterion;
extern crate emerald_city;

mod bench {
    use criterion::Criterion;
    use emerald_city::gg_2018::test;

    pub fn bench_full_keygen_party_one_two(c: &mut Criterion) {
        c.bench_function("keygen t=1 n=2", move |b| {
            b.iter(|| {
                test::keygen_t_n_parties(1, 2);
            })
        });
    }
    pub fn bench_full_keygen_party_two_three(c: &mut Criterion) {
        c.bench_function("keygen t=2 n=3", move |b| {
            b.iter(|| {
                test::keygen_t_n_parties(2, 3);
            })
        });
    }

    criterion_group! {
    name = keygen;
    config = Criterion::default().sample_size(10);
    targets =
    self::bench_full_keygen_party_one_two,
    self::bench_full_keygen_party_two_three}
}

criterion_main!(bench::keygen);
