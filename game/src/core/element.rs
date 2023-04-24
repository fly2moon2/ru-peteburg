// a notatioun/symbol for current status/position
const NOTATION_CURRENT: String = ".";

#[derive(Debug)]
pub enum TwoPointDirection {
    From,
    To,
    Nil,
}

#[derive(Debug)]
pub enum CompassDirection {
    North,
    East,
    South,
    West,
    Central,
}

#[derive(Debug)]
pub enum DataType {
    Number,
    String,
}

#[derive(Debug,Clone)]
pub struct DataSpecs {
    type: DataType,
    size: i8,
    required: bool,
}