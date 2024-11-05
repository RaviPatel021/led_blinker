# LED Blinking Game

This project demonstrates how to create an interactive LED blinking game using Rust on BeagleBone. The game involves LEDs blinking in a sequence, and the user must press the corresponding button to turn off the LED and advance to the next one.

## Hardware Requirements
1. BeagleBone or a similar development board
2. 3 LEDs (red, blue, yellow)
3. 3 Resistors (220Ω each)
4. 3 Momentary push-button switches
5. 3 Resistors (10kΩ each)
6. Breadboard and jumper wires


## Installation and Use

1. Setup Instructions
    ```bash
    # Install Rust
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source $HOME/.cargo/env

    # Install ARM GCC Toolchain
    sudo apt update
    sudo apt install gcc-arm-linux-gnueabihf
    
    # Add ARMv7 target for cross-compilation
    rustup target add armv7-unknown-linux-gnueabihf
    ```

2. Clone the Repository
    ```bash
    git clone https://github.com/RaviPatel021/led_blinker.git
    cd led_blinker
    ```

3. Build the Project for ARM
    Use Cargo to build the project for the armv7-unknown-linux-gnueabihf target:
    ```bash
    cargo build --target armv7-unknown-linux-gnueabihf --release
    ```

4. Deploy to BeagleBone
    Transfer the compiled binary to your BeagleBone and run it:
    ```bash
    # Copy the binary to BeagleBone (update IP address as necessary)
    scp target/armv7-unknown-linux-gnueabihf/release/led_blinker debian@192.168.6.2:~
    
    # SSH into the BeagleBone
    ssh debian@192.168.6.2
    
    # Run the project on the BeagleBone
    ./led_blinker
    ```
