fn find_unknown_params(states: &[i128]) -> (i128, i128, i128) {
    let offset_states = &states[1..];
    // Zip together the state lists, adjusted by one position.
    let zipped_states = states.iter().zip(offset_states.iter());
    // Calculate the difference of the states.
    let diffs: Vec<i128> =
        zipped_states.map(|(k0, k1)| k1 - k0).collect();
    // Build the matrix of the differences
    let offset_diffs_1 = diffs.iter().skip(1);
    let offset_diffs_2 = diffs.iter().skip(2);
    let zipped_diffs =
        diffs.iter().zip(offset_diffs_1).zip(offset_diffs_2);
    let zeroes = zipped_diffs.map(|((s0, s1), s2)| s2 * s0 - s1 * s1);
    // Find the greatest common divisor of the differences
    let modulus = zeroes.fold(0, |a, b| gcd(a, b));
    // find_unknown_multiplier(states, modulus)
}

find_unknown_params(&states)
=> (6329, 43291, 4294967301)