#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DeviceType {
    Unknown,
    LinePower,
    Battery,
    Ups,
    Monitor,
    Mouse,
    Keyboard,
    Pda,
    Phone,
    MediaPlayer,
    Tablet,
    Computer,
    GamingInput,
    Pen,
    Touchpad,
    Modem,
    Network,
    Headset,
    Speakers,
    Headphones,
    Video,
    OtherAudio,
    RemoteControl,
    Printer,
    Scanner,
    Camera,
    Wearable,
    Toy,
    BluetoothGenreic,
}

impl From<u32> for DeviceType {
    fn from(value: u32) -> Self {
        match value {
            1 => Self::LinePower,
            2 => Self::Battery,
            3 => Self::Ups,
            4 => Self::Monitor,
            5 => Self::Mouse,
            6 => Self::Keyboard,
            7 => Self::Pda,
            8 => Self::Phone,
            9 => Self::MediaPlayer,
            10 => Self::Tablet,
            11 => Self::Computer,
            12 => Self::GamingInput,
            13 => Self::Pen,
            14 => Self::Touchpad,
            15 => Self::Modem,
            16 => Self::Network,
            17 => Self::Headset,
            18 => Self::Speakers,
            19 => Self::Headphones,
            20 => Self::Video,
            21 => Self::OtherAudio,
            22 => Self::RemoteControl,
            23 => Self::Printer,
            24 => Self::Scanner,
            25 => Self::Camera,
            26 => Self::Wearable,
            27 => Self::Toy,
            28 => Self::BluetoothGenreic,
            _ => Self::Unknown,
        }
    }
}

impl From<DeviceType> for u32 {
    fn from(value: DeviceType) -> Self {
        match value {
            DeviceType::LinePower => 1,
            DeviceType::Battery => 2,
            DeviceType::Ups => 3,
            DeviceType::Monitor => 4,
            DeviceType::Mouse => 5,
            DeviceType::Keyboard => 6,
            DeviceType::Pda => 7,
            DeviceType::Phone => 8,
            DeviceType::MediaPlayer => 9,
            DeviceType::Tablet => 10,
            DeviceType::Computer => 11,
            DeviceType::GamingInput => 12,
            DeviceType::Pen => 13,
            DeviceType::Touchpad => 14,
            DeviceType::Modem => 15,
            DeviceType::Network => 16,
            DeviceType::Headset => 17,
            DeviceType::Speakers => 18,
            DeviceType::Headphones => 19,
            DeviceType::Video => 20,
            DeviceType::OtherAudio => 21,
            DeviceType::RemoteControl => 22,
            DeviceType::Printer => 23,
            DeviceType::Scanner => 24,
            DeviceType::Camera => 25,
            DeviceType::Wearable => 26,
            DeviceType::Toy => 27,
            DeviceType::BluetoothGenreic => 28,
            DeviceType::Unknown => 0,
        }
    }
}

impl From<DeviceType> for (u32, DeviceType) {
    fn from(value: DeviceType) -> Self {
        let num: u32 = u32::from(value);
        (num, value)
    }
}

impl DeviceType {
    pub fn iter_types() -> DeviceTypeIter {
        DeviceTypeIter { index: 0 }
    }
}

pub struct DeviceTypeIter {
    index: u32,
}

impl Iterator for DeviceTypeIter {
    type Item = DeviceType;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index > 28 {
            return None;
        }
        let item: DeviceType = self.index.into();
        self.index += 1;
        Some(item)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_device_type_enum() {
        assert!(DeviceType::iter_types().eq((0u32..=28u32).map(|i| DeviceType::from(i))));

        insta::assert_debug_snapshot!(DeviceType::iter_types()
            .map(|t| t.into())
            .collect::<Vec<(u32, DeviceType)>>(), @r###"
        [
            (
                0,
                Unknown,
            ),
            (
                1,
                LinePower,
            ),
            (
                2,
                Battery,
            ),
            (
                3,
                Ups,
            ),
            (
                4,
                Monitor,
            ),
            (
                5,
                Mouse,
            ),
            (
                6,
                Keyboard,
            ),
            (
                7,
                Pda,
            ),
            (
                8,
                Phone,
            ),
            (
                9,
                MediaPlayer,
            ),
            (
                10,
                Tablet,
            ),
            (
                11,
                Computer,
            ),
            (
                12,
                GamingInput,
            ),
            (
                13,
                Pen,
            ),
            (
                14,
                Touchpad,
            ),
            (
                15,
                Modem,
            ),
            (
                16,
                Network,
            ),
            (
                17,
                Headset,
            ),
            (
                18,
                Speakers,
            ),
            (
                19,
                Headphones,
            ),
            (
                20,
                Video,
            ),
            (
                21,
                OtherAudio,
            ),
            (
                22,
                RemoteControl,
            ),
            (
                23,
                Printer,
            ),
            (
                24,
                Scanner,
            ),
            (
                25,
                Camera,
            ),
            (
                26,
                Wearable,
            ),
            (
                27,
                Toy,
            ),
            (
                28,
                BluetoothGenreic,
            ),
        ]
        "###)
    }
}
