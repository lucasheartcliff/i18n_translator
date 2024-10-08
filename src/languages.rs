use std::collections::HashMap;

pub fn is_supported_language(lang: &str) -> bool {
    let languages = get_supported_languages();
    languages.contains_key(lang)
}

pub fn get_supported_languages() -> HashMap<&'static str, &'static str> {
    let mut langs = HashMap::new();

    langs.insert("auto", "Automatic");
    langs.insert("af", "Afrikaans");
    langs.insert("sq", "Albanian");
    langs.insert("am", "Amharic");
    langs.insert("ar", "Arabic");
    langs.insert("hy", "Armenian");
    langs.insert("az", "Azerbaijani");
    langs.insert("eu", "Basque");
    langs.insert("be", "Belarusian");
    langs.insert("bn", "Bengali");
    langs.insert("bs", "Bosnian");
    langs.insert("bg", "Bulgarian");
    langs.insert("ca", "Catalan");
    langs.insert("ceb", "Cebuano");
    langs.insert("ny", "Chichewa");
    langs.insert("zh-cn", "Chinese Simplified");
    langs.insert("zh-tw", "Chinese Traditional");
    langs.insert("co", "Corsican");
    langs.insert("hr", "Croatian");
    langs.insert("cs", "Czech");
    langs.insert("da", "Danish");
    langs.insert("nl", "Dutch");
    langs.insert("en", "English");
    langs.insert("eo", "Esperanto");
    langs.insert("et", "Estonian");
    langs.insert("tl", "Filipino");
    langs.insert("fi", "Finnish");
    langs.insert("fr", "French");
    langs.insert("fy", "Frisian");
    langs.insert("gl", "Galician");
    langs.insert("ka", "Georgian");
    langs.insert("de", "German");
    langs.insert("el", "Greek");
    langs.insert("gu", "Gujarati");
    langs.insert("ht", "Haitian Creole");
    langs.insert("ha", "Hausa");
    langs.insert("haw", "Hawaiian");
    langs.insert("iw", "Hebrew");
    langs.insert("hi", "Hindi");
    langs.insert("hmn", "Hmong");
    langs.insert("hu", "Hungarian");
    langs.insert("is", "Icelandic");
    langs.insert("ig", "Igbo");
    langs.insert("id", "Indonesian");
    langs.insert("ga", "Irish");
    langs.insert("it", "Italian");
    langs.insert("ja", "Japanese");
    langs.insert("jw", "Javanese");
    langs.insert("kn", "Kannada");
    langs.insert("kk", "Kazakh");
    langs.insert("km", "Khmer");
    langs.insert("ko", "Korean");
    langs.insert("ku", "Kurdish (Kurmanji)");
    langs.insert("ky", "Kyrgyz");
    langs.insert("lo", "Lao");
    langs.insert("la", "Latin");
    langs.insert("lv", "Latvian");
    langs.insert("lt", "Lithuanian");
    langs.insert("lb", "Luxembourgish");
    langs.insert("mk", "Macedonian");
    langs.insert("mg", "Malagasy");
    langs.insert("ms", "Malay");
    langs.insert("ml", "Malayalam");
    langs.insert("mt", "Maltese");
    langs.insert("mi", "Maori");
    langs.insert("mr", "Marathi");
    langs.insert("mn", "Mongolian");
    langs.insert("my", "Myanmar (Burmese)");
    langs.insert("ne", "Nepali");
    langs.insert("no", "Norwegian");
    langs.insert("ps", "Pashto");
    langs.insert("fa", "Persian");
    langs.insert("pl", "Polish");
    langs.insert("pt", "Portuguese");
    langs.insert("pa", "Punjabi");
    langs.insert("ro", "Romanian");
    langs.insert("ru", "Russian");
    langs.insert("sm", "Samoan");
    langs.insert("gd", "Scots Gaelic");
    langs.insert("sr", "Serbian");
    langs.insert("st", "Sesotho");
    langs.insert("sn", "Shona");
    langs.insert("sd", "Sindhi");
    langs.insert("si", "Sinhala");
    langs.insert("sk", "Slovak");
    langs.insert("sl", "Slovenian");
    langs.insert("so", "Somali");
    langs.insert("es", "Spanish");
    langs.insert("su", "Sundanese");
    langs.insert("sw", "Swahili");
    langs.insert("sv", "Swedish");
    langs.insert("tg", "Tajik");
    langs.insert("ta", "Tamil");
    langs.insert("te", "Telugu");
    langs.insert("th", "Thai");
    langs.insert("tr", "Turkish");
    langs.insert("uk", "Ukrainian");
    langs.insert("ur", "Urdu");
    langs.insert("uz", "Uzbek");
    langs.insert("vi", "Vietnamese");
    langs.insert("cy", "Welsh");
    langs.insert("xh", "Xhosa");
    langs.insert("yi", "Yiddish");
    langs.insert("yo", "Yoruba");
    langs.insert("zu", "Zulu");

    langs
}
pub fn get_languages() -> HashMap<&'static str, &'static str> {
    let mut languages = get_supported_languages();
    languages.remove("auto");
    languages
}
