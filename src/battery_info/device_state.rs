#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceState {
    Unknown,
    Charging,
    Discharging,
    Empty,
    FullyCharged,
    PendingCharge,
    PendingDischarge,
}

impl From<u32> for DeviceState {
    fn from(value: u32) -> Self {
        match value {
            1 => DeviceState::Charging,
            2 => DeviceState::Discharging,
            3 => DeviceState::Empty,
            4 => DeviceState::FullyCharged,
            5 => DeviceState::PendingCharge,
            6 => DeviceState::PendingDischarge,
            _ => DeviceState::Unknown,
        }
    }
}

impl From<DeviceState> for u32 {
    fn from(value: DeviceState) -> Self {
        match value {
            DeviceState::Charging => 1,
            DeviceState::Discharging => 2,
            DeviceState::Empty => 3,
            DeviceState::FullyCharged => 4,
            DeviceState::PendingCharge => 5,
            DeviceState::PendingDischarge => 6,
            DeviceState::Unknown => 0,
        }
    }
}

impl From<DeviceState> for (u32, DeviceState) {
    fn from(value: DeviceState) -> Self {
        let num: u32 = u32::from(value);
        (num, value)
    }
}

pub struct DeviceStateIter {
    index: u32,
}

impl Iterator for DeviceStateIter {
    type Item = DeviceState;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index > 6 {
            return None;
        }
        let item: DeviceState = self.index.into();
        self.index += 1;
        Some(item)
    }
}

impl DeviceState {
    pub fn iter_levels() -> DeviceStateIter {
        DeviceStateIter { index: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_device_state_enum() {
        assert!(DeviceState::iter_levels().eq((0u32..=6u32).map(|i| i.into())));

        insta::assert_debug_snapshot!(DeviceState::iter_levels()
            .map(|t| t.into())
            .collect::<Vec<(u32, DeviceState)>>(), @r###"
        [
            (
                0,
                Unknown,
            ),
            (
                1,
                Charging,
            ),
            (
                2,
                Discharging,
            ),
            (
                3,
                Empty,
            ),
            (
                4,
                FullyCharged,
            ),
            (
                5,
                PendingCharge,
            ),
            (
                6,
                PendingDischarge,
            ),
        ]
        "###)
    }
}
