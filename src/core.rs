type Typeidx = u32;

#[derive(Debug, PartialEq)]
pub enum ValType {
    i32, i64, f32, f64,
}

pub type FuncType = (Vec<ValType>, Vec<ValType>);

pub enum ImportDesc {
    Func(Typeidx),
    Table,
    Mem,
    Global,
}

#[derive(Default)]
pub struct Module {
    pub types: Vec<FuncType>, 
}

pub struct Context {
    types: Vec<FuncType>,
    funcs: (),
    tables: (),
    mems: (),
    globals: (),
    locals: (),
    labels: (),
    r#return: (),
}

impl Context {
    pub fn new(types: Vec<FuncType>) -> Self {
        Context {
            types: types,
            funcs: (),
            tables: (),
            mems: (),
            globals: (),
            locals: (),
            labels: (),
            r#return: (),
        }
    }
}
