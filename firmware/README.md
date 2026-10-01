# HackXpansion audio module driver

This crate is an `xpanse-api` module driver, not a standalone console firmware binary. It follows the driver interface used by the [HackXpansion firmware](https://github.com/hackclub/hackxpansion) and targets the RP235x console.

## Build check

```sh
cargo check --target thumbv8m.main-none-eabihf
```

## Audio interface and hardware

- The driver publishes `AUDIO_QUEUE` through the Xpanse resource registry. Producers enqueue mono signed 16-bit PCM buffers (`AudioBuffer`, 512 samples) with the Embassy channel API; its capacity is four buffers, so sending waits when playback falls behind.
- Samples are output on the module-standard PWM channel A: GPIO2 / PWM slice 2. The playback interval is 23 µs per sample (about 43.5 kHz), preserving the original timing.
- `AudioDriver::ID` is currently set to MD resistors `R1K1` and `R1K2`. The physical module must use that exact pair, or the ID constant must be changed to match its installed resistors. Module detection is resistor-coded by the console.

The driver API and GPIO roles are defined by [`xpanse-api`](https://docs.rs/xpanse-api/0.2.3/xpanse_api/); the console's reference firmware and driver examples are in the [official repository](https://github.com/hackclub/hackxpansion/tree/main/firmware).
