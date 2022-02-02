use inflector::nouns::*;

// fn 

fn main() {
    dothing("vin", MascArIr);
    dothing("veg", MascArIr);
    dothing("stað", MascArIr);
    dothing("gest", MascSIr);
    dothing("blett", MascSIr);
    dothing("hval", MascSIr);
    dothing("dans", MascSIr);
    
    dothing("veggʲ", MascArIr);
    dothing("ryggʲ", MascSIr);
    dothing("hylʲ", MascSAr);
    dothing("úlv", MascSAr);
    dothing("drongʲ", MascSIr);
    dothing("hey(g)ˠ", MascSAr);
    dothing("sjó(v)ˠ", MascSAr);
    dothing("skóˠ", MascSAr);
    dothing("skó(g)ˠ", MascArIr);


    dothing("vøkst(u)r", MascArIr);
    dothing("vøkst", MascArIr);
    dothing("meld(u)r", MascSAr);
    dothing("ak(u)r", MascSAr);
    dothing("stuð(u)l", MascSAr);
    dothing("him(ma)l", MascSAr);

    dothing("hand(i)l", MascSAr);
    dothing("ham(a)r", MascSAr);
    dothing("sum(ma)r", Neuter);
    dothing("heyst", Neuter);
    dothing("veð(u)r", Neuter);
    dothing("epl(i)", Neuter);
    dothing("tíðind(i)", Neuter);
    dothing("merk(i)ʲ", Neuter);
}

fn dothing<N: Noun>(form: &str, n: N) {
    let root = n.parse(form);
    println!("{:#}", Table(&root, n));
}
