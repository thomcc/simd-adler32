use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use rand::{thread_rng, RngCore};
use simd_adler32::imp;
use simd_adler32::imp::Adler32Imp;

pub fn bench(c: &mut Criterion) {
  let ones = [1; 100_000];
  let zeros = [0; 100_000];
  let mut random = [0; 100_000];

  thread_rng().fill_bytes(&mut random[..]);

  bench_group(c, "scalar", "ones", &ones, imp::scalar::update);
  bench_group(c, "scalar", "zeros", &zeros, imp::scalar::update);
  bench_group(c, "scalar", "random", &random, imp::scalar::update);
}

pub fn bench_group(
  c: &mut Criterion,
  group: &str,
  name: &str,
  data: &[u8],
  imp: Adler32Imp,
) {
  assert_eq!(data.len(), 100_000);

  c.benchmark_group(group)
    .throughput(Throughput::Bytes(10_000))
    .bench_with_input(format!("{}-10k", name), &data[..10_000], |b, data| {
      b.iter(|| black_box(imp(1, 0, data)))
    })
    //
    .throughput(Throughput::Bytes(100_000))
    .bench_with_input(format!("{}-100k", name), &data[..100_000], |b, data| {
      b.iter(|| black_box(imp(1, 0, data)))
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
