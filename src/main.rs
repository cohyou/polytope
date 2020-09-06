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

    println!("{:?}", pt);
}
