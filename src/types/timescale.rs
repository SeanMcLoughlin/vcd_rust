use crate::error::LoadError;
use std::str::FromStr;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Eq, PartialEq, EnumString, EnumIter)]
pub enum TimeUnit {
    #[strum(serialize = "ms")]
    MS,
    #[strum(serialize = "us")]
    US,
    #[strum(serialize = "ns")]
    NS,
    #[strum(serialize = "ps")]
    PS,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum BuildState {
    Value,
    Unit,
    Done,
}

impl BuildState {
    fn next(&self, line_num: usize) -> Result<Self, LoadError> {
        use BuildState::*;
        match *self {
            Value => Ok(Unit),
            Unit => Ok(Done),
            Done => Err(LoadError::TooManyParameters {
                line: line_num,
                command: "$timescale".to_string(),
            }),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TimeScale {
    pub value: usize,
    pub unit: TimeUnit,
    state: BuildState,
}

impl PartialEq for TimeScale {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.unit == other.unit
    }
}

impl Default for TimeScale {
    fn default() -> Self {
        TimeScale {
            value: 0,
            unit: TimeUnit::MS,
            state: BuildState::Value,
        }
    }
}

impl TimeScale {
    pub fn new(value: usize, unit: TimeUnit) -> TimeScale {
        TimeScale {
            value,
            unit,
            state: BuildState::Value,
        }
    }

    pub fn append(&mut self, word: &str, line_num: usize) -> Result<(), LoadError> {
        match self.state {
            BuildState::Value => self.write_value(word, line_num)?,
            BuildState::Unit => self.write_unit(word, line_num)?,
            _ => {}
        };
        self.state = self.state.next(line_num)?;
        Ok(())
    }

    fn write_unit(&mut self, word: &str, line_num: usize) -> Result<(), LoadError> {
        self.unit = match TimeUnit::from_str(word) {
            Ok(time_scale) => time_scale,
            Err(_) => {
                return Err(LoadError::InvalidTimeScale {
                    line: line_num,
                    time_scale: word.to_string(),
                });
            }
        };
        Ok(())
    }

    fn write_value(&mut self, word: &str, line_num: usize) -> Result<(), LoadError> {
        self.value = match word.parse::<usize>() {
            Ok(value) => value,
            Err(_) => {
                return Err(LoadError::InvalidTimeValue {
                    line: line_num,
                    value: word.to_string(),
                });
            }
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_timescale_1() {
        let mut time_scale = TimeScale::default();
        time_scale.append("10", 0).unwrap();
        time_scale.append("ns", 0).unwrap();
        assert_eq!(time_scale.value, 10);
        assert_eq!(time_scale.unit, TimeUnit::NS);
    }

    #[test]
    fn build_timescale_2() {
        let mut time_scale = TimeScale::default();
        time_scale.append("42", 0).unwrap();
        time_scale.append("ms", 0).unwrap();
        assert_eq!(time_scale.value, 42);
        assert_eq!(time_scale.unit, TimeUnit::MS);
    }

    #[test]
    fn invalid_number_throws_error() {
        let mut time_scale = TimeScale::default();
        let err = time_scale.append("NaN", 0).err();
        let exp_err = LoadError::InvalidTimeValue {
            line: 0,
            value: "NaN".to_string(),
        };
        assert_eq!(err, Some(exp_err));
    }

    #[test]
    fn invalid_timescale_throws_error() {
        let mut time_scale = TimeScale::default();
        time_scale.append("10", 0).unwrap();
        let err = time_scale.append("NotATimeScale", 0).err();
        let exp_err = LoadError::InvalidTimeScale {
            line: 0,
            time_scale: "NotATimeScale".to_string(),
        };
        assert_eq!(err, Some(exp_err));
    }

    #[test]
    fn extra_params_in_timescale_throws_error() {
        let mut time_scale = TimeScale::default();
        time_scale.append("10", 0).unwrap();
        time_scale.append("us", 0).unwrap();
        let err = time_scale.append("ExtraParameter", 0).err();
        let exp_err = LoadError::TooManyParameters {
            line: 0,
            command: "$timescale".to_string(),
        };
        assert_eq!(err, Some(exp_err));
    }
}
