use std::collections::HashMap;

enum Model { JETSON_XAVIER, JETSON_TX2, JETSON_TX1, JETSON_NANO };

pub struct gpio_pin_def{
    linux_pin :i8u,           // Linux GPIO pin number
    sysfs_dir : String,       // GPIO chip sysfs directory
    board_pin: String,        // Pin number (BOARD mode)
    bcm_pin: String,          // Pin number (BCM mode)
    cvm_pin: String,          // Pin name (CVM mode)
    tegra_pin: String,        // Pin name (TEGRA_SOC mode)
    pwm_sysfs_dir: String,     // PWM chip sysfs directory
    pwm_id: i8u,              // PWM ID within PWM chip
};  

struct gpio_pin_info{
    p1_revision : i32u,
    ram : String,
    revision : String,
    type : String,
    manufacturer : String,
    processor : String,
};

struct channel_info{

    channel : String,
    gpio_chip_dir : String,
    chip_gpio : i8u,
    gpio : i8u,
    pwm_chip_dir : String,
    pwm_id : i8u,
};

struct gpio_data{
    Model model;
    gpio_pin_info pin_info;
    std::map<GPIO::NumberingModes, std::map<std::string, ChannelInfo>> channel_data;
};

GPIO_data get_data();

