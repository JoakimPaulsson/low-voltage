use std::time::Duration;

use anyhow::bail;

use super::*;

pub(super) fn handle_time(
    key: &str,
    value: i64,
    batt_info: &mut BatteryInfo,
) -> anyhow::Result<()> {
    if let Some(t) = &batt_info.time_until {
        let duration: Duration = **t;

        let current_dur_is_zero = duration == Duration::from_secs(0);
        let key_check = key == t.as_str();

        // should never happen we are never mutating a BatteryInfo state that has been
        // initialized with real values, and keys should always be different from what's
        // already there
        if key_check {
            bail!(
                "encountered a time key that's already been encountered, this should never happen"
            );
        }

        // this state is weird - both 'TimeToFull' and 'TimeToEmpty' is zero
        if current_dur_is_zero && value == 0 {
            bail!("both 'TimeToFull' and 'TimeToEmpty' is zero, this should never happen");
        }

        // this state is weird - both 'TimeToFull' and 'TimeToEmpty' is n0n-zero
        if !current_dur_is_zero && value != 0 {
            bail!("both 'TimeToFull' and 'TimeToEmpty' is non-zero, this should never happen")
        }

        // dont update value, already correct
        if !current_dur_is_zero && value == 0 {
            return Ok(());
        }

        batt_info.set_propertry(BatteryInfoProperties::TimeUntil(
            (key, Duration::from_secs(value.try_into()?)).into(),
        ));

        return Ok(());
    }

    batt_info.set_propertry(BatteryInfoProperties::TimeUntil(
        (key, Duration::from_secs(value.try_into()?)).into(),
    ));

    Ok(())
}

pub(super) fn retry_result_with_delay<'a, T, const DURATION_MS: u64>(
    closure: impl Fn() -> anyhow::Result<&'a T, &'a anyhow::Error>,
) -> anyhow::Result<&'a T> {
    let duration = Duration::from_millis(DURATION_MS);

    let mut output = closure();

    seq!(K in 0..10 {
        output = match output {
            Ok(_) => output,
            Err(_) => {
                sleep(duration);
                closure()
            },
        };
    });

    match output {
        Ok(o) => Ok(o),
        Err(e) => bail!("Error at retry result with delay: {}", e),
    }
}

#[cfg(test)]
mod tests {

    use anyhow::Ok;

    use super::*;

    #[test]
    fn test_handle_time() -> anyhow::Result<()> {
        let mut batt_info = BatteryInfo::default();

        handle_time("TimeToEmpty", 1000, &mut batt_info)?;

        insta::assert_debug_snapshot!(batt_info.time_until, @r###"
        Some(
            Empty(
                1000s,
            ),
        )
        "###);

        handle_time("TimeToFull", 0, &mut batt_info)?;

        insta::assert_debug_snapshot!(batt_info.time_until, @r###"
        Some(
            Empty(
                1000s,
            ),
        )
        "###);

        let mut batt_info = BatteryInfo::default();

        handle_time("TimeToFull", 1000, &mut batt_info)?;

        insta::assert_debug_snapshot!(batt_info.time_until, @r###"
        Some(
            Full(
                1000s,
            ),
        )
        "###);

        handle_time("TimeToEmpty", 0, &mut batt_info)?;

        insta::assert_debug_snapshot!(batt_info.time_until, @r###"
        Some(
            Full(
                1000s,
            ),
        )
        "###);

        // weird case - have both time to empty and time to full
        let this_should_be_error = handle_time("TimeToEmpty", 999, &mut batt_info);

        insta::assert_debug_snapshot!(this_should_be_error, @r###"
        Err(
            "both 'TimeToFull' and 'TimeToEmpty' is non-zero, this should never happen",
        )
        "###);

        let mut batt_info = BatteryInfo::default();

        handle_time("TimeToEmpty", 0, &mut batt_info)?;
        let this_should_be_error = handle_time("TimeToFull", 0, &mut batt_info);

        insta::assert_debug_snapshot!(this_should_be_error, @r###"
        Err(
            "both 'TimeToFull' and 'TimeToEmpty' is zero, this should never happen",
        )
        "###);

        let mut batt_info = BatteryInfo::default();

        handle_time("TimeToEmpty", 1000, &mut batt_info)?;
        let this_should_be_error = handle_time("TimeToEmpty", 1893, &mut batt_info);

        insta::assert_debug_snapshot!(this_should_be_error, @r###"
        Err(
            "encountered a time key that's already been encountered, this should never happen",
        )
        "###);

        let mut batt_info = BatteryInfo::default();

        handle_time("TimeToFull", 1000, &mut batt_info)?;
        let this_should_be_error = handle_time("TimeToFull", 1893, &mut batt_info);

        insta::assert_debug_snapshot!(this_should_be_error, @r###"
        Err(
            "encountered a time key that's already been encountered, this should never happen",
        )
        "###);

        Ok(())
    }
}
