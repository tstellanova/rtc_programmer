use rtc_programmer::{
    release_rtc_driver, set_minutes_delay_alarm, set_rtc_date_time_to_system_time,
};
use linux_embedded_hal as p_hal;
// use embedded_hal::blocking::delay::DelayMs;

use p_hal::sysfs_gpio::{Pin, Direction, Edge};


/// Setup the RTC with an alarm value,
/// and configured to send an interrupt on INT/SQW output pin,
/// then watch an input GPIO pin to see whether we receive the interrupt.
///
fn main() {
    let mut rtc = rtc_programmer::get_rtc_driver();
    //update the current time
    set_rtc_date_time_to_system_time(&mut rtc);
    set_minutes_delay_alarm(&mut rtc, 1, true);

    //Use GPIO2_A3 to trigger: GPIO pin 67?
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
                   break; 
                }
            }
        }
        Ok(())
    });

    if follow.is_err() { println!("following error: {:?}", follow); }

    // let mut delay_source = p_hal::Delay {};
    // //wait around for 1 minute
    // for i in 0..90 {
    //     let trig = rtc.has_alarm1_matched().expect("couldn't check Alarm1");
    //     if trig {
    //         println!("\nAlarm 1 triggered after {} sec", i);
    //         break;
    //     }
    //     delay_source.delay_ms(1000u32);
    // }

    let trig = rtc.has_alarm1_matched().expect("couldn't check Alarm1");
    if !trig {
        println!("\nAlarm 1 never triggered?");
    }

    release_rtc_driver(rtc);
}


