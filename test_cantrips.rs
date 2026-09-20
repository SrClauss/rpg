fn main() {
    let json_data = std::fs::read_to_string("assets/srd_cantrips.json").unwrap();
    // we need to call load_homebrew_spells, but since this is inside the crate we can just run cargo test
}
