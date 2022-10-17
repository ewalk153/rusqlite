use crate::{Connection, DatabaseName, Result};
use crate::ffi;

impl Connection {
  pub unsafe fn serialize(
    &self,
    from_name: DatabaseName<'_>,
  ) -> Result<(*mut ffi::sqlite3_int64, *mut u8)> {

    let from_db = self.db.borrow_mut().db;
    let from_name = from_name.as_cstring()?;
    let size: *mut ffi::sqlite3_int64 = &mut 0;
    let m_flags = 0;
    let data = ffi::sqlite3_serialize(from_db, from_name.as_ptr(), size, m_flags);

    return Ok((size, data));
  }

  pub unsafe fn deserialize(
    &self,
    to_name: DatabaseName<'_>,
    p_data: *mut ::std::os::raw::c_uchar,
    size_buffer: ffi::sqlite3_int64,
    m_flags: ::std::os::raw::c_uint,
  ) -> Result<::std::os::raw::c_int> {

    let to_db = self.db.borrow_mut().db;
    let to_name = to_name.as_cstring()?;
    let size_db: ffi::sqlite3_int64 = 1;

    let result = ffi::sqlite3_deserialize(
      to_db, 
      to_name.as_ptr(), 
      p_data,
      size_db,
      size_buffer,
      m_flags,
    );

    return Ok(result);
  }
}
