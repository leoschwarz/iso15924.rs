extern crate iso15924;

fn main() {
    // Retrieve a `ScriptCode` based on its `num` value.
    iso15924::get_by_alias("070").is_some();
}
