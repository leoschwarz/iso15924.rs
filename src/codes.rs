// ISC License (ISC)
//
// Copyright (c) 2016, Zeyla Hellyer <zeylahellyer@gmail.com>
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
// Originally by zeyla on GitHub.

#[allow(unused_imports)]
use {ScriptCode, ScriptDate};

const SCRIPTS: &'static [ScriptCode] = &[
    // Begin.
    ScriptCode {
        alias: Some("Adlam"),
        code: "Adlm",
        date: ScriptDate {
            year: 2016,
            month: 12,
            day: 05
        },
        name: "Adlam",
        name_french: "adlam",
        num: "166",
    },
    ScriptCode {
        alias: None,
        code: "Afak",
        date: ScriptDate {
            year: 2010,
            month: 12,
            day: 21
        },
        name: "Afaka",
        name_french: "afaka",
        num: "439",
    },
    ScriptCode {
        alias: Some("Caucasian_Albanian"),
        code: "Aghb",
        date: ScriptDate {
            year: 2014,
            month: 11,
            day: 15
        },
        name: "Caucasian Albanian",
        name_french: "aghbanien",
        num: "239",
    },
    ScriptCode {
        alias: Some("Ahom"),
        code: "Ahom",
        date: ScriptDate {
            year: 2015,
            month: 07,
            day: 07
        },
        name: "Ahom, Tai Ahom",
        name_french: "âhom",
        num: "338",
    },
    ScriptCode {
        alias: Some("Arabic"),
        code: "Arab",
        date: ScriptDate {
            year: 2004,
            month: 05,
            day: 01
        },
        name: "Arabic",
        name_french: "arabe",
        num: "160",
    },
    ScriptCode {
        alias: None,
        code: "Aran",
        date: ScriptDate {
            year: 2014,
            month: 11,
            day: 15
        },
        name: "Arabic (Nastaliq variant)",
        name_french: "arabe (variante nastalique)",
        num: "161",
    },
    ScriptCode {
        alias: Some("Imperial_Aramaic"),
        code: "Armi",
        date: ScriptDate {
            year: 2009,
            month: 06,
            day: 01
        },
        name: "Imperial Aramaic",
        name_french: "araméen impérial",
        num: "124",
    },
    ScriptCode {
        alias: Some("Armenian"),
        code: "Armn",
        date: ScriptDate {
            year: 2004,
            month: 05,
            day: 01
        },
        name: "Armenian",
        name_french: "arménien",
        num: "230",
    },
    ScriptCode {
        alias: Some("Avestan"),
        code: "Avst",
        date: ScriptDate {
            year: 2009,
            month: 06,
            day: 01
        },
        name: "Avestan",
        name_french: "avestique",
        num: "134",
    },
    ScriptCode {
        alias: Some("Balinese"),
        code: "Bali",
        date: ScriptDate {
            year: 2006,
            month: 10,
            day: 10
        },
        name: "Balinese",
        name_french: "balinais",
        num: "360",
    },
    ScriptCode {
        alias: Some("Bamum"),
        code: "Bamu",
        date: ScriptDate {
            year: 2009,
            month: 06,
            day: 01
        },
        name: "Bamum",
        name_french: "bamoum",
        num: "435",
    },
    ScriptCode {
        alias: Some("Bassa_Vah"),
        code: "Bass",
        date: ScriptDate {
            year: 2014,
            month: 11,
            day: 15
        },
        name: "Bassa Vah",
        name_french: "bassa",
        num: "259",
    },
    ScriptCode {
        alias: Some("Batak"),
        code: "Batk",
        date: ScriptDate {
            year: 2010,
            month: 07,
            day: 23
        },
        name: "Batak",
        name_french: "batik",
        num: "365",
    },
    ScriptCode {
        alias: Some("Bengali"),
        code: "Beng",
        date: ScriptDate {
            year: 2016,
            month: 12,
            day: 05
        },
        name: "Bengali (Bangla)",
        name_french: "bengalî (bangla)",
        num: "325",
    },
    ScriptCode {
        alias: Some("Bhaiksuki"),
        code: "Bhks",
        date: ScriptDate {
            year: 2016,
            month: 12,
            day: 05
        },
        name: "Bhaiksuki",
        name_french: "bhaïksukî",
        num: "334",
    },
    ScriptCode {
        alias: None,
        code: "Blis",
        date: ScriptDate {
            year: 2004,
            month: 05,
            day: 01
        },
        name: "Blissymbols",
        name_french: "symboles Bliss",
        num: "550",
    },
    ScriptCode {
        alias: Some("Bopomofo"),
        code: "Bopo",
        date: ScriptDate {
            year: 2004,
            month: 05,
            day: 01
        },
        name: "Bopomofo",
        name_french: "bopomofo",
        num: "285",
    },
    ScriptCode {
        alias: Some("Brahmi"),
        code: "Brah",
        date: ScriptDate {
            year: 2010,
            month: 07,
            day: 23
        },
        name: "Brahmi",
        name_french: "brahma",
        num: "300",
    },
    ScriptCode {
        alias: Some("Braille"),
        code: "Brai",
        date: ScriptDate {
            year: 2004,
            month: 05,
            day: 01
        },
        name: "Braille",
        name_french: "braille",
        num: "570",
    },
    ScriptCode {
        alias: Some("Buginese"),
        code: "Bugi",
        date: ScriptDate {
            year: 2006,
            month: 06,
            day: 21
        },
        name: "Buginese",
        name_french: "bouguis",
        num: "367",
    },
    ScriptCode {
        alias: Some("Buhid"),
        code: "Buhd",
        date: ScriptDate {
            year: 2004,
            month: 05,
            day: 01
        },
        name: "Buhid",
        name_french: "bouhide",
        num: "372",
    },
    ScriptCode {
        alias: Some("Chakma"),
        code: "Cakm",
        date: ScriptDate {
            year: 2012,
            month: 02,
            day: 06
        },
        name: "Chakma",
        name_french: "chakma",
        num: "349",
    },
    ScriptCode {
        alias: Some("Canadian_Aboriginal"),
        code: "Cans",
        date: ScriptDate {
            year: 2004,
            month: 05,
            day: 29
        },
        name: "Unified Canadian Aboriginal Syllabics",
        name_french: "syllabaire autochtone canadien unifié",
        num: "440",
    },
    ScriptCode {
        alias: Some("Carian"),
        code: "Cari",
        date: ScriptDate {
            year: 2007,
            month: 07,
            day: 02
        },
        name: "Carian",
        name_french: "carien",
        num: "201",
    },
    ScriptCode {
        alias: Some("Cham"),
        code: "Cham",
        date: ScriptDate {
            year: 2009,
            month: 11,
            day: 11
        },
        name: "Cham",
        name_french: "cham (čam, tcham)",
        num: "358",
    },
    ScriptCode {
        alias: Some("Cherokee"),
        code: "Cher",
        date: ScriptDate {
            year: 2004,
            month: 05,
            day: 01
        },
        name: "Cherokee",
        name_french: "tchérokî",
        num: "445",
    },
    ScriptCode {
        alias: None,
        code: "Cirt",
        date: ScriptDate {
            year: 2004,
            month: 05,
            day: 01
        },
        name: "Cirth",
        name_french: "cirth",
        num: "291",
    },
    ScriptCode {
        alias: Some("Coptic"),
        code: "Copt",
        date: ScriptDate {
            year: 2006,
            month: 06,
            day: 21
        },
        name: "Coptic",
        name_french: "copte",
        num: "204",
    },
    ScriptCode {
        alias: None,
        code: "Cpmn",
        date: ScriptDate {
            year: 2017,
            month: 07,
            day: 26
        },
        name: "Cypro-Minoan",
        name_french: "syllabaire chypro-minoen",
        num: "402",
    },
    ScriptCode {
        alias: Some("Cypriot"),
        code: "Cprt",
        date: ScriptDate {
            year: 2017,
            month: 07,
            day: 26
        },
        name: "Cypriot syllabary",
        name_french: "syllabaire chypriote",
        num: "403",
    },
    ScriptCode {
        alias: Some("Cyrillic"),
        code: "Cyrl",
        date: ScriptDate {
            year: 2004,
            month: 05,
            day: 01
        },
        name: "Cyrillic",
        name_french: "cyrillique",
        num: "220",
    },
    ScriptCode {
        alias: None,
        code: "Cyrs",
        date: ScriptDate {
            year: 2004,
            month: 05,
            day: 01
        },
        name: "Cyrillic (Old Church Slavonic variant)",
        name_french: "cyrillique (variante slavonne)",
        num: "221",
    },
    ScriptCode {
        alias: Some("Devanagari"),
        code: "Deva",
        date: ScriptDate {
            year: 2004,
            month: 05,
            day: 01
        },
        name: "Devanagari (Nagari)",
        name_french: "dévanâgarî",
        num: "315",
    },
    ScriptCode {
        alias: Some("Dogra"),
        code: "Dogr",
        date: ScriptDate {
            year: 2016,
            month: 12,
            day: 05
        },
        name: "Dogra",
        name_french: "dogra",
        num: "328",
    },
    ScriptCode {
        alias: Some("Deseret"),
        code: "Dsrt",
        date: ScriptDate {
            year: 2004,
            month: 05,
            day: 01
        },
        name: "Deseret (Mormon)",
        name_french: "déseret (mormon)",
        num: "250",
    },
    ScriptCode {
        alias: Some("Duployan"),
        code: "Dupl",
        date: ScriptDate {
            year: 2014,
            month: 11,
            day: 15
        },
        name: "Duployan shorthand, Duployan stenography",
        name_french: "sténographie Duployé",
        num: "755",
    },
    ScriptCode {
        alias: None,
        code: "Egyd",
        date: ScriptDate {
            year: 2004,
            month: 05,
            day: 01
        },
        name: "Egyptian demotic",
        name_french: "démotique égyptien",
        num: "070",
    },
    ScriptCode {
        alias: None,
        code: "Egyh",
        date: ScriptDate {
            year: 2004,
            month: 05,
            day: 01
        },
        name: "Egyptian hieratic",
        name_french: "hiératique égyptien",
        num: "060",
    },
    ScriptCode {
        alias: Some("Egyptian_Hieroglyphs"),
        code: "Egyp",
        date: ScriptDate {
            year: 2009,
            month: 06,
            day: 01
        },
        name: "Egyptian hieroglyphs",
        name_french: "hiéroglyphes égyptiens",
        num: "050",
    },
    ScriptCode {
        alias: Some("Elbasan"),
        code: "Elba",
        date: ScriptDate {
            year: 2014,
            month: 11,
            day: 15
        },
        name: "Elbasan",
        name_french: "elbasan",
        num: "226",
    },
    ScriptCode {
        alias: Some("Elymaic"),
        code: "Elym",
        date: ScriptDate {
            year: 2018,
            month: 08,
            day: 26
        },
        name: "Elymaic",
        name_french: "élymaïque",
        num: "128",
    },
    ScriptCode {
        alias: Some("Ethiopic"),
        code: "Ethi",
        date: ScriptDate {
            year: 2004,
            month: 10,
            day: 25
        },
        name: "Ethiopic (Geʻez)",
        name_french: "éthiopien (geʻez, guèze)",
        num: "430",
    },
    ScriptCode {
        alias: Some("Georgian"),
        code: "Geok",
        date: ScriptDate {
            year: 2012,
            month: 10,
            day: 16
        },
        name: "Khutsuri (Asomtavruli and Nuskhuri)",
        name_french: "khoutsouri (assomtavrouli et nouskhouri)",
        num: "241",
    },
    ScriptCode {
        alias: Some("Georgian"),
        code: "Geor",
        date: ScriptDate {
            year: 2016,
            month: 12,
            day: 05
        },
        name: "Georgian (Mkhedruli and Mtavruli)",
        name_french: "géorgien (mkhédrouli et mtavrouli)",
        num: "240",
    },
    ScriptCode {
        alias: Some("Glagolitic"),
        code: "Glag",
        date: ScriptDate {
            year: 2006,
            month: 06,
            day: 21
        },
        name: "Glagolitic",
        name_french: "glagolitique",
        num: "225",
    },
    ScriptCode {
        alias: Some("Gunjala_Gondi"),
        code: "Gong",
        date: ScriptDate {
            year: 2016,
            month: 12,
            day: 05
        },
        name: "Gunjala Gondi",
        name_french: "gunjala gondî",
        num: "312",
    },
    ScriptCode {
        alias: Some("Masaram_Gondi"),
        code: "Gonm",
        date: ScriptDate {
            year: 2017,
            month: 07,
            day: 26
        },
        name: "Masaram Gondi",
        name_french: "masaram gondî",
        num: "313",
    },
    ScriptCode {
        alias: Some("Gothic"),
        code: "Goth",
        date: ScriptDate {
            year: 2004,
            month: 05,
            day: 01
        },
        name: "Gothic",
        name_french: "gotique",
        num: "206",
    },
    ScriptCode {
        alias: Some("Grantha"),
        code: "Gran",
        date: ScriptDate {
            year: 2014,
            month: 11,
            day: 15
        },
        name: "Grantha",
        name_french: "grantha",
        num: "343",
    },
    ScriptCode {
        alias: Some("Greek"),
        code: "Grek",
        date: ScriptDate {
            year: 2004,
            month: 05,
            day: 01
        },
        name: "Greek",
        name_french: "grec",
        num: "200",
    },
    ScriptCode {
        alias: Some("Gujarati"),
        code: "Gujr",
        date: ScriptDate {
            year: 2004,
            month: 05,
            day: 01
        },
        name: "Gujarati",
        name_french: "goudjarâtî (gujrâtî)",
        num: "320",
    },
    ScriptCode {
        alias: Some("Gurmukhi"),
        code: "Guru",
        date: ScriptDate {
            year: 2004,
            month: 05,
            day: 01
        },
        name: "Gurmukhi",
        name_french: "gourmoukhî",
        num: "310",
    },
    ScriptCode {
        alias: None,
        code: "Hanb",
        date: ScriptDate {
            year: 2016,
            month: 01,
            day: 19
        },
        name: "Han with Bopomofo (alias for Han + Bopomofo)",
        name_french: "han avec bopomofo (alias pour han + bopomofo)",
        num: "503",
    },
    ScriptCode {
        alias: Some("Hangul"),
        code: "Hang",
        date: ScriptDate {
            year: 2004,
            month: 05,
            day: 29
        },
        name: "Hangul (Hangŭl, Hangeul)",
        name_french: "hangûl (hangŭl, hangeul)",
        num: "286",
    },
    ScriptCode {
        alias: Some("Han"),
        code: "Hani",
        date: ScriptDate {
            year: 2009,
            month: 02,
            day: 23
        },
        name: "Han (Hanzi, Kanji, Hanja)",
        name_french: "idéogrammes han (sinogrammes)",
        num: "500",
    },
    ScriptCode {
        alias: Some("Hanunoo"),
        code: "Hano",
        date: ScriptDate {
            year: 2004,
            month: 05,
            day: 29
        },
        name: "Hanunoo (Hanunóo)",
        name_french: "hanounóo",
        num: "371",
    },
    ScriptCode {
        alias: None,
        code: "Hans",
        date: ScriptDate {
            year: 2004,
            month: 05,
            day: 29
        },
        name: "Han (Simplified variant)",
        name_french: "idéogrammes han (variante simplifiée)",
        num: "501",
    },
    ScriptCode {
        alias: None,
        code: "Hant",
        date: ScriptDate {
            year: 2004,
            month: 05,
            day: 29
        },
        name: "Han (Traditional variant)",
        name_french: "idéogrammes han (variante traditionnelle)",
        num: "502",
    },
    ScriptCode {
        alias: Some("Hatran"),
        code: "Hatr",
        date: ScriptDate {
            year: 2015,
            month: 07,
            day: 07
        },
        name: "Hatran",
        name_french: "hatrénien",
        num: "127",
    },
    ScriptCode {
        alias: Some("Hebrew"),
        code: "Hebr",
        date: ScriptDate {
            year: 2004,
            month: 05,
            day: 01
        },
        name: "Hebrew",
        name_french: "hébreu",
        num: "125",
    },
    ScriptCode {
        alias: Some("Hiragana"),
        code: "Hira",
        date: ScriptDate {
            year: 2004,
            month: 05,
            day: 01
        },
        name: "Hiragana",
        name_french: "hiragana",
        num: "410",
    },
    ScriptCode {
        alias: Some("Anatolian_Hieroglyphs"),
        code: "Hluw",
        date: ScriptDate {
            year: 2015,
            month: 07,
            day: 07
        },
        name: "Anatolian Hieroglyphs (Luwian Hieroglyphs, Hittite Hieroglyphs)",
        name_french: "hiéroglyphes anatoliens (hiéroglyphes louvites, hiéroglyphes hittites)",
        num: "080",
    },
    ScriptCode {
        alias: Some("Pahawh_Hmong"),
        code: "Hmng",
        date: ScriptDate {
            year: 2014,
            month: 11,
            day: 15
        },
        name: "Pahawh Hmong",
        name_french: "pahawh hmong",
        num: "450",
    },
    ScriptCode {
        alias: Some("Nyiakeng_Puachue_Hmong"),
        code: "Hmnp",
        date: ScriptDate {
            year: 2017,
            month: 07,
            day: 26
        },
        name: "Nyiakeng Puachue Hmong",
        name_french: "nyiakeng puachue hmong",
        num: "451",
    },
    ScriptCode {
        alias: Some("Katakana_Or_Hiragana"),
        code: "Hrkt",
        date: ScriptDate {
            year: 2011,
            month: 06,
            day: 21
        },
        name: "Japanese syllabaries (alias for Hiragana + Katakana)",
        name_french: "syllabaires japonais (alias pour hiragana + katakana)",
        num: "412",
    },
    ScriptCode {
        alias: Some("Old_Hungarian"),
        code: "Hung",
        date: ScriptDate {
            year: 2015,
            month: 07,
            day: 07
        },
        name: "Old Hungarian (Hungarian Runic)",
        name_french: "runes hongroises (ancien hongrois)",
        num: "176",
    },
    ScriptCode {
        alias: None,
        code: "Inds",
        date: ScriptDate {
            year: 2004,
            month: 05,
            day: 01
        },
        name: "Indus (Harappan)",
        name_french: "indus",
        num: "610",
    },
    ScriptCode {
        alias: Some("Old_Italic"),
        code: "Ital",
        date: ScriptDate {
            year: 2004,
            month: 05,
            day: 29
        },
        name: "Old Italic (Etruscan, Oscan, etc.)",
        name_french: "ancien italique (étrusque, osque, etc.)",
        num: "210",
    },
    ScriptCode {
        alias: None,
        code: "Jamo",
        date: ScriptDate {
            year: 2016,
            month: 01,
            day: 19
        },
        name: "Jamo (alias for Jamo subset of Hangul)",
        name_french: "jamo (alias pour le sous-ensemble jamo du hangûl)",
        num: "284",
    },
    ScriptCode {
        alias: Some("Javanese"),
        code: "Java",
        date: ScriptDate {
            year: 2009,
            month: 06,
            day: 01
        },
        name: "Javanese",
        name_french: "javanais",
        num: "361",
    },
    ScriptCode {
        alias: None,
        code: "Jpan",
        date: ScriptDate {
            year: 2006,
            month: 06,
            day: 21
        },
        name: "Japanese (alias for Han + Hiragana + Katakana)",
        name_french: "japonais (alias pour han + hiragana + katakana)",
        num: "413",
    },
    ScriptCode {
        alias: None,
        code: "Jurc",
        date: ScriptDate {
            year: 2010,
            month: 12,
            day: 21
        },
        name: "Jurchen",
        name_french: "jurchen",
        num: "510",
    },
    ScriptCode {
        alias: Some("Kayah_Li"),
        code: "Kali",
        date: ScriptDate {
            year: 2007,
            month: 07,
            day: 02
        },
        name: "Kayah Li",
        name_french: "kayah li",
        num: "357",
    },
    ScriptCode {
        alias: Some("Katakana"),
        code: "Kana",
        date: ScriptDate {
            year: 2004,
            month: 05,
            day: 01
        },
        name: "Katakana",
        name_french: "katakana",
        num: "411",
    },
    ScriptCode {
        alias: Some("Kharoshthi"),
        code: "Khar",
        date: ScriptDate {
            year: 2006,
            month: 06,
            day: 21
        },
        name: "Kharoshthi",
        name_french: "kharochthî",
        num: "305",
    },
    ScriptCode {
        alias: Some("Khmer"),
        code: "Khmr",
        date: ScriptDate {
            year: 2004,
            month: 05,
            day: 29
        },
        name: "Khmer",
        name_french: "khmer",
        num: "355",
    },
    ScriptCode {
        alias: Some("Khojki"),
        code: "Khoj",
        date: ScriptDate {
            year: 2014,
            month: 11,
            day: 15
        },
        name: "Khojki",
        name_french: "khojkî",
        num: "322",
    },
    ScriptCode {
        alias: None,
        code: "Kitl",
        date: ScriptDate {
            year: 2015,
            month: 07,
            day: 15
        },
        name: "Khitan large script",
        name_french: "grande écriture khitan",
        num: "505",
    },
    ScriptCode {
        alias: None,
        code: "Kits",
        date: ScriptDate {
            year: 2015,
            month: 07,
            day: 15
        },
        name: "Khitan small script",
        name_french: "petite écriture khitan",
        num: "288",
    },
    ScriptCode {
        alias: Some("Kannada"),
        code: "Knda",
        date: ScriptDate {
            year: 2004,
            month: 05,
            day: 29
        },
        name: "Kannada",
        name_french: "kannara (canara)",
        num: "345",
    },
    ScriptCode {
        alias: None,
        code: "Kore",
        date: ScriptDate {
            year: 2007,
            month: 06,
            day: 13
        },
        name: "Korean (alias for Hangul + Han)",
        name_french: "coréen (alias pour hangûl + han)",
        num: "287",
    },
    ScriptCode {
        alias: None,
        code: "Kpel",
        date: ScriptDate {
            year: 2010,
            month: 03,
            day: 26
        },
        name: "Kpelle",
        name_french: "kpèllé",
        num: "436",
    },
    ScriptCode {
        alias: Some("Kaithi"),
        code: "Kthi",
        date: ScriptDate {
            year: 2009,
            month: 06,
            day: 01
        },
        name: "Kaithi",
        name_french: "kaithî",
        num: "317",
    },
    ScriptCode {
        alias: Some("Tai_Tham"),
        code: "Lana",
        date: ScriptDate {
            year: 2009,
            month: 06,
            day: 01
        },
        name: "Tai Tham (Lanna)",
        name_french: "taï tham (lanna)",
        num: "351",
    },
    ScriptCode {
        alias: Some("Lao"),
        code: "Laoo",
        date: ScriptDate {
            year: 2004,
            month: 05,
            day: 01
        },
        name: "Lao",
        name_french: "laotien",
        num: "356",
    },
    ScriptCode {
        alias: None,
        code: "Latf",
        date: ScriptDate {
            year: 2004,
            month: 05,
            day: 01
        },
        name: "Latin (Fraktur variant)",
        name_french: "latin (variante brisée)",
        num: "217",
    },
    ScriptCode {
        alias: None,
        code: "Latg",
        date: ScriptDate {
            year: 2004,
            month: 05,
            day: 01
        },
        name: "Latin (Gaelic variant)",
        name_french: "latin (variante gaélique)",
        num: "216",
    },
    ScriptCode {
        alias: Some("Latin"),
        code: "Latn",
        date: ScriptDate {
            year: 2004,
            month: 05,
            day: 01
        },
        name: "Latin",
        name_french: "latin",
        num: "215",
    },
    ScriptCode {
        alias: None,
        code: "Leke",
        date: ScriptDate {
            year: 2015,
            month: 07,
            day: 07
        },
        name: "Leke",
        name_french: "léké",
        num: "364",
    },
    ScriptCode {
        alias: Some("Lepcha"),
        code: "Lepc",
        date: ScriptDate {
            year: 2007,
            month: 07,
            day: 02
        },
        name: "Lepcha (Róng)",
        name_french: "lepcha (róng)",
        num: "335",
    },
    ScriptCode {
        alias: Some("Limbu"),
        code: "Limb",
        date: ScriptDate {
            year: 2004,
            month: 05,
            day: 29
        },
        name: "Limbu",
        name_french: "limbou",
        num: "336",
    },
    ScriptCode {
        alias: Some("Linear_A"),
        code: "Lina",
        date: ScriptDate {
            year: 2014,
            month: 11,
            day: 15
        },
        name: "Linear A",
        name_french: "linéaire A",
        num: "400",
    },
    ScriptCode {
        alias: Some("Linear_B"),
        code: "Linb",
        date: ScriptDate {
            year: 2004,
            month: 05,
            day: 29
        },
        name: "Linear B",
        name_french: "linéaire B",
        num: "401",
    },
    ScriptCode {
        alias: Some("Lisu"),
        code: "Lisu",
        date: ScriptDate {
            year: 2009,
            month: 06,
            day: 01
        },
        name: "Lisu (Fraser)",
        name_french: "lisu (Fraser)",
        num: "399",
    },
    ScriptCode {
        alias: None,
        code: "Loma",
        date: ScriptDate {
            year: 2010,
            month: 03,
            day: 26
        },
        name: "Loma",
        name_french: "loma",
        num: "437",
    },
    ScriptCode {
        alias: Some("Lycian"),
        code: "Lyci",
        date: ScriptDate {
            year: 2007,
            month: 07,
            day: 02
        },
        name: "Lycian",
        name_french: "lycien",
        num: "202",
    },
    ScriptCode {
        alias: Some("Lydian"),
        code: "Lydi",
        date: ScriptDate {
            year: 2007,
            month: 07,
            day: 02
        },
        name: "Lydian",
        name_french: "lydien",
        num: "116",
    },
    ScriptCode {
        alias: Some("Mahajani"),
        code: "Mahj",
        date: ScriptDate {
            year: 2014,
            month: 11,
            day: 15
        },
        name: "Mahajani",
        name_french: "mahâjanî",
        num: "314",
    },
    ScriptCode {
        alias: Some("Makasar"),
        code: "Maka",
        date: ScriptDate {
            year: 2016,
            month: 12,
            day: 05
        },
        name: "Makasar",
        name_french: "makassar",
        num: "366",
    },
    ScriptCode {
        alias: Some("Mandaic"),
        code: "Mand",
        date: ScriptDate {
            year: 2010,
            month: 07,
            day: 23
        },
        name: "Mandaic, Mandaean",
        name_french: "mandéen",
        num: "140",
    },
    ScriptCode {
        alias: Some("Manichaean"),
        code: "Mani",
        date: ScriptDate {
            year: 2014,
            month: 11,
            day: 15
        },
        name: "Manichaean",
        name_french: "manichéen",
        num: "139",
    },
    ScriptCode {
        alias: Some("Marchen"),
        code: "Marc",
        date: ScriptDate {
            year: 2016,
            month: 12,
            day: 05
        },
        name: "Marchen",
        name_french: "marchen",
        num: "332",
    },
    ScriptCode {
        alias: None,
        code: "Maya",
        date: ScriptDate {
            year: 2004,
            month: 05,
            day: 01
        },
        name: "Mayan hieroglyphs",
        name_french: "hiéroglyphes mayas",
        num: "090",
    },
    ScriptCode {
        alias: Some("Medefaidrin"),
        code: "Medf",
        date: ScriptDate {
            year: 2016,
            month: 12,
            day: 05
        },
        name: "Medefaidrin (Oberi Okaime, Oberi Ɔkaimɛ)",
        name_french: "médéfaïdrine",
        num: "265",
    },
    ScriptCode {
        alias: Some("Mende_Kikakui"),
        code: "Mend",
        date: ScriptDate {
            year: 2014,
            month: 11,
            day: 15
        },
        name: "Mende Kikakui",
        name_french: "mendé kikakui",
        num: "438",
    },
    ScriptCode {
        alias: Some("Meroitic_Cursive"),
        code: "Merc",
        date: ScriptDate {
            year: 2012,
            month: 02,
            day: 06
        },
        name: "Meroitic Cursive",
        name_french: "cursif méroïtique",
        num: "101",
    },
    ScriptCode {
        alias: Some("Meroitic_Hieroglyphs"),
        code: "Mero",
        date: ScriptDate {
            year: 2012,
            month: 02,
            day: 06
        },
        name: "Meroitic Hieroglyphs",
        name_french: "hiéroglyphes méroïtiques",
        num: "100",
    },
    ScriptCode {
        alias: Some("Malayalam"),
        code: "Mlym",
        date: ScriptDate {
            year: 2004,
            month: 05,
            day: 01
        },
        name: "Malayalam",
        name_french: "malayâlam",
        num: "347",
    },
    ScriptCode {
        alias: Some("Modi"),
        code: "Modi",
        date: ScriptDate {
            year: 2014,
            month: 11,
            day: 15
        },
        name: "Modi, Moḍī",
        name_french: "modî",
        num: "324",
    },
    ScriptCode {
        alias: Some("Mongolian"),
        code: "Mong",
        date: ScriptDate {
            year: 2004,
            month: 05,
            day: 01
        },
        name: "Mongolian",
        name_french: "mongol",
        num: "145",
    },
    ScriptCode {
        alias: None,
        code: "Moon",
        date: ScriptDate {
            year: 2006,
            month: 12,
            day: 11
        },
        name: "Moon (Moon code, Moon script, Moon type)",
        name_french: "écriture Moon",
        num: "218",
    },
    ScriptCode {
        alias: Some("Mro"),
        code: "Mroo",
        date: ScriptDate {
            year: 2016,
            month: 12,
            day: 05
        },
        name: "Mro, Mru",
        name_french: "mro",
        num: "264",
    },
    ScriptCode {
        alias: Some("Meetei_Mayek"),
        code: "Mtei",
        date: ScriptDate {
            year: 2009,
            month: 06,
            day: 01
        },
        name: "Meitei Mayek (Meithei, Meetei)",
        name_french: "meitei mayek",
        num: "337",
    },
    ScriptCode {
        alias: Some("Multani"),
        code: "Mult",
        date: ScriptDate {
            year: 2015,
            month: 07,
            day: 07
        },
        name: "Multani",
        name_french: "multanî",
        num: "323",
    },
    ScriptCode {
        alias: Some("Myanmar"),
        code: "Mymr",
        date: ScriptDate {
            year: 2004,
            month: 05,
            day: 01
        },
        name: "Myanmar (Burmese)",
        name_french: "birman",
        num: "350",
    },
    ScriptCode {
        alias: Some("Nandinagari"),
        code: "Nand",
        date: ScriptDate {
            year: 2018,
            month: 08,
            day: 26
        },
        name: "Nandinagari",
        name_french: "nandinâgarî",
        num: "311",
    },
    ScriptCode {
        alias: Some("Old_North_Arabian"),
        code: "Narb",
        date: ScriptDate {
            year: 2014,
            month: 11,
            day: 15
        },
        name: "Old North Arabian (Ancient North Arabian)",
        name_french: "nord-arabique",
        num: "106",
    },
    ScriptCode {
        alias: Some("Nabataean"),
        code: "Nbat",
        date: ScriptDate {
            year: 2014,
            month: 11,
            day: 15
        },
        name: "Nabataean",
        name_french: "nabatéen",
        num: "159",
    },
    ScriptCode {
        alias: Some("Newa"),
        code: "Newa",
        date: ScriptDate {
            year: 2016,
            month: 12,
            day: 05
        },
        name: "Newa, Newar, Newari, Nepāla lipi",
        name_french: "néwa, néwar, néwari, nepāla lipi",
        num: "333",
    },
    ScriptCode {
        alias: None,
        code: "Nkdb",
        date: ScriptDate {
            year: 2017,
            month: 07,
            day: 26
        },
        name: "Naxi Dongba (na²¹ɕi³³ to³³ba²¹, Nakhi Tomba)",
        name_french: "naxi dongba",
        num: "085",
    },
    ScriptCode {
        alias: None,
        code: "Nkgb",
        date: ScriptDate {
            year: 2017,
            month: 07,
            day: 26
        },
        name: "Naxi Geba (na²¹ɕi³³ gʌ²¹ba²¹, 'Na-'Khi ²Ggŏ-¹baw, Nakhi Geba)",
        name_french: "naxi geba, nakhi geba",
        num: "420",
    },
    ScriptCode {
        alias: Some("Nko"),
        code: "Nkoo",
        date: ScriptDate {
            year: 2006,
            month: 10,
            day: 10
        },
        name: "N’Ko",
        name_french: "n’ko",
        num: "165",
    },
    ScriptCode {
        alias: Some("Nushu"),
        code: "Nshu",
        date: ScriptDate {
            year: 2017,
            month: 07,
            day: 26
        },
        name: "Nüshu",
        name_french: "nüshu",
        num: "499",
    },
    ScriptCode {
        alias: Some("Ogham"),
        code: "Ogam",
        date: ScriptDate {
            year: 2004,
            month: 05,
            day: 01
        },
        name: "Ogham",
        name_french: "ogam",
        num: "212",
    },
    ScriptCode {
        alias: Some("Ol_Chiki"),
        code: "Olck",
        date: ScriptDate {
            year: 2007,
            month: 07,
            day: 02
        },
        name: "Ol Chiki (Ol Cemet’, Ol, Santali)",
        name_french: "ol tchiki",
        num: "261",
    },
    ScriptCode {
        alias: Some("Old_Turkic"),
        code: "Orkh",
        date: ScriptDate {
            year: 2009,
            month: 06,
            day: 01
        },
        name: "Old Turkic, Orkhon Runic",
        name_french: "orkhon",
        num: "175",
    },
    ScriptCode {
        alias: Some("Oriya"),
        code: "Orya",
        date: ScriptDate {
            year: 2016,
            month: 12,
            day: 05
        },
        name: "Oriya (Odia)",
        name_french: "oriyâ (odia)",
        num: "327",
    },
    ScriptCode {
        alias: Some("Osage"),
        code: "Osge",
        date: ScriptDate {
            year: 2016,
            month: 12,
            day: 05
        },
        name: "Osage",
        name_french: "osage",
        num: "219",
    },
    ScriptCode {
        alias: Some("Osmanya"),
        code: "Osma",
        date: ScriptDate {
            year: 2004,
            month: 05,
            day: 01
        },
        name: "Osmanya",
        name_french: "osmanais",
        num: "260",
    },
    ScriptCode {
        alias: Some("Palmyrene"),
        code: "Palm",
        date: ScriptDate {
            year: 2014,
            month: 11,
            day: 15
        },
        name: "Palmyrene",
        name_french: "palmyrénien",
        num: "126",
    },
    ScriptCode {
        alias: Some("Pau_Cin_Hau"),
        code: "Pauc",
        date: ScriptDate {
            year: 2014,
            month: 11,
            day: 15
        },
        name: "Pau Cin Hau",
        name_french: "paou chin haou",
        num: "263",
    },
    ScriptCode {
        alias: Some("Old_Permic"),
        code: "Perm",
        date: ScriptDate {
            year: 2014,
            month: 11,
            day: 15
        },
        name: "Old Permic",
        name_french: "ancien permien",
        num: "227",
    },
    ScriptCode {
        alias: Some("Phags_Pa"),
        code: "Phag",
        date: ScriptDate {
            year: 2006,
            month: 10,
            day: 10
        },
        name: "Phags-pa",
        name_french: "’phags pa",
        num: "331",
    },
    ScriptCode {
        alias: Some("Inscriptional_Pahlavi"),
        code: "Phli",
        date: ScriptDate {
            year: 2009,
            month: 06,
            day: 01
        },
        name: "Inscriptional Pahlavi",
        name_french: "pehlevi des inscriptions",
        num: "131",
    },
    ScriptCode {
        alias: Some("Psalter_Pahlavi"),
        code: "Phlp",
        date: ScriptDate {
            year: 2014,
            month: 11,
            day: 15
        },
        name: "Psalter Pahlavi",
        name_french: "pehlevi des psautiers",
        num: "132",
    },
    ScriptCode {
        alias: None,
        code: "Phlv",
        date: ScriptDate {
            year: 2007,
            month: 07,
            day: 15
        },
        name: "Book Pahlavi",
        name_french: "pehlevi des livres",
        num: "133",
    },
    ScriptCode {
        alias: Some("Phoenician"),
        code: "Phnx",
        date: ScriptDate {
            year: 2006,
            month: 10,
            day: 10
        },
        name: "Phoenician",
        name_french: "phénicien",
        num: "115",
    },
    ScriptCode {
        alias: Some("Miao"),
        code: "Plrd",
        date: ScriptDate {
            year: 2012,
            month: 02,
            day: 06
        },
        name: "Miao (Pollard)",
        name_french: "miao (Pollard)",
        num: "282",
    },
    ScriptCode {
        alias: None,
        code: "Piqd",
        date: ScriptDate {
            year: 2015,
            month: 12,
            day: 16
        },
        name: "Klingon (KLI pIqaD)",
        name_french: "klingon (pIqaD du KLI)",
        num: "293",
    },
    ScriptCode {
        alias: Some("Inscriptional_Parthian"),
        code: "Prti",
        date: ScriptDate {
            year: 2009,
            month: 06,
            day: 01
        },
        name: "Inscriptional Parthian",
        name_french: "parthe des inscriptions",
        num: "130",
    },
    ScriptCode {
        alias: None,
        code: "Qaaa",
        date: ScriptDate {
            year: 2004,
            month: 05,
            day: 29
        },
        name: "Reserved for private use (start)",
        name_french: "réservé à l’usage privé (début)",
        num: "900",
    },
    ScriptCode {
        alias: None,
        code: "Qabx",
        date: ScriptDate {
            year: 2004,
            month: 05,
            day: 29
        },
        name: "Reserved for private use (end)",
        name_french: "réservé à l’usage privé (fin)",
        num: "949",
    },
    ScriptCode {
        alias: Some("Rejang"),
        code: "Rjng",
        date: ScriptDate {
            year: 2009,
            month: 02,
            day: 23
        },
        name: "Rejang (Redjang, Kaganga)",
        name_french: "redjang (kaganga)",
        num: "363",
    },
    ScriptCode {
        alias: Some("Hanifi_Rohingya"),
        code: "Rohg",
        date: ScriptDate {
            year: 2017,
            month: 11,
            day: 21
        },
        name: "Hanifi Rohingya",
        name_french: "hanifi rohingya",
        num: "167",
    },
    ScriptCode {
        alias: None,
        code: "Roro",
        date: ScriptDate {
            year: 2004,
            month: 05,
            day: 01
        },
        name: "Rongorongo",
        name_french: "rongorongo",
        num: "620",
    },
    ScriptCode {
        alias: Some("Runic"),
        code: "Runr",
        date: ScriptDate {
            year: 2004,
            month: 05,
            day: 01
        },
        name: "Runic",
        name_french: "runique",
        num: "211",
    },
    ScriptCode {
        alias: Some("Samaritan"),
        code: "Samr",
        date: ScriptDate {
            year: 2009,
            month: 06,
            day: 01
        },
        name: "Samaritan",
        name_french: "samaritain",
        num: "123",
    },
    ScriptCode {
        alias: None,
        code: "Sara",
        date: ScriptDate {
            year: 2004,
            month: 05,
            day: 29
        },
        name: "Sarati",
        name_french: "sarati",
        num: "292",
    },
    ScriptCode {
        alias: Some("Old_South_Arabian"),
        code: "Sarb",
        date: ScriptDate {
            year: 2009,
            month: 06,
            day: 01
        },
        name: "Old South Arabian",
        name_french: "sud-arabique, himyarite",
        num: "105",
    },
    ScriptCode {
        alias: Some("Saurashtra"),
        code: "Saur",
        date: ScriptDate {
            year: 2007,
            month: 07,
            day: 02
        },
        name: "Saurashtra",
        name_french: "saurachtra",
        num: "344",
    },
    ScriptCode {
        alias: Some("SignWriting"),
        code: "Sgnw",
        date: ScriptDate {
            year: 2015,
            month: 07,
            day: 07
        },
        name: "SignWriting",
        name_french: "SignÉcriture, SignWriting",
        num: "095",
    },
    ScriptCode {
        alias: Some("Shavian"),
        code: "Shaw",
        date: ScriptDate {
            year: 2004,
            month: 05,
            day: 01
        },
        name: "Shavian (Shaw)",
        name_french: "shavien (Shaw)",
        num: "281",
    },
    ScriptCode {
        alias: Some("Sharada"),
        code: "Shrd",
        date: ScriptDate {
            year: 2012,
            month: 02,
            day: 06
        },
        name: "Sharada, Śāradā",
        name_french: "charada, shard",
        num: "319",
    },
    ScriptCode {
        alias: None,
        code: "Shui",
        date: ScriptDate {
            year: 2017,
            month: 07,
            day: 26
        },
        name: "Shuishu",
        name_french: "shuishu",
        num: "530",
    },
    ScriptCode {
        alias: Some("Siddham"),
        code: "Sidd",
        date: ScriptDate {
            year: 2014,
            month: 11,
            day: 15
        },
        name: "Siddham, Siddhaṃ, Siddhamātṛkā",
        name_french: "siddham",
        num: "302",
    },
    ScriptCode {
        alias: Some("Khudawadi"),
        code: "Sind",
        date: ScriptDate {
            year: 2014,
            month: 11,
            day: 15
        },
        name: "Khudawadi, Sindhi",
        name_french: "khoudawadî, sindhî",
        num: "318",
    },
    ScriptCode {
        alias: Some("Sinhala"),
        code: "Sinh",
        date: ScriptDate {
            year: 2004,
            month: 05,
            day: 01
        },
        name: "Sinhala",
        name_french: "singhalais",
        num: "348",
    },
    ScriptCode {
        alias: Some("Sogdian"),
        code: "Sogd",
        date: ScriptDate {
            year: 2017,
            month: 11,
            day: 21
        },
        name: "Sogdian",
        name_french: "sogdien",
        num: "141",
    },
    ScriptCode {
        alias: Some("Old_Sogdian"),
        code: "Sogo",
        date: ScriptDate {
            year: 2017,
            month: 11,
            day: 21
        },
        name: "Old Sogdian",
        name_french: "ancien sogdien",
        num: "142",
    },
    ScriptCode {
        alias: Some("Sora_Sompeng"),
        code: "Sora",
        date: ScriptDate {
            year: 2012,
            month: 02,
            day: 06
        },
        name: "Sora Sompeng",
        name_french: "sora sompeng",
        num: "398",
    },
    ScriptCode {
        alias: Some("Soyombo"),
        code: "Soyo",
        date: ScriptDate {
            year: 2017,
            month: 07,
            day: 26
        },
        name: "Soyombo",
        name_french: "soyombo",
        num: "329",
    },
    ScriptCode {
        alias: Some("Sundanese"),
        code: "Sund",
        date: ScriptDate {
            year: 2007,
            month: 07,
            day: 02
        },
        name: "Sundanese",
        name_french: "sundanais",
        num: "362",
    },
    ScriptCode {
        alias: Some("Syloti_Nagri"),
        code: "Sylo",
        date: ScriptDate {
            year: 2006,
            month: 06,
            day: 21
        },
        name: "Syloti Nagri",
        name_french: "sylotî nâgrî",
        num: "316",
    },
    ScriptCode {
        alias: Some("Syriac"),
        code: "Syrc",
        date: ScriptDate {
            year: 2004,
            month: 05,
            day: 01
        },
        name: "Syriac",
        name_french: "syriaque",
        num: "135",
    },
    ScriptCode {
        alias: None,
        code: "Syre",
        date: ScriptDate {
            year: 2004,
            month: 05,
            day: 01
        },
        name: "Syriac (Estrangelo variant)",
        name_french: "syriaque (variante estranghélo)",
        num: "138",
    },
    ScriptCode {
        alias: None,
        code: "Syrj",
        date: ScriptDate {
            year: 2004,
            month: 05,
            day: 01
        },
        name: "Syriac (Western variant)",
        name_french: "syriaque (variante occidentale)",
        num: "137",
    },
    ScriptCode {
        alias: None,
        code: "Syrn",
        date: ScriptDate {
            year: 2004,
            month: 05,
            day: 01
        },
        name: "Syriac (Eastern variant)",
        name_french: "syriaque (variante orientale)",
        num: "136",
    },
    ScriptCode {
        alias: Some("Tagbanwa"),
        code: "Tagb",
        date: ScriptDate {
            year: 2004,
            month: 05,
            day: 01
        },
        name: "Tagbanwa",
        name_french: "tagbanoua",
        num: "373",
    },
    ScriptCode {
        alias: Some("Takri"),
        code: "Takr",
        date: ScriptDate {
            year: 2012,
            month: 02,
            day: 06
        },
        name: "Takri, Ṭākrī, Ṭāṅkrī",
        name_french: "tâkrî",
        num: "321",
    },
    ScriptCode {
        alias: Some("Tai_Le"),
        code: "Tale",
        date: ScriptDate {
            year: 2004,
            month: 10,
            day: 25
        },
        name: "Tai Le",
        name_french: "taï-le",
        num: "353",
    },
    ScriptCode {
        alias: Some("New_Tai_Lue"),
        code: "Talu",
        date: ScriptDate {
            year: 2006,
            month: 06,
            day: 21
        },
        name: "New Tai Lue",
        name_french: "nouveau taï-lue",
        num: "354",
    },
    ScriptCode {
        alias: Some("Tamil"),
        code: "Taml",
        date: ScriptDate {
            year: 2004,
            month: 05,
            day: 01
        },
        name: "Tamil",
        name_french: "tamoul",
        num: "346",
    },
    ScriptCode {
        alias: Some("Tangut"),
        code: "Tang",
        date: ScriptDate {
            year: 2016,
            month: 12,
            day: 05
        },
        name: "Tangut",
        name_french: "tangoute",
        num: "520",
    },
    ScriptCode {
        alias: Some("Tai_Viet"),
        code: "Tavt",
        date: ScriptDate {
            year: 2009,
            month: 06,
            day: 01
        },
        name: "Tai Viet",
        name_french: "taï viêt",
        num: "359",
    },
    ScriptCode {
        alias: Some("Telugu"),
        code: "Telu",
        date: ScriptDate {
            year: 2004,
            month: 05,
            day: 01
        },
        name: "Telugu",
        name_french: "télougou",
        num: "340",
    },
    ScriptCode {
        alias: None,
        code: "Teng",
        date: ScriptDate {
            year: 2004,
            month: 05,
            day: 01
        },
        name: "Tengwar",
        name_french: "tengwar",
        num: "290",
    },
    ScriptCode {
        alias: Some("Tifinagh"),
        code: "Tfng",
        date: ScriptDate {
            year: 2006,
            month: 06,
            day: 21
        },
        name: "Tifinagh (Berber)",
        name_french: "tifinagh (berbère)",
        num: "120",
    },
    ScriptCode {
        alias: Some("Tagalog"),
        code: "Tglg",
        date: ScriptDate {
            year: 2009,
            month: 02,
            day: 23
        },
        name: "Tagalog (Baybayin, Alibata)",
        name_french: "tagal (baybayin, alibata)",
        num: "370",
    },
    ScriptCode {
        alias: Some("Thaana"),
        code: "Thaa",
        date: ScriptDate {
            year: 2004,
            month: 05,
            day: 01
        },
        name: "Thaana",
        name_french: "thâna",
        num: "170",
    },
    ScriptCode {
        alias: Some("Thai"),
        code: "Thai",
        date: ScriptDate {
            year: 2004,
            month: 05,
            day: 01
        },
        name: "Thai",
        name_french: "thaï",
        num: "352",
    },
    ScriptCode {
        alias: Some("Tibetan"),
        code: "Tibt",
        date: ScriptDate {
            year: 2004,
            month: 05,
            day: 01
        },
        name: "Tibetan",
        name_french: "tibétain",
        num: "330",
    },
    ScriptCode {
        alias: Some("Tirhuta"),
        code: "Tirh",
        date: ScriptDate {
            year: 2014,
            month: 11,
            day: 15
        },
        name: "Tirhuta",
        name_french: "tirhouta",
        num: "326",
    },
    ScriptCode {
        alias: Some("Ugaritic"),
        code: "Ugar",
        date: ScriptDate {
            year: 2004,
            month: 05,
            day: 01
        },
        name: "Ugaritic",
        name_french: "ougaritique",
        num: "040",
    },
    ScriptCode {
        alias: Some("Vai"),
        code: "Vaii",
        date: ScriptDate {
            year: 2007,
            month: 07,
            day: 02
        },
        name: "Vai",
        name_french: "vaï",
        num: "470",
    },
    ScriptCode {
        alias: None,
        code: "Visp",
        date: ScriptDate {
            year: 2004,
            month: 05,
            day: 01
        },
        name: "Visible Speech",
        name_french: "parole visible",
        num: "280",
    },
    ScriptCode {
        alias: Some("Warang_Citi"),
        code: "Wara",
        date: ScriptDate {
            year: 2014,
            month: 11,
            day: 15
        },
        name: "Warang Citi (Varang Kshiti)",
        name_french: "warang citi",
        num: "262",
    },
    ScriptCode {
        alias: Some("Wancho"),
        code: "Wcho",
        date: ScriptDate {
            year: 2017,
            month: 07,
            day: 26
        },
        name: "Wancho",
        name_french: "wantcho",
        num: "283",
    },
    ScriptCode {
        alias: None,
        code: "Wole",
        date: ScriptDate {
            year: 2010,
            month: 12,
            day: 21
        },
        name: "Woleai",
        name_french: "woléaï",
        num: "480",
    },
    ScriptCode {
        alias: Some("Old_Persian"),
        code: "Xpeo",
        date: ScriptDate {
            year: 2006,
            month: 06,
            day: 21
        },
        name: "Old Persian",
        name_french: "cunéiforme persépolitain",
        num: "030",
    },
    ScriptCode {
        alias: Some("Cuneiform"),
        code: "Xsux",
        date: ScriptDate {
            year: 2006,
            month: 10,
            day: 10
        },
        name: "Cuneiform, Sumero-Akkadian",
        name_french: "cunéiforme suméro-akkadien",
        num: "020",
    },
    ScriptCode {
        alias: Some("Yi"),
        code: "Yiii",
        date: ScriptDate {
            year: 2004,
            month: 05,
            day: 01
        },
        name: "Yi",
        name_french: "yi",
        num: "460",
    },
    ScriptCode {
        alias: Some("Zanabazar_Square"),
        code: "Zanb",
        date: ScriptDate {
            year: 2017,
            month: 07,
            day: 26
        },
        name: "Zanabazar Square (Zanabazarin Dörböljin Useg, Xewtee Dörböljin Bicig, Horizontal Square Script)",
        name_french: "zanabazar quadratique",
        num: "339",
    },
    ScriptCode {
        alias: Some("Inherited"),
        code: "Zinh",
        date: ScriptDate {
            year: 2009,
            month: 02,
            day: 23
        },
        name: "Code for inherited script",
        name_french: "codet pour écriture héritée",
        num: "994",
    },
    ScriptCode {
        alias: None,
        code: "Zmth",
        date: ScriptDate {
            year: 2007,
            month: 11,
            day: 26
        },
        name: "Mathematical notation",
        name_french: "notation mathématique",
        num: "995",
    },
    ScriptCode {
        alias: None,
        code: "Zsye",
        date: ScriptDate {
            year: 2015,
            month: 12,
            day: 16
        },
        name: "Symbols (Emoji variant)",
        name_french: "symboles (variante émoji)",
        num: "993",
    },
    ScriptCode {
        alias: None,
        code: "Zsym",
        date: ScriptDate {
            year: 2007,
            month: 11,
            day: 26
        },
        name: "Symbols",
        name_french: "symboles",
        num: "996",
    },
    ScriptCode {
        alias: None,
        code: "Zxxx",
        date: ScriptDate {
            year: 2011,
            month: 06,
            day: 21
        },
        name: "Code for unwritten documents",
        name_french: "codet pour les documents non écrits",
        num: "997",
    },
    ScriptCode {
        alias: Some("Common"),
        code: "Zyyy",
        date: ScriptDate {
            year: 2004,
            month: 05,
            day: 29
        },
        name: "Code for undetermined script",
        name_french: "codet pour écriture indéterminée",
        num: "998",
    },
    ScriptCode {
        alias: Some("Unknown"),
        code: "Zzzz",
        date: ScriptDate {
            year: 2006,
            month: 10,
            day: 10
        },
        name: "Code for uncoded script",
        name_french: "codet pour écriture non codée",
        num: "999",
    },
    // End.
];

/// Retrieve all of the `ScriptCode`s. This is updated automatically via
/// `make update`.
pub fn all() -> &'static [ScriptCode<'static>] {
    &SCRIPTS
}
