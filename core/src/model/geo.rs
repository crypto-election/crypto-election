//#[derive(Serialize, Deserialize, ProtobufConvert, Clone)]
//#[exonum(pb = "proto::PolygonDef")]
//pub struct PolygonDef {
//    exterior: LineStringDef,
//    interiors: Vec<LineStringDef>,
//}
//
//impl From<&PolygonDef> for Polygon<f64> {
//    fn from(def: &PolygonDef) -> Self {
//        Self::new(
//            LineString::from(&def.exterior),
//            def.interiors.iter().map(LineString::<f64>::from).collect(),
//        )
//    }
//}
//
//impl From<&Polygon<f64>> for PolygonDef {
//    fn from(polygon: &Polygon<f64>) -> Self {
//        let (exterior, interiors) = polygon.clone().into_inner();
//        PolygonDef {
//            exterior: LineStringDef::from(&exterior),
//            interiors: interiors.iter().map(LineStringDef::from).collect(),
//        }
//    }
//}
//
//#[derive(Serialize, Deserialize, ProtobufConvert, Clone)]
//#[exonum(pb = "proto::LineStringDef")]
//struct LineStringDef {
//    pub items: Vec<CoordinateDef>,
//}
//
//impl From<&LineString<f64>> for LineStringDef {
//    fn from(l_str: &LineString<f64>) -> Self {
//        LineStringDef {
//            items: l_str
//                .0
//                .iter()
//                .map(|coord| CoordinateDef {
//                    x: coord.x,
//                    y: coord.y,
//                })
//                .collect(),
//        }
//    }
//}
//
//impl From<&LineStringDef> for LineString<f64> {
//    fn from(def: &LineStringDef) -> Self {
//        def.items.iter().map(Coordinate::<f64>::from).collect()
//    }
//}
//
//#[derive(Serialize, Deserialize, ProtobufConvert, Copy, Clone)]
//#[exonum(pb = "proto::CoordinateDef")]
//struct CoordinateDef {
//    pub x: f64,
//    pub y: f64,
//}
//
//impl From<&CoordinateDef> for Coordinate<f64> {
//    fn from(def: &CoordinateDef) -> Self {
//        Coordinate { x: def.x, y: def.y }
//    }
//}
