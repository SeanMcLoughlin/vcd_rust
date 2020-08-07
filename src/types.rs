use crate::error::LoadError;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TimeUnit {
    MS, US, NS, PS
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TimeScale {
    pub value: usize,
    pub unit: TimeUnit,
}

impl TimeScale {
    pub fn new() -> TimeScale {
        TimeScale {value: 0, unit: TimeUnit::MS }
    }

    pub fn load_from_str(s: String) -> TimeScale {
        match s.len() {
            0 => TimeScale::new(),
            _ => {
                let value = TimeScale::value_from_str(&s).unwrap();
                let unit = TimeScale::unit_from_str(&s).unwrap();
                TimeScale {value, unit}
            }
        }
    }

    fn value_from_str(s: &String) -> Result<usize, LoadError> {
        let value_str = TimeScale::remove_alpha(s); 
        if value_str != "" {
            Ok(value_str.parse::<usize>().unwrap())
        } else {
            Err(LoadError{ line: 0, info: String::new() })
        }
    }

    fn unit_from_str(s: &String) -> Result<TimeUnit, LoadError> {
        let unit_str = TimeScale::remove_digit(s);
        match unit_str.as_ref() {
            "ms" => Ok(TimeUnit::MS),
            "us" => Ok(TimeUnit::US),
            "ns" => Ok(TimeUnit::NS),
            "ps" => Ok(TimeUnit::PS),
            _ => Err(LoadError{ line: 0, info: String::new() }),
        }
    }

    fn remove_alpha(s: &String) -> String {
        s.chars().filter(|c| !c.is_alphabetic()).collect()
    }

    fn remove_digit(s: &String) -> String {
        s.chars().filter(|c| c.is_alphabetic()).collect()
    }

}

#[cfg(tests)]
mod tests {
    use super::*;
    #[test]
    fn build_timescale() {
        let time_scale = TimeScale::new().value(10).unit(TimeUnit::NS);
        assert_eq!(time_scale.value, 10);
        assert_eq!(time_scale.unit, TimeUnit::NS);
    }
}
