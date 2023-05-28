use serde::{Deserialize, Serialize};


/* // a notatioun/symbol for this/current status/position
pub const AS_Current:&str = ".";*/

/// GeneralSymbol:
/// symbols/1-2 characters representing general position concept
/// Current  - a notatioun/symbol for this/current status/position
/// UpLevel - a notatioun/symbol for one level up, parent/ancestor Path
#[derive(Debug)]
pub enum GeneralSymbol {
    Current,
    UpLevel,
    Home,
    Never,
    Path,
    Back,
    Next,
}

impl GeneralSymbol{
    pub fn symbol(&self) -> String {
        match *self {
            GeneralSymbol::Current => ".".to_string(),
            GeneralSymbol::UpLevel => "..".to_string(),
            GeneralSymbol::Home => "~".to_string(),
            GeneralSymbol::Never => "!".to_string(),
            GeneralSymbol::Path => "/".to_string(),
            GeneralSymbol::Back => "<".to_string(),
            GeneralSymbol::Next => ">".to_string(),
        }
    }
}

/// Domain
#[derive(Debug)]
pub enum Domain {
    Current,
    Common,
    Security,
}

impl Domain{
/*     pub fn new_born(code: &str) -> Domain {
        match code {
            GeneralSymbol::Current.symbol() => Domain::Current,
            "CMN" => Domain::Common,
            "SQT" => Domain::SECURITY,
        }
    } */

    pub fn code(&self) -> String {
        match *self {
            Domain::Current => GeneralSymbol::Current.symbol(),
            Domain::Common => "CMN".to_string(),
            Domain::Security => "SQT".to_string(),
        }
    }
}

/// Relationship
#[derive(Debug)]
pub enum Relationship {
    Current,
    IsA,
    HasA,
    FromTo,
}

/// OnDataAvailStrategy:
/// strategy when specific data/value is found/available vs. not found/unavailable
/// e.g. used in (EnvProp) app_properties.json (Inherit from upper level if not defined)
/// DefaultOnUnavail -  fall Back to the default when no match is found,
/// ErrOnUnavil - matched value is required; error/exception when no match is found, 
/// Inherit      - not to define strategy at this level; Inherit strategy from upper level instead 
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum OnDataAvailStrategy {
    Inherit,
    SilentOnUnavil,
    DefaultOnUnavail,
    ErrOnUnavil,
}

/// ErrorOnthefly - 
/// simply a custom enum for errors, purposely not to name as Error / Err
/// to differentiate itself from common libraries
/// unnamed String elements are for field name or other description
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum ErrorOnthefly {
    /// date read errors
    RecordNotFound,
    TooManyRecordFound{no_of_rec_expected:String,no_of_rec_found:String},
    /// data write/input errors
    RecordNotFoundForUpdate,
    TooManyRecordFoundForUpdate{no_of_rec_expected:String,no_of_rec_found:String},
    DuplicateKey,
    RequiredFieldBlank(String),
    UnacceptedValue(String),
    /// io problem
    IOProblem(String),
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




/* /// AppErr
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum AppErr {
    (DataErr),
}
 */
/* 
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
} */