#[derive(Debug, Clone, Copy)]
pub enum Region {
    NTSCU,
    NTSCJ,
    PAL,
    INVALID
}

impl Default for Region {
    fn default() -> Self {
        Region::INVALID
    }
}