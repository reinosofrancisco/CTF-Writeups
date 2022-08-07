
fn main() {
    fn gcd(a: i128, b: i128) -> i128 {
        if b == 0 {
            a.abs()
        } else {
            gcd(b, a % b)
        }
    }
    
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
       
        
        let multiplier = 7904681699700731398014734140051852539595806699214201704996640156917030632322659247608208994194840235514587046537148300460058962186080655943804500265088604049870276334033409850015651340974377752209566343260236095126079946537115705967909011471361527517536608234561184232228641232031445095605905800675590040729;
        let increment = 16276123569406561065481657801212560821090379741833362117064628294630146690975007397274564762071994252430611109538448562330994891595998956302505598671868738461167036849263008183930906881997588494441620076078667417828837239330797541019054284027314592321358909551790371565447129285494856611848340083448507929914;
    }



    
}