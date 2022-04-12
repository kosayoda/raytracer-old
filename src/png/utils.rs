use once_cell::sync::Lazy;

static CRC_TABLE: Lazy<[u32; 256]> = Lazy::new(|| {
    let mut crc_table = [0_u32; 256];
    let mut c: u32;
    for n in 0..256 {
        c = n;
        for _ in 0..8 {
            if (c & 1) != 0 {
                c = 0xedb88320_u32 ^ (c >> 1);
            } else {
                c >>= 1;
            }
        }
        crc_table[n as usize] = c;
    }
    crc_table
});

// Cyclic Redundancy Check for PNG chunks
// Retrieved from: http://www.libpng.org/pub/png/spec/1.2/PNG-CRCAppendix.html
pub fn crc_checksum(bytes: &[u8]) -> u32 {
    // Compute CRC
    let mut crc = 0xFFFFFFFF_u32;
    for n in 0..bytes.len() {
        crc = CRC_TABLE[((crc ^ bytes[n] as u32) & 0xFF) as usize] ^ (crc >> 8);
    }

    crc ^ 0xFFFFFFFF_u32
}
