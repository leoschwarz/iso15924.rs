// ISC License (ISC)
//
// Copyright (c) 2016, Austin Hellyer <hello@austinhellyer.me>
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY
// SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER
// RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF
// CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN
// CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//
// What is ISO 15924?
//
// | ISO 15924, Codes for the representation of names of scripts, defines two
// | sets of codes for a number of writing systems (scripts). Each script is
// | given both a four-letter code and a numeric one.[1] Script is defined as
// | "set of graphic characters used for the written form of one or more
// | languages".
// |
// | - [Wikipedia](http://en.wikipedia.org/wiki/ISO_15924)
//
// Originally by taiyaeix on GitHub.

#[allow(unused_imports)]
use {ScriptCode, ScriptDate};

#[allow(unused_mut)]
pub fn all<'a>() -> Vec<ScriptCode<'a>> {
    let mut list: Vec<ScriptCode> = vec![];

    // Begin.
    list.push(ScriptCode {
        alias: None,
        code: "Adlm",
        date: ScriptDate::new(2014, 11, 15).unwrap(),
        name: "Adlam",
        name_french: "adlam",
        num: "166",
    });
    list.push(ScriptCode {
        alias: None,
        code: "Afak",
        date: ScriptDate::new(2010, 12, 21).unwrap(),
        name: "Afaka",
        name_french: "afaka",
        num: "439",
    });
    list.push(ScriptCode {
        alias: Some("Caucasian​_Albanian"),
        code: "Aghb",
        date: ScriptDate::new(2014, 11, 15).unwrap(),
        name: "Caucasian Albanian",
        name_french: "aghbanien",
        num: "239",
    });
    list.push(ScriptCode {
        alias: Some("Ahom"),
        code: "Ahom",
        date: ScriptDate::new(2015, 07, 07).unwrap(),
        name: "Ahom, Tai Ahom",
        name_french: "âhom",
        num: "338",
    });
    list.push(ScriptCode {
        alias: Some("Arabic"),
        code: "Arab",
        date: ScriptDate::new(2004, 05, 01).unwrap(),
        name: "Arabic",
        name_french: "arabe",
        num: "160",
    });
    list.push(ScriptCode {
        alias: None,
        code: "Aran",
        date: ScriptDate::new(2014, 11, 15).unwrap(),
        name: "Arabic (Nastaliq variant)",
        name_french: "arabe (variante nastalique)",
        num: "161",
    });
    list.push(ScriptCode {
        alias: Some("Imperial​_Aramaic"),
        code: "Armi",
        date: ScriptDate::new(2009, 06, 01).unwrap(),
        name: "Imperial Aramaic",
        name_french: "araméen impérial",
        num: "124",
    });
    list.push(ScriptCode {
        alias: Some("Armenian"),
        code: "Armn",
        date: ScriptDate::new(2004, 05, 01).unwrap(),
        name: "Armenian",
        name_french: "arménien",
        num: "230",
    });
    list.push(ScriptCode {
        alias: Some("Avestan"),
        code: "Avst",
        date: ScriptDate::new(2009, 06, 01).unwrap(),
        name: "Avestan",
        name_french: "avestique",
        num: "134",
    });
    list.push(ScriptCode {
        alias: Some("Balinese"),
        code: "Bali",
        date: ScriptDate::new(2006, 10, 10).unwrap(),
        name: "Balinese",
        name_french: "balinais",
        num: "360",
    });
    list.push(ScriptCode {
        alias: Some("Bamum"),
        code: "Bamu",
        date: ScriptDate::new(2009, 06, 01).unwrap(),
        name: "Bamum",
        name_french: "bamoum",
        num: "435",
    });
    list.push(ScriptCode {
        alias: Some("Bassa​_Vah"),
        code: "Bass",
        date: ScriptDate::new(2014, 11, 15).unwrap(),
        name: "Bassa Vah",
        name_french: "bassa",
        num: "259",
    });
    list.push(ScriptCode {
        alias: Some("Batak"),
        code: "Batk",
        date: ScriptDate::new(2010, 07, 23).unwrap(),
        name: "Batak",
        name_french: "batik",
        num: "365",
    });
    list.push(ScriptCode {
        alias: Some("Bengali"),
        code: "Beng",
        date: ScriptDate::new(2004, 05, 01).unwrap(),
        name: "Bengali",
        name_french: "bengalî",
        num: "325",
    });
    list.push(ScriptCode {
        alias: None,
        code: "Bhks",
        date: ScriptDate::new(2015, 07, 15).unwrap(),
        name: "Bhaiksuki",
        name_french: "bhaïksukî",
        num: "334",
    });
    list.push(ScriptCode {
        alias: None,
        code: "Blis",
        date: ScriptDate::new(2004, 05, 01).unwrap(),
        name: "Blissymbols",
        name_french: "symboles Bliss",
        num: "550",
    });
    list.push(ScriptCode {
        alias: Some("Bopomofo"),
        code: "Bopo",
        date: ScriptDate::new(2004, 05, 01).unwrap(),
        name: "Bopomofo",
        name_french: "bopomofo",
        num: "285",
    });
    list.push(ScriptCode {
        alias: Some("Brahmi"),
        code: "Brah",
        date: ScriptDate::new(2010, 07, 23).unwrap(),
        name: "Brahmi",
        name_french: "brahma",
        num: "300",
    });
    list.push(ScriptCode {
        alias: Some("Braille"),
        code: "Brai",
        date: ScriptDate::new(2004, 05, 01).unwrap(),
        name: "Braille",
        name_french: "braille",
        num: "570",
    });
    list.push(ScriptCode {
        alias: Some("Buginese"),
        code: "Bugi",
        date: ScriptDate::new(2006, 06, 21).unwrap(),
        name: "Buginese",
        name_french: "bouguis",
        num: "367",
    });
    list.push(ScriptCode {
        alias: Some("Buhid"),
        code: "Buhd",
        date: ScriptDate::new(2004, 05, 01).unwrap(),
        name: "Buhid",
        name_french: "bouhide",
        num: "372",
    });
    list.push(ScriptCode {
        alias: Some("Chakma"),
        code: "Cakm",
        date: ScriptDate::new(2012, 02, 06).unwrap(),
        name: "Chakma",
        name_french: "chakma",
        num: "349",
    });
    list.push(ScriptCode {
        alias: Some("Canadian​_Aboriginal"),
        code: "Cans",
        date: ScriptDate::new(2004, 05, 29).unwrap(),
        name: "Unified Canadian Aboriginal Syllabics",
        name_french: "syllabaire autochtone canadien unifié",
        num: "440",
    });
    list.push(ScriptCode {
        alias: Some("Carian"),
        code: "Cari",
        date: ScriptDate::new(2007, 07, 02).unwrap(),
        name: "Carian",
        name_french: "carien",
        num: "201",
    });
    list.push(ScriptCode {
        alias: Some("Cham"),
        code: "Cham",
        date: ScriptDate::new(2009, 11, 11).unwrap(),
        name: "Cham",
        name_french: "cham (čam, tcham)",
        num: "358",
    });
    list.push(ScriptCode {
        alias: Some("Cherokee"),
        code: "Cher",
        date: ScriptDate::new(2004, 05, 01).unwrap(),
        name: "Cherokee",
        name_french: "tchérokî",
        num: "445",
    });
    list.push(ScriptCode {
        alias: None,
        code: "Cirt",
        date: ScriptDate::new(2004, 05, 01).unwrap(),
        name: "Cirth",
        name_french: "cirth",
        num: "291",
    });
    list.push(ScriptCode {
        alias: Some("Coptic"),
        code: "Copt",
        date: ScriptDate::new(2006, 06, 21).unwrap(),
        name: "Coptic",
        name_french: "copte",
        num: "204",
    });
    list.push(ScriptCode {
        alias: Some("Cypriot"),
        code: "Cprt",
        date: ScriptDate::new(2004, 05, 01).unwrap(),
        name: "Cypriot",
        name_french: "syllabaire chypriote",
        num: "403",
    });
    list.push(ScriptCode {
        alias: Some("Cyrillic"),
        code: "Cyrl",
        date: ScriptDate::new(2004, 05, 01).unwrap(),
        name: "Cyrillic",
        name_french: "cyrillique",
        num: "220",
    });
    list.push(ScriptCode {
        alias: None,
        code: "Cyrs",
        date: ScriptDate::new(2004, 05, 01).unwrap(),
        name: "Cyrillic (Old Church Slavonic variant)",
        name_french: "cyrillique (variante slavonne)",
        num: "221",
    });
    list.push(ScriptCode {
        alias: Some("Devanagari"),
        code: "Deva",
        date: ScriptDate::new(2004, 05, 01).unwrap(),
        name: "Devanagari (Nagari)",
        name_french: "dévanâgarî",
        num: "315",
    });
    list.push(ScriptCode {
        alias: Some("Deseret"),
        code: "Dsrt",
        date: ScriptDate::new(2004, 05, 01).unwrap(),
        name: "Deseret (Mormon)",
        name_french: "déseret (mormon)",
        num: "250",
    });
    list.push(ScriptCode {
        alias: Some("Duployan"),
        code: "Dupl",
        date: ScriptDate::new(2014, 11, 15).unwrap(),
        name: "Duployan shorthand, Duployan stenography",
        name_french: "sténographie Duployé",
        num: "755",
    });
    list.push(ScriptCode {
        alias: None,
        code: "Egyd",
        date: ScriptDate::new(2004, 05, 01).unwrap(),
        name: "Egyptian demotic",
        name_french: "démotique égyptien",
        num: "070",
    });
    list.push(ScriptCode {
        alias: None,
        code: "Egyh",
        date: ScriptDate::new(2004, 05, 01).unwrap(),
        name: "Egyptian hieratic",
        name_french: "hiératique égyptien",
        num: "060",
    });
    list.push(ScriptCode {
        alias: Some("Egyptian​_Hieroglyphs"),
        code: "Egyp",
        date: ScriptDate::new(2009, 06, 01).unwrap(),
        name: "Egyptian hieroglyphs",
        name_french: "hiéroglyphes égyptiens",
        num: "050",
    });
    list.push(ScriptCode {
        alias: Some("Elbasan"),
        code: "Elba",
        date: ScriptDate::new(2014, 11, 15).unwrap(),
        name: "Elbasan",
        name_french: "elbasan",
        num: "226",
    });
    list.push(ScriptCode {
        alias: Some("Ethiopic"),
        code: "Ethi",
        date: ScriptDate::new(2004, 10, 25).unwrap(),
        name: "Ethiopic (Geʻez)",
        name_french: "éthiopien (geʻez, guèze)",
        num: "430",
    });
    list.push(ScriptCode {
        alias: Some("Georgian"),
        code: "Geok",
        date: ScriptDate::new(2012, 10, 16).unwrap(),
        name: "Khutsuri (Asomtavruli and Nuskhuri)",
        name_french: "khoutsouri (assomtavrouli et nouskhouri)",
        num: "241",
    });
    list.push(ScriptCode {
        alias: Some("Georgian"),
        code: "Geor",
        date: ScriptDate::new(2004, 05, 29).unwrap(),
        name: "Georgian (Mkhedruli)",
        name_french: "géorgien (mkhédrouli)",
        num: "240",
    });
    list.push(ScriptCode {
        alias: Some("Glagolitic"),
        code: "Glag",
        date: ScriptDate::new(2006, 06, 21).unwrap(),
        name: "Glagolitic",
        name_french: "glagolitique",
        num: "225",
    });
    list.push(ScriptCode {
        alias: Some("Gothic"),
        code: "Goth",
        date: ScriptDate::new(2004, 05, 01).unwrap(),
        name: "Gothic",
        name_french: "gotique",
        num: "206",
    });
    list.push(ScriptCode {
        alias: Some("Grantha"),
        code: "Gran",
        date: ScriptDate::new(2014, 11, 15).unwrap(),
        name: "Grantha",
        name_french: "grantha",
        num: "343",
    });
    list.push(ScriptCode {
        alias: Some("Greek"),
        code: "Grek",
        date: ScriptDate::new(2004, 05, 01).unwrap(),
        name: "Greek",
        name_french: "grec",
        num: "200",
    });
    list.push(ScriptCode {
        alias: Some("Gujarati"),
        code: "Gujr",
        date: ScriptDate::new(2004, 05, 01).unwrap(),
        name: "Gujarati",
        name_french: "goudjarâtî (gujrâtî)",
        num: "320",
    });
    list.push(ScriptCode {
        alias: Some("Gurmukhi"),
        code: "Guru",
        date: ScriptDate::new(2004, 05, 01).unwrap(),
        name: "Gurmukhi",
        name_french: "gourmoukhî",
        num: "310",
    });
    list.push(ScriptCode {
        alias: None,
        code: "Hanb",
        date: ScriptDate::new(2016, 01, 19).unwrap(),
        name: "Han with Bopomofo (alias for Han + Bopomofo)",
        name_french: "han avec bopomofo (alias pour han + bopomofo)",
        num: "503",
    });
    list.push(ScriptCode {
        alias: Some("Hangul"),
        code: "Hang",
        date: ScriptDate::new(2004, 05, 29).unwrap(),
        name: "Hangul (Hangŭl, Hangeul)",
        name_french: "hangûl (hangŭl, hangeul)",
        num: "286",
    });
    list.push(ScriptCode {
        alias: Some("Han"),
        code: "Hani",
        date: ScriptDate::new(2009, 02, 23).unwrap(),
        name: "Han (Hanzi, Kanji, Hanja)",
        name_french: "idéogrammes han (sinogrammes)",
        num: "500",
    });
    list.push(ScriptCode {
        alias: Some("Hanunoo"),
        code: "Hano",
        date: ScriptDate::new(2004, 05, 29).unwrap(),
        name: "Hanunoo (Hanunóo)",
        name_french: "hanounóo",
        num: "371",
    });
    list.push(ScriptCode {
        alias: None,
        code: "Hans",
        date: ScriptDate::new(2004, 05, 29).unwrap(),
        name: "Han (Simplified variant)",
        name_french: "idéogrammes han (variante simplifiée)",
        num: "501",
    });
    list.push(ScriptCode {
        alias: None,
        code: "Hant",
        date: ScriptDate::new(2004, 05, 29).unwrap(),
        name: "Han (Traditional variant)",
        name_french: "idéogrammes han (variante traditionnelle)",
        num: "502",
    });
    list.push(ScriptCode {
        alias: Some("Hatran"),
        code: "Hatr",
        date: ScriptDate::new(2015, 07, 07).unwrap(),
        name: "Hatran",
        name_french: "hatrénien",
        num: "127",
    });
    list.push(ScriptCode {
        alias: Some("Hebrew"),
        code: "Hebr",
        date: ScriptDate::new(2004, 05, 01).unwrap(),
        name: "Hebrew",
        name_french: "hébreu",
        num: "125",
    });
    list.push(ScriptCode {
        alias: Some("Hiragana"),
        code: "Hira",
        date: ScriptDate::new(2004, 05, 01).unwrap(),
        name: "Hiragana",
        name_french: "hiragana",
        num: "410",
    });
    list.push(ScriptCode {
        alias: Some("Anatolian​_Hieroglyphs"),
        code: "Hluw",
        date: ScriptDate::new(2015, 07, 07).unwrap(),
        name: "Anatolian Hieroglyphs (Luwian Hieroglyphs, Hittite Hieroglyphs)",
        name_french: "hiéroglyphes anatoliens (hiéroglyphes louvites, hiéroglyphes hittites)",
        num: "080",
    });
    list.push(ScriptCode {
        alias: Some("Pahawh​_Hmong"),
        code: "Hmng",
        date: ScriptDate::new(2014, 11, 15).unwrap(),
        name: "Pahawh Hmong",
        name_french: "pahawh hmong",
        num: "450",
    });
    list.push(ScriptCode {
        alias: Some("Katakana​_Or​_Hiragana"),
        code: "Hrkt",
        date: ScriptDate::new(2011, 06, 21).unwrap(),
        name: "Japanese syllabaries (alias for Hiragana + Katakana)",
        name_french: "syllabaires japonais (alias pour hiragana + katakana)",
        num: "412",
    });
    list.push(ScriptCode {
        alias: Some("Old​_Hungarian"),
        code: "Hung",
        date: ScriptDate::new(2015, 07, 07).unwrap(),
        name: "Old Hungarian (Hungarian Runic)",
        name_french: "runes hongroises (ancien hongrois)",
        num: "176",
    });
    list.push(ScriptCode {
        alias: None,
        code: "Inds",
        date: ScriptDate::new(2004, 05, 01).unwrap(),
        name: "Indus (Harappan)",
        name_french: "indus",
        num: "610",
    });
    list.push(ScriptCode {
        alias: Some("Old​_Italic"),
        code: "Ital",
        date: ScriptDate::new(2004, 05, 29).unwrap(),
        name: "Old Italic (Etruscan, Oscan, etc.)",
        name_french: "ancien italique (étrusque, osque, etc.)",
        num: "210",
    });
    list.push(ScriptCode {
        alias: None,
        code: "Jamo",
        date: ScriptDate::new(2016, 01, 19).unwrap(),
        name: "Jamo (alias for Jamo subset of Hangul)",
        name_french: "jamo (alias pour le sous-ensemble jamo du hangûl)",
        num: "284",
    });
    list.push(ScriptCode {
        alias: Some("Javanese"),
        code: "Java",
        date: ScriptDate::new(2009, 06, 01).unwrap(),
        name: "Javanese",
        name_french: "javanais",
        num: "361",
    });
    list.push(ScriptCode {
        alias: None,
        code: "Jpan",
        date: ScriptDate::new(2006, 06, 21).unwrap(),
        name: "Japanese (alias for Han + Hiragana + Katakana)",
        name_french: "japonais (alias pour han + hiragana + katakana)",
        num: "413",
    });
    list.push(ScriptCode {
        alias: None,
        code: "Jurc",
        date: ScriptDate::new(2010, 12, 21).unwrap(),
        name: "Jurchen",
        name_french: "jurchen",
        num: "510",
    });
    list.push(ScriptCode {
        alias: Some("Kayah​_Li"),
        code: "Kali",
        date: ScriptDate::new(2007, 07, 02).unwrap(),
        name: "Kayah Li",
        name_french: "kayah li",
        num: "357",
    });
    list.push(ScriptCode {
        alias: Some("Katakana"),
        code: "Kana",
        date: ScriptDate::new(2004, 05, 01).unwrap(),
        name: "Katakana",
        name_french: "katakana",
        num: "411",
    });
    list.push(ScriptCode {
        alias: Some("Kharoshthi"),
        code: "Khar",
        date: ScriptDate::new(2006, 06, 21).unwrap(),
        name: "Kharoshthi",
        name_french: "kharochthî",
        num: "305",
    });
    list.push(ScriptCode {
        alias: Some("Khmer"),
        code: "Khmr",
        date: ScriptDate::new(2004, 05, 29).unwrap(),
        name: "Khmer",
        name_french: "khmer",
        num: "355",
    });
    list.push(ScriptCode {
        alias: Some("Khojki"),
        code: "Khoj",
        date: ScriptDate::new(2014, 11, 15).unwrap(),
        name: "Khojki",
        name_french: "khojkî",
        num: "322",
    });
    list.push(ScriptCode {
        alias: None,
        code: "Kitl",
        date: ScriptDate::new(2015, 07, 15).unwrap(),
        name: "Khitan large script",
        name_french: "grande écriture khitan",
        num: "505",
    });
    list.push(ScriptCode {
        alias: None,
        code: "Kits",
        date: ScriptDate::new(2015, 07, 15).unwrap(),
        name: "Khitan small script",
        name_french: "petite écriture khitan",
        num: "288",
    });
    list.push(ScriptCode {
        alias: Some("Kannada"),
        code: "Knda",
        date: ScriptDate::new(2004, 05, 29).unwrap(),
        name: "Kannada",
        name_french: "kannara (canara)",
        num: "345",
    });
    list.push(ScriptCode {
        alias: None,
        code: "Kore",
        date: ScriptDate::new(2007, 06, 13).unwrap(),
        name: "Korean (alias for Hangul + Han)",
        name_french: "coréen (alias pour hangûl + han)",
        num: "287",
    });
    list.push(ScriptCode {
        alias: None,
        code: "Kpel",
        date: ScriptDate::new(2010, 03, 26).unwrap(),
        name: "Kpelle",
        name_french: "kpèllé",
        num: "436",
    });
    list.push(ScriptCode {
        alias: Some("Kaithi"),
        code: "Kthi",
        date: ScriptDate::new(2009, 06, 01).unwrap(),
        name: "Kaithi",
        name_french: "kaithî",
        num: "317",
    });
    list.push(ScriptCode {
        alias: Some("Tai​_Tham"),
        code: "Lana",
        date: ScriptDate::new(2009, 06, 01).unwrap(),
        name: "Tai Tham (Lanna)",
        name_french: "taï tham (lanna)",
        num: "351",
    });
    list.push(ScriptCode {
        alias: Some("Lao"),
        code: "Laoo",
        date: ScriptDate::new(2004, 05, 01).unwrap(),
        name: "Lao",
        name_french: "laotien",
        num: "356",
    });
    list.push(ScriptCode {
        alias: None,
        code: "Latf",
        date: ScriptDate::new(2004, 05, 01).unwrap(),
        name: "Latin (Fraktur variant)",
        name_french: "latin (variante brisée)",
        num: "217",
    });
    list.push(ScriptCode {
        alias: None,
        code: "Latg",
        date: ScriptDate::new(2004, 05, 01).unwrap(),
        name: "Latin (Gaelic variant)",
        name_french: "latin (variante gaélique)",
        num: "216",
    });
    list.push(ScriptCode {
        alias: Some("Latin"),
        code: "Latn",
        date: ScriptDate::new(2004, 05, 01).unwrap(),
        name: "Latin",
        name_french: "latin",
        num: "215",
    });
    list.push(ScriptCode {
        alias: None,
        code: "Leke",
        date: ScriptDate::new(2015, 07, 07).unwrap(),
        name: "Leke",
        name_french: "léké",
        num: "364",
    });
    list.push(ScriptCode {
        alias: Some("Lepcha"),
        code: "Lepc",
        date: ScriptDate::new(2007, 07, 02).unwrap(),
        name: "Lepcha (Róng)",
        name_french: "lepcha (róng)",
        num: "335",
    });
    list.push(ScriptCode {
        alias: Some("Limbu"),
        code: "Limb",
        date: ScriptDate::new(2004, 05, 29).unwrap(),
        name: "Limbu",
        name_french: "limbou",
        num: "336",
    });
    list.push(ScriptCode {
        alias: Some("Linear​_A"),
        code: "Lina",
        date: ScriptDate::new(2014, 11, 15).unwrap(),
        name: "Linear A",
        name_french: "linéaire A",
        num: "400",
    });
    list.push(ScriptCode {
        alias: Some("Linear​_B"),
        code: "Linb",
        date: ScriptDate::new(2004, 05, 29).unwrap(),
        name: "Linear B",
        name_french: "linéaire B",
        num: "401",
    });
    list.push(ScriptCode {
        alias: Some("Lisu"),
        code: "Lisu",
        date: ScriptDate::new(2009, 06, 01).unwrap(),
        name: "Lisu (Fraser)",
        name_french: "lisu (Fraser)",
        num: "399",
    });
    list.push(ScriptCode {
        alias: None,
        code: "Loma",
        date: ScriptDate::new(2010, 03, 26).unwrap(),
        name: "Loma",
        name_french: "loma",
        num: "437",
    });
    list.push(ScriptCode {
        alias: Some("Lycian"),
        code: "Lyci",
        date: ScriptDate::new(2007, 07, 02).unwrap(),
        name: "Lycian",
        name_french: "lycien",
        num: "202",
    });
    list.push(ScriptCode {
        alias: Some("Lydian"),
        code: "Lydi",
        date: ScriptDate::new(2007, 07, 02).unwrap(),
        name: "Lydian",
        name_french: "lydien",
        num: "116",
    });
    list.push(ScriptCode {
        alias: Some("Mahajani"),
        code: "Mahj",
        date: ScriptDate::new(2014, 11, 15).unwrap(),
        name: "Mahajani",
        name_french: "mahâjanî",
        num: "314",
    });
    list.push(ScriptCode {
        alias: Some("Mandaic"),
        code: "Mand",
        date: ScriptDate::new(2010, 07, 23).unwrap(),
        name: "Mandaic, Mandaean",
        name_french: "mandéen",
        num: "140",
    });
    list.push(ScriptCode {
        alias: Some("Manichaean"),
        code: "Mani",
        date: ScriptDate::new(2014, 11, 15).unwrap(),
        name: "Manichaean",
        name_french: "manichéen",
        num: "139",
    });
    list.push(ScriptCode {
        alias: None,
        code: "Marc",
        date: ScriptDate::new(2014, 11, 15).unwrap(),
        name: "Marchen",
        name_french: "marchen",
        num: "332",
    });
    list.push(ScriptCode {
        alias: None,
        code: "Maya",
        date: ScriptDate::new(2004, 05, 01).unwrap(),
        name: "Mayan hieroglyphs",
        name_french: "hiéroglyphes mayas",
        num: "090",
    });
    list.push(ScriptCode {
        alias: Some("Mende​_Kikakui"),
        code: "Mend",
        date: ScriptDate::new(2014, 11, 15).unwrap(),
        name: "Mende Kikakui",
        name_french: "mendé kikakui",
        num: "438",
    });
    list.push(ScriptCode {
        alias: Some("Meroitic​_Cursive"),
        code: "Merc",
        date: ScriptDate::new(2012, 02, 06).unwrap(),
        name: "Meroitic Cursive",
        name_french: "cursif méroïtique",
        num: "101",
    });
    list.push(ScriptCode {
        alias: Some("Meroitic​_Hieroglyphs"),
        code: "Mero",
        date: ScriptDate::new(2012, 02, 06).unwrap(),
        name: "Meroitic Hieroglyphs",
        name_french: "hiéroglyphes méroïtiques",
        num: "100",
    });
    list.push(ScriptCode {
        alias: Some("Malayalam"),
        code: "Mlym",
        date: ScriptDate::new(2004, 05, 01).unwrap(),
        name: "Malayalam",
        name_french: "malayâlam",
        num: "347",
    });
    list.push(ScriptCode {
        alias: Some("Modi"),
        code: "Modi",
        date: ScriptDate::new(2014, 11, 15).unwrap(),
        name: "Modi, Moḍī",
        name_french: "modî",
        num: "324",
    });
    list.push(ScriptCode {
        alias: Some("Mongolian"),
        code: "Mong",
        date: ScriptDate::new(2004, 05, 01).unwrap(),
        name: "Mongolian",
        name_french: "mongol",
        num: "145",
    });
    list.push(ScriptCode {
        alias: None,
        code: "Moon",
        date: ScriptDate::new(2006, 12, 11).unwrap(),
        name: "Moon (Moon code, Moon script, Moon type)",
        name_french: "écriture Moon",
        num: "218",
    });
    list.push(ScriptCode {
        alias: Some("Mro"),
        code: "Mroo",
        date: ScriptDate::new(2014, 11, 15).unwrap(),
        name: "Mro, Mru",
        name_french: "mro",
        num: "199",
    });
    list.push(ScriptCode {
        alias: Some("Meetei​_Mayek"),
        code: "Mtei",
        date: ScriptDate::new(2009, 06, 01).unwrap(),
        name: "Meitei Mayek (Meithei, Meetei)",
        name_french: "meitei mayek",
        num: "337",
    });
    list.push(ScriptCode {
        alias: Some("Multani"),
        code: "Mult",
        date: ScriptDate::new(2015, 07, 07).unwrap(),
        name: "Multani",
        name_french: "multanî",
        num: "323",
    });
    list.push(ScriptCode {
        alias: Some("Myanmar"),
        code: "Mymr",
        date: ScriptDate::new(2004, 05, 01).unwrap(),
        name: "Myanmar (Burmese)",
        name_french: "birman",
        num: "350",
    });
    list.push(ScriptCode {
        alias: Some("Old​_North​_Arabian"),
        code: "Narb",
        date: ScriptDate::new(2014, 11, 15).unwrap(),
        name: "Old North Arabian (Ancient North Arabian)",
        name_french: "nord-arabique",
        num: "106",
    });
    list.push(ScriptCode {
        alias: Some("Nabataean"),
        code: "Nbat",
        date: ScriptDate::new(2014, 11, 15).unwrap(),
        name: "Nabataean",
        name_french: "nabatéen",
        num: "159",
    });
    list.push(ScriptCode {
        alias: None,
        code: "Newa",
        date: ScriptDate::new(2015, 12, 16).unwrap(),
        name: "Newa, Newar, Newari, Nepāla lipi",
        name_french: "néwa, néwar, néwari, nepāla lipi",
        num: "333",
    });
    list.push(ScriptCode {
        alias: None,
        code: "Nkgb",
        date: ScriptDate::new(2009, 02, 23).unwrap(),
        name: "Nakhi Geba ('Na-'Khi ²Ggŏ-¹baw, Naxi Geba)",
        name_french: "nakhi géba",
        num: "420",
    });
    list.push(ScriptCode {
        alias: Some("Nko"),
        code: "Nkoo",
        date: ScriptDate::new(2006, 10, 10).unwrap(),
        name: "N’Ko",
        name_french: "n’ko",
        num: "165",
    });
    list.push(ScriptCode {
        alias: None,
        code: "Nshu",
        date: ScriptDate::new(2010, 12, 21).unwrap(),
        name: "Nüshu",
        name_french: "nüshu",
        num: "499",
    });
    list.push(ScriptCode {
        alias: Some("Ogham"),
        code: "Ogam",
        date: ScriptDate::new(2004, 05, 01).unwrap(),
        name: "Ogham",
        name_french: "ogam",
        num: "212",
    });
    list.push(ScriptCode {
        alias: Some("Ol​_Chiki"),
        code: "Olck",
        date: ScriptDate::new(2007, 07, 02).unwrap(),
        name: "Ol Chiki (Ol Cemet’, Ol, Santali)",
        name_french: "ol tchiki",
        num: "261",
    });
    list.push(ScriptCode {
        alias: Some("Old​_Turkic"),
        code: "Orkh",
        date: ScriptDate::new(2009, 06, 01).unwrap(),
        name: "Old Turkic, Orkhon Runic",
        name_french: "orkhon",
        num: "175",
    });
    list.push(ScriptCode {
        alias: Some("Oriya"),
        code: "Orya",
        date: ScriptDate::new(2004, 05, 01).unwrap(),
        name: "Oriya",
        name_french: "oriyâ",
        num: "327",
    });
    list.push(ScriptCode {
        alias: None,
        code: "Osge",
        date: ScriptDate::new(2014, 11, 15).unwrap(),
        name: "Osage",
        name_french: "osage",
        num: "219",
    });
    list.push(ScriptCode {
        alias: Some("Osmanya"),
        code: "Osma",
        date: ScriptDate::new(2004, 05, 01).unwrap(),
        name: "Osmanya",
        name_french: "osmanais",
        num: "260",
    });
    list.push(ScriptCode {
        alias: Some("Palmyrene"),
        code: "Palm",
        date: ScriptDate::new(2014, 11, 15).unwrap(),
        name: "Palmyrene",
        name_french: "palmyrénien",
        num: "126",
    });
    list.push(ScriptCode {
        alias: Some("Pau​_Cin​_Hau"),
        code: "Pauc",
        date: ScriptDate::new(2014, 11, 15).unwrap(),
        name: "Pau Cin Hau",
        name_french: "paou chin haou",
        num: "263",
    });
    list.push(ScriptCode {
        alias: Some("Old​_Permic"),
        code: "Perm",
        date: ScriptDate::new(2014, 11, 15).unwrap(),
        name: "Old Permic",
        name_french: "ancien permien",
        num: "227",
    });
    list.push(ScriptCode {
        alias: Some("Phags​_Pa"),
        code: "Phag",
        date: ScriptDate::new(2006, 10, 10).unwrap(),
        name: "Phags-pa",
        name_french: "’phags pa",
        num: "331",
    });
    list.push(ScriptCode {
        alias: Some("Inscriptional​_Pahlavi"),
        code: "Phli",
        date: ScriptDate::new(2009, 06, 01).unwrap(),
        name: "Inscriptional Pahlavi",
        name_french: "pehlevi des inscriptions",
        num: "131",
    });
    list.push(ScriptCode {
        alias: Some("Psalter​_Pahlavi"),
        code: "Phlp",
        date: ScriptDate::new(2014, 11, 15).unwrap(),
        name: "Psalter Pahlavi",
        name_french: "pehlevi des psautiers",
        num: "132",
    });
    list.push(ScriptCode {
        alias: None,
        code: "Phlv",
        date: ScriptDate::new(2007, 07, 15).unwrap(),
        name: "Book Pahlavi",
        name_french: "pehlevi des livres",
        num: "133",
    });
    list.push(ScriptCode {
        alias: Some("Phoenician"),
        code: "Phnx",
        date: ScriptDate::new(2006, 10, 10).unwrap(),
        name: "Phoenician",
        name_french: "phénicien",
        num: "115",
    });
    list.push(ScriptCode {
        alias: None,
        code: "Piqd",
        date: ScriptDate::new(2015, 12, 16).unwrap(),
        name: "Klingon (KLI pIqaD)",
        name_french: "klingon (pIqaD du KLI)",
        num: "293",
    });
    list.push(ScriptCode {
        alias: Some("Miao"),
        code: "Plrd",
        date: ScriptDate::new(2012, 02, 06).unwrap(),
        name: "Miao (Pollard)",
        name_french: "miao (Pollard)",
        num: "282",
    });
    list.push(ScriptCode {
        alias: Some("Inscriptional​_Parthian"),
        code: "Prti",
        date: ScriptDate::new(2009, 06, 01).unwrap(),
        name: "Inscriptional Parthian",
        name_french: "parthe des inscriptions",
        num: "130",
    });
    list.push(ScriptCode {
        alias: None,
        code: "Qaaa",
        date: ScriptDate::new(2004, 05, 29).unwrap(),
        name: "Reserved for private use (start)",
        name_french: "réservé à l’usage privé (début)",
        num: "900",
    });
    list.push(ScriptCode {
        alias: None,
        code: "Qabx",
        date: ScriptDate::new(2004, 05, 29).unwrap(),
        name: "Reserved for private use (end)",
        name_french: "réservé à l’usage privé (fin)",
        num: "949",
    });
    list.push(ScriptCode {
        alias: Some("Rejang"),
        code: "Rjng",
        date: ScriptDate::new(2009, 02, 23).unwrap(),
        name: "Rejang (Redjang, Kaganga)",
        name_french: "redjang (kaganga)",
        num: "363",
    });
    list.push(ScriptCode {
        alias: None,
        code: "Roro",
        date: ScriptDate::new(2004, 05, 01).unwrap(),
        name: "Rongorongo",
        name_french: "rongorongo",
        num: "620",
    });
    list.push(ScriptCode {
        alias: Some("Runic"),
        code: "Runr",
        date: ScriptDate::new(2004, 05, 01).unwrap(),
        name: "Runic",
        name_french: "runique",
        num: "211",
    });
    list.push(ScriptCode {
        alias: Some("Samaritan"),
        code: "Samr",
        date: ScriptDate::new(2009, 06, 01).unwrap(),
        name: "Samaritan",
        name_french: "samaritain",
        num: "123",
    });
    list.push(ScriptCode {
        alias: None,
        code: "Sara",
        date: ScriptDate::new(2004, 05, 29).unwrap(),
        name: "Sarati",
        name_french: "sarati",
        num: "292",
    });
    list.push(ScriptCode {
        alias: Some("Old​_South​_Arabian"),
        code: "Sarb",
        date: ScriptDate::new(2009, 06, 01).unwrap(),
        name: "Old South Arabian",
        name_french: "sud-arabique, himyarite",
        num: "105",
    });
    list.push(ScriptCode {
        alias: Some("Saurashtra"),
        code: "Saur",
        date: ScriptDate::new(2007, 07, 02).unwrap(),
        name: "Saurashtra",
        name_french: "saurachtra",
        num: "344",
    });
    list.push(ScriptCode {
        alias: Some("SignWriting"),
        code: "Sgnw",
        date: ScriptDate::new(2015, 07, 07).unwrap(),
        name: "SignWriting",
        name_french: "SignÉcriture, SignWriting",
        num: "095",
    });
    list.push(ScriptCode {
        alias: Some("Shavian"),
        code: "Shaw",
        date: ScriptDate::new(2004, 05, 01).unwrap(),
        name: "Shavian (Shaw)",
        name_french: "shavien (Shaw)",
        num: "281",
    });
    list.push(ScriptCode {
        alias: Some("Sharada"),
        code: "Shrd",
        date: ScriptDate::new(2012, 02, 06).unwrap(),
        name: "Sharada, Śāradā",
        name_french: "charada, shard",
        num: "319",
    });
    list.push(ScriptCode {
        alias: Some("Siddham"),
        code: "Sidd",
        date: ScriptDate::new(2014, 11, 15).unwrap(),
        name: "Siddham, Siddhaṃ, Siddhamātṛkā",
        name_french: "siddham",
        num: "302",
    });
    list.push(ScriptCode {
        alias: Some("Khudawadi"),
        code: "Sind",
        date: ScriptDate::new(2014, 11, 15).unwrap(),
        name: "Khudawadi, Sindhi",
        name_french: "khoudawadî, sindhî",
        num: "318",
    });
    list.push(ScriptCode {
        alias: Some("Sinhala"),
        code: "Sinh",
        date: ScriptDate::new(2004, 05, 01).unwrap(),
        name: "Sinhala",
        name_french: "singhalais",
        num: "348",
    });
    list.push(ScriptCode {
        alias: Some("Sora​_Sompeng"),
        code: "Sora",
        date: ScriptDate::new(2012, 02, 06).unwrap(),
        name: "Sora Sompeng",
        name_french: "sora sompeng",
        num: "398",
    });
    list.push(ScriptCode {
        alias: Some("Sundanese"),
        code: "Sund",
        date: ScriptDate::new(2007, 07, 02).unwrap(),
        name: "Sundanese",
        name_french: "sundanais",
        num: "362",
    });
    list.push(ScriptCode {
        alias: Some("Syloti​_Nagri"),
        code: "Sylo",
        date: ScriptDate::new(2006, 06, 21).unwrap(),
        name: "Syloti Nagri",
        name_french: "sylotî nâgrî",
        num: "316",
    });
    list.push(ScriptCode {
        alias: Some("Syriac"),
        code: "Syrc",
        date: ScriptDate::new(2004, 05, 01).unwrap(),
        name: "Syriac",
        name_french: "syriaque",
        num: "135",
    });
    list.push(ScriptCode {
        alias: None,
        code: "Syre",
        date: ScriptDate::new(2004, 05, 01).unwrap(),
        name: "Syriac (Estrangelo variant)",
        name_french: "syriaque (variante estranghélo)",
        num: "138",
    });
    list.push(ScriptCode {
        alias: None,
        code: "Syrj",
        date: ScriptDate::new(2004, 05, 01).unwrap(),
        name: "Syriac (Western variant)",
        name_french: "syriaque (variante occidentale)",
        num: "137",
    });
    list.push(ScriptCode {
        alias: None,
        code: "Syrn",
        date: ScriptDate::new(2004, 05, 01).unwrap(),
        name: "Syriac (Eastern variant)",
        name_french: "syriaque (variante orientale)",
        num: "136",
    });
    list.push(ScriptCode {
        alias: Some("Tagbanwa"),
        code: "Tagb",
        date: ScriptDate::new(2004, 05, 01).unwrap(),
        name: "Tagbanwa",
        name_french: "tagbanoua",
        num: "373",
    });
    list.push(ScriptCode {
        alias: Some("Takri"),
        code: "Takr",
        date: ScriptDate::new(2012, 02, 06).unwrap(),
        name: "Takri, Ṭākrī, Ṭāṅkrī",
        name_french: "tâkrî",
        num: "321",
    });
    list.push(ScriptCode {
        alias: Some("Tai​_Le"),
        code: "Tale",
        date: ScriptDate::new(2004, 10, 25).unwrap(),
        name: "Tai Le",
        name_french: "taï-le",
        num: "353",
    });
    list.push(ScriptCode {
        alias: Some("New​_Tai​_Lue"),
        code: "Talu",
        date: ScriptDate::new(2006, 06, 21).unwrap(),
        name: "New Tai Lue",
        name_french: "nouveau taï-lue",
        num: "354",
    });
    list.push(ScriptCode {
        alias: Some("Tamil"),
        code: "Taml",
        date: ScriptDate::new(2004, 05, 01).unwrap(),
        name: "Tamil",
        name_french: "tamoul",
        num: "346",
    });
    list.push(ScriptCode {
        alias: None,
        code: "Tang",
        date: ScriptDate::new(2010, 12, 21).unwrap(),
        name: "Tangut",
        name_french: "tangoute",
        num: "520",
    });
    list.push(ScriptCode {
        alias: Some("Tai​_Viet"),
        code: "Tavt",
        date: ScriptDate::new(2009, 06, 01).unwrap(),
        name: "Tai Viet",
        name_french: "taï viêt",
        num: "359",
    });
    list.push(ScriptCode {
        alias: Some("Telugu"),
        code: "Telu",
        date: ScriptDate::new(2004, 05, 01).unwrap(),
        name: "Telugu",
        name_french: "télougou",
        num: "340",
    });
    list.push(ScriptCode {
        alias: None,
        code: "Teng",
        date: ScriptDate::new(2004, 05, 01).unwrap(),
        name: "Tengwar",
        name_french: "tengwar",
        num: "290",
    });
    list.push(ScriptCode {
        alias: Some("Tifinagh"),
        code: "Tfng",
        date: ScriptDate::new(2006, 06, 21).unwrap(),
        name: "Tifinagh (Berber)",
        name_french: "tifinagh (berbère)",
        num: "120",
    });
    list.push(ScriptCode {
        alias: Some("Tagalog"),
        code: "Tglg",
        date: ScriptDate::new(2009, 02, 23).unwrap(),
        name: "Tagalog (Baybayin, Alibata)",
        name_french: "tagal (baybayin, alibata)",
        num: "370",
    });
    list.push(ScriptCode {
        alias: Some("Thaana"),
        code: "Thaa",
        date: ScriptDate::new(2004, 05, 01).unwrap(),
        name: "Thaana",
        name_french: "thâna",
        num: "170",
    });
    list.push(ScriptCode {
        alias: Some("Thai"),
        code: "Thai",
        date: ScriptDate::new(2004, 05, 01).unwrap(),
        name: "Thai",
        name_french: "thaï",
        num: "352",
    });
    list.push(ScriptCode {
        alias: Some("Tibetan"),
        code: "Tibt",
        date: ScriptDate::new(2004, 05, 01).unwrap(),
        name: "Tibetan",
        name_french: "tibétain",
        num: "330",
    });
    list.push(ScriptCode {
        alias: Some("Tirhuta"),
        code: "Tirh",
        date: ScriptDate::new(2014, 11, 15).unwrap(),
        name: "Tirhuta",
        name_french: "tirhouta",
        num: "326",
    });
    list.push(ScriptCode {
        alias: Some("Ugaritic"),
        code: "Ugar",
        date: ScriptDate::new(2004, 05, 01).unwrap(),
        name: "Ugaritic",
        name_french: "ougaritique",
        num: "040",
    });
    list.push(ScriptCode {
        alias: Some("Vai"),
        code: "Vaii",
        date: ScriptDate::new(2007, 07, 02).unwrap(),
        name: "Vai",
        name_french: "vaï",
        num: "470",
    });
    list.push(ScriptCode {
        alias: None,
        code: "Visp",
        date: ScriptDate::new(2004, 05, 01).unwrap(),
        name: "Visible Speech",
        name_french: "parole visible",
        num: "280",
    });
    list.push(ScriptCode {
        alias: Some("Warang​_Citi"),
        code: "Wara",
        date: ScriptDate::new(2014, 11, 15).unwrap(),
        name: "Warang Citi (Varang Kshiti)",
        name_french: "warang citi",
        num: "262",
    });
    list.push(ScriptCode {
        alias: None,
        code: "Wole",
        date: ScriptDate::new(2010, 12, 21).unwrap(),
        name: "Woleai",
        name_french: "woléaï",
        num: "480",
    });
    list.push(ScriptCode {
        alias: Some("Old​_Persian"),
        code: "Xpeo",
        date: ScriptDate::new(2006, 06, 21).unwrap(),
        name: "Old Persian",
        name_french: "cunéiforme persépolitain",
        num: "030",
    });
    list.push(ScriptCode {
        alias: Some("Cuneiform"),
        code: "Xsux",
        date: ScriptDate::new(2006, 10, 10).unwrap(),
        name: "Cuneiform, Sumero-Akkadian",
        name_french: "cunéiforme suméro-akkadien",
        num: "020",
    });
    list.push(ScriptCode {
        alias: Some("Yi"),
        code: "Yiii",
        date: ScriptDate::new(2004, 05, 01).unwrap(),
        name: "Yi",
        name_french: "yi",
        num: "460",
    });
    list.push(ScriptCode {
        alias: Some("Inherited"),
        code: "Zinh",
        date: ScriptDate::new(2009, 02, 23).unwrap(),
        name: "Code for inherited script",
        name_french: "codet pour écriture héritée",
        num: "994",
    });
    list.push(ScriptCode {
        alias: None,
        code: "Zmth",
        date: ScriptDate::new(2007, 11, 26).unwrap(),
        name: "Mathematical notation",
        name_french: "notation mathématique",
        num: "995",
    });
    list.push(ScriptCode {
        alias: None,
        code: "Zsye",
        date: ScriptDate::new(2015, 12, 16).unwrap(),
        name: "Symbols (Emoji variant)",
        name_french: "symboles (variante émoji)",
        num: "993",
    });
    list.push(ScriptCode {
        alias: None,
        code: "Zsym",
        date: ScriptDate::new(2007, 11, 26).unwrap(),
        name: "Symbols",
        name_french: "symboles",
        num: "996",
    });
    list.push(ScriptCode {
        alias: None,
        code: "Zxxx",
        date: ScriptDate::new(2011, 06, 21).unwrap(),
        name: "Code for unwritten documents",
        name_french: "codet pour les documents non écrits",
        num: "997",
    });
    list.push(ScriptCode {
        alias: Some("Common"),
        code: "Zyyy",
        date: ScriptDate::new(2004, 05, 29).unwrap(),
        name: "Code for undetermined script",
        name_french: "codet pour écriture indéterminée",
        num: "998",
    });
    list.push(ScriptCode {
        alias: Some("Unknown"),
        code: "Zzzz",
        date: ScriptDate::new(2006, 10, 10).unwrap(),
        name: "Code for uncoded script",
        name_french: "codet pour écriture non codée",
        num: "999",
    });
    // End.

    list
}
