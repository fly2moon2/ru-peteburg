/* // a notatioun/symbol for this/current status/position
pub const AS_CURRENT:&str = ".";*/

/// GeneralPosSymbol:
/// symbols/1-2 characters representing general position concept
/// CURRENT  - a notatioun/symbol for this/current status/position
/// UP_LEVEL - a notatioun/symbol for one level up, parent/ancestor path
#[derive(Debug)]
pub enum GeneralPosSymbol {
    CURRENT,
    UP_LEVEL,
    HOME,
    NEVER,
    PATH,
}

impl GeneralPosSymbol{
    pub fn value(&self) -> String {
        match *self {
            GeneralPosSymbol::CURRENT => ".".to_string(),
            GeneralPosSymbol::UP_LEVEL => "..".to_string(),
            GeneralPosSymbol::HOME => "~".to_string(),
            GeneralPosSymbol::NEVER => "!".to_string(),
            GeneralPosSymbol::PATH => "/".to_string(),
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
    pub fn value(&self) -> String {
        match *self {
            Domain::CURRENT => GeneralPosSymbol::CURRENT.value(),
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

/// OnNoMatchStrategy:
/// strategy when given value is not found/matched
/// e.g. used in (EnvProp) app_properties.json (Inherit from upper level if not defined)
/// DefaultOnNoMatch -  fall back to the default when no match is found,
/// ErrOnNoMatch - matched value is required; error/exception when no match is found, 
/// Inherit      - not to define strategy at this level; inherit strategy from upper level instead 
#[derive(Debug)]
pub enum OnNoMatchStrategy {
    Inherit,
    DefaultOnNoMatch,
    ErrOnNoMatch,
}

impl OnNoMatchStrategy{
    pub fn value(&self) -> String {
        match *self {
            OnNoMatchStrategy::Inherit => "Inherit".to_string(),
            OnNoMatchStrategy::DefaultOnNoMatch => "Default on no match".to_string(),
            OnNoMatchStrategy::ErrOnNoMatch => "Error on no match".to_string(),
        }
    }
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