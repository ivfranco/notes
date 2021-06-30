use checksum::{crc::Crc16, fletcher::Fletcher16, xor::Xor, Checksum};
use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use rand::{distributions::Standard, rngs::StdRng, Rng, SeedableRng};

fn checksum_benchmark<C, const N: usize>(name: &str, mut checksum: C, c: &mut Criterion)
where
    C: Checksum<N>,
{
    const SEED: u64 = 0x123445678;
    let mut rng = StdRng::seed_from_u64(SEED);

    c.bench_function(name, |b| {
        b.iter_batched_ref(
            || {
                (&mut rng)
                    .sample_iter(Standard)
                    .take(1024 * 1024)
                    .collect::<Vec<u8>>()
            },
            |bytes| {
                checksum.clear();
                checksum.digest(bytes);
            },
            BatchSize::LargeInput,
        )
    });
}

pub fn checksum_xor(c: &mut Criterion) {
    checksum_benchmark("xor", Xor::new(), c);
}

pub fn checksum_fletcher(c: &mut Criterion) {
    checksum_benchmark("fletcher", Fletcher16::new(), c);
}

pub fn checksum_crc(c: &mut Criterion) {
    checksum_benchmark("crc", Crc16::ccitt_false(), c);
}

criterion_group!(checksum, checksum_xor, checksum_fletcher, checksum_crc);
criterion_main!(checksum);
