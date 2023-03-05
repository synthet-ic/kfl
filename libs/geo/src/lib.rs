/*!
<https://www.rfc-editor.org/rfc/rfc7946>
*/

use kfl::Decode;

pub enum Geometry {
    Point {
        properties: HashMap<String, >,
        coordinate: Position
    },
    MultiPoint,
    LineString(#[kfl(children)] Vec<Position>),
    MultiLineString,
    Polygon {
        properties: ,
        coordinates: 
    },
    MultiPolygon,
    GeometryCollection
}

#[derive(Debug, Decode)]
pub struct Feature {
    
}

#[derive(Decode)]
pub struct Position(
    #[kfl(argument) f32,
    #[kfl(argument) f32
);

pub struct BBox;

pub enum Pole {
    MinLat,
    MaxLat,
    WestLon,
    EastLon
}
