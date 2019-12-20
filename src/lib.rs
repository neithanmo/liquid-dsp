extern crate libc;
#[macro_use]
extern crate bitflags;
extern crate liquid_dsp_sys;
extern crate num;

mod agc;
mod ampmodem;
mod autocorr;
mod cbuffer;
mod channel;
mod cvsd;
mod fec;
mod fft;
mod filter;
mod tvmpch;

mod callbacks;
mod enums;
mod errors;
mod utils;

pub use agc::AgcCrcf;
pub use ampmodem::AmpModem;
pub use autocorr::{AutoCorrCccf, AutoCorrRrrf};
pub use cbuffer::{CbufferCf, CbufferRf};
pub use channel::ChannelCccf;
pub use cvsd::Cvsd;
pub use fec::{Fec, Interleaver, Packetizer};
pub use fft::{AsgramCf, AsgramRf, Fft, FftPlan};
pub use filter::{Firdespm, Firdes, Fir, FirdespmBtype, FirdespmWtype, FirdesFilterType, IirdesFilterType, IirdesBandType, IirdesFormat};
pub use tvmpch::TvmpchCccf;

pub use enums::{
    AgcSquelchMode, AmpModemType, CrcScheme, FecScheme, FftType, 
};

pub use errors::LiquidError;


pub type LiquidResult<T> = Result<T, LiquidError>;
