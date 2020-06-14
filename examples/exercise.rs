use rtc_programmer::{
    release_rtc_driver, set_minutes_delay_alarm, set_rtc_date_time_to_system_time,
};
use linux_embedded_hal as p_hal;
use embedded_hal::blocking::delay::DelayMs;

fn main() {
    let mut rtc = rtc_programmer::get_rtc_driver();
    //update the current time
    set_rtc_date_time_to_system_time(&mut rtc);
    set_minutes_delay_alarm(&mut rtc, 1, true);

    let mut delay_source = p_hal::Delay {};
    //wait around for 1 minute
    for i in 0..90 {
        delay_source.delay_ms(1000u32);
        let trig = rtc.has_alarm1_matched().expect("couldn't check Alarm1");
        if trig {
            println!("Alarm 1 triggered after {} sec", i);
        }
    }
    let trig = rtc.has_alarm1_matched().expect("couldn't check Alarm1");
    if !trig {
        println!("Alarm 1 never triggered?");
    }

    release_rtc_driver(rtc);

}
