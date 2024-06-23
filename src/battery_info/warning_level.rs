#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WarningLevel {
    Unknown,
    NoWarning,
    Discharging,
    Low,
    Critical,
    Action,
}

impl From<u32> for WarningLevel {
    fn from(value: u32) -> Self {
        match value {
            1 => WarningLevel::NoWarning,
            2 => WarningLevel::Discharging,
            3 => WarningLevel::Low,
            4 => WarningLevel::Critical,
            5 => WarningLevel::Action,
            _ => WarningLevel::Unknown,
        }
    }
}

impl From<WarningLevel> for u32 {
    fn from(value: WarningLevel) -> Self {
        match value {
            WarningLevel::NoWarning=> 1,
            WarningLevel::Discharging => 2,
            WarningLevel::Low => 3,
            WarningLevel::Critical => 4,
            WarningLevel::Action => 5,
            WarningLevel::Unknown => 0,
        }
    }
}

impl From<WarningLevel> for (u32, WarningLevel) {
    fn from(value: WarningLevel) -> Self {
        let num: u32 = u32::from(value);
        (num, value)
    }
}

pub struct WarningLevelIter {
    index: u32,
}

impl Iterator for WarningLevelIter {
    type Item = WarningLevel;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index > 6 {
            return None;
        }
        let item: WarningLevel = self.index.into();
        self.index += 1;
        Some(item)
    }
}

impl WarningLevel {
    pub fn iter_levels() -> WarningLevelIter {
        WarningLevelIter { index: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_warning_level_enum() {
        assert!(WarningLevel::iter_levels().eq((0u32..=6u32).map(|i| i.into())));

        insta::assert_debug_snapshot!(WarningLevel::iter_levels()
            .map(|t| t.into())
            .collect::<Vec<(u32, WarningLevel)>>(), @r###"
        [
            (
                0,
                Unknown,
            ),
            (
                1,
                NoWarning,
            ),
            (
                2,
                Discharging,
            ),
            (
                3,
                Low,
            ),
            (
                4,
                Critical,
            ),
            (
                5,
                Action,
            ),
            (
                0,
                Unknown,
            ),
        ]
        "###)
    }
}
