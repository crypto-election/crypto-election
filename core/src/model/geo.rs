use std::iter::FromIterator;

use geo;
use serde::{Deserialize, Serialize};

use crate::proto;

#[derive(Serialize, Deserialize, ProtobufConvert, Clone, Debug)]
#[exonum(pb = "proto::Polygon")]
pub struct Polygon {
    pub exterior: LineString,
    pub interiors: Vec<LineString>,
}

#[derive(Serialize, Deserialize, ProtobufConvert, Clone, Debug)]
#[exonum(pb = "proto::LineString")]
pub struct LineString {
    pub items: Vec<Coordinate>,
}

#[derive(Serialize, Deserialize, ProtobufConvert, Copy, Clone, Debug)]
#[exonum(pb = "proto::Coordinate")]
pub struct Coordinate {
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
        Self {
            items: l_str.0.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<LineString> for geo::LineString<f64> {
    fn from(l_str: LineString) -> Self {
        l_str.items.into_iter().collect()
    }
}

impl<IC: Into<Coordinate>> From<Vec<IC>> for LineString {
    fn from(line_string: Vec<IC>) -> Self {
        Self {
            items: line_string.into_iter().map(Into::into).collect(),
        }
    }
}

impl<IC: Into<Coordinate>> FromIterator<IC> for LineString {
    fn from_iter<I: IntoIterator<Item = IC>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<geo::Coordinate<f64>> for Coordinate {
    fn from(coord: geo::Coordinate<f64>) -> Self {
        Self {
            x: coord.x,
            y: coord.y,
        }
    }
}

impl From<Coordinate> for geo::Coordinate<f64> {
    fn from(coord: Coordinate) -> Self {
        Self {
            x: coord.x,
            y: coord.y,
        }
    }
}

impl From<[f64; 2]> for Coordinate {
    fn from(coordinate: [f64; 2]) -> Self {
        Coordinate {
            x: coordinate[0],
            y: coordinate[1],
        }
    }
}

impl From<(f64, f64)> for Coordinate {
    fn from(coordinate: (f64, f64)) -> Self {
        Coordinate {
            x: coordinate.0,
            y: coordinate.1,
        }
    }
}
