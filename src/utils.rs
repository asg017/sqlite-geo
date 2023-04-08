use geo::{BoundingRect, Geometry};
use geozero::wkb::GpkgWkb;
use geozero::{CoordDimensions, ToGeo, ToWkb};

use geojson::GeoJson;

use sqlite_loadable::prelude::*;
use sqlite_loadable::{api, Error, Result};

use std::os::raw::c_void;
use wkt::TryFromWkt;

pub fn result_geo(context: *mut sqlite3_context, geometry: Geometry) {
    let srid = None;
    let bbox = geometry.bounding_rect().unwrap();
    let min = bbox.min();
    let max = bbox.max();
    let wkb = geometry
        .to_gpkg_wkb(
            CoordDimensions::default(),
            srid,
            vec![min.x, min.y, max.x, max.y],
        )
        .unwrap();
    api::result_blob(context, wkb.as_slice());
}

pub fn value_as_geometry(value: &*mut sqlite3_value) -> Result<geo_types::Geometry> {
    match api::value_type(value) {
        api::ValueType::Text => {
            match serde_json::from_str::<serde_json::Value>(api::value_text(value)?) {
                Ok(value) => {
                    let gj = GeoJson::from_json_value(value).unwrap();
                    Ok(gj.try_into().unwrap())
                }
                Err(_) => {
                    Ok(geo_types::Geometry::try_from_wkt_str(api::value_text(value)?).unwrap())
                }
            }
        }
        api::ValueType::Blob => {
            let b = api::value_blob(value);
            let wkb = GpkgWkb(b.to_vec());
            let geom = wkb.to_geo().unwrap();
            Ok(geom)
        }
        _ => Err(Error::new_message("asdf")),
    }
}

pub enum GeoInputType {
    TextInitial(usize),
    GetAuxdata,
}
pub fn geo_from_value_or_cache(
    context: *mut sqlite3_context,
    values: &[*mut sqlite3_value],
    at: usize,
) -> Result<(Box<geo_types::Geometry>, GeoInputType)> {
    let value = values
        .get(at)
        .ok_or_else(|| Error::new_message("expected 1st argument as pattern"))?;

    // Step 1: If sqlite3_get_auxdata returns a pointer, then use that.
    let auxdata = api::auxdata_get(context, at as i32);
    if !auxdata.is_null() {
        Ok((
            unsafe { Box::from_raw(auxdata.cast::<geo_types::Geometry>()) },
            GeoInputType::GetAuxdata,
        ))
    } else {
        // Step 3: if a string is passed in, then try to make
        // a geom from that, and return a flag to call sqlite3_set_auxdata
        match value_as_geometry(value) {
            Ok(geo) => Ok((Box::new(geo), GeoInputType::TextInitial(at))),
            Err(err) => Err(err),
        }
    }
}

unsafe extern "C" fn cleanup(p: *mut c_void) {
    drop(Box::from_raw(p.cast::<*mut geo_types::Geometry>()))
}

pub fn cleanup_geo_value_cached(
    context: *mut sqlite3_context,
    geom: Box<geo_types::Geometry>,
    input_type: GeoInputType,
) {
    let pointer = Box::into_raw(geom);
    match input_type {
        GeoInputType::GetAuxdata => {}
        GeoInputType::TextInitial(at) => {
            api::auxdata_set(
                context,
                at as i32,
                pointer.cast::<c_void>(),
                // TODO memory leak, box not destroyed?
                Some(cleanup),
            )
        }
    }
}
