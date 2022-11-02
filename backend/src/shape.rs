pub struct Point3D(u32, u32, u32);
pub struct Rotation3D(f64, f64);

pub struct Direction {
    pub point: Point3D,
    pub rotation: Rotation3D,
}

pub trait Shape {
    fn center() -> Point3D;
    fn scale_directions() -> Vec<Direction>;
    fn rotation_directions() -> Vec<Direction>;
    fn draw();
}

pub struct Line {
    
}
