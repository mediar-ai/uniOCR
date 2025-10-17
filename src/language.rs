use serde::Serialize;
use std::fmt;

macro_rules! languages {
    ($( $name:ident => ($iso:expr, $tess:expr) ),* $(,)?) => {
        #[derive(Clone, Debug, Serialize, Hash, Eq, PartialEq, Copy)]
        #[serde(rename_all = "kebab-case")]
        #[repr(usize)]
        pub enum Language {
            $($name),*
        }

        impl Language {
            pub fn as_lang_code(&self) -> &'static str {
                match self {
                    $(Language::$name => $iso),*
                }
            }

            /// Returns Some(tesseract_code) if supported, None if not.
            pub fn as_tesseract_code(&self) -> Option<&'static str> {
                match self {
                    $(Language::$name => $tess),*
                }
            }
        }
    };
}

languages! {
    English => ("en", Some("eng")),
    Chinese => ("zh", Some("chi_sim")),
    German => ("de", Some("deu")),
    Spanish => ("es", Some("spa")),
    Russian => ("ru", Some("rus")),
    Korean => ("ko", Some("kor")),
    French => ("fr", Some("fra")),
    Japanese => ("ja", Some("jpn")),
    Portuguese => ("pt", Some("por")),
    Turkish => ("tr", Some("tur")),
    Polish => ("pl", Some("pol")),
    Catalan => ("ca", Some("cat")),
    Dutch => ("nl", Some("nld")),
    Arabic => ("ar", Some("ara")),
    Swedish => ("sv", Some("swe")),
    Italian => ("it", Some("ita")),
    Indonesian => ("id", Some("ind")),
    Hindi => ("hi", Some("hin")),
    Finnish => ("fi", Some("fin")),
    Hebrew => ("he", Some("heb")),
    Ukrainian => ("uk", Some("ukr")),
    Greek => ("el", Some("ell")),
    Malay => ("ms", Some("msa")),
    Czech => ("cs", Some("ces")),
    Romanian => ("ro", Some("ron")),
    Danish => ("da", Some("dan")),
    Hungarian => ("hu", Some("hun")),
    Norwegian => ("no", Some("nor")),
    Thai => ("th", Some("tha")),
    Urdu => ("ur", Some("urd")),
    Croatian => ("hr", Some("hrv")),
    Bulgarian => ("bg", Some("bul")),
    Lithuanian => ("lt", Some("lit")),
    Latin => ("la", Some("lat")),
    Malayalam => ("ml", Some("mal")),
    Welsh => ("cy", Some("cym")),
    Slovak => ("sk", Some("slk")),
    Persian => ("fa", Some("fas")),
    Latvian => ("lv", Some("lav")),
    Bengali => ("bn", Some("ben")),
    Serbian => ("sr", Some("srp")),
    Azerbaijani => ("az", Some("aze")),
    Slovenian => ("sl", Some("slv")),
    Estonian => ("et", Some("est")),
    Macedonian => ("mk", Some("mkd")),
    Nepali => ("ne", Some("nep")),
    Mongolian => ("mn", Some("mon")),
    Bosnian => ("bs", Some("bos")),
    Kazakh => ("kk", Some("kaz")),
    Albanian => ("sq", Some("sqi")),
    Swahili => ("sw", Some("swa")),
    Galician => ("gl", Some("glg")),
    Marathi => ("mr", Some("mar")),
    Punjabi => ("pa", Some("pan")),
    Sinhala => ("si", Some("sin")),
    Khmer => ("km", Some("khm")),
    Afrikaans => ("af", Some("afr")),
    Belarusian => ("be", Some("bel")),
    Gujarati => ("gu", Some("guj")),
    Amharic => ("am", Some("amh")),
    Yiddish => ("yi", Some("yid")),
    Lao => ("lo", Some("lao")),
    Uzbek => ("uz", Some("uzb")),
    Faroese => ("fo", Some("fo")),
    Pashto => ("ps", Some("pus")),
    Maltese => ("mt", Some("mlt")),
    Sanskrit => ("sa", Some("san")),
    Luxembourgish => ("lb", Some("lb")),
    Myanmar => ("my", Some("mya")),
    Tibetan => ("bo", Some("bod")),
    Tagalog => ("tl", Some("tgl")),
    Assamese => ("as", Some("asm")),
    Tatar => ("tt", Some("tat")),
    Hausa => ("ha", Some("hau")),
    Javanese => ("jw", Some("jav")),
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_lang_code())
    }
}

impl PartialEq<&str> for Language {
    fn eq(&self, other: &&str) -> bool {
        self.to_string().as_str() == *other
    }
}
