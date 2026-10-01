#![no_std]

use xpanse_api::{
    bus::allocator::BusAllocator,
    driver::{Driver, DriverError, DriverMeta},
    gpio_bank::{BankPins, GpioBank},
    metadata::{ModuleDetectResistor, ModuleID, ModuleSlot},
    reexports::{
        embassy_rp::pwm::{self, SetDutyCycle},
        embassy_time::{Duration, Ticker},
    },
    registry::Registry,
};

use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};

pub const AUDIO_BUFFER_SAMPLES: usize = 512;
pub const AUDIO_QUEUE_CAPACITY: usize = 4;
pub const AUDIO_SAMPLE_PERIOD_MICROS: u64 = 23;
const PWM_TOP: u16 = 255;

pub type AudioBuffer = [i16; AUDIO_BUFFER_SAMPLES];

pub static AUDIO_QUEUE: Channel<CriticalSectionRawMutex, AudioBuffer, AUDIO_QUEUE_CAPACITY> =
    Channel::new();

#[embassy_executor::task]
async fn play_sound_task(mut pwm: pwm::Pwm<'static>) {
    let max_duty = pwm.max_duty_cycle();
    let silence_duty = max_duty / 2;
    let _ = pwm.set_duty_cycle(silence_duty);
    let mut ticker = Ticker::every(Duration::from_micros(AUDIO_SAMPLE_PERIOD_MICROS));

    loop {
        let chunk = AUDIO_QUEUE.receive().await;
        ticker.reset();

        for sample in chunk {
            let _ = pwm.set_duty_cycle(sample_to_duty(sample, max_duty));
            ticker.next().await;
        }

        // Leave the speaker at the PCM midpoint instead of holding the final
        // sample's DC level while the queue is empty.
        let _ = pwm.set_duty_cycle(silence_duty);
    }
}

fn sample_to_duty(sample: i16, max_duty: u16) -> u16 {
    let unsigned_sample = (sample as i32 + i16::MAX as i32 + 1) as u32;
    ((unsigned_sample * max_duty as u32) / u16::MAX as u32) as u16
}

pub struct AudioDriver;

impl DriverMeta for AudioDriver {
    const ID: ModuleID = ModuleID {
        // The physical module's MD resistors must match this ID. Keep it
        // distinct from the console's test-driver ID (R1K, R1K1).
        md0: ModuleDetectResistor::R1K1,
        md1: ModuleDetectResistor::R1K2,
    };
}

impl<G: BankPins> Driver<G> for AudioDriver {
    async fn create(
        gpio_bank: GpioBank<G>,
        slot: ModuleSlot,
        registry: &mut Registry,
        bus_allocator: &mut BusAllocator,
    ) -> Result<(), DriverError> {
        let mut pwm_config = pwm::Config::default();
        pwm_config.top = PWM_TOP;

        // GPIO2 is the module standard's PWM channel A on PWM slice 2.
        let pwm = pwm::Pwm::new_output_a(gpio_bank.pwm_slice2, gpio_bank.gpio2, pwm_config);
        let task = play_sound_task(pwm).map_err(|_| DriverError::InitFailed)?;
        // SAFETY: Xpanse calls module initialization from the Embassy executor on core 1.
        let spawner = unsafe { embassy_executor::Spawner::for_current_executor().await };
        spawner.spawn(task);

        registry.register(slot, Self::ID, &AUDIO_QUEUE);
        let _ = bus_allocator;
        Ok(())
    }
}
