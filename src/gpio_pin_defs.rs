use std::collections::HashMap;

enum Model { JETSON_XAVIER, JETSON_TX2, JETSON_TX1, JETSON_NANO };
enum Directions{ UNKNOWN, OUT, IN, HARD_PWM };
#[derive(Hash, Eq, PartialEq, Debug)]
enum NumberingModes{ BOARD, BCM, TEGRA_SOC, CVM, None };

let JETSON_XAVIER   : Model = JETSON_XAVIER;
let JETSON_TX2      : Model = JETSON_TX2;
let JETSON_TX1      : Model = JETSON_TX1;
let JETSON_NANO     : Model = JETSON_NANO;
let UNKNOWN         : Directions = Directions::UNKNOWN;
let HARD_PWM        : Directions = Directions::HARD_PWM
let IN              : Directions = Directions::IN;
let OUT             : Directions = Directions::OUT;
let SYSFS_ROOT      : String = "/sys/class/gpio";

pub struct gpio_pin_def{
    linux_pin       : u8,           // Linux GPIO pin number
    sysfs_dir       : String,       // GPIO chip sysfs directory
    board_pin       : String,        // Pin number (BOARD mode)
    bcm_pin         : String,          // Pin number (BCM mode)
    cvm_pin         : String,          // Pin name (CVM mode)
    tegra_pin       : String,        // Pin name (TEGRA_SOC mode)
    pwm_sysfs_dir   : String,     // PWM chip sysfs directory
    pwm_id          : u8,              // PWM ID within PWM chip
};  

struct gpio_pin_info{
    p1_revision     : u32,
    ram             : String,
    revision        : String,
    type            : String,
    manufacturer    : String,
    processor       : String,
};

struct channel_info{

    channel         : String,
    gpio_chip_dir   : String,
    chip_gpio       : u8,
    gpio            : u8,
    pwm_chip_dir    : String,
    pwm_id          : u8,
};

struct gpio_data{
    model : Model,
    pin_info : gpio_pin_info,
    channel_data: HashMap<NumberingModes, HashMap<String, channel_info> >
};

GPIO_data get_data();

