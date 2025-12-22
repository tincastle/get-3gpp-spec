use regex::Regex;

/// Struct holding parsed spec number parts.
#[derive(Debug, PartialEq, Eq)]
pub struct SpecNumber {
    pub series: String,
    pub number: String,
}

impl std::str::FromStr for SpecNumber {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_spec_number(s)
    }
}

/// Parse a `spec_number` into its two-character series and the trailing number part.
/// Accepts strings that start with two digits, optionally a dot, then at least one alphanumeric.
/// Returns `Ok(SpecNumber)` on success, or `Err(String)` with a human-friendly message on failure.
pub fn parse_spec_number(spec: &str) -> Result<SpecNumber, String> {
    let re = Regex::new(r"^\d{2}\.?[A-Za-z0-9]+$")
        .map_err(|e| format!("internal regex error: {}", e))?;
    if !re.is_match(spec) {
        return Err(format!(
            "invalid spec_number '{}': must start with two digits, optionally a dot, then at least one alphanumeric character",
            spec
        ));
    }

    let series = &spec[0..2];
    let mut rest = &spec[2..];
    if rest.starts_with('.') {
        rest = &rest[1..];
    }

    Ok(SpecNumber {
        series: series.to_string(),
        number: rest.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::{parse_spec_number, SpecNumber};

    #[test]
    fn valid_examples() {
        assert_eq!(
            parse_spec_number("23a"),
            Ok(SpecNumber { series: "23".to_string(), number: "a".to_string() })
        );
        assert_eq!(
            parse_spec_number("23.a"),
            Ok(SpecNumber { series: "23".to_string(), number: "a".to_string() })
        );
        assert_eq!(
            parse_spec_number("00Z"),
            Ok(SpecNumber { series: "00".to_string(), number: "Z".to_string() })
        );
        assert_eq!(
            parse_spec_number("99.1"),
            Ok(SpecNumber { series: "99".to_string(), number: "1".to_string() })
        );
        assert_eq!(
            parse_spec_number("45B6"),
            Ok(SpecNumber { series: "45".to_string(), number: "B6".to_string() })
        );
    }

    #[test]
    fn invalid_examples() {
        assert!(parse_spec_number("2a").is_err()); // only one leading digit
        assert!(parse_spec_number(".23a").is_err()); // starts with dot
        assert!(parse_spec_number("ab23").is_err()); // doesn't start with digits
        assert!(parse_spec_number("23.").is_err()); // dot with no alnum after
        assert!(parse_spec_number("").is_err());
    }
}
