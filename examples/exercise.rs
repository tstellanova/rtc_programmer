use rtc_programmer::{
    release_rtc_driver, set_minutes_delay_alarm, set_rtc_date_time_to_system_time,
};
use linux_embedded_hal as p_hal;
use embedded_hal::blocking::delay::DelayMs;
use ds323x::{ic::DS3231, interface::I2cInterface, Alarm1Matching, DayAlarm1, Ds323x, Hours, Rtcc};

fn main() {
    let mut rtc = rtc_programmer::get_rtc_driver();
    //update the current time
    set_rtc_date_time_to_system_time(&mut rtc);
    set_minutes_delay_alarm(&mut rtc, 1, true);

    //wait around for 1 minute
    let mut delay_source = p_hal::Delay {};
    delay_source.delay_ms(60_000);

    let triggered = rtc.has_alarm1_matched().expect("couldn't check Alarm1");
    println!("Alarm 1 triggered? {}", triggered);

    release_rtc_driver(rtc);

}
