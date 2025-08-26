pub mod duelist;
pub mod testing;
pub mod text;

mod image;

fn print_card_list(card_list: &duelist::CardList, card_names: &[String]) {
    assert!(card_names.len() == duelist::NUMBER_OF_CARDS);

    for i in 0..duelist::NUMBER_OF_CARDS {
        if card_list.card_odds[i] == 0 {
            continue;
        }

        println!(
            "  {}: {}",
            format!("{:04}", card_list.card_odds[i]),
            card_names[i]
        );
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

    let duelist_info = duelist::read_all_duelists(&slus, &wa_mrg);
    let card_names = duelist::get_card_names(&slus);

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
