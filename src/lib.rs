use chrono::{DateTime, Datelike, Timelike, Utc};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

use ds323x::{ic::DS3231, interface::I2cInterface, Alarm1Matching, DayAlarm1, Ds323x, Hours, Rtcc};
use linux_embedded_hal as p_hal;

pub type RtcDriver = Ds323x<I2cInterface<p_hal::I2cdev>, DS3231>;

pub fn get_rtc_driver() -> RtcDriver {
    let i2c_port = p_hal::I2cdev::new("/dev/i2c-1").expect("could not grab i2c-1");
    let rtc = Ds323x::new_ds3231(i2c_port);
    rtc
}

pub fn release_rtc_driver(rtc: RtcDriver) {
    let _ = rtc.destroy_ds3231();
}
/// Ensure the RTC is set to the same time as the system clock
pub fn set_rtc_date_time_to_system_time(rtc: &mut RtcDriver) {
    let actual_time = Utc::now();
    let time_str = actual_time.format("%Y%m%d_%H%M%SZ").to_string();
    println!("old: {}", time_str);

    let system_date =
        NaiveDate::from_ymd(actual_time.year(), actual_time.month(), actual_time.day());
    let system_time = NaiveTime::from_hms_milli(
        actual_time.hour(),
        actual_time.minute(),
        actual_time.second(),
        0,
    );

    let datetime = NaiveDateTime::new(system_date, system_time);
    rtc.set_datetime(&datetime).expect("couldn't set_datetime");

    let dt = rtc.get_datetime().expect("Couldn't get the date time");

    println!(
        "new: {:04}{:02}{:02}_{:02}{:02}{:02}Z",
        dt.year(),
        dt.month(),
        dt.day(),
        dt.hour(),
        dt.minute(),
        dt.second()
    );
}

/// Get the date and time according to the RTC
pub fn get_date_time(rtc: &mut RtcDriver) -> chrono::DateTime<chrono::Utc> {
    let ndt = rtc.get_datetime().expect("could not get time");
    let cdt = DateTime::<Utc>::from_utc(ndt, Utc);
    cdt
}

pub fn set_alarm_at_time_date(rtc: &mut RtcDriver, datetime: NaiveDateTime, interrupt: bool) {
    let alarm1 = DayAlarm1 {
        day: datetime.date().day() as u8,
        hour: Hours::H24(datetime.time().hour() as u8),
        minute: datetime.minute() as u8,
        second: 1,
    };

    set_alarm1(rtc, &alarm1, interrupt);
}

/// Tell the RTC to set an alarm by delay from the current time
pub fn set_minutes_delay_alarm(rtc: &mut RtcDriver, minutes_delay: u8, interrupt: bool) {
    let dt = rtc.get_datetime().expect("could not get time");

    // The INT/SQW output will be latched low if the alarm has already fired: clear it
    rtc.clear_alarm1_matched_flag()
        .expect("couldn't clear alarm1 flag");

    let now_hours: u32 = dt.hour();
    let future_minutes: u32 = dt.minute() + minutes_delay as u32;
    let minutes = future_minutes % 60;
    let hours = now_hours + (future_minutes / 60);
    let alarm1 = DayAlarm1 {
        day: 1, // unused since we're using HoursMinutesAndSecondsMatch below
        hour: Hours::H24(hours as u8),
        minute: minutes as u8,
        second: dt.second() as u8,
    };

    set_alarm1(rtc, &alarm1, interrupt);
}

pub fn set_alarm1(rtc: &mut RtcDriver, alarm1: &DayAlarm1, interrupt: bool) {
    rtc.clear_alarm1_matched_flag()
        .expect("couldn't clear alarm");
    //  Alarm should fire when hours, minutes, and seconds match
    rtc.set_alarm1_day(*alarm1, Alarm1Matching::HoursMinutesAndSecondsMatch)
        .expect("Couldn't set alarm");
    if interrupt {
        rtc.disable_alarm1_interrupts().unwrap();
        rtc.disable_32khz_output().unwrap();
        rtc.use_int_sqw_output_as_interrupt()
            .expect("Couldn't enable INTCN");
        rtc.enable_alarm1_interrupts().expect("Couldn't enable AIE");
    }
    println!("alarm1 set to: {:?}", alarm1);
}
