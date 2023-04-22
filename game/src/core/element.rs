pub enum TwoPointDirection {
    From,
    To,
    Nil,
}

pub enum CompassDirection {
    North,
    East,
    South,
    West,
    Central,
}

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