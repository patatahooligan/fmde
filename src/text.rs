/// Text conversion helpers.
///
/// I'm not sure what format YGO:FM uses, so I've just hard-coded it. If
/// it turns out to be some standard encoding, we can significantly
/// improve this code.

const STRING_TERMINATOR: u8 = 255;

/// Convert a single byte to `char`. This is probably redundant since
/// code outside this module would most likely prefer to work directly
/// with Strings. This might be made private in the future.
pub fn u8_to_char(byte: u8) -> char {
    // I've verified that this is correct for decoding card text in the US
    // version (SLUS-01411). I tried it on the EU version (SLES-03947) and
    // it seems that some characters are correct, but not all. It seems that
    // mods are generally based on SLUS-01411 so let's support only that for
    // now and we'll see about SLES-03947 later.
    match byte {
        0x18 => 'A',
        0x2D => 'B',
        0x2B => 'C',
        0x20 => 'D',
        0x25 => 'E',
        0x31 => 'F',
        0x29 => 'G',
        0x23 => 'H',
        0x1A => 'I',
        0x3B => 'J',
        0x33 => 'K',
        0x2A => 'L',
        0x1E => 'M',
        0x2C => 'N',
        0x21 => 'O',
        0x2F => 'P',
        0x3E => 'Q',
        0x26 => 'R',
        0x1D => 'S',
        0x1C => 'T',
        0x35 => 'U',
        0x39 => 'V',
        0x22 => 'W',
        0x46 => 'X',
        0x24 => 'Y',
        0x3F => 'Z',
        0x03 => 'a',
        0x15 => 'b',
        0x0F => 'c',
        0x0C => 'd',
        0x01 => 'e',
        0x13 => 'f',
        0x10 => 'g',
        0x09 => 'h',
        0x05 => 'i',
        0x34 => 'j',
        0x16 => 'k',
        0x0A => 'l',
        0x0E => 'm',
        0x06 => 'n',
        0x04 => 'o',
        0x14 => 'p',
        0x37 => 'q',
        0x08 => 'r',
        0x07 => 's',
        0x02 => 't',
        0x0D => 'u',
        0x19 => 'v',
        0x12 => 'w',
        0x36 => 'x',
        0x11 => 'y',
        0x32 => 'z',
        0x38 => '0',
        0x3D => '1',
        0x3A => '2',
        0x41 => '3',
        0x4A => '4',
        0x42 => '5',
        0x4E => '6',
        0x45 => '7',
        0x57 => '8',
        0x59 => '9',
        0x00 => ' ',
        0x30 => '-',
        0x3C => '#',
        0x43 => '&',
        0x0B => '.',
        0x1F => ',',
        0x55 => 'a',
        0x17 => '!',
        0x1B => '\'',
        0x27 => '<',
        0x28 => '>',
        0x2E => '?',
        0x44 => '/',
        0x48 => ':',
        0x4B => ')',
        0x4C => '(',
        0x4F => '$',
        0x50 => '*',
        0x51 => '>',
        0x54 => '<',
        0x40 => '"',
        0x56 => '+',
        0x5B => '%',
        _ => '_',
    }
}

/// Read bytes from the start of the slice until the string terminator
/// is found. Convert them to `char`s and return a String containing the
/// result.
///
/// Because this function will return immediately after finding the
/// string terminator, you don't have to worry about restricting the
/// upper bound of the slice. The normal use case would be
/// something like this:
///
/// ```
/// let binary_data = ...; // This can be as large as the entire ROM
/// let offset = ...;      // Somehow determine where your string starts
///
/// let my_string = read_terminated_string(binary_data[offset..]);
/// ```
///
/// The function requires that the string terminator byte exists in the
/// given slice. If it doesn't exist then it is assumed that something
/// has gone wrong in the handling of the binary data and there is not
/// meaningful way to recover. Therefore, in this case we panic.
pub fn read_terminated_string(binary_data: &[u8]) -> String {
    let mut result = String::new();

    for byte in binary_data {
        if *byte == STRING_TERMINATOR {
            return result;
        }

        result.push(u8_to_char(*byte));
    }

    panic!("No terminator character in buffer!");
}
