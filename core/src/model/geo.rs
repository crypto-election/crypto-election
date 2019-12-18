use serde::{Serialize, Deserialize};
use geo;

use crate::proto;


#[derive(Serialize, Deserialize, ProtobufConvert, Clone)]
#[exonum(pb = "proto::Polygon")]
pub struct Polygon {
   exterior: LineString,
   interiors: Vec<LineString>,
}

#[derive(Serialize, Deserialize, ProtobufConvert, Clone)]
#[exonum(pb = "proto::LineString")]
struct LineString {
   pub items: Vec<Coordinate>,
}

#[derive(Serialize, Deserialize, ProtobufConvert, Copy, Clone)]
#[exonum(pb = "proto::Coordinate")]
struct Coordinate {
   pub x: f64,
   pub y: f64,
}

impl From<geo::Polygon<f64>> for Polygon {
   fn from(polygon: geo::Polygon<f64>) -> Self {
       let (exterior, interiors) = polygon.into_inner();
       Polygon {
           exterior: exterior.into(),
           interiors: interiors.into_iter().map(Into::into).collect(),
       }
   }
}

impl From<Polygon> for geo::Polygon<f64> {
   fn from(def: Polygon) -> Self {
       Self::new(
           def.exterior.into(),
           def.interiors.into_iter().map(Into::into).collect(),
       )
   }
}

impl From<geo::LineString<f64>> for LineString {
   fn from(l_str: geo::LineString<f64>) -> Self {
       LineString {
           items: l_str
               .0
               .into_iter()
               .map(Into::into)
               .collect(),
       }
   }
}

impl From<LineString> for geo::LineString<f64> {
   fn from(l_str: LineString) -> Self {
       l_str.items.into_iter().map(Into::<geo::Coordinate<f64>>::into).collect()
   }
}

impl From<geo::Coordinate<f64>> for Coordinate {
   fn from(coord: geo::Coordinate<f64>) -> Self {
       Coordinate { x: coord.x, y: coord.y }
   }
}

impl From<Coordinate> for geo::Coordinate<f64> {
   fn from(coord: Coordinate) -> Self {
       geo::Coordinate { x: coord.x, y: coord.y }
   }
}
