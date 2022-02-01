use inflector::{*, nouns::*};

fn main() {
    dothing::<MascArIr>("vin");
    dothing::<MascArIr>("veg");
    dothing::<MascArIr>("stað");
    dothing::<MascSIr>("gest");
    dothing::<MascSIr>("blett");
    dothing::<MascSIr>("hval");
    dothing::<MascSIr>("dans");
    
    dothing::<MascArIr>("veggj");
    dothing::<MascSIr>("ryggj");
    dothing::<MascSAr>("hylj");
    dothing::<MascSIr>("dreingj");
    dothing::<MascSAr>("heyggj");
    

    dothing::<MascSAr>("meld(u)r");
    dothing::<MascSAr>("ak(u)r");
    dothing::<MascSAr>("stuð(u)l");
    dothing::<MascSAr>("him(ma)l");

    dothing::<MascSAr>("hand(i)l");
    dothing::<MascSAr>("ham(a)r");
    dothing::<Neuter>("sum(ma)r");
    dothing::<Neuter>("heyst");
    dothing::<Neuter>("veð(u)r");
    dothing::<Neuter>("epl(i)");
    dothing::<Neuter>("tíðind(i)");
    dothing::<Neuter>("merk(i)j");
}

fn dothing<N: Noun>(s: &str) {
    let root = Root::parse(s);
    println!("{}", table::<N>(&root));
}

fn parse(s: &str) -> (Root, String) {
    let end = s.find('-');
    if let Some(end) = end {
        (Root::parse(&s[..end]), s[end+1..].to_owned())
    } else {
        (Root::parse(s), String::new())
    }
}

fn make_table(root: Root, nom_ending: String) -> String {
    let end_len = nom_ending.len();
    let sg_len = (root.full().len()+end_len).max(root.full().len()+1);
    let pl_len = root.contracted().len()+2;

    format!(
        r#"{f:sg_len$}{nom_ending} | {c:pl_len$}ar
{f:sg_len$} | {c:pl_len$}ar
{c:sg_len$}i | {c:pl_len$}um
{f:sg_len$}s | {c:pl_len$}a
"#, f=root.full(), c=root.contracted())
}
