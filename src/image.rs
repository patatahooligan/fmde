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

use crc;

const SECTOR_SIZE_BYTES: usize = 2352;

const DATA_OFFSET_BYTES: usize = 24;
const DATA_SIZE_BYTES: usize = 2048;

const CRC_OFFSET_BYTES: usize = 2072;

const SLUS_OFFSET_SECTORS: usize = 24;
const SLUS_SIZE_SECTORS: usize = 929;

const WA_MRG_OFFSET_SECTORS: usize = 10102;
const WA_MRG_SIZE_SECTORS: usize = 18432;

fn calculate_crc(raw_data: &[u8]) -> u32 {
    assert!(raw_data.len() == DATA_SIZE_BYTES + 8);

    const CUSTOM_ALG: crc::Algorithm<u32> = crc::Algorithm {
        width: 32,
        poly: 0x8001801b,
        init: 0x0,
        refin: true,
        refout: true,
        xorout: 0x0,
        // I'm not sure what these last two are, but they don't seem to
        // be part of the calculation. They are probably either for
        // testing or for use on the receiving side.
        check: 0x0,
        residue: 0x0,
    };

    let crc = crc::Crc::<u32>::new(&CUSTOM_ALG);
    let mut digest = crc.digest();
    digest.update(raw_data);
    return digest.finalize();
}

/// Extract `SLUS-014.11` from the bin file. This conversion throws away
/// all the metadata required by CD-ROM/XA and returns a concatenated
/// vector of the raw data.
pub fn read_slus_from_bin(rom_file: &Vec<u8>) -> Vec<u8> {
    let start = SLUS_OFFSET_SECTORS * SECTOR_SIZE_BYTES;
    let end = start + SLUS_SIZE_SECTORS * SECTOR_SIZE_BYTES;
    return cdxa_form1_to_raw_data(&rom_file[start..end]);
}

/// Write `SLUS-014.11` into the bin file. This doesn't touch the
/// CD-ROM/XA metadata that is already in the bin file. This would
/// probably make it break on real hardware because it would
/// detect corrupted data, but it's probably OK for emulators.
pub fn write_slus_to_bin(rom_file: &mut Vec<u8>, slus: &[u8]) {
    assert!(slus.len() == SLUS_SIZE_SECTORS * DATA_SIZE_BYTES);

    let start = SLUS_OFFSET_SECTORS * SECTOR_SIZE_BYTES;
    let end = start + SLUS_SIZE_SECTORS * SECTOR_SIZE_BYTES;

    raw_data_to_cdxa_form1(&slus, &mut rom_file[start..end]);
}

/// Extract `WA_MRG.MRG` from the bin file. This conversion throws away
/// all the metadata required by CD-ROM/XA and returns a concatenated
/// vector of the raw data.
pub fn read_wa_mrg_from_bin(rom_file: &Vec<u8>) -> Vec<u8> {
    let start = WA_MRG_OFFSET_SECTORS * SECTOR_SIZE_BYTES;
    let end = start + WA_MRG_SIZE_SECTORS * SECTOR_SIZE_BYTES;
    return cdxa_form1_to_raw_data(&rom_file[start..end]);
}

/// Write `WA_MRG.MRG` into the bin file. This doesn't touch the
/// CD-ROM/XA metadata that is already in the bin file. This would
/// probably make it break on real hardware because it would
/// detect corrupted data, but it's probably OK for emulators.
pub fn write_wa_mrg_to_bin(rom_file: &mut Vec<u8>, wa_mrg: &[u8]) {
    assert!(wa_mrg.len() == WA_MRG_SIZE_SECTORS * DATA_SIZE_BYTES);

    let start = WA_MRG_OFFSET_SECTORS * SECTOR_SIZE_BYTES;
    let end = start + WA_MRG_SIZE_SECTORS * SECTOR_SIZE_BYTES;

    raw_data_to_cdxa_form1(&wa_mrg, &mut rom_file[start..end]);
}

/// Get the raw data from a single CD-ROM/XA Form 1 sector. Usually you
/// don't want to operate on a single sector. This function's main
/// intent is to be called by other functions that will work on spans of
/// sectors.
fn read_data_from_sector(sector: &[u8]) -> &[u8] {
    assert!(sector.len() == SECTOR_SIZE_BYTES);
    return &sector[DATA_OFFSET_BYTES..DATA_OFFSET_BYTES + DATA_SIZE_BYTES];
}

/// Write the raw data into a single CD-ROM/XA Form 1 sector. This only
/// writes the payload into the sector and leaves all the metadata
/// zeroed out.
///
/// If the raw data is smaller than the payload size of the sector, the
/// rest will be zeroed out. If the raw data is larger, the function
/// panics.
///
/// Usually you don't want to operate on a single sector. This
/// function's main intent is to be called by other functions that will
/// work on spans of sectors.
fn write_data_to_sector(raw_data: &[u8], sector: &mut [u8]) {
    assert!(raw_data.len() <= DATA_SIZE_BYTES);
    assert!(sector.len() == SECTOR_SIZE_BYTES);

    sector[DATA_OFFSET_BYTES..DATA_OFFSET_BYTES + DATA_SIZE_BYTES]
        .copy_from_slice(raw_data);

    let crc_segment = &sector[16..DATA_OFFSET_BYTES + DATA_SIZE_BYTES];
    let crc = calculate_crc(crc_segment);

    sector[CRC_OFFSET_BYTES] = crc as u8;
    sector[CRC_OFFSET_BYTES + 1] = (crc >> 8) as u8;
    sector[CRC_OFFSET_BYTES + 2] = (crc >> 16) as u8;
    sector[CRC_OFFSET_BYTES + 3] = (crc >> 24) as u8;
}

/// Extract and concatenate the raw data from a slice of CD-ROM/XA Form
/// 1 data. `cdxa_data` must be comprised of whole sectors, ie its size
/// must be (2352 * n) bytes.
fn cdxa_form1_to_raw_data(cdxa_data: &[u8]) -> Vec<u8> {
    assert!(
        cdxa_data.len() % SECTOR_SIZE_BYTES == 0,
        "cdxa_data must be whole sectors"
    );

    let mut raw_data = Vec::new();

    for i in 0..cdxa_data.len() / SECTOR_SIZE_BYTES {
        let start = i * SECTOR_SIZE_BYTES;
        let end = start + SECTOR_SIZE_BYTES;
        let sector = &cdxa_data[start..end];
        raw_data.extend_from_slice(read_data_from_sector(&sector));
    }

    return raw_data;
}

/// Write raw data into a slice of CD-ROM/XA Form 1 data. Doesn't touch
/// the metadata. `cdxa_data` must be comprised of whole sectors, ie its
/// size must be (2352 * n) bytes.
fn raw_data_to_cdxa_form1(raw_data: &[u8], cdxa_data: &mut [u8]) {
    assert!(
        cdxa_data.len() % SECTOR_SIZE_BYTES == 0,
        "cdxa_data must be whole sectors"
    );
    assert!(
        raw_data.len() / DATA_SIZE_BYTES <= cdxa_data.len() / SECTOR_SIZE_BYTES,
        "raw data doesn't fit into cdxa data"
    );

    let raw_data_chunks = raw_data.chunks(DATA_SIZE_BYTES);
    let cdxa_chunks = cdxa_data.chunks_mut(SECTOR_SIZE_BYTES);

    for (raw_data_chunk, current_sector) in raw_data_chunks.zip(cdxa_chunks) {
        write_data_to_sector(raw_data_chunk, current_sector);
    }
}
