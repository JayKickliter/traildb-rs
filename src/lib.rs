#![feature(question_mark)]
#[allow(non_camel_case_types,dead_code,non_snake_case,private_in_public)]
mod ffi;
use std::path::Path;
use std::ffi::CString;

#[derive(PartialEq)]
#[repr(C)]
pub enum Error {
    Nomem,
    PathTooLong,
    UnknownField,
    UnknownUuid,
    InvalidTrailId,
    HandleIsNull,
    HandleAlreadyOpened,
    UnknownOption,
    InvalidOptionValue,
    InvalidUuid,
    IoOpen,
    IoClose,
    IoWrite,
    IoRead,
    IoTruncate,
    IoPackage,
    InvalidInfoFile,
    InvalidVersionFile,
    IncompatibleVersion,
    InvalidFieldsFile,
    InvalidUuidsFile,
    InvalidCodebookFile,
    InvalidTrailsFile,
    InvalidLexiconFile,
    InvalidPackage,
    TooManyFields,
    DuplicateFields,
    InvalidFieldname,
    TooManyTrails,
    ValueTooLong,
    AppendFieldsMismatch,
    LexiconTooLarge,
    TimestampTooLarge,
    TrailTooLong,
    OnlyDiffFilter,
}

fn translate_tdb_error(val: i32) -> Error {
    match val {
        -2 => Error::Nomem,
        -3 => Error::PathTooLong,
        -4 => Error::UnknownField,
        -5 => Error::UnknownUuid,
        -6 => Error::InvalidTrailId,
        -7 => Error::HandleIsNull,
        -8 => Error::HandleAlreadyOpened,
        -9 => Error::UnknownOption,
        -10 => Error::InvalidOptionValue,
        -11 => Error::InvalidUuid,
        -65 => Error::IoOpen,
        -66 => Error::IoClose,
        -67 => Error::IoWrite,
        -68 => Error::IoRead,
        -69 => Error::IoTruncate,
        -70 => Error::IoPackage,
        -129 => Error::InvalidInfoFile,
        -130 => Error::InvalidVersionFile,
        -131 => Error::IncompatibleVersion,
        -132 => Error::InvalidFieldsFile,
        -133 => Error::InvalidUuidsFile,
        -134 => Error::InvalidCodebookFile,
        -135 => Error::InvalidTrailsFile,
        -136 => Error::InvalidLexiconFile,
        -137 => Error::InvalidPackage,
        -257 => Error::TooManyFields,
        -258 => Error::DuplicateFields,
        -259 => Error::InvalidFieldname,
        -260 => Error::TooManyTrails,
        -261 => Error::ValueTooLong,
        -262 => Error::AppendFieldsMismatch,
        -263 => Error::LexiconTooLarge,
        -264 => Error::TimestampTooLarge,
        -265 => Error::TrailTooLong,
        -513 => Error::OnlyDiffFilter,
        _ => panic!("traildb returned unrecognized error {}", val),
    }
}

pub struct Constructor {
    handle: *mut ffi::tdb_cons,
}
// pub enum OptKey {}
// pub enum OptVal {}
// pub type TimeStamp = u64;

impl Constructor {
    pub fn new() -> Result<Constructor, ()> {
        let handle = unsafe { ffi::tdb_cons_init() };
        if handle.is_null() {
            Err(())
        } else {
            Ok(Constructor { handle: handle })
        }
    }

    pub fn open<'a>(&mut self, path: &Path, fields: &[&'a str]) -> Result<(), Error> {
        let mut names = Vec::new();
        for field in fields.iter() {
            names.push(CString::new(field.as_bytes()).unwrap());
        }
        let c_path = path_cstr(path).as_ptr();
        let c_names = names.as_slice().as_ptr() as *mut *const i8;
        let ret = unsafe {
            ffi::tdb_cons_open(self.handle,
                               c_path,
                               c_names,
                               names.len() as u64)
        };
        match ret {
            0 => Ok(()),
            _ => Err(translate_tdb_error(ret)),
        }
    }
}


fn path_cstr(path: &Path) -> CString {
    CString::new(path.to_str().unwrap()).unwrap()
}

#[cfg(test)]
mod test_constructor {
    use super::Constructor;
use std::path::Path;

    #[test]
    fn main() {
        let mut constructor = Constructor::new().unwrap();
        assert!(!constructor.handle.is_null());
        let field_names = ["user", "event"];
        let db_path = Path::new("test");
        let res = constructor.open(db_path, &field_names);
        // assert!(Ok(()) == res);
    }

}
