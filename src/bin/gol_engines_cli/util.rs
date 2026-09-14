use chrono::Local;
use gol_engines::Pattern;

// BigInt's decimal representation is ASCII. Group the exact digits rather
// than narrowing to a machine integer or relying on num-format's separate
// num-bigint version. This preserves the CLI's underscore-separated output.
fn grouped_decimal(decimal: &str) -> String {
    let (sign, digits) = match decimal.strip_prefix('-') {
        Some(digits) => ("-", digits),
        None => ("", decimal),
    };
    let mut result = String::with_capacity(decimal.len() + digits.len() / 3);
    result.push_str(sign);
    for (index, digit) in digits.chars().enumerate() {
        if index > 0 && (digits.len() - index) % 3 == 0 {
            result.push('_');
        }
        result.push(digit);
    }
    result
}

pub(super) fn print_population(pattern: &Pattern) {
    println!("Population: {}", grouped_decimal(&pattern.population().to_string()));
}

pub(super) fn local_time() -> String {
    Local::now().format("%Y-%m-%dT%H:%M:%S%.3f").to_string()
}

#[cfg(test)]
mod tests {
    use super::grouped_decimal;

    #[test]
    fn groups_decimal_boundaries_and_signs() {
        for (input, expected) in [
            ("0", "0"),
            ("12", "12"),
            ("123", "123"),
            ("1234", "1_234"),
            ("123456", "123_456"),
            ("1234567", "1_234_567"),
            ("-1234567", "-1_234_567"),
        ] {
            assert_eq!(grouped_decimal(input), expected);
        }
    }

    #[test]
    fn preserves_numbers_larger_than_machine_integers() {
        let decimal = "340282366920938463463374607431768211456";
        assert_eq!(
            grouped_decimal(decimal),
            "340_282_366_920_938_463_463_374_607_431_768_211_456"
        );
    }
}
