extern crate iso15924;

fn main() {
    // Retrieve a `ScriptCode` based on its `code` value.
    iso15924::get_by_code("Blis").is_some();
    iso15924::get_by_code("Abza").is_none();
}
