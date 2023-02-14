use geo::Geometry;
use geo_postgis::{FromPostgis, ToPostgis};

use geojson::GeoJson;
use postgis::ewkb::{self, AsEwkbGeometry, EwkbRead, EwkbWrite};

use sqlite_loadable::prelude::*;
use sqlite_loadable::{api, Error, Result};

use std::os::raw::c_void;
use wkt::TryFromWkt;

pub fn result_geo(context: *mut sqlite3_context, geometry: Geometry) {
    let mut buff = std::io::Cursor::new(vec![]);
    let x = geometry.to_postgis_with_srid(None);
    //let p = postgis::ewkb::Geometry::from(geometry);
    x.as_ewkb().write_ewkb(&mut buff).unwrap();
    api::result_blob(context, buff.into_inner().as_slice());
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
            let mut x = std::io::Cursor::new(b);
            let x = ewkb::Geometry::read_ewkb(&mut x).unwrap();
            match x {
                ewkb::GeometryT::Point(v) => Ok(geo_types::Point::from_postgis(&v).into()),
                ewkb::GeometryT::LineString(_) => todo!(),
                ewkb::GeometryT::Polygon(_) => todo!(),
                ewkb::GeometryT::MultiPoint(_) => todo!(),
                ewkb::GeometryT::MultiLineString(_) => todo!(),
                ewkb::GeometryT::MultiPolygon(_) => todo!(),
                ewkb::GeometryT::GeometryCollection(_) => todo!(),
            }
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
        // a regex from that, and return a flag to call sqlite3_set_auxdata
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
    regex: Box<geo_types::Geometry>,
    input_type: GeoInputType,
) {
    let pointer = Box::into_raw(regex);
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
