use rtc_programmer::{
    release_rtc_driver, set_minutes_delay_alarm, set_rtc_date_time_to_system_time,
};
use linux_embedded_hal as p_hal;
// use embedded_hal::blocking::delay::DelayMs;

use p_hal::sysfs_gpio::{Pin, Direction, Edge};
use linux_embedded_hal::sysfs_gpio::Error;


/// Setup the RTC with an alarm value,
/// and configured to send an interrupt on INT/SQW output pin,
/// then watch an input GPIO pin to see whether we receive the interrupt.
///
fn main() {
    let mut rtc = rtc_programmer::get_rtc_driver();
    rtc.disable_alarm1_interrupts().unwrap();
    rtc.disable_alarm2_interrupts().unwrap();

    //update the current time
    set_rtc_date_time_to_system_time(&mut rtc);
    set_minutes_delay_alarm(&mut rtc, 1, true);

    //Use GPIO2_A3 to trigger: GPIO pin 67
    let input = p_hal::Pin::new(67);
    let follow  = input.with_exported(|| {
        input.set_direction(Direction::In)?;
        //input.set_active_low(true);
        input.set_edge(Edge::FallingEdge)?;
        let mut poller = input.get_poller()?;
        for i in 0..90 {
            if let Some(val) = poller.poll(1000)? {
                //looking for low (indicates triger)
                if 0 == val {
                    println!("\nINT triggered after {} sec", i);
                    return Ok(());
                }
            }
        }
        Err( Error::Unexpected(format!("No external interrupt!")))
    });

    if follow.is_ok() {
        let trig = rtc.has_alarm1_matched().expect("Couldn't check Alarm1");
        if !trig {
            println!("\nAlarm1 match never triggered?");
        }
    }
    else {
        println!("following error: {:?}", follow);
    }

    release_rtc_driver(rtc);
}


