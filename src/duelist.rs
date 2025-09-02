//! Module to manipulate duelist data. The function names follow this
//! (arbitrary) convention to indicate what they operate on:
//! - read/write if they operate on the ROM file
//! - load/dump if they operate on csv files

use csv::{ReaderBuilder, Writer};

use crate::text;

// I don't know if there is any way in the ROM to figure out where the
// ends of these arrays are, so we have to use these constants for the
// iteration.
pub const NUMBER_OF_CARDS: usize = 722;
pub const NUMBER_OF_DUELISTS: usize = 39;

// Card rates are stored as 2 bytes
pub const CARDLIST_SIZE: usize = 2 * NUMBER_OF_CARDS;

const CARD_NAME_INDICES_OFFSET: usize = 0x1C6002;
const NAME_OFFSET: usize = 0x1C0800;

const DUELIST_DATA_OFFSET: usize = 0xE9B000;
const DUELIST_NAME_INDICES_OFFSET: usize = 0x1C6652;
const DUELIST_DATA_SIZE: usize = 0x1800;
const DUELIST_DECK_RELATIVE_OFFSET: usize = 0x0;
const DUELIST_SAPOW_OFFSET: usize = 0x5B4;
const DUELIST_BCD_OFFSET: usize = 0xB68;
const DUELIST_SATEC_OFFSET: usize = 0x111C;

/// A list of weights corresponding to each card. These are used to:
/// - Generate the duelist's deck
/// - Determine the card dropped at the end of a victory
pub struct CardList {
    pub card_rate: [u16; NUMBER_OF_CARDS],
}

impl CardList {
    /// Create a CardList where all weights are `0`. This is not a valid
    /// card list according to the game logic and it should be modified
    /// before being written.
    pub fn new() -> CardList {
        return CardList {
            card_rate: [0; NUMBER_OF_CARDS],
        };
    }

    /// Check that a CardList is valid. This means that all weights
    /// should add to 2048.
    pub fn is_valid(&self) -> bool {
        return self.card_rate.iter().sum::<u16>() == 2048;
    }

    pub fn print(&self) {
        for (id, cr) in self.card_rate.iter().enumerate() {
            if *cr != 0 {
                println!("{}: {}", id, cr);
            }
        }
    }
}

pub struct Duelist {
    pub name: String,
    pub deck: CardList,
    pub drops_sa_pow: CardList,
    pub drops_sa_tec: CardList,
    pub drops_bcd: CardList,
}

impl Duelist {
    pub fn new() -> Duelist {
        return Duelist {
            name: String::new(),
            deck: CardList::new(),
            drops_sa_pow: CardList::new(),
            drops_bcd: CardList::new(),
            drops_sa_tec: CardList::new(),
        };
    }
}

/// Read all the card names from the given slus file.
pub fn get_card_names(slus: &Vec<u8>) -> Vec<String> {
    let mut card_names = Vec::new();

    for i in 0..NUMBER_OF_CARDS {
        // The game stores a relative offset starting from NAME_OFFSET
        let low_byte: usize = slus[CARD_NAME_INDICES_OFFSET + 2 * i].into();
        let high_byte: usize =
            slus[CARD_NAME_INDICES_OFFSET + 2 * i + 1].into();
        let name_relative_offset: usize = (high_byte << 8) + low_byte;

        let name_absolute_offset = NAME_OFFSET + name_relative_offset;
        let card_name =
            text::read_terminated_string(&slus[name_absolute_offset..]);
        card_names.push(card_name);
    }

    return card_names;
}

/// Read a CardList from the format used in the wa_mrg file.
fn read_card_list(card_list_data: &[u8]) -> CardList {
    assert!(
        card_list_data.len() == CARDLIST_SIZE,
        "Card lists must be exactly 1444 bytes (2 per card)"
    );

    let mut card_list = CardList::new();

    for i in 0..NUMBER_OF_CARDS {
        let low_byte: u16 = card_list_data[2 * i].into();
        let high_byte: u16 = card_list_data[2 * i + 1].into();

        card_list.card_rate[i] = (high_byte << 8) + low_byte;
    }

    return card_list;
}

/// Write a CardList into the given slice. This is written in the format
/// expected for the wa_mrg file.
fn write_card_list_to_slice(card_list: &CardList, target: &mut [u8]) {
    assert!(
        target.len() == CARDLIST_SIZE,
        "Card lists must be exactly 1444 bytes (2 per card)"
    );
    assert!(
        card_list.is_valid(),
        "Card list is invalid - weights do not add to 2024"
    );

    for i in 0..NUMBER_OF_CARDS {
        let low_byte = (card_list.card_rate[i]) as u8;
        let high_byte = (card_list.card_rate[i] >> 8) as u8;

        target[2 * i] = low_byte;
        target[2 * i + 1] = high_byte;
    }
}

/// Read a single duelists info.
fn read_duelist(
    slus: &Vec<u8>,
    wa_mrg: &Vec<u8>,
    duelist_id: usize,
) -> Duelist {
    let mut duelist_info = Duelist::new();

    // The game stores a relative offset starting from NAME_OFFSET
    let low_byte: usize =
        slus[DUELIST_NAME_INDICES_OFFSET + 2 * duelist_id].into();
    let high_byte: usize =
        slus[DUELIST_NAME_INDICES_OFFSET + 2 * duelist_id + 1].into();
    let name_relative_offset: usize = (high_byte << 8) + low_byte;

    let name_absolute_offset = NAME_OFFSET + name_relative_offset;
    let duelist_name =
        text::read_terminated_string(&slus[name_absolute_offset..]);
    duelist_info.name = duelist_name;

    // Relative offset from the start of the duelist data array.
    let current_duelist_offset =
        DUELIST_DATA_OFFSET + (DUELIST_DATA_SIZE * duelist_id);

    let deck_offset = current_duelist_offset + DUELIST_DECK_RELATIVE_OFFSET;
    let drops_sa_pow_offset = current_duelist_offset + DUELIST_SAPOW_OFFSET;
    let drops_bcd_offset = current_duelist_offset + DUELIST_BCD_OFFSET;
    let drops_sa_tec_offset = current_duelist_offset + DUELIST_SATEC_OFFSET;

    duelist_info.deck =
        read_card_list(&wa_mrg[deck_offset..deck_offset + CARDLIST_SIZE]);
    duelist_info.drops_sa_pow = read_card_list(
        &wa_mrg[drops_sa_pow_offset..drops_sa_pow_offset + CARDLIST_SIZE],
    );
    duelist_info.drops_bcd = read_card_list(
        &wa_mrg[drops_bcd_offset..drops_bcd_offset + CARDLIST_SIZE],
    );
    duelist_info.drops_sa_tec = read_card_list(
        &wa_mrg[drops_sa_tec_offset..drops_sa_tec_offset + CARDLIST_SIZE],
    );

    return duelist_info;
}

/// Write a single duelist into the given wa_mrg file.
fn write_duelist(
    wa_mrg: &mut Vec<u8>,
    duelist_id: usize,
    duelist_info: &Duelist,
) {
    // Relative offset from the start of the duelist data array.
    let current_duelist_offset =
        DUELIST_DATA_OFFSET + (DUELIST_DATA_SIZE * duelist_id);

    let deck_offset = current_duelist_offset + DUELIST_DECK_RELATIVE_OFFSET;
    let drops_sa_pow_offset = current_duelist_offset + DUELIST_SAPOW_OFFSET;
    let drops_bcd_offset = current_duelist_offset + DUELIST_BCD_OFFSET;
    let drops_sa_tec_offset = current_duelist_offset + DUELIST_SATEC_OFFSET;

    write_card_list_to_slice(
        &duelist_info.deck,
        &mut wa_mrg[deck_offset..deck_offset + CARDLIST_SIZE],
    );
    write_card_list_to_slice(
        &duelist_info.drops_sa_pow,
        &mut wa_mrg[drops_sa_pow_offset..drops_sa_pow_offset + CARDLIST_SIZE],
    );
    write_card_list_to_slice(
        &duelist_info.drops_bcd,
        &mut wa_mrg[drops_bcd_offset..drops_bcd_offset + CARDLIST_SIZE],
    );
    write_card_list_to_slice(
        &duelist_info.drops_sa_tec,
        &mut wa_mrg[drops_sa_tec_offset..drops_sa_tec_offset + CARDLIST_SIZE],
    );
}

/// Read all the duelists from the given slus and wa_mrg files. Return
/// them as a vector.
pub fn read_all_duelists(slus: &Vec<u8>, wa_mrg: &Vec<u8>) -> Vec<Duelist> {
    let mut duelists = Vec::new();

    for duelist_id in 0..NUMBER_OF_DUELISTS {
        let duelist_info = read_duelist(slus, wa_mrg, duelist_id);

        duelists.push(duelist_info);
    }

    return duelists;
}

/// Write all duelist data into the given wa_mrg. Modifying the duelist
/// name is not supported at this moment so we don't need to touch the
/// slus file.
pub fn write_all_duelists(wa_mrg: &mut Vec<u8>, duelists: &[Duelist]) {
    assert!(duelists.len() == NUMBER_OF_DUELISTS);

    for duelist_id in 0..NUMBER_OF_DUELISTS {
        write_duelist(wa_mrg, duelist_id, &duelists[duelist_id]);
    }
}

/// Dump a cardlist into a .csv file at the given path. The csv has no
/// header and follows the following form:
///
/// card_id,rate,card_name
///
/// This function does not create rows for cards whose rate is equal to
/// `0`. The game does have entries for zero-rate cards because the data
/// is stored in plain arrays and the CardList objects reflect that, but
/// we don't want to include these entries in the csv files. If we did
/// they would become too tedious to work with.
fn dump_cardlist_csv(
    csv_path: &std::path::Path,
    cardlist: &CardList,
    card_names: &[String],
) {
    let mut csv = Writer::from_path(csv_path).unwrap();
    for (card_id, card_rate) in cardlist.card_rate.iter().enumerate() {
        if *card_rate != 0 {
            csv.write_record(&[
                // Shift card_id by 1 to match the official number which
                // starts at 1.
                &(card_id + 1).to_string(),
                &card_rate.to_string(),
                &card_names[card_id],
            ])
            .unwrap();
        }
    }
}

/// Load a cardlist from a .csv file at the given path.
fn load_cardlist_csv(csv_path: &std::path::Path) -> CardList {
    let mut card_list = CardList::new();
    let mut csv = ReaderBuilder::new()
        .has_headers(false)
        .from_path(csv_path)
        .unwrap();

    for record_result in csv.records() {
        let record = record_result.unwrap();
        // Shift card_id by 1 to match the official number which
        // starts at 1.
        let card_id = record.get(0).unwrap().parse::<usize>().unwrap() - 1;
        let card_rate = record.get(1).unwrap().parse::<u16>().unwrap();

        // We don't have to check that the numbers are >0 because they
        // are unsigned types. If they are negative, they will simply
        // fail to parse above.
        assert!(card_id < NUMBER_OF_CARDS);
        assert!(card_rate < 2048);

        card_list.card_rate[card_id] = card_rate;
    }

    if !card_list.is_valid() {
        println!("Invalid card list at {}", csv_path.display());
        card_list.print();
        panic!();
    }
    return card_list;
}

/// Dump a single duelist's data into a collection of .csv's under the
/// given directory:
/// - deck.csv
/// - drops-bcd.csv
/// - drops-sa-pow.csv
/// - drops-sa-tec.csv
fn dump_duelist_csv(
    dir_path: &std::path::Path,
    duelist: &Duelist,
    card_names: &[String],
) {
    dump_cardlist_csv(&dir_path.join("deck.csv"), &duelist.deck, &card_names);
    dump_cardlist_csv(
        &dir_path.join("drops-bcd.csv"),
        &duelist.drops_bcd,
        &card_names,
    );
    dump_cardlist_csv(
        &dir_path.join("drops-sa-pow.csv"),
        &duelist.drops_sa_pow,
        &card_names,
    );
    dump_cardlist_csv(
        &dir_path.join("drops-sa-tec.csv"),
        &duelist.drops_sa_tec,
        &card_names,
    );
}

/// Load all duelist data from a collection of .csv's under the given
/// directory. Any and all files might be missing, as well as the
/// directory itself. This exists to allow the use of "sparse" files,
/// where you can define only the parts of the mod that you intend to
/// change from the original rom. This is also why we have to take the
/// Duelist as a `&mut`. The caller must make sure that the object is
/// valid before passing it to this function. The intended use is for
/// the object to have been created by reading the rom so that this
/// function can selectively update parts of it as the user desires.
///
/// On the other hand, the function panics if:
/// - the file exists but cannot be parsed
/// - the check itself for the file's existence fails
/// - the file is deleted while the program is running (possibly)
fn load_duelist_csv(dir_path: &std::path::Path, duelist: &mut Duelist) {
    let deck_path = dir_path.join("deck.csv");
    let drops_bcd_path = dir_path.join("drops-bcd.csv");
    let drops_sa_pow_path = dir_path.join("drops-sa-pow.csv");
    let drops_sa_tec_path = dir_path.join("drops-sa-tec.csv");

    if deck_path.try_exists().unwrap() {
        duelist.deck = load_cardlist_csv(&deck_path);
    }
    if drops_bcd_path.try_exists().unwrap() {
        duelist.drops_bcd = load_cardlist_csv(&drops_bcd_path);
    }
    if drops_sa_pow_path.try_exists().unwrap() {
        duelist.drops_sa_pow = load_cardlist_csv(&drops_sa_pow_path);
    }
    if drops_sa_tec_path.try_exists().unwrap() {
        duelist.drops_sa_tec = load_cardlist_csv(&drops_sa_tec_path);
    }
}

/// Dump all of the cardlists - both decks and drops - to the given
/// directory.
pub fn dump_all_duelists_csv(
    top_level_dir: &std::path::Path,
    duelists: &[Duelist],
    card_names: &[String],
) {
    std::fs::create_dir_all(top_level_dir).unwrap();
    for (duelist_id, duelist) in duelists.iter().enumerate() {
        let duelist_dir = top_level_dir
            .join((duelist_id + 1).to_string() + "." + &duelist.name);
        std::fs::create_dir(&duelist_dir).unwrap();

        dump_duelist_csv(&duelist_dir, &duelist, &card_names);
    }
}

/// Load all the duelists from csv files and return them as a vector.
pub fn load_all_duelists_csv(
    top_level_dir: &std::path::Path,
    duelists: &mut Vec<Duelist>,
) {
    for (duelist_id, duelist) in duelists.iter_mut().enumerate() {
        // TODO: Allow directory names in the form "1. <ARBITRARY
        // NAME>". The name would exist to aid the user and would be
        // completely ignored by us.
        let duelist_dir = top_level_dir
            .join((duelist_id + 1).to_string() + "." + &duelist.name);

        load_duelist_csv(&duelist_dir, duelist);
    }
}
