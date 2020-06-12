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
