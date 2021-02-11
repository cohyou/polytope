mod reader;

pub(super) use reader::read_instr;

use std::rc::Rc;
use std::cell::RefCell;

use crate::valtype::ValType;
use crate::functype::FuncType;
use super::expr::Expr;
use super::idx::{Labelidx, Funcidx, Localidx, Globalidx, Typeidx};

type ResultType = Vec<ValType>;

type Addr = usize;  // 仕様は自由なのでひとまずusize
type FuncAddr = Addr;
type TableAddr = Addr;
type MemAddr = Addr;
type GlobalAddr = Addr;

#[derive(Clone)]
pub(super) struct MemArg {
    align: u32,
    offset: u32,
}

enum ExternVal {
    Func(FuncAddr),
    Table(TableAddr),
    Mem(MemAddr),
    Global(GlobalAddr),
}

struct ExportInst { name: String, value: ExternVal }

struct ModuleInst {
    pub types: Vec<FuncType>,
    pub func_addrs: Vec<FuncAddr>,
    pub table_addrs: Vec<TableAddr>,
    pub mem_addrs: Vec<MemAddr>,
    pub global_addrs: Vec<GlobalAddr>,
    pub exports: Vec<ExportInst>,
}

#[derive(Clone)]
enum Val {
    I32Const(u32),
    I64Const(u64),
    F32Const(f32),
    F64Const(f64),
}

#[derive(Clone)]
pub(super) struct Frame {
    locals: Vec<Val>,
    module: Rc<RefCell<ModuleInst>>,
}

#[derive(Clone)]
pub(super) enum BlockType {
    Vt(Option<ValType>),
    X(Typeidx),
}

#[derive(Clone)]
pub(super) enum Instr {
    /* Block Instructions */

    // Control Instructions
    Block(BlockType, Expr),
    Loop(BlockType, Expr),
    If(BlockType, Expr, Option<Expr>),

    /* Plain Instructions */

    // Control Instructions
    Unreachable,
    Nop,
    Br(Labelidx),
    BrIf(Labelidx),
    BrTable(Vec<Labelidx>, Labelidx),
    Return,
    Call(Funcidx),
    CallIndirect(Funcidx),

    // Parametric Instructions
    Drop,
    Select,

    // Variable Instructions
    LocalGet(Localidx),
    LocalSet(Localidx),
    LocalTee(Localidx),
    GlobalGet(Globalidx),
    GlobalSet(Globalidx),

    // Memory Instructions
    Load(ValType, MemArg),
    Store(ValType, MemArg),
    ILoad8(ValSize, ValSign, MemArg),
    ILoad16(ValSize, ValSign, MemArg),
    I64Load32(ValSign, MemArg),
    IStore8(ValSize, MemArg),
    IStore16(ValSize, MemArg),
    I64Store32(MemArg),
    MemorySize,
    MemoryGrow,

    // Numeric Instructions
    I32Const(i32),
    I64Const(i64),
    F32Const(f32),
    F64Const(f64),

    IUnOp(ValSize, IUnOp),
    FUnOp(ValSize, FUnOp),

    IBinOp(ValSize, IBinOp),
    FBinOp(ValSize, FBinOp),

    ITestOp(ValSize, ITestOp),

    IRelOp(ValSize, IRelOp),
    FRelOp(ValSize, FRelOp),

    CvtOp(CvtOp),

    // Administrative Instructions
    Trap,
    Invoke(FuncAddr),
    InitElem(TableAddr, u32, Vec<Funcidx>),
    InitData(MemAddr, u32, Vec<u8>),
    Label(usize, Vec<Instr>, Vec<Instr>),
    Frame(usize, Frame, Vec<Instr>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum ValSize { V32, V64 }

#[derive(Debug, PartialEq, Clone)]
pub enum ValSign { U, S }

#[derive(Debug, Clone, PartialEq)]
pub enum IUnOp { Clz, Ctz, Popcnt, }

#[derive(Debug, Clone, PartialEq)]
pub enum IBinOp {
    Add, Sub, Mul, Div(ValSign), Rem(ValSign),
    And, Or, Xor, Shl, Shr(ValSign), Rotl, Rotr,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FUnOp { Abs, Neg, Sqrt, Ceil, Floor, Trunc, Nearest, }

#[derive(Debug, Clone, PartialEq)]
pub enum FBinOp { Add, Sub, Mul, Div, Min, Max, Copysign, }

#[derive(Debug, Clone, PartialEq)]
pub enum ITestOp { Eqz, }

#[derive(Debug, Clone, PartialEq)]
pub enum IRelOp { Eq, Ne, Lt(ValSign), Gt(ValSign), Le(ValSign), Ge(ValSign), }

#[derive(Debug, Clone, PartialEq)]
pub enum FRelOp { Eq, Ne, Lt, Gt, Le, Ge, }

#[derive(Debug, Clone, PartialEq)]
pub enum CvtOp {
    I32WrapFromI64,
    I64ExtendFromI32(ValSign),
    ITruncFromF(ValSize, ValSize, ValSign),
    F32DemoteFromF64,
    F64PromoteFromF32,
    FConvertFromI(ValSize, ValSize, ValSign),
    IReinterpretFromF(ValSize),
    FReinterpretFromI(ValSize),
}

