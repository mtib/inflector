use crate::{Inflection, Root, WordForm};

pub fn table<N: Noun>(r: &Root) -> String {
    let sg = "sg";
    let pl = "pl";

    let nom_sg = N::nom_sg(&r);
    let acc_sg = N::acc_sg(&r);
    let dat_sg = N::dat_sg(&r);
    let gen_sg = N::gen_sg(&r);
    let nom_pl = N::nom_pl(&r);
    let acc_pl = N::acc_pl(&r);
    let dat_pl = N::dat_pl(&r);
    let gen_pl = N::gen_pl(&r);

    let sg_len = (nom_sg.len().max(acc_sg.len())).max(dat_sg.len().max(gen_sg.len())).max(sg.len());
    let pl_len = (nom_pl.len().max(acc_pl.len())).max(dat_pl.len().max(gen_pl.len())).max(pl.len());

    format!(r#"    | {sg:sg_len$} | {pl:pl_len$} |
nom | {nom_sg:sg_len$} | {nom_pl:pl_len$} |
acc | {acc_sg:sg_len$} | {acc_pl:pl_len$} |
dat | {dat_sg:sg_len$} | {dat_pl:pl_len$} |
gen | {gen_sg:sg_len$} | {gen_pl:pl_len$} |
"#)
}

pub trait Noun {
    fn nom_sg(r: &Root) -> Inflection;
    fn acc_sg(r: &Root) -> Inflection;
    fn dat_sg(r: &Root) -> Inflection;
    fn gen_sg(r: &Root) -> Inflection;
    fn nom_pl(r: &Root) -> Inflection;
    fn acc_pl(r: &Root) -> Inflection { Self::nom_pl(r) }
    fn dat_pl(r: &Root) -> Inflection { Inflection(r.contracted(), if r.patalised { "jum" } else { "um" }) }
    fn gen_pl(r: &Root) -> Inflection { Inflection(r.contracted(), if r.patalised { "ja" } else { "a" }) }
}

pub struct MascSAr;

impl Noun for MascSAr {
    fn nom_sg(r: &Root) -> Inflection { Inflection(r.full(), if r.deletable.is_empty() { "ur" } else { "" } ) }
    fn acc_sg(r: &Root) -> Inflection { Inflection(r.full(), "") }
    fn dat_sg(r: &Root) -> Inflection { Inflection(r.contracted(), "i") }
    fn gen_sg(r: &Root) -> Inflection { Inflection(r.full(), "s") }
    fn nom_pl(r: &Root) -> Inflection { Inflection(r.contracted(), if r.patalised { "jar" } else { "ar" }) }
}
pub struct MascArIr;

impl Noun for MascArIr {
    fn nom_sg(r: &Root) -> Inflection { Inflection(r.full(), if r.deletable.is_empty() { "ur" } else { "" } ) }
    fn acc_sg(r: &Root) -> Inflection { Inflection(r.full(), "") }
    fn dat_sg(r: &Root) -> Inflection { Inflection(r.contracted(), "i") }
    fn gen_sg(r: &Root) -> Inflection { Inflection(r.full(), if r.patalised { "jar" } else { "ar" }) }
    fn nom_pl(r: &Root) -> Inflection { Inflection(r.contracted(), "ir") }
}
pub struct MascSIr;

impl Noun for MascSIr {
    fn nom_sg(r: &Root) -> Inflection { Inflection(r.full(), if r.deletable.is_empty() { "ur" } else { "" } ) }
    fn acc_sg(r: &Root) -> Inflection { Inflection(r.full(), "") }
    fn dat_sg(r: &Root) -> Inflection { Inflection(r.contracted(), "i") }
    fn gen_sg(r: &Root) -> Inflection { Inflection(r.full(), "s") }
    fn nom_pl(r: &Root) -> Inflection { Inflection(r.contracted(), "ir") }
}

pub struct Neuter;

impl Noun for Neuter {
    fn nom_sg(r: &Root) -> Inflection { Inflection(r.full(), "" ) }
    fn acc_sg(r: &Root) -> Inflection { Inflection(r.full(), "") }
    fn dat_sg(r: &Root) -> Inflection { Inflection(r.contracted(), "i") }
    fn gen_sg(r: &Root) -> Inflection { Inflection(r.full(), "s") }
    // U-umlaut
    fn nom_pl(r: &Root) -> Inflection { Inflection(r.full(), if r.deletable == "i" { "(r)" } else { "" }) }
}
