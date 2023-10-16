# Week 3 - Questions and Answers

1. What build environment are you using?

    This project uses Rust and the probe-run CLI tool
    cargo is configured in .cargo/config such that running
    `cargo run` will invoke probe-run for the correct chipset.
    probe-run is powered by [probe-rs](https://probe.rs/docs/getting-started/installation/)

2. What are the hardware registers that cause the LED to turn on and off? (From the processor manual, don’t worry about initialization.)

    The LED on the STM32L1 discovery board is connected to pin PB6. There are two registers from you can set the state of the LED. The 6th bit of GPIOB_ODR sets the state directly, but the 6th and 22nd second bit of the GPIOB_BSRR register can be used to set and clear the value of GPIOB_ODR respectively. 

3. What are the registers that you read in order to find out the state of the button?

    The user button is connceted to pin PA0. The 0th bit if the GPIOA_IDR register can be read to get the current state of the button. 

4. Can you read the register directly and see the button change in a debugger or by printing out thes value of the memory at the register’s address?

    I have yet to configure the proper tooling for remote debugging, but I am looking into setting up `cargo embed` for that purpose. I am looking into adding a dependency on the "defmt" crate to enable printf-style debugging.
    [cargo embed](https://probe.rs/docs/tools/cargo-embed/)
    [defmt documentation](https://defmt.ferrous-systems.com/introduction)
