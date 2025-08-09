pub mod text;

mod duelist;
mod image;

const CARD_NAME_INDICES_OFFSET: usize = 0x1C6002;
const DUELIST_NAME_INDICES_OFFSET: usize = 0x1C6652;
const NAME_OFFSET: usize = 0x1C0800;

const DUELIST_DATA_OFFSET: usize = 0xE9B000;
const DUELIST_DATA_SIZE: usize = 0x1800;
const DUELIST_DECK_RELATIVE_OFFSET: usize = 0x0;
const DUELIST_SAPOW_OFFSET: usize = 0x5B4;
const DUELIST_BCD_OFFSET: usize = 0xB68;
const DUELIST_SATEC_OFFSET: usize = 0x111C;

// Card rates are stored as 2 bytes
const CARDLIST_SIZE: usize = 2 * duelist::NUMBER_OF_CARDS;

fn get_card_names(slus: &Vec<u8>) -> Vec<String> {
    let mut card_names = Vec::new();

    for i in 0..duelist::NUMBER_OF_CARDS {
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

fn get_card_list(card_list_data: &[u8]) -> duelist::CardList {
    assert!(
        card_list_data.len() == CARDLIST_SIZE,
        "Card lists must be exactly 1444 bytes (2 per card)");

    let mut card_list = duelist::CardList::new();

    for i in 0..duelist::NUMBER_OF_CARDS {
        let low_byte: u16 = card_list_data[2 * i].into();
        let high_byte: u16 =
            card_list_data[2 * i + 1].into();

        card_list.card_odds[i] = (high_byte << 8) + low_byte;
    }

    return card_list;
}

fn get_duelist_info(slus: &Vec<u8>, wa_mrg: &Vec<u8>) -> Vec<duelist::Duelist> {
    let mut duelists = Vec::new();

    for duel in 0..duelist::NUMBER_OF_DUELISTS {
        let mut duelist_info = duelist::Duelist::new();

        // The game stores a relative offset starting from NAME_OFFSET
        let low_byte: usize =
            slus[DUELIST_NAME_INDICES_OFFSET + 2 * duel].into();
        let high_byte: usize =
            slus[DUELIST_NAME_INDICES_OFFSET + 2 * duel + 1].into();
        let name_relative_offset: usize = (high_byte << 8) + low_byte;

        let name_absolute_offset = NAME_OFFSET + name_relative_offset;
        let duelist_name =
            text::read_terminated_string(&slus[name_absolute_offset..]);
        duelist_info.name = duelist_name;

        // Relative offset from the start of the duelist data array.
        let current_duelist_offset =
            DUELIST_DATA_OFFSET + (DUELIST_DATA_SIZE * duel);

        let deck_offset =
            current_duelist_offset + DUELIST_DECK_RELATIVE_OFFSET;
        let drops_sa_pow_offset =
            current_duelist_offset + DUELIST_SAPOW_OFFSET;
        let drops_bcd_offset =
            current_duelist_offset + DUELIST_BCD_OFFSET;
        let drops_sa_tec_offset =
            current_duelist_offset + DUELIST_SATEC_OFFSET;

        duelist_info.deck = get_card_list(
            &wa_mrg[deck_offset..deck_offset + CARDLIST_SIZE]);
        duelist_info.drops_sa_pow = get_card_list(
            &wa_mrg[drops_sa_pow_offset..drops_sa_pow_offset + CARDLIST_SIZE]);
        duelist_info.drops_bcd = get_card_list(
            &wa_mrg[drops_bcd_offset..drops_bcd_offset + CARDLIST_SIZE]);
        duelist_info.drops_sa_tec = get_card_list(
            &wa_mrg[drops_sa_tec_offset..drops_sa_tec_offset + CARDLIST_SIZE]);

        duelists.push(duelist_info);
    }

    return duelists;
}

fn print_card_list(card_list: &duelist::CardList, card_names: &[String]) {
    assert!(card_names.len() == duelist::NUMBER_OF_CARDS);

    for i in 0..duelist::NUMBER_OF_CARDS {
        if card_list.card_odds[i] == 0 {
            continue;
        }

        println!("  {}: {}",
            format!("{:04}", card_list.card_odds[i]),
            card_names[i]);
    }
}

/// Find some arbitrary stuff in the ROM file and print it. This is a
/// placeholder function which is used to verify that our manipulation
/// of the image file is logically sound. It will eventually be removed
/// and replaced by functions that are more practical to work with.
pub fn print_game_data(rom_file: &Vec<u8>) {
    //TODO: Assert that rom_file size is correct.

    let slus = image::get_slus_from_bin(rom_file);
    let wa_mrg = image::get_wa_mrg_from_bin(rom_file);

    let duelist_info = get_duelist_info(&slus, &wa_mrg);
    let card_names = get_card_names(&slus);

    for d in duelist_info {
        println!("{}", d.name);
        println!("================");

        println!("Deck:");
        print_card_list(&d.deck, &card_names);
        println!("SA-POW:");
        print_card_list(&d.drops_sa_pow, &card_names);
        println!("BCD:");
        print_card_list(&d.drops_bcd, &card_names);
        println!("SA-TEC:");
        print_card_list(&d.drops_sa_tec, &card_names);
        println!();
    }
}
