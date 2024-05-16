searchState.loadedDescShard("esp_ward", 0, "esp-ward\nThe type of the value returned by <code>capture</code>\nEnumeration of channels that can be used with this <code>Capture</code> …\nEnumeration of channels that can be used with this <code>Pwm</code> …\nFrozen clock frequencies\nThe type of the value returned by <code>count</code>\nType for the <code>duty</code> methods\nType for the <code>duty</code> methods\nError type returned by ADC methods\nError type\nError type\nError type\nError type\nThe type of error that can occur when writing\nError type\nError type\nError type\nError type\nError type\nRead error\nWrite error\nAn enumeration of SPI errors\nEnumeration of <code>Capture</code> errors\nFull-duplex operation\nShorthand for creating a rate which represents hertz.\nShorthand for creating a rate which represents hertz.\nI2C peripheral container (I2C)\nI2C Peripheral Instance\nGeneral Purpose Input/Output driver\nShorthand for creating a rate which represents megahertz.\nShorthand for creating a rate which represents megahertz.\nPeripheral singleton type\nTrait for any type that can be used as a peripheral of …\nSPI peripheral driver\nSPI modes\nThe unit of time used by this timer\nUnit of time used by the watchdog\nA time unit that can be converted into a human time unit …\nA time unit that can be converted into a human time unit …\nTrait for buffers that can be given to DMA for reading.\nTrait for <code>Deref</code> targets used by the blanket <code>DmaReadBuffer</code> …\nTrait for DMA word types used by the blanket DMA buffer …\nTrait for buffers that can be given to DMA for writing.\nTrait for <code>DerefMut</code> targets used by the blanket …\nInput capture\nPulse Width Modulation\nA single PWM channel / pin\nQuadrature encoder interface\nADCs that sample on single channels per request, and do so …\nMillisecond delay\nMicrosecond delay\nBlocking read\nBlocking write\nBlocking write + read\nBlocking read\nWrite half of a serial interface (blocking variant)\nBlocking transfer\nBlocking write\nSingle digital input pin\nSingle digital push-pull output pin\nOutput pin that can be toggled\nSingle digital input pin\nSingle digital push-pull output pin\nPush-pull output pin that can read its output state\nOutput pin that can be toggled\nRead half of a serial interface\nWrite half of a serial interface\nFull duplex (master mode)\nA count down timer\nFeeds an existing watchdog to ensure the processor isn’t …\nDisables a running watchdog timer so the processor won’t …\nEnables A watchdog timer to reset the processor if …\nTrait to be implemented for an in progress dma transfer.\nTrait to be implemented for an in progress dma transfer.\nI2C Peripheral Instance\nChannel HW interface\nChannel interface\nInterface for HW configuration of timer\nInterface for Timers\nExtension trait to split a SYSTEM/DPORT peripheral in …\nTimer peripheral instance\nUART peripheral instance\nPins used by the UART interface\nExtension trait for simple short-hands for u32 Durations\nExtension trait for simple short-hands for u64 Durations\nExtension trait for simple short-hands for u32 Rate\nExtension trait for simple short-hands for u64 Rate\nBlock until the serial interface has sent all buffered …\nWrites a slice, blocking until everything has been written\n“Waits” for a transition in the capture <code>channel</code> and …\nClear the interrupt status bit for this Pin\nUnsafely clone (duplicate) a peripheral singleton.\nConfigure channel\nConfigure the timer\nConfigure Channel HW except for the duty which is set via …\nConfigure the HW for the timer\nConnectivity Features\nReturns the current pulse count of the encoder\nMacro for creating a <code>Joystick</code> instance.\nMacro to create a network stack for WiFi communication.\nPauses execution for <code>ms</code> milliseconds\nPauses execution for <code>us</code> microseconds\nReturns the count direction\nDisables the watchdog\nDisables a capture <code>channel</code>\nDisables a PWM <code>channel</code>\nDisables a PWM <code>channel</code>\nRemove a connected <code>signal</code> from this input pin.\nRemove a connected <code>signal</code> from this input pin.\nRemove this output pin from a connected signal.\nRemove this output pin from a connected signal.\nDisplay Module\nEnables a capture <code>channel</code>\nEnables a PWM <code>channel</code>\nEnables a PWM <code>channel</code>\nAttribute to declare the entry point of the program\nTriggers the watchdog. This must be done once the watchdog …\nEnsures that none of the previously written words are …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the current duty cycle\nReturns the current duty cycle\nReturn the duty resolution of the timer\nReturn the frequency of the timer\nGet the current source timer frequency from the HW\nReturn the timer frequency, or 0 if not configured\nMacro to retrieve the IP configuration from the network …\nReturns the maximum duty cycle value\nReturns the maximum duty cycle value\nReturn the timer number\nReturns the current PWM period\nReturns the current resolution\nMacro to obtain a suitable timer based on the ESP device …\nShorthand for creating a duration which represents hours.\nShorthand for creating a duration which represents hours.\nInitialize for full-duplex 1 bit mode\nInitializes the system clocks and IO pins, providing the …\nInitializes a custom I2C configuration, allowing for …\nInitializes the default I2C configuration for the ESP …\nInitializes a custom SPI configuration, allowing for …\nInitializes the default SPI configuration for the chip. …\nMacro to initialize the WiFi interface with the given SSID …\nMarks a function as an interrupt handler\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nConvert a value into a <code>PeripheralRef</code>.\nCheck if the timer has been configured\nCheck if the transfer is finished.\nCheck if the transfer is finished.\nCheck whether a duty-cycle fade is running\nCheck whether a duty-cycle fade is running HW\nIs the input pin high?\nIs the input pin high?\nChecks if the interrupt status bit for this Pin is set\nChecks if listening for interrupts is enabled for this Pin\nIs the input pin low?\nIs the input pin low?\nIs the pin in drive high mode?\nIs the pin in drive low mode?\nShorthand for creating a rate which represents kilohertz.\nShorthand for creating a rate which represents kilohertz.\nListen for interrupts\nListen for interrupts\nLoad code to be run on the LP/ULP core.\nCreates a new <code>executor</code> instance and declares an …\nCreate an enum for erased GPIO pins, using the …\nRead bytes from a target slave with the address <code>addr</code> The …\nRead bytes from a target slave with the address <code>addr</code> The …\nSend data bytes from the <code>bytes</code> array to a target slave …\nSend data bytes from the <code>bytes</code> array to a target slave …\nWrite bytes from the <code>bytes</code> array first and then read n …\nWrite bytes from the <code>bytes</code> array first and then read n …\nShorthand for creating a duration which represents …\nShorthand for creating a duration which represents …\nShorthand for creating a duration which represents …\nShorthand for creating a duration which represents …\nShorthand for creating a duration which represents minutes.\nShorthand for creating a duration which represents minutes.\nShorthand for creating a duration which represents …\nShorthand for creating a duration which represents …\nCreate a new I2C instance This will enable the peripheral …\nConstructs an SPI instance in 8bit dataframe mode.\nConstructs an SPI instance in half-duplex mode.\nCreate a new I2C instance with a custom timeout value. …\nPeripherals Module\nSets up a global allocator for heap memory, required for …\nMacro to prepare buffers with fixed sizes for MQTT …\nThis attribute allows placing statics and functions into …\nRequest that the ADC begin a conversion on the specified …\nReads enough bytes from slave with <code>address</code> to fill <code>buffer</code>\nReads enough bytes from hardware random number generator …\nReads a single word from the serial interface\nReads the word stored in the shift register\nProvide a buffer usable for DMA reads.\nRead bytes from SPI.\nRead received bytes from SPI FIFO.\nResets the I2C controller (FIFO + FSM + command list)\nResets the I2C controller (FIFO + FSM + command list)\nResets the I2C peripheral’s command registers\nResets the I2C peripheral’s command registers\nResets the transmit and receive FIFO buffers\nResets the transmit and receive FIFO buffers\nShorthand for creating a duration which represents seconds.\nShorthand for creating a duration which represents seconds.\nSends a word to the slave\nSets a new duty cycle\nSets a new duty cycle\nSet channel duty HW\nSet channel duty HW\nSets the filter with a supplied threshold in clock cycles …\nSets the filter with a supplied threshold in clock cycles …\nSets the frequency of the I2C interface by calculating and …\nSets the frequency of the I2C interface by calculating and …\nDrives the pin high\nDrives the pin high\nDrives the pin low\nDrives the pin low\nSets a new PWM period\nSets the resolution of the capture timer\nDrives the pin high or low depending on the provided value\nSplits the SYSTEM/DPORT peripheral into it’s parts.\nStarts a new count down\nStarts the watchdog with a given period, typically once …\nStart a duty-cycle fade\nStart a duty-cycle fade HW\nUnsafely create an instance of this peripheral out of thin …\nReturns all the peripherals <em>once</em>\nTakes the ESP peripherals. This should be one of the first …\nSplits the <code>SYSTEM</code> peripheral into its constituent parts. …\nToggle pin output.\nToggle pin output.\nSends <code>words</code> to the slave. Returns the <code>words</code> received from …\nStop listening for interrupts\nUpdate the timer in HW\nNon-blockingly “waits” until the count down finishes\nWait for the transfer to finish.\nWait for the transfer to finish.\nPauses the execution for a specified number of …\nMacro to wait until WiFi is connected in async variation …\nSetup pins for this SPI instance.\nSetup pins for this SPI instance.\nWrites bytes to slave with address <code>address</code>\nSends <code>words</code> to the slave, ignoring all the incoming words\nWrites a single word to the serial interface\nProvide a buffer usable for DMA writes.\nWrite bytes to SPI.\nWrites bytes to slave with address <code>address</code> and then reads …\nthis module was written based on a basis of an analysis of …\nThe functions for dealing with time and timestamps were …\nRepresents the IP address for the HiveMQ MQTT broker.\nRepresents the port number for the HiveMQ MQTT broker.\nMacro to create a network stack for WiFi communication.\nMacro to retrieve the IP configuration from the network …\nEstablishes a custom MQTT connection with the specified …\nEstablishes a default MQTT connection with predefined …\nWaits for and receives a message from the subscribed MQTT …\nThis function attempts to send the message to a specific …\nSubscribes to an MQTT topic.\nMacro to prepare buffers with fixed sizes for MQTT …\nMacro to wait until WiFi is connected in async variation …\nRepresents the IP address for the WorldTime API server.\nCreates a new socket for communication over WiFi.\nExtracts a UNIX timestamp from a server response.\nReceives a message over the specified socket.\nRetrieves the current time from the WorldTimeAPI.\nRetrieves the current time as a UNIX timestamp from the …\nConverts a string IP address into a 4-byte array.\nSends a request over the specified socket.\nConverts a UNIX timestamp into hours, minutes, and seconds.\nGets a weekday from a UNIX timestamp\nProvides a basic set of operations for interacting with a …\nRepresents segments of a display which can be targeted for …\nExtension of the <code>Display</code> trait to integrate with the …\nReturns the argument unchanged.\nILI9341 Display Driver\nCalls <code>U::from(self)</code>.\nPCD8544 Display Driver\nResets the display.\nSets a single pixel on the display to a specified …\nWrites a section name to a specific segment of the display …\nWrites a string to the display at the current cursor …\nWrites a string to a specific segment of the display using …\nThe <code>Ili9341Display</code> struct represents an ILI9341 display …\nConstructs a new <code>Ili9341Display</code>.\nReturns the argument unchanged.\nThe inner display instance from the <code>mipidsi</code> crate …\nCalls <code>U::from(self)</code>.\nResets the display, filling it with a white color.\nSets a single pixel on the display\nWrites a section name to a specified display segment using …\nWrites a string to the center segment of the display using …\nWrites a string to a specified display segment using the …\nRepresents a PCD8544 display and provides methods to …\nCreates and initializes a new <code>Pcd8544Display</code>.\nReturns the argument unchanged.\nThe underlying PCD8544 driver instance.\nCalls <code>U::from(self)</code>.\nResets the display\nSets a pixel on the display at the specified coordinates.\nWrites a string to the display.\nA non-blocking error\nA different kind of error\nA non-blocking result\nThis operation requires blocking behavior to complete\nTurns the non-blocking expression <code>$e</code> into a blocking …\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nMaps an <code>Error&lt;E&gt;</code> to <code>Error&lt;T&gt;</code> by applying a function to a …\nTrait for peripherals that can measure CO2 (or CO2 …\nTrait for peripherals that can measure distance.\nTrait for peripherals that can sense humidity levels.\nTrait for peripherals that communicate over I2C. …\nTrait for peripherals that can measure luminance.\nRepresents basic errors that can occur in peripheral …\nTrait for peripherals that can sense atmospheric pressure.\nTrait for peripherals that can be explicitly shut down or …\nTrait for peripherals that communicate over SPI. …\nTrait for peripherals that can sense temperature.\nTrait for peripherals capable of returning data (which …\nTrait for peripherals that can measure Volatile Organic …\nTrait for peripherals capable of writing data.\nAHT20 Sensor Interface\nBME280 Environmental Sensor Driver\nButton Module\nReturns the argument unchanged.\nMeasures the CO2 (or CO2eq) concentration in the air.\nMeasures the distance from the sensor to the nearest …\nReads the humidity level as a percentage.\nMeasures the ambient light intensity in lux.\nReads the atmospheric pressure in hPa (hectopascals).\nMeasures the concentration of VOCs in the air.\nCalls <code>U::from(self)</code>.\nJoystick Module\nPassive Infrared (PIR) Sensor Module\nSGP30 Sensor Module\nTSL2591 Light Sensor Module\nUltrasonic Distance Sensor Module\nA sensor instance for the AHT20\nCreates a new instance of the AHT20 sensor using the …\nReturns the argument unchanged.\nReads the current relative humidity from the AHT20 sensor.\nReads the current temperature from the AHT20 sensor.\nThe internal AHT20 driver from the <code>embedded_aht20</code> crate.\nCalls <code>U::from(self)</code>.\nReads the current relative humidity and temperature from …\nA sensor instance for the BME280 that provides access to …\nCreates a new instance of the BME280 sensor using the …\nA delay provider for timing-dependent operations.\nReturns the argument unchanged.\nReads the current relative humidity from the BME280 sensor.\nReads the current atmospheric pressure from the BME280 …\nReads the current temperature from the BME280 sensor.\nThe internal BME280 driver from the <code>bme280</code> crate used over …\nCalls <code>U::from(self)</code>.\nReads the current relative humidity, temperature and …\nA generic button that can report press and release events.\nRepresents possible events from a button press.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nReads the current state of a Button\nRepresents a joystick with two axes and a select button.\nA threshold value to interpret the joystick’s value in …\nMacro for creating a <code>Joystick</code> instance.\nReturns the argument unchanged.\nRetrieves the current positions of both axes.\nRetrieves the current position of the X-axis.\nRetrieves the current position of the Y-axis.\nCalls <code>U::from(self)</code>.\nThe select button of the joystick, wrapped in a <code>Button</code> …\nChecks if the select button is currently pressed.\nThe analog input pin for the X-axis.\nThe analog input pin for the Y-axis.\nRepresents a PIR motion sensor connected to a single …\nConstructs a new <code>PirSensor</code> with the given input pin.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nReads the current state of a PIR sensor data pin\nRepresents an SGP30 air quality sensor.\nCreates and initializes an SGP30 sensor over the I2C bus.\nDelay provider for timing-sensitive operations.\nReturns the argument unchanged.\nMeasures the CO2 concentration in the air.\nMeasures the VOC in the air.\nThe internal SGP30 sensor instance.\nCalls <code>U::from(self)</code>.\nReads the CO2 concentration in the air and VOC from the …\nRepresents a TSL2591 ambient light sensor.\nInitializes the TSL2591 sensor over the I2C bus.\nDelay provider for timing-sensitive operations.\nReturns the argument unchanged.\nMeasures the ambient light intensity.\nThe internal TSL2591 driver instance.\nCalls <code>U::from(self)</code>.\nMeasures the ambient light intensity.\nRepresents an ultrasonic distance sensor with trigger and …\nInitializes a new ultrasonic distance sensor.\nReturns the argument unchanged.\nMeasures the distance to an object by sending an …\nCalls <code>U::from(self)</code>.")