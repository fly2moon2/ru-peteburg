// a notatioun/symbol for this/current status/position
pub const AS_THIS:&str = ".";
// a notatioun/symbol for one level up, parent/ancestor path
pub const AS_UP_LEVEL:&str = "..";
// a notatioun/symbol for home path
pub const AS_HOME:&str = "~";
// a notatioun/symbol for never
pub const AS_NEVER_SEPARATOR:&str = "!";
// a notatioun/symbol for path separator (Unix/Linux standard)
pub const AS_PATH_SEPARATOR:&str = "/";

/// Error enum for data reading errors
///
/// unnamed String elements are for field name or other description
#[derive(Debug)]
pub enum DataReadErr {
    RecordNotFound,
    TooManyRecordFound{no_of_rec_expected:String,no_of_rec_found:String},
    StorageReadProblem(String),
}

/// Error enum for data writing errors
///
/// unnamed String elements are for field name or other description
#[derive(Debug)]
pub enum DataWriteErr {
    RecordNotFoundForUpdate,
    TooManyRecordFoundForUpdate{no_of_rec_expected:String,no_of_rec_found:String},
    DuplicateKey,
    RequiredFieldBlank(String),
    UnacceptedValue(String),
    StorageWriteProblem(String),
}

#[derive(Debug)]
pub enum DataErr {
    CannotRead(DataReadErr),
    CannotWrite(DataWriteErr),
}

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
pub enum DataTyp {
    Number,
    String,
}

#[derive(Debug)]
pub struct DataSpecs {
    typ: DataTyp,
    size: u8,
    required: bool,
}