use crate::utils::*;
use sqlite_loadable::prelude::*;
use sqlite_loadable::{api, Error, Result};

use geo::prelude::*;
use geo::{Area, EuclideanDistance};

pub fn geo_within(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let (geo_a, input_a) = geo_from_value_or_cache(context, values, 0)?;
    let (geo_b, input_b) = geo_from_value_or_cache(context, values, 1)?;
    api::result_bool(context, (*geo_a).is_within(&(*geo_b)));
    cleanup_geo_value_cached(context, geo_a, input_a);
    cleanup_geo_value_cached(context, geo_b, input_b);

    Ok(())
}

pub fn geo_area(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    match value_as_geometry(values.get(0).unwrap()) {
        Ok(geo) => api::result_double(context, geo.signed_area()),
        Err(err) => todo!(),
    }
    Ok(())
}
pub fn geo_centroid(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    match value_as_geometry(values.get(0).unwrap()) {
        Ok(geo) => match geo.centroid() {
            Some(centroid) => result_geo(context, geo::Geometry::Point(centroid)),
            None => api::result_null(context),
        },
        Err(err) => todo!(),
    }
    Ok(())
}

pub fn geo_bounding_rect(
    context: *mut sqlite3_context,
    values: &[*mut sqlite3_value],
) -> Result<()> {
    match value_as_geometry(values.get(0).unwrap()) {
        Ok(geo) => match geo.bounding_rect() {
            Some(centroid) => result_geo(context, geo::Geometry::Rect(centroid)),
            None => api::result_null(context),
        },
        Err(err) => todo!(),
    }
    Ok(())
}

pub fn geo_distance_euclidean(
    context: *mut sqlite3_context,
    values: &[*mut sqlite3_value],
) -> Result<()> {
    let (geo_a, input_a) = geo_from_value_or_cache(context, values, 0)?;
    let (geo_b, input_b) = geo_from_value_or_cache(context, values, 1)?;

    // https://docs.rs/geo/latest/geo/algorithm/euclidean_distance/trait.EuclideanDistance.html#implementors
    let x = match (*geo_a.clone(), *geo_b.clone()) {
        (geo::Geometry::Rect(_), _) => todo!(),
        (_, geo::Geometry::Rect(_)) => todo!(),
        (geo::Geometry::GeometryCollection(_), _) => todo!(),
        (_, geo::Geometry::GeometryCollection(_)) => todo!(),

        (geo::Geometry::Triangle(a), geo::Geometry::Point(b)) => a.euclidean_distance(&b),
        (geo::Geometry::Triangle(_), _) => todo!(),

        (geo::Geometry::Point(a), geo::Geometry::Point(b)) => a.euclidean_distance(&b),
        (geo::Geometry::Point(a), geo::Geometry::Line(b)) => a.euclidean_distance(&b),
        (geo::Geometry::Point(a), geo::Geometry::LineString(b)) => a.euclidean_distance(&b),
        (geo::Geometry::Point(a), geo::Geometry::Polygon(b)) => a.euclidean_distance(&b),
        (geo::Geometry::Point(a), geo::Geometry::MultiPoint(b)) => a.euclidean_distance(&b),
        (geo::Geometry::Point(a), geo::Geometry::MultiLineString(b)) => a.euclidean_distance(&b),
        (geo::Geometry::Point(a), geo::Geometry::MultiPolygon(b)) => a.euclidean_distance(&b),
        (geo::Geometry::Point(_), geo::Geometry::Triangle(_)) => todo!(),
        (geo::Geometry::Line(a), geo::Geometry::Point(b)) => a.euclidean_distance(&b),
        (geo::Geometry::Line(a), geo::Geometry::Line(b)) => a.euclidean_distance(&b),
        (geo::Geometry::Line(a), geo::Geometry::LineString(b)) => a.euclidean_distance(&b),
        (geo::Geometry::Line(a), geo::Geometry::Polygon(b)) => a.euclidean_distance(&b),
        (geo::Geometry::Line(_), _) => todo!(),
        (geo::Geometry::LineString(a), geo::Geometry::Point(b)) => a.euclidean_distance(&b),
        (geo::Geometry::LineString(a), geo::Geometry::Line(b)) => a.euclidean_distance(&b),
        (geo::Geometry::LineString(a), geo::Geometry::LineString(b)) => a.euclidean_distance(&b),
        (geo::Geometry::LineString(a), geo::Geometry::Polygon(b)) => a.euclidean_distance(&b),
        (geo::Geometry::LineString(_), _) => todo!(),
        (geo::Geometry::Polygon(a), geo::Geometry::Point(b)) => a.euclidean_distance(&b),
        (geo::Geometry::Polygon(a), geo::Geometry::Line(b)) => a.euclidean_distance(&b),
        (geo::Geometry::Polygon(a), geo::Geometry::LineString(b)) => a.euclidean_distance(&b),
        (geo::Geometry::Polygon(a), geo::Geometry::Polygon(b)) => a.euclidean_distance(&b),
        (geo::Geometry::Polygon(_), _) => todo!(),
        (geo::Geometry::MultiPoint(_), _) => todo!(),
        (geo::Geometry::MultiLineString(a), geo::Geometry::Point(b)) => a.euclidean_distance(&b),
        (geo::Geometry::MultiLineString(_), _) => todo!(),
        (geo::Geometry::MultiPolygon(a), geo::Geometry::Point(b)) => a.euclidean_distance(&b),
        (geo::Geometry::MultiPolygon(a), geo::Geometry::Line(b)) => a.euclidean_distance(&b),
        (geo::Geometry::MultiPolygon(_), _) => todo!(),
    };
    //api::result_bool(context, (*geo_a).euclidean_distance(&(*geo_b)));
    api::result_double(context, x);
    cleanup_geo_value_cached(context, geo_a, input_a);
    cleanup_geo_value_cached(context, geo_b, input_b);

    Ok(())
}
