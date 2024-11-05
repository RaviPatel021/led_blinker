use sysfs_gpio::{Direction, Pin};
use std::thread::sleep;
use std::time::Duration;
use rand::Rng;

fn generate_random_sequence(length: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    (0..length).map(|_| rng.gen_range(0..3)).collect()
}

fn main() {
    let red_led = Pin::new(50);
    let blue_led = Pin::new(60);
    let yellow_led = Pin::new(51);

    let red_switch = Pin::new(31);
    let blue_switch = Pin::new(30);
    let yellow_switch = Pin::new(48);

    red_led.with_exported(|| {
        red_led.set_direction(Direction::Out)?;
        red_switch.with_exported(|| {
            red_switch.set_direction(Direction::In)?;
            blue_led.with_exported(|| {
                blue_led.set_direction(Direction::Out)?;
                blue_switch.with_exported(|| {
                    blue_switch.set_direction(Direction::In)?;
                    yellow_led.with_exported(|| {
                        yellow_led.set_direction(Direction::Out)?;
                        yellow_switch.with_exported(|| {
                            yellow_switch.set_direction(Direction::In)?;

                            loop {
                                let sequence = generate_random_sequence(9);

                                for val in &sequence {
                                    match val {
                                        0 => {
                                            red_led.set_value(1)?;
                                            blue_led.set_value(0)?;
                                            yellow_led.set_value(0)?;
                                            
                                            while red_switch.get_value()? == 0 {}
                                            red_led.set_value(0)?;
                                        },
                                        1 => {
                                            red_led.set_value(0)?;
                                            blue_led.set_value(1)?;
                                            yellow_led.set_value(0)?;
                                            
                                            while blue_switch.get_value()? == 0 {}
                                            blue_led.set_value(0)?;
                                        },
                                        2 => {
                                            red_led.set_value(0)?;
                                            blue_led.set_value(0)?;
                                            yellow_led.set_value(1)?;
                                            
                                            while yellow_switch.get_value()? == 0 {}
                                            yellow_led.set_value(0)?;
                                        },
                                        _ => {},
                                    }
                                    sleep(Duration::from_secs(1));
                                }
                            }
                        })
                    })
                })
            })
        })
    }).unwrap();
}
