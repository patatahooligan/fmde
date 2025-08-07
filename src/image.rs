//! Functions to manipulate the ROM image. We need to find two files
//! inside the filesystem:
//! - slus_014.11
//! - data/wa_mrg.mrg
//!
//! The format seems to be CD-ROM/XA Form 1. I don't know exactly how it
//! works but it seems to have the following layout:
//! - 12 bytes sync pattern
//! - 3 bytes address
//! - 1 byte mode
//! - 8 bytes subheader
//! - 2048 bytes data
//! - 4 bytes error detection
//! - 276 bytes error correction
//!
//! I'm wondering if emulators even check the error detection and
//! correction. It's possible that we can ignore them and work solely on
//! the data.

// TODO: Implement a stronger typing system. In the end we are going to
// have at least three different objects that are all basically &[u8],
// and we should prevent getting them mixed up. For example a function
// that wants to operate on the SLUS file should not accept the entire
// ROM. Even just some types that do nothing more than wrap the &[u8]
// would be an improvement of the interface.

const SECTOR_SIZE_BYTES: usize = 2352;
const DATA_OFFSET_BYTES: usize = 24;
const DATA_SIZE_BYTES: usize = 2048;

const SLUS_OFFSET_SECTORS: usize = 24;
const SLUS_SIZE_SECTORS: usize = 929;

/// Extract `SLUS-014.11` from the bin file. This conversion throws away
/// all the metadata required by CD-ROM/XA and returns a concatenated
/// vector of the raw data.
pub fn get_slus_from_bin(rom_file: &Vec<u8>) -> Vec<u8> {
    let start = SLUS_OFFSET_SECTORS * SECTOR_SIZE_BYTES;
    let end = start + SLUS_SIZE_SECTORS * SECTOR_SIZE_BYTES;
    return cdxa_form1_to_raw_data(&rom_file[start..end]);
}

/// Get the raw data from a single CD-ROM/XA Form 1 sector. Usually you
/// don't want to operate on a single sector. This function's main
/// intent is to be called by other functions that will work on spans of
/// sectors.
fn get_data_from_sector(sector: &[u8]) -> &[u8] {
    assert!(sector.len() == SECTOR_SIZE_BYTES);
    return &sector[DATA_OFFSET_BYTES..DATA_OFFSET_BYTES + DATA_SIZE_BYTES];
}

/// Extract and concatenate the raw data from a slice of CD-ROM/XA Form
/// 1 data. `cd_xa` data must be comprised of whole sectors, ie its size
/// must be (2352 * n) bytes.
fn cdxa_form1_to_raw_data(cdxa_data: &[u8]) -> Vec<u8> {
    let mut raw_data = Vec::new();

    assert!(
        cdxa_data.len() % SECTOR_SIZE_BYTES == 0,
        "cdxa_data must be whole sectors"
    );

    for i in 0..cdxa_data.len() / SECTOR_SIZE_BYTES {
        let start = i * SECTOR_SIZE_BYTES;
        let end = start + SECTOR_SIZE_BYTES;
        let sector = &cdxa_data[start..end];
        raw_data.extend_from_slice(get_data_from_sector(&sector));
    }

    return raw_data;
}
