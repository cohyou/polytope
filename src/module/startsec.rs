use std::io::Read;

use super::idx::{Funcidx, read_funcidx};


pub(super) struct Start(Funcidx);


pub(super) fn read_startsec(reader: &mut impl Read) -> Start {
    // prefixはsection number 8
    read_start(reader)
}

fn read_start(reader: &mut impl Read) -> Start {
    Start(read_funcidx(reader))
}