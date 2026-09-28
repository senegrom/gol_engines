use gol_engines::{BigInt, Pattern, PatternFormat};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

#[test]
fn seeded_patterns_preserve_chacha_bytes_and_small_row_masks() {
    for size in 0u32..=6 {
        let side = 1usize << size;
        for seed in [0, 1, 42, u64::MAX] {
            let mut bytes = vec![0u8; (side * side).div_ceil(8).max(side)];
            ChaCha8Rng::seed_from_u64(seed).fill_bytes(&mut bytes);
            if size < 3 {
                for byte in &mut bytes {
                    *byte &= ((1u32 << side) - 1) as u8;
                }
            }
            let expected = Pattern::from_format(PatternFormat::PackedCells, &bytes).unwrap();
            let actual = Pattern::random(size, Some(seed)).unwrap();
            assert_eq!(actual.get_size_log2(), size);
            assert_eq!(
                actual.to_format(PatternFormat::PackedCells).unwrap(),
                bytes,
                "size={size}, seed={seed}"
            );
            assert_eq!(actual.population(), expected.population());
            assert_eq!(
                actual.to_format(PatternFormat::PackedCells).unwrap(),
                Pattern::random(size, Some(seed))
                    .unwrap()
                    .to_format(PatternFormat::PackedCells)
                    .unwrap()
            );
        }
    }
}

#[test]
fn unseeded_patterns_work_at_small_and_multinode_sizes() {
    for size in 0u32..=6 {
        let pattern = Pattern::random(size, None).unwrap();
        assert_eq!(pattern.get_size_log2(), size);
        assert!(pattern.population() >= BigInt::from(0));
        assert!(pattern.population() <= BigInt::from(1u64 << (2 * size)));
        let bytes = pattern.to_format(PatternFormat::PackedCells).unwrap();
        let restored = Pattern::from_format(PatternFormat::PackedCells, &bytes).unwrap();
        assert_eq!(restored.get_size_log2(), size);
        assert_eq!(
            restored.to_format(PatternFormat::PackedCells).unwrap(),
            bytes
        );
    }
}

#[test]
fn oversized_patterns_fail_before_allocating_or_seeding() {
    for seed in [None, Some(42)] {
        assert!(Pattern::random(usize::BITS / 2, seed).is_err());
    }
}
