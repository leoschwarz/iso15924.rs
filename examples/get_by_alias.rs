extern crate iso15924;

fn main() {
    // Retrieve a `ScriptCode` based on its `alias` value.
    iso15924::get_by_alias("Ahom").is_some();
}
