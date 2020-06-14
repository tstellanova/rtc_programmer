

## What

Enables progamming the date / time and alarm functionality on the DS3231 real time clock.

Tested with rock64 running Armbian Buster.

Wiring: attach the i2c pins of the DS3231 to pins 27 (SDA) and 28 (SCL) on the roc64
and then `i2cdetect` should find the device as above.

### Troubleshooting i2c

- By default Armbian Buster on rock64 enables i2c1 and i2c4.  
- Check eg `ls /dev/*i2c*` and verify `/dev/i2c-1` is visible
- Install i2c tools:  `sudo apt-get install -y i2c-tools`
- Attach a [suitable DS3231 RTC module](http://a.co/d/0KolyPX) and check that `sudo i2cdetect -y 1` 
shows a device at address `68`

### License
BSD-3: See LICENSE file
