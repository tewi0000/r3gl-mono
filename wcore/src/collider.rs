pub mod collide {
    use cgmath::{Vector2, MetricSpace};
 
    pub fn circle(p1: Vector2<f32>, p2: Vector2<f32>, radius: f32) -> bool {
        return p1.distance(p2) <= radius;
    }

    pub fn square(pos: Vector2<f32>, size: Vector2<f32>, p: Vector2<f32>) -> bool {
        return (p.x >= pos.x && p.x <= pos.x + size.x)
            && (p.y >= pos.y && p.y <= pos.y + size.y);
    }
}