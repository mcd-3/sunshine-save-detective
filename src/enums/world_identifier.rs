#[derive(Debug, Clone, Copy)]
pub enum WorldIdentifier {
    DelfinoId,
    BiancoId,
    RiccoId,
    GelatoId,
    PinnaId,
    SirenaId,
    NokiId,
    PiantaId,
    CoronaId,
}

impl Default for WorldIdentifier {
    fn default() -> Self {
        WorldIdentifier::DelfinoId
    }
}