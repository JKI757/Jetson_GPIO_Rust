

pub init(channel : u8, frequency : u8){
}
pub start(duty_cycle_percent : u8){ //0-100 integer percentage value passed
}
pub stop(){
}
pub change_frequency(int frequency_hz){
}
pub change_duty_cycle(duty_cycle_percent : i8u){
}

      //struct Impl;
      //const std::unique_ptr<Impl> pImpl;
struct IMPL {
    ch_info             : ChannelInfo,
    started             : bool;
    frequency_hz        : u32;
    period_ns           : u32;
    duty_cycle_percent  : u8;
    duty_cycle_ns       : u8;
};

void GPIO::PWM::Impl::_reconfigure(int frequency_hz, double duty_cycle_percent, bool start){

    if (duty_cycle_percent < 0.0 || duty_cycle_percent > 100.0)
            throw runtime_error("invalid duty_cycle_percent");
    bool restart = start || _started;

    if (_started){
        _started = false;
        _disable_pwm(_ch_info);
    }

    _frequency_hz = frequency_hz;
    _period_ns = int(1000000000.0 / frequency_hz);
    _set_pwm_period(_ch_info, _period_ns);

    _duty_cycle_percent = duty_cycle_percent;
    _duty_cycle_ns = int(_period_ns * (duty_cycle_percent / 100.0));
    _set_pwm_duty_cycle(_ch_info, _duty_cycle_ns);

    if (restart){
        _enable_pwm(_ch_info);
        _started = true;
    }

}


GPIO::PWM::PWM(int channel, int frequency_hz)
    : pImpl(make_unique<Impl>(Impl{_channel_to_info(to_string(channel), false, true), 
            false, 0, 0, 0.0, 0}))  //temporary values
{
    try{

        Directions app_cfg = _app_channel_configuration(pImpl->_ch_info);
        if(app_cfg == HARD_PWM)
            throw runtime_error("Can't create duplicate PWM objects");
        /*
        Apps typically set up channels as GPIO before making them be PWM,
        because RPi.GPIO does soft-PWM. We must undo the GPIO export to
        allow HW PWM to run on the pin.
        */
        if(app_cfg == IN || app_cfg == OUT) cleanup(channel);

        if (GlobalVariables._gpio_warnings){
            auto sysfs_cfg = _sysfs_channel_configuration(pImpl->_ch_info);
            app_cfg = _app_channel_configuration(pImpl->_ch_info);

            // warn if channel has been setup external to current program
            if (app_cfg == UNKNOWN && sysfs_cfg != UNKNOWN){
                cerr <<  "[WARNING] This channel is already in use, continuing anyway. "
                            "Use GPIO::setwarnings(false) to disable warnings" << endl;
            }
        }
        
        _export_pwm(pImpl->_ch_info);
        pImpl->_reconfigure(frequency_hz, 50.0);
        GlobalVariables._channel_configuration[to_string(channel)] = HARD_PWM;

    }
    catch (exception& e){
        cerr << "[Exception] " << e.what() << " (catched from: PWM::PWM())" << endl;
        _cleanup_all();
        terminate();
    }

}

GPIO::PWM::~PWM(){
    auto itr = GlobalVariables._channel_configuration.find(pImpl->_ch_info.channel);
    if (itr == GlobalVariables._channel_configuration.end() || itr->second != HARD_PWM){
        /* The user probably ran cleanup() on the channel already, so avoid
           attempts to repeat the cleanup operations. */
       return;
    }
    try{
        stop();
        _unexport_pwm(pImpl->_ch_info);
        GlobalVariables._channel_configuration.erase(pImpl->_ch_info.channel);
        }
    catch(...){
        cerr << "[Exception] ~PWM Exception! shut down the program." << endl;
        _cleanup_all();
        terminate();
    }
}

void GPIO::PWM::start(double  duty_cycle_percent){
    try{
        pImpl->_reconfigure(pImpl->_frequency_hz, duty_cycle_percent, true);
    }
    catch(exception& e){
        cerr << "[Exception] " << e.what() << " (catched from: PWM::start())" << endl;
        _cleanup_all();
        terminate();
    }
}

void GPIO::PWM::start(int frequency_hz, double  duty_cycle_percent){
    try{
        pImpl->_reconfigure(frequency_hz, duty_cycle_percent, true);
    }
    catch(exception& e){
        cerr << "[Exception] " << e.what() << " (catched from: PWM::start())" << endl;
        _cleanup_all();
        terminate();
    }
}

void GPIO::PWM::ChangeFrequency(int frequency_hz){
    try{
        pImpl->_reconfigure(frequency_hz, pImpl->_duty_cycle_percent);
    }
    catch(exception& e){
        cerr << "[Exception] " << e.what() << " (catched from: PWM::ChangeFrequency())" << endl;
        terminate();
    }
}

void GPIO::PWM::ChangeDutyCycle(double duty_cycle_percent){
    try{
        pImpl->_reconfigure(pImpl->_frequency_hz, duty_cycle_percent);
    }
    catch(exception& e){
        cerr << "[Exception] " << e.what() << " (catched from: PWM::ChangeDutyCycle())" << endl;
        terminate();
    }
}

void GPIO::PWM::stop(){
    try{
        if (!pImpl->_started)
            return;
        
        _disable_pwm(pImpl->_ch_info);
    }
    catch(exception& e){
        cerr << "[Exception] " << e.what() << " (catched from: PWM::stop())" << endl;
        throw runtime_error("Exeception from GPIO::PWM::stop");
    }
      
      