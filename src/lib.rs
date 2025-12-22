use regex::Regex;

/// Struct holding parsed spec number parts.
#[derive(Debug, PartialEq, Eq, Clone)]
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

/// Month of year with explicit numeric values 1..=12.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Month {
    January = 1,
    February = 2,
    March = 3,
    April = 4,
    May = 5,
    June = 6,
    July = 7,
    August = 8,
    September = 9,
    October = 10,
    November = 11,
    December = 12,
}

/// Simple filter holding a year and month.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DateFilter {
    pub year: u32,
    pub month: Month,
}

impl std::convert::TryFrom<u8> for Month {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Month::January),
            2 => Ok(Month::February),
            3 => Ok(Month::March),
            4 => Ok(Month::April),
            5 => Ok(Month::May),
            6 => Ok(Month::June),
            7 => Ok(Month::July),
            8 => Ok(Month::August),
            9 => Ok(Month::September),
            10 => Ok(Month::October),
            11 => Ok(Month::November),
            12 => Ok(Month::December),
            _ => Err(format!("invalid month value: {} (expected 1..=12)", value)),
        }
    }
}

impl std::str::FromStr for DateFilter {
    type Err = String;

    /// Parse a date string in YYYY-MM format into `DateFilter`.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^(\d{4})-(\d{2})$")
            .map_err(|e| format!("internal regex error: {}", e))?;
        let caps = re.captures(s).ok_or_else(|| format!("invalid date '{}': must be YYYY-MM", s))?;
        let year: u32 = caps.get(1)
            .ok_or("missing year")?
            .as_str()
            .parse()
            .map_err(|e| format!("invalid year: {}", e))?;
        let month_num: u8 = caps.get(2)
            .ok_or("missing month")?
            .as_str()
            .parse()
            .map_err(|e| format!("invalid month: {}", e))?;
        let month = Month::try_from(month_num)?;
        Ok(DateFilter { year, month })
    }
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
