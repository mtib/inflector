use std::fmt::Display;

use crate::{Inflection, Root, WordForm, filters, Extra};

pub struct Table<'a, N>(pub &'a Root, pub N);

impl<'a, N: Noun> Display for Table<'a, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let &Table(r, ref n) = self;

        let sg = "sg";
        let pl = "pl";
        
        let nom_sg = n.nom_sg(r);
        let acc_sg = n.acc_sg(r);
        let dat_sg = n.dat_sg(r);
        let gen_sg = n.gen_sg(r);
        let nom_pl = n.nom_pl(r);
        let acc_pl = n.acc_pl(r);
        let dat_pl = n.dat_pl(r);
        let gen_pl = n.gen_pl(r);

        let sg_len = (nom_sg.char_count().max(acc_sg.char_count())).max(dat_sg.char_count().max(gen_sg.char_count())).max(sg.char_count());
        let pl_len = (nom_pl.char_count().max(acc_pl.char_count())).max(dat_pl.char_count().max(gen_pl.char_count())).max(pl.char_count());

        if f.alternate() {
            writeln!(f, "{}", self.0)?;
        }
        write!(f, r#"    | {sg:sg_len$} | {pl:pl_len$} |
nom | {nom_sg:sg_len$} | {nom_pl:pl_len$} |
acc | {acc_sg:sg_len$} | {acc_pl:pl_len$} |
dat | {dat_sg:sg_len$} | {dat_pl:pl_len$} |
gen | {gen_sg:sg_len$} | {gen_pl:pl_len$} |
"#)
    }
}

pub trait Noun {
    fn parse(&self, dictionary_form: &str) -> Root {
        Root::parse(dictionary_form)
    }
    fn nom_sg<'r>(&self, r: &'r Root) -> Inflection<'r>;
    fn acc_sg<'r>(&self, r: &'r Root) -> Inflection<'r>;
    fn dat_sg<'r>(&self, r: &'r Root) -> Inflection<'r>;
    fn gen_sg<'r>(&self, r: &'r Root) -> Inflection<'r>;
    fn nom_pl<'r>(&self, r: &'r Root) -> Inflection<'r>;
    fn acc_pl<'r>(&self, r: &'r Root) -> Inflection<'r> { Self::nom_pl(self, r) }
    fn dat_pl<'r>(&self, r: &'r Root) -> Inflection<'r> { Inflection::new(r.vowelled().drop_yod_if_palatalised(), if r.palatalised() { "jum" } else { "um" }) }
    fn gen_pl<'r>(&self, r: &'r Root) -> Inflection<'r> { Inflection::new(r.vowelled().drop_yod_if_palatalised(), if r.palatalised() { "ja" } else { "a" }) }
}

pub enum MasculineGenitive {
    S,
    Ar,
    Both,
}

pub enum MasculinePlural {
    Ar,
    Ir,
}

pub enum MasculineDative {
    I,
    Both,
    None,
}

pub struct Masculine(pub MasculineGenitive, pub MasculinePlural, pub MasculineDative);

// drongur, -s, -ir
// hylur, -jar/-s, -jar
// himmal, -s, himlar
// stuðul, -s, stuðlar
// vegur, -ar, -ir
// heyggjur, heygs, -ar
// dansur, dans, -ir
// blettur, blets, -ir
// veggur, -jar, jir
// sjógvur, -vs/sjós/sjóvar, -ar
// vøkstur, vakstrar, vøkstrir

impl Masculine {
    fn from(nom: &str, gen: &str, pl: &str) -> (Root, Self) {
        let stem = if nom.ends_with("ur") { &nom[..nom.len()-2] } else { nom };

        let gen = gen.replace(filters::dash, stem);
        let pl = pl.replace(filters::dash, stem);

        let gen_ending = if gen.ends_with("ar") {
            MasculineGenitive::Ar
        } else if gen.ends_with("s") {
            MasculineGenitive::S
        } else { panic!("Invalid genitive {}", gen) };
        let pl_ending = if pl.ends_with("ar") {
            MasculinePlural::Ar
        } else if pl.ends_with("ir") {
            MasculinePlural::Ir
        } else {
            panic!("Invalid plural {}", pl);
        };


        let mut first_stem = String::new();
        let mut deletable = String::new();
        let mut end = String::new();
        let mut extra = Extra::Nothing;

        let mut nom_ci = nom.char_indices();
        let mut gen_ci = gen.char_indices();
        let mut pl_ci = pl.char_indices();

        let (n, g, p) = loop {
            let n = nom_ci.next().map(|(_, c)| c);
            let g = gen_ci.next().map(|(_, c)| c);
            let p = pl_ci.next().map(|(_, c)| c);

            if n == g && g == p {
                first_stem.push(n.unwrap());
            } else { break (n, g, p); }
        };

        // Somehow determine the deletable, end and extra from the rest

        (Root {
            first_stem,
            deletable,
            end,
            extra,
        }, Masculine(gen_ending, pl_ending, MasculineDative::I))
    }
}

impl Noun for Masculine {
    fn nom_sg<'r>(&self, r: &'r Root) -> Inflection<'r> { Inflection::new(r.full(), if r.deletable.is_empty() || r.skerping() { "ur" } else { "" } ) }
    fn acc_sg<'r>(&self, r: &'r Root) -> Inflection<'r> { Inflection::new(r.full(), "") }
    fn dat_sg<'r>(&self, r: &'r Root) -> Inflection<'r> { Inflection::new(r.vowelled().drop_yod(), "i") }
    fn gen_sg<'r>(&self, r: &'r Root) -> Inflection<'r> {
        match self.0 {
            MasculineGenitive::S => Inflection::new(r.consonanted(), "s"),
            MasculineGenitive::Ar => Inflection::new(r.vowelled().drop_yod_if_palatalised(), if r.palatalised() { "jar" } else { "ar" }),
            MasculineGenitive::Both => unimplemented!(),
        }
    }
    fn nom_pl<'r>(&self, r: &'r Root) -> Inflection<'r> {
        match self.1 {
            MasculinePlural::Ar => Inflection::new(r.vowelled().drop_yod_if_palatalised(), if r.palatalised() { "jar" } else { "ar" }),
            MasculinePlural::Ir => Inflection::new(r.vowelled().drop_yod(), "ir"),
        }
    }
}

pub struct MascSAr;

impl Noun for MascSAr {
    // fn parse(&self, s: &str) -> Root {
        // let end = s.find('-');
        // if let Some(end) = end {
        //     if s.get(end-1..end) == Some("j") {
        //         Root {
        //             first_stem: s[..end-1].to_owned(),
        //             deletable: String::new(),
        //             end: "j".to_owned(),
        //             patalised: false,
        //         }
        //     } else {
        //         Root {
        //             first_stem: s[..end].to_owned(),
        //             deletable: String::new(),
        //             end: String::new(),
        //             patalised: &s[end+1..] == "jur",
        //         }
        //     }
        // } else {
        //     let end = &s[s.len()-1..];
        //     assert!(end == "l" || end == "r" || end == "n", "Coalescing masculine nouns must end in L, R or N");
        //     let mut i = s.len()-2;

        //     if s.get(i-1..i).and_then(|a| s.get(i-2..i-1).map(|b| a==b)).unwrap_or(false) {
        //         i -= 1;
        //     }

        //     Root {
        //         first_stem: s[..i].to_owned(),
        //         deletable: s[i..s.len()-1].to_owned(),
        //         end: end.to_owned(),
        //         patalised: false,
        //     }
        // }
    // }
    fn nom_sg<'r>(&self, r: &'r Root) -> Inflection<'r> { Inflection::new(r.full(), if r.deletable.is_empty() || r.skerping() { "ur" } else { "" } ) }
    fn acc_sg<'r>(&self, r: &'r Root) -> Inflection<'r> { Inflection::new(r.full(), "") }
    fn dat_sg<'r>(&self, r: &'r Root) -> Inflection<'r> { Inflection::new(r.vowelled().drop_yod(), "i") }
    fn gen_sg<'r>(&self, r: &'r Root) -> Inflection<'r> { Inflection::new(r.consonanted(), "s") }
    fn nom_pl<'r>(&self, r: &'r Root) -> Inflection<'r> { Inflection::new(r.vowelled().drop_yod_if_palatalised(), if r.palatalised() { "jar" } else { "ar" }) }
}
pub struct MascArIr;

impl Noun for MascArIr {
    fn nom_sg<'r>(&self, r: &'r Root) -> Inflection<'r> { Inflection::new(r.full(), if r.deletable.is_empty() || r.skerping() { "ur" } else { "" } ) }
    fn acc_sg<'r>(&self, r: &'r Root) -> Inflection<'r> { Inflection::new(r.full(), "") }
    fn dat_sg<'r>(&self, r: &'r Root) -> Inflection<'r> { Inflection::new(r.vowelled().drop_yod(), "i") }
    fn gen_sg<'r>(&self, r: &'r Root) -> Inflection<'r> { Inflection::new(r.vowelled().drop_yod_if_palatalised(), if r.palatalised() { "jar" } else { "ar" }) }
    fn nom_pl<'r>(&self, r: &'r Root) -> Inflection<'r> { Inflection::new(r.vowelled().drop_yod(), "ir") }
}
pub struct MascSIr;

impl Noun for MascSIr {
    fn nom_sg<'r>(&self, r: &'r Root) -> Inflection<'r> { Inflection::new(r.full(), if r.deletable.is_empty() || r.skerping() { "ur" } else { "" } ) }
    fn acc_sg<'r>(&self, r: &'r Root) -> Inflection<'r> { Inflection::new(r.full(), "") }
    fn dat_sg<'r>(&self, r: &'r Root) -> Inflection<'r> { Inflection::new(r.vowelled().drop_yod(), "i") }
    fn gen_sg<'r>(&self, r: &'r Root) -> Inflection<'r> { Inflection::new(r.consonanted(), "s") }
    fn nom_pl<'r>(&self, r: &'r Root) -> Inflection<'r> { Inflection::new(r.vowelled().drop_yod(), "ir") }
}

pub struct Neuter;

impl Noun for Neuter {
    // fn parse(&self, s: &str) -> Root {
        // if s.ends_with('i') {
        //     let first_stem = s[..s.len()-1].to_owned();
        //     Root {
        //         patalised: first_stem.ends_with(|c| c == 'g' || c == 'k'),
        //         first_stem,
        //         deletable: "i".to_owned(),
        //         end: String::new(),
        //     }
        // } else if s.ends_with(|c| c == 'l' || c == 'r' || c == 'n') {
        //     let end = &s[s.len()-1..];
        //     let mut i = s.len()-2;

        //     if s.get(i-1..i).and_then(|a| s.get(i-2..i-1).map(|b| a==b)).unwrap_or(false) {
        //         i -= 1;
        //     }

        //     Root {
        //         first_stem: s[..i].to_owned(),
        //         deletable: s[i..s.len()-1].to_owned(),
        //         end: end.to_owned(),
        //         patalised: false,
        //     }
        // } else {
        //     Root {
        //         first_stem: s.to_owned(),
        //         deletable: String::new(),
        //         end: String::new(),
        //         patalised: false,
        //     }
        // }
    // }
    fn nom_sg<'r>(&self, r: &'r Root) -> Inflection<'r> { Inflection::new(r.full(), "" ) }
    fn acc_sg<'r>(&self, r: &'r Root) -> Inflection<'r> { Inflection::new(r.full(), "") }
    fn dat_sg<'r>(&self, r: &'r Root) -> Inflection<'r> { Inflection::new(r.vowelled().drop_yod(), "i") }
    fn gen_sg<'r>(&self, r: &'r Root) -> Inflection<'r> { Inflection::new(r.consonanted(), "s") }
    // U-umlaut
    fn nom_pl<'r>(&self, r: &'r Root) -> Inflection<'r> { Inflection::new(r.full(), if r.deletable == "i" { "(r)" } else { "" }) }
}
