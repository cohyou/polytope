// use std::io::Read;
use super::limits::Limits;

pub(crate) struct MemType(Limits);

impl MemType {
    pub(crate) fn new(limits: Limits) -> Self {
        MemType(limits)
    }
}
