use crate::utils::*;
use sqlite_loadable::prelude::*;
use sqlite_loadable::{api, Error, Result};

use geo::prelude::*;
pub fn geo_as_geojson(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    match value_as_geometry(values.get(0).unwrap()) {
        Ok(geo) => {
            api::result_text(context, geojson::Value::from(&geo).to_string())?;
            api::result_subtype(context, 76);
            Ok(())
        }
        Err(err) => todo!(),
    }
}
