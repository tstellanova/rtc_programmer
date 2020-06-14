use rtc_programmer::{
    release_rtc_driver, set_minutes_delay_alarm, set_rtc_date_time_to_system_time,
};

fn main() {
    let mut rtc = rtc_programmer::get_rtc_driver();
    //update the current time
    set_rtc_date_time_to_system_time(&mut rtc);
    set_minutes_delay_alarm(&mut rtc, 3, true);
    release_rtc_driver(rtc);
}
