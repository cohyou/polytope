use polytope::*;

fn main() {
    let mut pt = Polytope::new();

    pt.make_one("false");
    pt.make_one("true");

    let f = pt.get_apg("false");
    let t = pt.get_apg("true");

    let inl = pt.make_morphism_coproduct(f.clone());
    let inr = pt.make_morphism_coproduct(t.clone());

    pt.make_coproduct("bool", f, t, inl, inr);

    pt.make_one("small");
    pt.make_one("medium");
    pt.make_one("large");

    let s = pt.get_apg("small");
    let m = pt.get_apg("medium");
    let l = pt.get_apg("large");

    let inl = pt.make_morphism_coproduct(s.clone());
    let inr = pt.make_morphism_coproduct(m.clone());

    pt.make_coproduct("sm", s, m, inl, inr);

    let sm = pt.get_apg("sm");

    let inl = pt.make_morphism_coproduct(sm.clone());
    let inr = pt.make_morphism_coproduct(l.clone());
    pt.make_coproduct("size", sm, l, inl, inr);


    let bool_ = pt.get_apg("bool");
    let size = pt.get_apg("size");
    let inl = pt.make_morphism_product(bool_.clone());
    let inr = pt.make_morphism_product(size.clone());
    pt.make_product("bool*size", bool_, size, inl, inr);

    println!("{:?}", pt);
}
