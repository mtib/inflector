use std::{fmt::{self, Display}};

pub mod nouns;

pub trait WordForm: Display {
    fn len(&self) -> usize;
}

impl<'a, T: WordForm> WordForm for &'a T {
    fn len(&self) -> usize {
        (*self).len()
    }
}

#[derive(Clone)]
pub struct Inflection<'a>(RootForm<'a>, &'a str);

impl<'a> Display for Inflection<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let width = f.width().unwrap_or(0).saturating_sub(self.0.len());
        write!(f, "{}{:width$}", self.0, self.1)
    }
}

impl<'a> WordForm for Inflection<'a> {
    fn len(&self) -> usize {
        self.0.len() + self.1.len()
    }
}

#[derive(Debug, Default, Clone)]
pub struct Root {
    pub first_stem: String,
    pub deletable: String,
    pub end: String,
    pub patalised: bool,
}

impl Root {
    pub fn parse(mut s: &str) -> Self {
        let patalised = s.ends_with("j");
        if patalised {
            s = &s[..s.len()-1];
        }

        let start = s.find('(');
        let end = s.find(')');

        if let (Some(start), Some(end)) = (start, end) {
            Root {
                first_stem: s[..start].to_owned(),
                deletable: s[start+1..end].to_owned(),
                end: s[end+1..].to_owned(),
                patalised,
            }
        } else {
            Root {
                first_stem: s.to_owned(),
                patalised,
                .. Default::default()
            }
        }
    }
    pub fn full(&self) -> RootForm {
        RootForm::new(self, false)
    }
    pub fn contracted(&self) -> RootForm {
        RootForm::new(self, true)
    }
}

#[derive(Debug, Clone)]
pub struct RootForm<'a>{
    root: &'a Root,
    contracted: bool,
}

impl<'a> RootForm<'a> {
    fn new(root: &'a Root, contracted: bool) -> Self {
        RootForm { root, contracted }
    }
}

impl<'a> Display for RootForm<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut width = f.width().unwrap_or(0);
        write!(f, "{}", self.root.first_stem)?;
        width = width.saturating_sub(self.root.first_stem.len());
        if !self.contracted {
            write!(f, "{}", self.root.deletable)?;
            width = width.saturating_sub(self.root.deletable.len());
        }
        write!(f, "{:width$}", self.root.end)
    }
}

impl<'a> WordForm for RootForm<'a> {
    fn len(&self) -> usize {
        let l = self.root.first_stem.len() + self.root.end.len();
        if self.contracted {
            l
        } else {
            l + self.root.deletable.len()
        }
    }
}
