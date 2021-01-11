#[derive(Default)]
struct Module {
    types: Vec<FuncType>, 
}

struct Context {
    types: Vec<FuncType>,
    funcs: (),
    tables: (),
    mems: (),
    globals: (),
    locals: (),
    labels: (),
    r#return: (),
}