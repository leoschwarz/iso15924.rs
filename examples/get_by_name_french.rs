extern crate iso15924;

fn main() {
    // Retrieve a `ScriptCode` based on its `name_french` value.
    iso15924::get_by_alias("arabe").is_some();
}
