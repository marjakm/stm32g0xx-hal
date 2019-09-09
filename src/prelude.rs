pub use hal::digital::v2::*;
pub use hal::prelude::*;

pub use hal::adc::OneShot as _;
pub use hal::watchdog::Watchdog as _;
pub use hal::watchdog::WatchdogEnable as _;

pub use crate::adc::AdcExt as _;
pub use crate::crc::CrcExt as _;
#[cfg(any(feature = "stm32g07x", feature = "stm32g081"))]
pub use crate::dac::DacExt as _;
#[cfg(any(feature = "stm32g07x", feature = "stm32g081"))]
pub use crate::dac::DacOut as _;
#[cfg(any(feature = "stm32g07x", feature = "stm32g081"))]
pub use crate::dac::DacPin as _;
pub use crate::dma::CopyDma as _;
pub use crate::dma::ReadDma as _;
pub use crate::dma::WriteDma as _;
pub use crate::delay::DelayExt as _;
pub use crate::exti::ExtiExt as _;
pub use crate::gpio::GpioExt as _;
pub use crate::i2c::I2cExt as _;
pub use crate::pwm::PwmExt as _;
pub use crate::qei::QeiExt as _;
pub use crate::rcc::LSCOExt as _;
pub use crate::rcc::MCOExt as _;
pub use crate::rcc::RccExt as _;
pub use crate::rng::RngCore as _;
pub use crate::rng::RngExt as _;
pub use crate::serial::SerialExt as _;
pub use crate::spi::SpiExt as _;
pub use crate::stopwatch::StopwatchExt as _;
pub use crate::time::U32Ext as _;
pub use crate::timer::TimerExt as _;
pub use crate::watchdog::IWDGExt as _;
pub use crate::watchdog::WWDGExt as _;
