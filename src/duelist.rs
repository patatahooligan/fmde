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
    pub card_odds: [u16; NUMBER_OF_CARDS],
}

impl CardList {
    pub fn new() -> CardList {
        return CardList {
            card_odds: [0; NUMBER_OF_CARDS],
        };
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

fn read_card_list(card_list_data: &[u8]) -> CardList {
    assert!(
        card_list_data.len() == CARDLIST_SIZE,
        "Card lists must be exactly 1444 bytes (2 per card)"
    );

    let mut card_list = CardList::new();

    for i in 0..NUMBER_OF_CARDS {
        let low_byte: u16 = card_list_data[2 * i].into();
        let high_byte: u16 = card_list_data[2 * i + 1].into();

        card_list.card_odds[i] = (high_byte << 8) + low_byte;
    }

    return card_list;
}

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

pub fn read_all_duelists(slus: &Vec<u8>, wa_mrg: &Vec<u8>) -> Vec<Duelist> {
    let mut duelists = Vec::new();

    for duelist_id in 0..NUMBER_OF_DUELISTS {
        let duelist_info = read_duelist(slus, wa_mrg, duelist_id);

        duelists.push(duelist_info);
    }

    return duelists;
}
