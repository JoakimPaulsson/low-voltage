#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BatteryLevel {
    Unknown,
    NotApplicable,
    Low,
    Critical,
    Normal,
    High,
    Full,
}

impl From<u32> for BatteryLevel {
    fn from(value: u32) -> Self {
        match value {
            1 => BatteryLevel::NotApplicable,
            2 => BatteryLevel::Low,
            3 => BatteryLevel::Critical,
            4 => BatteryLevel::Normal,
            5 => BatteryLevel::High,
            6 => BatteryLevel::Full,
            _ => BatteryLevel::Unknown,
        }
    }
}

impl From<BatteryLevel> for u32 {
    fn from(value: BatteryLevel) -> Self {
        match value {
            BatteryLevel::NotApplicable => 1,
            BatteryLevel::Low => 2,
            BatteryLevel::Critical => 3,
            BatteryLevel::Normal => 4,
            BatteryLevel::High => 5,
            BatteryLevel::Full => 6,
            BatteryLevel::Unknown => 0,
        }
    }
}

impl From<BatteryLevel> for (u32, BatteryLevel) {
    fn from(value: BatteryLevel) -> Self {
        let num: u32 = u32::from(value);
        (num, value)
    }
}

pub struct BatteryLevelIter {
    index: u32,
}

impl Iterator for BatteryLevelIter {
    type Item = BatteryLevel;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index > 6 {
            return None;
        }
        let item: BatteryLevel = self.index.into();
        self.index += 1;
        Some(item)
    }
}

impl BatteryLevel {
    pub fn iter_levels() -> BatteryLevelIter {
        BatteryLevelIter { index: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_battery_level_enum() {
        assert!(BatteryLevel::iter_levels().eq((0u32..=6u32).map(|i| i.into())));

        insta::assert_debug_snapshot!(BatteryLevel::iter_levels()
            .map(|t| t.into())
            .collect::<Vec<(u32, BatteryLevel)>>(), @r###"
        [
            (
                0,
                Unknown,
            ),
            (
                1,
                NotApplicable,
            ),
            (
                2,
                Low,
            ),
            (
                3,
                Critical,
            ),
            (
                4,
                Normal,
            ),
            (
                5,
                High,
            ),
            (
                6,
                Full,
            ),
        ]
        "###)
    }
}
