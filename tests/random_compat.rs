use gol_engines::Pattern;

#[test]
fn seeded_random_preserves_the_rand_09_stream() {
    // Captured from unmodified commit f323c544 with rand 0.9, before the upgrade.
    // Fixed expected values detect stream changes that recomputing the expected
    // bytes with the currently installed RNG would silently accept.
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
