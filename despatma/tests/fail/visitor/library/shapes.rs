pub trait Circle {
    fn get_center(&self) -> (usize, usize);
}

pub trait Cube {
    fn get_center(&self) -> (usize, usize, usize);
    fn get_front(&self) -> &Rectangle;
}

pub trait Sphere {
    fn get_center(&self) -> (usize, usize, usize);
}

pub struct Arc {
    radius: usize,
}

impl Arc {
    pub fn get_radius(&self) -> usize {
        self.radius
    }
}

pub struct Rectangle {
    width: usize,
    height: usize,
}

impl Rectangle {
    pub fn get_height(&self) -> usize {
        self.height
    }
    pub fn get_width(&self) -> usize {
        self.width
    }
}
