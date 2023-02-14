mod cast;
mod constructors;
mod operations;
mod utils;
use crate::{cast::*, constructors::*, operations::*};

use sqlite_loadable::prelude::*;
use sqlite_loadable::{define_scalar_function, Result};

#[sqlite_entrypoint]
pub fn sqlite3_geo_init(db: *mut sqlite3) -> Result<()> {
    let common = FunctionFlags::UTF8 | FunctionFlags::DETERMINISTIC;
    // convertors/casting
    define_scalar_function(db, "geo_as_geojson", 1, geo_as_geojson, common)?;

    // constructoring shapes
    define_scalar_function(db, "geo_point", 2, geo_point, common)?;

    // geo operations
    define_scalar_function(db, "geo_within", 2, geo_within, common)?;
    define_scalar_function(db, "geo_area", 1, geo_area, common)?;
    define_scalar_function(db, "geo_centroid", 1, geo_centroid, common)?;
    define_scalar_function(db, "geo_bounding_rect", 1, geo_bounding_rect, common)?;
    define_scalar_function(
        db,
        "geo_distance_euclidean",
        2,
        geo_distance_euclidean,
        common,
    )?;
    Ok(())
}
