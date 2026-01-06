#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub cx: f32,
    pub cy: f32,
    pub r: f32,
}

#[derive(Debug, Clone, Copy)]
pub enum Shape {
    Rect { rect: Rect, rx: f32, ry: f32 }, // rounded rect
    Circle(Circle),
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}
