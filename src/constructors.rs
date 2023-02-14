use postgis::ewkb::{AsEwkbPoint, EwkbWrite};

use sqlite_loadable::prelude::*;
use sqlite_loadable::{api, Result};

pub fn geo_point(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let x = api::value_double(values.get(0).unwrap());
    let y = api::value_double(values.get(1).unwrap());
    let srid = values.get(1).map(api::value_int);

    let mut buff = std::io::Cursor::new(vec![]);
    let p = postgis::ewkb::Point::new(x, y, srid);
    p.as_ewkb().write_ewkb(&mut buff).unwrap();
    api::result_blob(context, buff.into_inner().as_slice());

    Ok(())
}
