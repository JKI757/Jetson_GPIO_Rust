    let VERSION : String = "0.1.1";
    let JETSON_INFO : String
    let model : String;


    // Pin Numbering Modes
    enum NumberingModes{ BOARD, BCM, TEGRA_SOC, CVM, None };
    
    // GPIO::BOARD, GPIO::BCM, GPIO::TEGRA_SOC, GPIO::CVM
    let BOARD: NumberingModes = NumberingModes::BOARD;
    let BCM : NumberingModes = NumberingModes::BCM;
    let TEGRA_SOC : NumberingModes = NumberingModes::TEGRA_SOC;
    let CVM NumberingModes = NumberingModes::CVM;
   // Pull up/down options are removed because they are unused in NVIDIA's original python libarary.
   // check: https://github.com/NVIDIA/jetson-gpio/issues/5

    let HIGH = 1;
    let LOW = 0;

    // GPIO directions. 
    // UNKNOWN constant is for gpios that are not yet setup
    // If the user uses UNKNOWN or HARD_PWM as a parameter to GPIO::setmode function,
    // An exception will occur
    enum Directions{ UNKNOWN, OUT, IN, HARD_PWM };

    // GPIO::IN, GPIO::OUT
    let IN : Directions = Directions::IN;
    let OUT : Directions = Directions::OUT;
    
    // Function used to enable/disable warnings during setup and cleanup.
    void setwarnings(bool state);

    // Function used to set the pin mumbering mode. 
    // Possible mode values are BOARD, BCM, TEGRA_SOC and CVM
    void setmode(NumberingModes mode);

    // Function used to get the currently set pin numbering mode
    NumberingModes getmode();

    /* Function used to setup individual pins as Input or Output. 
       direction must be IN or OUT, initial must be 
       HIGH or LOW and is only valid when direction is OUT  */
    void setup(const std::string& channel, Directions direction, int initial = -1);
    void setup(int channel, Directions direction, int initial = -1);


    /* Function used to cleanup channels at the end of the program.
       If no channel is provided, all channels are cleaned */
    void cleanup(const std::string& channel = "None");
    void cleanup(int channel);
    

    /* Function used to return the current value of the specified channel.
       Function returns either HIGH or LOW */
    int input(const std::string& channel);
    int input(int channel);
    

    /* Function used to set a value to a channel.
       Values must be either HIGH or LOW */
    void output(const std::string& channel, int value);
    void output(int channel, int value);

    /* Function used to check the currently set function of the channel specified. */
    Directions gpio_function(const std::string& channel);
    Directions gpio_function(int channel);

   //----------------------------------
   

}
