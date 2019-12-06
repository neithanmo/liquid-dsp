
use crate::liquid_dsp_sys as raw;
use crate::enums::{FecScheme, CrcScheme};

pub struct Packetizer {
    inner: raw::packetizer,
    n: u32,
}

impl Packetizer {

    /// creates and returns a packetizer object which accepts *n* uncoded input bytes and uses the specified CRC and bi-level FEC schemes.
    pub fn create(n: u32, crc: CrcScheme, fec0: FecScheme, fec1: FecScheme) -> Self {
        unsafe {
            Self {
                inner: raw::packetizer_create(n as _, u8::from(crc) as _, u8::from(fec0) as _, u8::from(fec1) as _),
                n,
            }
        }
    }
   
    /// re-creates an existing packetizer object with new parameters. 
    pub fn recreate(mut self, n: u32, crc: CrcScheme, fec0: FecScheme, fec1: FecScheme) -> Self {
        unsafe {
            self.inner = raw::packetizer_recreate(self.inner, n as _, u8::from(crc) as _, u8::from(fec0) as _, u8::from(fec1) as _);
            self.n = n;
        }
        self
    }
    
    /// prints the internal state of the packetizer object to the standard output.
    pub fn print(&self) {
        unsafe {
            raw::packetizer_print(self.inner);
        }
    }
    
    pub fn compute_enc_msg_len(decoded_len: usize, crc: CrcScheme, fec0: FecScheme, fec1: FecScheme) -> usize {
        unsafe {
            raw::packetizer_compute_enc_msg_len(decoded_len as _, u8::from(crc) as _, u8::from(fec0) as _, u8::from(fec1) as _) as usize
        }
    }
    
    pub fn compute_dec_msg_len(encoded_len: usize, crc: CrcScheme, fec0: FecScheme, fec1: FecScheme) -> usize {
        unsafe {
            raw::packetizer_compute_dec_msg_len(encoded_len as _, u8::from(crc) as _, u8::from(fec0) as _, u8::from(fec1) as _) as usize
        }
    }
    
    ///  returns the specified decoded message length n in bytes
    pub fn get_dec_msg_len(&self) -> usize {
        self.n as usize
    }
   
    /// returns the fully-encoded message length k in bytes
    pub fn get_enc_msg_len(&self) -> usize {
        unsafe {
            raw::packetizer_get_enc_msg_len(self.inner) as usize
        }
    }

    pub fn get_crc(&self) -> CrcScheme {
        CrcScheme::from(unsafe {
            raw::packetizer_get_crc(self.inner) as u8
        })
    }

    pub fn get_fec0(&self) -> FecScheme {
        FecScheme::from(unsafe {
            raw::packetizer_get_fec0(self.inner) as u8
        })
    }

    pub fn get_fec1(&self) -> FecScheme {
        FecScheme::from(unsafe {
            raw::packetizer_get_fec1(self.inner) as u8
        })
    }

    /// encodes the n -byte input message storing the result in the k -byte encoded output message.
    /// panics if the provided buffers are not the same len as n and k internal buffers.
    pub fn encode(&self, raw: &[u8], pckt: &mut[u8]) {
        assert!(raw.len() == self.get_dec_msg_len(), "raw data must have the same size as the internal buffer, 
            use packetizer_get_dec_msg_len");
        assert!(pckt.len() == self.get_enc_msg_len(), "pckt array must have the same size as the pckt internal buffer, 
            use packetizer_get_enc_msg_len");
        unsafe {
            raw::packetizer_encode(self.inner, raw.as_ptr() as _, pckt.as_mut_ptr() as _);
        }
    }

    /// decodes the k -byte encoded input message storing the result in the n -byte output. 
    /// The function returns a 1 if the internal CRC passed and a 0 if it failed. 
    /// If no CRC was specified (e.g. LIQUID_CRC_NONE ) then a 1 is always returned. 
    pub fn decode(&self, pckt: &[u8], raw: &mut [u8]) -> u8 {
        assert!(raw.len() == self.get_dec_msg_len(), "raw data must have the same size as the internal buffer, 
            use packetizer_get_dec_msg_len");
        assert!(pckt.len() == self.get_enc_msg_len(), "pckt array must have the same size as the pckt internal buffer, 
            use packetizer_get_enc_msg_len");
        unsafe {
            raw::packetizer_decode(self.inner, pckt.as_ptr() as _, raw.as_mut_ptr() as _) as u8
        }
    }
   
    /// decodes the encoded input message just like packetizer_decode() but with soft bits instead of hard bytes. 
    /// The input is an array of type unsigned char with 8×k elements representing soft bits. 
    /// As before, the function returns a 1 if the internal CRC passed and a 0 if it failed. 
    /// See [section-fec-soft] for more information on soft-decision decoding. 
    pub fn decode_soft(&self, pckt: &[u8], raw: &mut [u8]) {
        assert!(raw.len() == self.get_dec_msg_len(), "raw data must have the same size as the internal buffer, 
            use packetizer_get_dec_msg_len");
        assert!(pckt.len() == 8*self.get_enc_msg_len(), "pckt array must have 8 * k elements");
        unsafe {
            raw::packetizer_decode_soft(self.inner, pckt.as_ptr() as _, raw.as_mut_ptr() as _);
        }
    }
}


impl Drop for Packetizer {
    fn drop(&mut self) {
        unsafe {
            raw::packetizer_destroy(self.inner);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::{Packetizer};
    use crate::enums::{FecScheme, CrcScheme};

    #[test]
    fn test_packetizer_encode_decode() {
        // set up the options
        let n = 16;                      // uncoded data length
        let crc  = CrcScheme::CRC_32;        // validity check
        let fec0 = FecScheme::HAMMING74; // inner code
        let fec1 = FecScheme::REP3;      // outer code

        // compute resulting packet length
        let k = Packetizer::compute_enc_msg_len(n,crc,fec0,fec1);

        // set up the arrays
        let mut msg = vec![0u8; n];       // original message
        let mut packet = vec![0u8; k];    // encoded message
        let mut msg_dec = vec![0u8; n];   // decoded message

        // create the packetizer object
        let p = Packetizer::create(n as _,crc,fec0,fec1);

        // initialize msg here
        for i in 0..n {
            msg[i] = (i & (0xff as usize)) as u8 ;
        }

        // encode the packet
        p.encode(&msg, &mut packet);

        // decode the packet, returning validity
        p.decode( &packet, &mut msg_dec);

        assert_eq!(&msg, &msg_dec);

    }
}