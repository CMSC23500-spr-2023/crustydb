use criterion::{black_box, Criterion};

pub fn sm_ins_bench(c: &mut Criterion) {
    let to_insert = get_random_vec_of_byte_vec(1000, 80, 100);
}
