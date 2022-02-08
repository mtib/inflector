use std::fmt::{self, Display};
use wasm_bindgen::prelude::*;

mod filters;
pub mod nouns;
pub use filters::*;

pub trait WordForm: Display {
    fn char_count(&self) -> usize;
}

impl WordForm for str {
    fn char_count(&self) -> usize {
        self.chars().count()
    }
}

impl WordForm for String {
    fn char_count(&self) -> usize {
        self.chars().count()
    }
}

impl<'a, T: WordForm> WordForm for &'a T {
    fn char_count(&self) -> usize {
        (*self).char_count()
    }
}

#[derive(Clone)]
pub struct Inflection<'a> {
    r: RootForm<'a>,
    e: &'a str,
}

impl<'a> Inflection<'a> {
    pub fn new(r: RootForm<'a>, e: &'a str) -> Self {
        Inflection { r, e }
    }
}

impl<'a> Display for Inflection<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let width = f.width().unwrap_or(0).saturating_sub(self.r.char_count());
        write!(f, "{}{:width$}", self.r, self.e)
    }
}

impl<'a> WordForm for Inflection<'a> {
    fn char_count(&self) -> usize {
        self.r.char_count() + self.e.char_count()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Extra {
    Nothing,
    Skerping,
    Palatalised,
}

impl Default for Extra {
    fn default() -> Self {
        Extra::Nothing
    }
}

#[derive(Debug, Clone, Default)]
pub struct Root {
    pub first_stem: String,
    pub deletable: String,
    pub end: String,
    pub extra: Extra,
}

const PATALISER_MARKER: &str = "ʲ";
const SKERPING_MARKER: &str = "ˠ";

#[wasm_bindgen]
pub fn parse_root(s: &str) -> String {
    format!("{:?}", Root::parse(s))
}

impl Display for Root {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "`{}", self.first_stem)?;

        if !self.deletable.is_empty() || !self.end.is_empty() {
            write!(f, "({}){}", self.deletable, self.end)?;
        }

        match self.extra {
            Extra::Nothing => (),
            Extra::Palatalised => write!(f, "{}", PATALISER_MARKER)?,
            Extra::Skerping => write!(f, "{}", SKERPING_MARKER)?,
        }

        write!(f, "`")
    }
}

impl Root {
    pub fn parse(mut s: &str) -> Self {
        let extra = if s.ends_with(PATALISER_MARKER) {
            s = &s[..s.len() - PATALISER_MARKER.len()];
            Extra::Palatalised
        } else if s.ends_with(SKERPING_MARKER) {
            s = &s[..s.len() - SKERPING_MARKER.len()];
            Extra::Skerping
        } else {
            Extra::Nothing
        };

        let start = s.find('(');
        let end = s.find(')');

        if let (Some(start), Some(end)) = (start, end) {
            Root {
                first_stem: s[..start].to_owned(),
                deletable: s[start + 1..end].to_owned(),
                end: s[end + 1..].to_owned(),
                extra,
            }
        } else {
            Root {
                first_stem: s.to_owned(),
                extra,
                ..Self::default()
            }
        }
    }
    pub fn full(&self) -> RootForm {
        RootForm::new(self, Form::Full)
    }
    pub fn consonanted(&self) -> RootForm {
        RootForm::new(self, Form::ConsonantAdded)
    }
    pub fn vowelled(&self) -> RootForm {
        RootForm::new(self, Form::VowelAdded)
    }
    pub fn palatalised(&self) -> bool {
        matches!(self.extra, Extra::Palatalised)
    }
    pub fn skerping(&self) -> bool {
        matches!(self.extra, Extra::Skerping)
    }
}

#[derive(Debug, Clone, Copy)]
enum Form {
    Full,
    ConsonantAdded,
    VowelAdded,
}

#[derive(Debug, Clone, Copy)]
pub struct RootForm<'a> {
    root: &'a Root,
    form: Form,
    drop_yod: bool,
}

impl<'a> RootForm<'a> {
    fn new(root: &'a Root, form: Form) -> Self {
        RootForm {
            root,
            form,
            drop_yod: false,
        }
    }
    pub fn drop_yod(self) -> Self {
        RootForm {
            drop_yod: true,
            ..self
        }
    }
    pub fn drop_yod_if_palatalised(self) -> Self {
        RootForm {
            drop_yod: self.root.palatalised(),
            ..self
        }
    }
    fn skerping(&self) -> &'static str {
        let mut skerping = filters::skerping(self.root.first_stem.chars().last().unwrap()).unwrap();
        if skerping.ends_with('j') && self.drop_yod {
            skerping = &skerping[..skerping.len() - 1];
        }
        skerping
    }
}

impl<'a> Display for RootForm<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut width = f.width().unwrap_or(0);
        write!(f, "{}", self.root.first_stem)?;
        width = width.saturating_sub(self.root.first_stem.len());

        match (self.form, self.root.skerping()) {
            (Form::Full | Form::VowelAdded, true) => {
                let skerping = self.skerping();
                write!(f, "{}", skerping)?;
                width = width.saturating_sub(skerping.len());
            }
            (Form::ConsonantAdded, true) => {
                write!(f, "{}", self.root.deletable)?;
                width = width.saturating_sub(self.root.deletable.len());
            }
            (Form::Full | Form::ConsonantAdded, false) => {
                write!(f, "{}", self.root.deletable)?;
                width = width.saturating_sub(self.root.deletable.len());
            }
            (Form::VowelAdded, false) => (),
        }
        write!(f, "{:width$}", self.root.end)
    }
}

impl<'a> WordForm for RootForm<'a> {
    fn char_count(&self) -> usize {
        let l = self.root.first_stem.char_count() + self.root.end.char_count();

        match (self.form, self.root.skerping()) {
            (Form::Full | Form::VowelAdded, true) => l + self.skerping().char_count(),
            (Form::ConsonantAdded, true) | (Form::Full | Form::ConsonantAdded, false) => {
                l + self.root.deletable.char_count()
            }
            (Form::VowelAdded, false) => l,
        }
    }
}
