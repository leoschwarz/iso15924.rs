extern crate iso15924;

fn main() {
    // Retrieve a `ScriptCode` based on its `name` value.
    iso15924::get_by_name("Adlam").is_some();
    iso15924::get_by_name("Aaaaa").is_none();
}
