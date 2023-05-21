use serde::{Deserialize, Serialize};

/* // a notatioun/symbol for this/current status/position
pub const AS_CURRENT:&str = ".";*/

/// GeneralSymbol:
/// symbols/1-2 characters representing general position concept
/// CURRENT  - a notatioun/symbol for this/current status/position
/// UP_LEVEL - a notatioun/symbol for one level up, parent/ancestor path
#[derive(Debug)]
pub enum GeneralSymbol {
    CURRENT,
    UP_LEVEL,
    HOME,
    NEVER,
    PATH,
    BACK,
    NEXT,
}

impl GeneralSymbol{
    pub fn symbol(&self) -> String {
        match *self {
            GeneralSymbol::CURRENT => ".".to_string(),
            GeneralSymbol::UP_LEVEL => "..".to_string(),
            GeneralSymbol::HOME => "~".to_string(),
            GeneralSymbol::NEVER => "!".to_string(),
            GeneralSymbol::PATH => "/".to_string(),
            GeneralSymbol::BACK => "<".to_string(),
            GeneralSymbol::NEXT => ">".to_string(),
        }
    }
}

/// Domain
#[derive(Debug)]
pub enum Domain {
    CURRENT,
    COMMON,
    SECURITY,
}

impl Domain{
/*     pub fn new_born(code: &str) -> Domain {
        match code {
            GeneralSymbol::CURRENT.symbol() => Domain::CURRENT,
            "CMN" => Domain::COMMON,
            "SQT" => Domain::SECURITY,
        }
    } */

    pub fn code(&self) -> String {
        match *self {
            Domain::CURRENT => GeneralSymbol::CURRENT.symbol(),
            Domain::COMMON => "CMN".to_string(),
            Domain::SECURITY => "SQT".to_string(),
        }
    }
}

/// Relationship
#[derive(Debug)]
pub enum Relationship {
    CURRENT,
    IS_A,
    HAS_A,
    FROM_TO,
}

/// OnDataAvailStrategy:
/// strategy when specific data/value is found/available vs. not found/unavailable
/// e.g. used in (EnvProp) app_properties.json (INHERIT from upper level if not defined)
/// DEFAULT_ON_UNAVAIL -  fall back to the default when no match is found,
/// ERR_ON_UNAVAIL - matched value is required; error/exception when no match is found, 
/// INHERIT      - not to define strategy at this level; INHERIT strategy from upper level instead 
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum OnDataAvailStrategy {
    INHERIT,
    SILENT_ON_UNAVIL,
    DEFAULT_ON_UNAVAIL,
    ERR_ON_UNAVAIL,
}

/// DataReadErr
#[derive(Debug)]
pub enum DataReadErr {
    RecordNotFound,
    TooManyRecordFound{no_of_rec_expected:String,no_of_rec_found:String},
    StorageReadProblem(String),
}
/// Error enum for data writing errors
///
/// unnamed String carbonas are for field name or other description
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