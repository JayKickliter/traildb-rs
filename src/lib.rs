#[allow(non_camel_case_types,dead_code,non_snake_case,private_in_public)]
mod ffi;

#[repr(C)]
pub enum Error {
    Nomem = -2,
    PathTooLong = -3,
    UnknownField = -4,
    UnknownUuid = -5,
    InvalidTrailId = -6,
    HandleIsNull = -7,
    HandleAlreadyOpened = -8,
    UnknownOption = -9,
    InvalidOptionValue = -10,
    InvalidUuid = -11,
    IoOpen = -65,
    IoClose = -66,
    IoWrite = -67,
    IoRead = -68,
    IoTruncate = -69,
    IoPackage = -70,
    InvalidInfoFile = -129,
    InvalidVersionFile = -130,
    IncompatibleVersion = -131,
    InvalidFieldsFile = -132,
    InvalidUuidsFile = -133,
    InvalidCodebookFile = -134,
    InvalidTrailsFile = -135,
    InvalidLexiconFile = -136,
    InvalidPackage = -137,
    TooManyFields = -257,
    DuplicateFields = -258,
    InvalidFieldname = -259,
    TooManyTrails = -260,
    ValueTooLong = -261,
    AppendFieldsMismatch = -262,
    LexiconTooLarge = -263,
    TimestampTooLarge = -264,
    TrailTooLong = -265,
    OnlyDiffFilter = -513,
}

pub struct Constructor(*mut ffi::tdb_cons);

impl Constructor {
    pub fn new() -> Result<Constructor, ()> {
        let handle = unsafe { ffi::tdb_cons_init() };
        if handle.is_null() {
            Err(())
        } else {
            Ok(Constructor(handle))
        }
    }
}

#[cfg(test)]
mod test_constructor {
    use super::Constructor;

    #[test]
    fn main() {
        let Constructor(handle) = Constructor::new().unwrap();
        assert!(!handle.is_null());
    }

}
