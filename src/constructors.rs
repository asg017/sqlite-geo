use geozero::{CoordDimensions, ToWkb};

use sqlite_loadable::prelude::*;
use sqlite_loadable::{api, Result};

pub fn geo_point(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let x = api::value_double(values.get(0).unwrap());
    let y = api::value_double(values.get(1).unwrap());
    let srid = values.get(1).map(api::value_int);

    let geom: geo::Geometry<f64> = geo::Point::new(x, y).into();
    let wkb = geom
        .to_gpkg_wkb(CoordDimensions::default(), srid, vec![x, y, x, y])
        .unwrap();
    api::result_blob(context, wkb.as_slice());

    Ok(())
}
