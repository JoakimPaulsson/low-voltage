use std::{
    fmt::{
        Debug, Display
    },
    marker::PhantomData,
};

pub struct Device<T, U> {
    value: T,
    is_valid_pred: fn(&T) -> bool,
    _phantom: PhantomData<U>,
}

impl<T, U> Device<T, U> {
    pub fn is_valid(&self) -> bool {
        (self.is_valid_pred)(&self.value)
    }
}

pub trait ValidDevice<T> {
    fn is_valid(value: &T) -> bool;
}

pub trait IntoDevice<T> {
    fn into_device<U>(self) -> Device<T, U>
    where
        U: ValidDevice<T>;
}

impl<T> IntoDevice<T> for T {
    fn into_device<U>(self) -> Device<T, U>
    where
        U: ValidDevice<T>,
    {
        Device {
            value: self,
            is_valid_pred: <U as ValidDevice<T>>::is_valid,
            _phantom: Default::default(),
        }
    }
}

pub trait FormatDevice<T, U>
where
    T: Display,
    U: ValidDevice<T>,
{
    fn format_device(&self) -> String;
}

impl<T, U> FormatDevice<T, U> for Device<T, U>
where
    T: Display,
    U: ValidDevice<T>,
{
    fn format_device(&self) -> String {
        let valid_str = match self.is_valid() {
            true => "valid",
            false => "invalid",
        };

        format!("Device<{}>: {}", valid_str, self.value)
    }
}

impl<T, U> From<T> for Device<T, U>
where
    U: ValidDevice<T>,
{
    fn from(value: T) -> Self {
        Self {
            value,
            is_valid_pred: <U as ValidDevice<T>>::is_valid,
            _phantom: Default::default(),
        }
    }
}

impl<T, U> Display for Device<T, U>
where
    T: Display,
    U: ValidDevice<T>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.format_device())
    }
}

impl<T, U> Debug for Device<T, U>
where
    T: Debug,
    U: ValidDevice<T>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Device {{ value: {:?} }}", self.value)
    }
}

#[cfg(test)]
mod test {
    use insta;

    use super::*;

    #[test]
    fn str_device() {
        struct TestStrDevice;
        impl ValidDevice<&'_ str> for TestStrDevice {
            fn is_valid(value: &&str) -> bool {
                value.ends_with("my_device")
            }
        }

        let my_valid_device = "this is my_device".into_device::<TestStrDevice>();
        let my_invalid_device = "this is my_invalid_device".into_device::<TestStrDevice>();

        insta::assert_snapshot!(my_valid_device, @"Device<valid>: this is my_device");
        insta::assert_snapshot!(my_invalid_device, @"Device<invalid>: this is my_invalid_device");
    }
}
