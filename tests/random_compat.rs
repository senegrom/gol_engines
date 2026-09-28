use gol_engines::Pattern;

#[test]
fn seeded_random_preserves_the_rand_09_stream() {
    // Captured from the unmodified main-branch implementation with rand 0.9.
    let cases: &[(u32, u64, u64)] = &[
        (0, 0u64, 0u64),
        (0, 1u64, 1u64),
        (0, 18446744073709551615u64, 0u64),
        (1, 0u64, 768u64),
        (1, 1u64, 257u64),
        (1, 18446744073709551615u64, 2u64),
        (2, 0u64, 118098700u64),
        (2, 1u64, 201592065u64),
        (2, 18446744073709551615u64, 50793486u64),
        (3, 0u64, 13080132717333068652u64),
        (3, 1u64, 7424550030962593201u64),
        (3, 18446744073709551615u64, 12619125218144107694u64),
        (4, 0u64, 2329968226987268931u64),
        (4, 1u64, 8480771531554549419u64),
        (4, 18446744073709551615u64, 5454549052769321427u64),
        (6, 0u64, 3121705770391285476u64),
        (6, 1u64, 327900113390959785u64),
        (6, 18446744073709551615u64, 12633332913864900153u64),
    ];
    for &(size, seed, expected_hash) in cases {
        let pattern = Pattern::random(size, Some(seed)).unwrap();
        assert_eq!(pattern.hash(), expected_hash, "size={size}, seed={seed}");
    }
}

#[test]
fn unseeded_random_builds_valid_patterns() {
    for size in [3u32, 4, 6] {
        let pattern = Pattern::random(size, None).unwrap();
        assert_eq!(pattern.get_size_log2(), size);
        assert!(pattern.population() >= 0.into());
        assert!(pattern.population() <= (1u64 << (2 * size)).into());
    }
}

#[test]
fn oversized_random_pattern_is_rejected() {
    assert!(Pattern::random(usize::BITS / 2, Some(0)).is_err());
    assert!(Pattern::random(usize::BITS / 2, None).is_err());
}
