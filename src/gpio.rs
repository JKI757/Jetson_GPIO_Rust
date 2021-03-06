    constexpr auto VERSION = "0.1.1";
    extern const std::string JETSON_INFO;
    extern const std::string model;


    // Pin Numbering Modes
    enum class NumberingModes{ BOARD, BCM, TEGRA_SOC, CVM, None };
    
    // GPIO::BOARD, GPIO::BCM, GPIO::TEGRA_SOC, GPIO::CVM
    constexpr NumberingModes BOARD = NumberingModes::BOARD;
    constexpr NumberingModes BCM = NumberingModes::BCM;
    constexpr NumberingModes TEGRA_SOC = NumberingModes::TEGRA_SOC;
    constexpr NumberingModes CVM = NumberingModes::CVM;

   // Pull up/down options are removed because they are unused in NVIDIA's original python libarary.
   // check: https://github.com/NVIDIA/jetson-gpio/issues/5

    constexpr int HIGH = 1;
    constexpr int LOW = 0;

    // GPIO directions. 
    // UNKNOWN constant is for gpios that are not yet setup
    // If the user uses UNKNOWN or HARD_PWM as a parameter to GPIO::setmode function,
    // An exception will occur
    enum class Directions{ UNKNOWN, OUT, IN, HARD_PWM };

    // GPIO::IN, GPIO::OUT
    constexpr Directions IN = Directions::IN;
    constexpr Directions OUT = Directions::OUT;
    
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
   

    class PWM{
    public:
      PWM(int channel, int frequency_hz);
      PWM(const PWM&) = delete;              // Can't create duplicate PWM objects
      PWM& operator=(const PWM&) = delete;   // Can't create duplicate PWM objects
      ~PWM();
      void start(double duty_cycle_percent);
      void stop();
      void ChangeFrequency(int frequency_hz);
      void ChangeDutyCycle(double duty_cycle_percent);

   private:
      struct Impl;
      const std::unique_ptr<Impl> pImpl;
    };
}
