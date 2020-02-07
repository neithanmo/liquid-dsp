extern crate libc;
#[macro_use]
extern crate bitflags;
extern crate liquid_dsp_sys;
extern crate num;

mod agc;
mod cbuffer;
mod channel;
mod cvsd;
mod fec;
mod fft;
mod filter;
mod modem;
mod tvmpch;
mod equalization;

mod callbacks;
mod enums;
mod errors;
mod utils;

pub use agc::{AgcCrcf, AgcRrrf};
pub use equalization::{EqlmsRrrf, EqlmsCccf};
pub use cbuffer::{CbufferCf, CbufferRf};
pub use channel::ChannelCccf;
pub use cvsd::Cvsd;
pub use fec::{Fec, Interleaver, Packetizer};
pub use fft::{AsgramCf, AsgramRf, Fft, FftPlan};
pub use filter::{
    FftFiltCccf, FftFiltCrcf, FftFiltRrrf, FirFiltCccf, FirFiltCrcf, FirFiltRrrf, FirHilbt,
    FirInterpCccf, FirInterpCrcf, FirInterpRrrf, FirdesFilterType, Firdespm, FirdespmBtype,
    FirdespmWtype, IirFiltCccf, IirFiltCrcf, IirFiltRrrf, IirHilbt,AutoCorrRrrf, AutoCorrCccf
};
pub use tvmpch::TvmpchCccf;

pub use modem::{AmpModem, AmpModemType, CpfskDem, CpfskMod};

pub use enums::{AgcSquelchMode, CrcScheme, FecScheme, FftType};

pub use errors::LiquidError;

pub type LiquidResult<T> = Result<T, LiquidError>;
