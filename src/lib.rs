#[allow(non_camel_case_types,dead_code,non_snake_case,private_in_public)]
mod ffi;
use std::path::Path;
use std::ffi::CString;
use std::fmt;
use std::mem::transmute;

#[derive(Debug)]
#[derive(PartialEq)]
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

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            &Error::Nomem => "Nomem",
            &Error::PathTooLong => "PathTooLong",
            &Error::UnknownField => "UnknownField",
            &Error::UnknownUuid => "UnknownUuid",
            &Error::InvalidTrailId => "InvalidTrailId",
            &Error::HandleIsNull => "HandleIsNull",
            &Error::HandleAlreadyOpened => "HandleAlreadyOpened",
            &Error::UnknownOption => "UnknownOption",
            &Error::InvalidOptionValue => "InvalidOptionValue",
            &Error::InvalidUuid => "InvalidUuid",
            &Error::IoOpen => "IoOpen",
            &Error::IoClose => "IoClose",
            &Error::IoWrite => "IoWrite",
            &Error::IoRead => "IoRead",
            &Error::IoTruncate => "IoTruncate",
            &Error::IoPackage => "IoPackage",
            &Error::InvalidInfoFile => "InvalidInfoFile",
            &Error::InvalidVersionFile => "InvalidVersionFile",
            &Error::IncompatibleVersion => "IncompatibleVersion",
            &Error::InvalidFieldsFile => "InvalidFieldsFile",
            &Error::InvalidUuidsFile => "InvalidUuidsFile",
            &Error::InvalidCodebookFile => "InvalidCodebookFile",
            &Error::InvalidTrailsFile => "InvalidTrailsFile",
            &Error::InvalidLexiconFile => "InvalidLexiconFile",
            &Error::InvalidPackage => "InvalidPackage",
            &Error::TooManyFields => "TooManyFields",
            &Error::DuplicateFields => "DuplicateFields",
            &Error::InvalidFieldname => "InvalidFieldname",
            &Error::TooManyTrails => "TooManyTrails",
            &Error::ValueTooLong => "ValueTooLong",
            &Error::AppendFieldsMismatch => "AppendFieldsMismatch",
            &Error::LexiconTooLarge => "LexiconTooLarge",
            &Error::TimestampTooLarge => "TimestampTooLarge",
            &Error::TrailTooLong => "TrailTooLong",
            &Error::OnlyDiffFilter => "OnlyDiffFilter",
        };
        write!(f, "Error::{}", s)
    }
}

/// Convert a tdb_error to either a Ok(T) or Err(Error)
#[inline(always)]
fn wrap_tdb_err<T>(err: i32, val: T) -> Result<T, Error> {
    match err {
        0 => Ok(val),
        _ => Err(unsafe { transmute(err) }),
    }
}

pub type Timestamp = u64;
pub type Version = u64;
pub type TrailId = u64;
pub type Uuid = [u8; 16];
pub type Item = u64;
pub type Value = u64;
pub type Field = u32;

pub struct Trail<'a> {
    pub id: TrailId,
    cursor: Cursor<'a>,
}
pub struct Constructor {
    obj: *mut ffi::tdb_cons,
}

impl Constructor {
    pub fn new(path: &Path, fields: &[&str]) -> Result<Self, Error> {
        let mut field_ptrs = Vec::new();
        for f in fields.iter() {
            field_ptrs.push(f.as_ptr());
        }
        let ptr = unsafe { ffi::tdb_cons_init() };
        let ret = unsafe {
            ffi::tdb_cons_open(ptr,
                               path_cstr(path).as_ptr(),
                               field_ptrs.as_slice().as_ptr() as *mut *const i8,
                               field_ptrs.len() as u64)
        };
        wrap_tdb_err(ret, Constructor { obj: ptr })
    }

    pub fn add(&mut self, uuid: &Uuid, timestamp: Timestamp, values: &[&str]) -> Result<(), Error> {
        let mut val_ptrs = Vec::new();
        let mut val_lens = Vec::new();
        for v in values.iter() {
            val_ptrs.push(v.as_ptr());
            val_lens.push(v.len() as u64);
        }
        let ret = unsafe {
            ffi::tdb_cons_add(self.obj,
                              uuid.as_ptr() as *mut u8,
                              timestamp,
                              val_ptrs.as_slice().as_ptr() as *mut *const i8,
                              val_lens.as_slice().as_ptr() as *const u64)
        };
        wrap_tdb_err(ret, ())
    }

    pub fn close(&mut self) {
        unsafe { ffi::tdb_cons_close(self.obj) };
    }

    pub fn finalize(&mut self) -> Result<(), Error> {
        let ret = unsafe { ffi::tdb_cons_finalize(self.obj) };
        wrap_tdb_err(ret, ())
    }

    pub fn append(&mut self, db: &Db) -> Result<(), Error> {
        let ret = unsafe { ffi::tdb_cons_append(self.obj, transmute(db)) };
        wrap_tdb_err(ret, ())
    }
}


pub struct Db<'a> {
    obj: &'a mut ffi::tdb,
}

impl<'a> Db<'a> {
    pub fn open(path: &Path) -> Result<Self, Error> {
        let ptr = unsafe { ffi::tdb_init() };
        let ret = unsafe { ffi::tdb_open(ptr, path_cstr(path).as_ptr()) };
        unsafe { wrap_tdb_err(ret, Db { obj: transmute(ptr) }) }
    }

    pub fn close(&mut self) {
        unsafe {
            ffi::tdb_close(self.obj);
        }
    }

    pub fn num_trails(&self) -> u64 {
        unsafe { ffi::tdb_num_trails(self.obj) }
    }

    pub fn num_events(&self) -> u64 {
        unsafe { ffi::tdb_num_events(self.obj) }
    }

    pub fn num_fields(&self) -> u64 {
        unsafe { ffi::tdb_num_fields(self.obj) }
    }

    pub fn min_timestamp(&self) -> Timestamp {
        unsafe { ffi::tdb_min_timestamp(self.obj) }
    }

    pub fn max_timestamp(&self) -> Timestamp {
        unsafe { ffi::tdb_max_timestamp(self.obj) }
    }

    pub fn version(&self) -> Version {
        unsafe { ffi::tdb_version(self.obj) }
    }

    pub fn will_need(&self) {
        unsafe { ffi::tdb_willneed(self.obj) };
    }

    pub fn dont_need(&self) {
        unsafe { ffi::tdb_dontneed(self.obj) };
    }

    pub fn get_trail(&self, trail_id: TrailId) -> Option<Trail> {
        let mut cursor = self.cursor();
        if cursor.get_trail(trail_id).is_err() {
            return None;
        };
        Some(Trail {
            id: trail_id,
            cursor: cursor,
        })
    }

    pub fn get_trail_id(&self, uuid: &Uuid) -> Option<TrailId> {
        let mut id: TrailId = 0;
        let ret = unsafe {
            ffi::tdb_get_trail_id(self.obj, uuid.as_ptr() as *mut u8, &mut id as *mut TrailId)
        };
        match ret {
            0 => Some(id),
            _ => None,
        }
    }

    pub fn get_uuid(&self, trail_id: TrailId) -> Option<&Uuid> {
        unsafe {
            let ptr = ffi::tdb_get_uuid(self.obj, trail_id) as *const [u8; 16];
            ptr.as_ref()
        }
    }

    pub fn cursor(&self) -> Cursor<'a> {
        unsafe {
            let ptr = ffi::tdb_cursor_new(self.obj);
            Cursor { obj: transmute(ptr) }
        }
    }

    pub fn iter(&'a mut self) -> DbIter<'a> {
        DbIter { pos: 0, db: self }
    }
}

pub struct DbIter<'a> {
    pos: u64,
    db: &'a mut Db<'a>,
}

impl<'a> Iterator for DbIter<'a> {
    type Item = Trail<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let id = self.pos;
        self.pos += 1;
        let mut cursor = self.db.cursor();
        match cursor.get_trail(id) {
            Err(_) => None,
            Ok(()) => {
                let trail = Trail {
                    id: id,
                    cursor: cursor,
                };
                Some(trail)
            }
        }
    }
}

pub struct Cursor<'a> {
    obj: &'a mut ffi::tdb_cursor,
}
impl<'a> Cursor<'a> {
    pub fn get_trail(&mut self, trail_id: TrailId) -> Result<(), Error> {
        let ret = unsafe { ffi::tdb_get_trail(self.obj, trail_id) };
        wrap_tdb_err(ret, ())
    }

    pub fn len(&mut self) -> u64 {
        unsafe { ffi::tdb_get_trail_length(self.obj) }
    }
}

impl<'a> Drop for Cursor<'a> {
    fn drop(&mut self) {
        unsafe { ffi::tdb_cursor_free(self.obj) };
    }
}

impl<'a> Iterator for Cursor<'a> {
    type Item = Event;

    fn next(&mut self) -> Option<Event> {
        unsafe {
            let e: *const ffi::tdb_event = ffi::tdb_cursor_next(self.obj);
            if e.is_null() {
                return Option::None;
            } else {
                let e = e.as_ref().unwrap();
                let items = &e.items as *const Item;
                let num_items = e.num_items;
                let timestamp = e.timestamp;
                let event = Event::from_raw_parts(timestamp, items, num_items as usize);
                Some(event)
            }
        }
    }
}

impl<'a> Iterator for Trail<'a> {
    type Item = Event;

    fn next(&mut self) -> Option<Event> {
        self.cursor.next()
    }
}


fn path_cstr(path: &Path) -> CString {
    CString::new(path.to_str().unwrap()).unwrap()
}

pub struct Event {
    pub timestamp: Timestamp,
    pub items: &'static [Item],
}

impl Event {
    unsafe fn from_raw_parts(timestamp: Timestamp, items: *const Item, num_items: usize) -> Self {
        Event {
            timestamp: timestamp,
            items: std::slice::from_raw_parts(items, num_items),
        }
    }
}

#[cfg(test)]
mod test_traildb {
    extern crate uuid;
    use super::{Constructor, Db};
    use std::path::Path;

    #[test]
    #[no_mangle]
    fn test_traildb() {
        // create a new constructor
        let field_names = ["field1", "field2"];
        let db_path = Path::new("test");
        let mut cons = Constructor::new(db_path, &field_names).unwrap();

        // add an event
        let events_per_trail = 100;
        let mut trail_cnt = 0;
        let mut event_cnt = 0;
        let mut uuids = Vec::new();
        let mut timestamp = 0;
        let mut timestamps = Vec::new();
        for _ in 0..100 {
            let uuid = *uuid::Uuid::new_v4().as_bytes();
            for _ in 0..events_per_trail {
                let vals = ["cats", "dogs"];
                assert!(&cons.add(&uuid, timestamp, &vals).is_ok());
                timestamps.push(timestamp);
                event_cnt += 1;
                timestamp += 1;
            }
            uuids.push(uuid);
            trail_cnt += 1;
        }


        // finalize db (saves it to disk)
        assert!(cons.finalize().is_ok());

        // open test database
        let db_path = Path::new("test");
        let db = Db::open(db_path).unwrap();

        // check number of fields
        let num_fields = db.num_fields();
        println!("Num fields: {}", num_fields);
        assert_eq!(num_fields, 1 + field_names.len() as u64);

        // check number of trails
        let num_trails = db.num_trails();
        println!("Num trails: {}", num_trails);
        assert_eq!(num_trails, trail_cnt);

        // check number of events
        let num_events = db.num_events();
        println!("Num events: {}", num_events);
        assert_eq!(num_events, event_cnt);

        // Check round-trip get_uuid/get_trail_id
        for uuid in &uuids {
            let trail_id = db.get_trail_id(&uuid).unwrap();
            let uuid_rt = db.get_uuid(trail_id).unwrap();
            assert_eq!(&uuid, &uuid_rt);
        }

        // check max/min timestamp
        let min_timestamp = *timestamps.iter().min().unwrap();
        let max_timestamp = *timestamps.iter().max().unwrap();
        println!("Mix/Max timestamp: {}/{}", min_timestamp, max_timestamp);
        assert_eq!(db.min_timestamp(), min_timestamp);
        assert_eq!(db.max_timestamp(), max_timestamp);

        // test cursor
        let mut cursor = db.cursor();
        for uuid in &uuids {
            let trail_id = db.get_trail_id(&uuid).unwrap();
            cursor.get_trail(trail_id).unwrap();
            assert_eq!(events_per_trail, cursor.len());
        }
    }

    #[test]
    #[no_mangle]
    fn test_wiki_dump() {
        // open the example db
        let db_path = Path::new("assets/wikipedia-history-small.tdb");
        let mut db = Db::open(db_path).unwrap();

        // iterate through some of the events
        for trail in db.iter() {
            if trail.id > 10 {break;}
            println!("Trail {}", trail.id);
            for event in trail {
                println!("Timestamp {}", event.timestamp);
            }
        }
    }
}
