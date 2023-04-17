use pyo3::prelude::*;
use rand::{thread_rng, Rng};

static _PRECISION: u32 = 2;

#[pyfunction]
pub fn rotate_point(x: f32, y: f32, cx: f32, cy: f32, angle: f32) -> (f32, f32) {
    let angle_rads = angle.to_radians();
    let s = angle_rads.sin();
    let c = angle_rads.cos();

    // translate point back to origin:
    let temp_x = x - cx;
    let temp_y = y - cy;

    // rotate point
    let xnew = (temp_x * c - temp_y * s) + cx;
    let ynew = (temp_x * s + temp_y * c) + cy;

    let precision = 10i32.pow(_PRECISION) as f32;
    let x_rounded = (xnew * precision).round() / precision;
    let y_rounded = (ynew * precision).round() / precision;

    (x_rounded, y_rounded)
}

#[pyfunction]
pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

#[pyfunction]
pub fn lerp(v1: f32, v2: f32, u: f32) -> f32 {
    v1 + ((v2 - v1) * u)
}

#[pyfunction]
pub fn lerp_vec(v1: (f32, f32), v2: (f32, f32), u: f32) -> (f32, f32) {
    (lerp(v1.0, v2.0, u), lerp(v1.1, v2.1, u))
}

#[pyfunction]
pub fn lerp_angle(start_angle: f32, end_angle: f32, u: f32) -> f32 {
    let temp_start = start_angle % 360.0;
    let mut temp_end = end_angle % 360.0;

    while temp_start - temp_end > 180.0 {
        temp_end += 360.0;
    }

    while temp_start - temp_end < -180.0 {
        temp_end -= 360.0;
    }

    lerp(temp_start, temp_end, u) % 360.0
}

#[pyfunction]
pub fn get_distance(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    let x = x2 - x1;
    let y = y2 - y1;

    (x * x + y * y).sqrt()
}

#[pyfunction]
pub fn get_angle_degrees(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    let x_diff = x2 - x1;
    let y_diff = y2 - y1;
    let radians = y_diff.atan2(x_diff);

    radians.to_degrees()
}

#[pyfunction]
pub fn get_angle_radians(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    let x_diff = x2 - x1;
    let y_diff = y2 - y1;

    y_diff.atan2(x_diff)
}

#[pyfunction]
pub fn rand_in_rect(bottom_left: (f32, f32), width: f32, height: f32) -> (f32, f32) {
    let mut rng = thread_rng();

    let random_x: f32 = rng.gen_range(bottom_left.0..bottom_left.0 + width);
    let random_y: f32 = rng.gen_range(bottom_left.1..bottom_left.1 + height);

    (random_x, random_y)
}

#[pyfunction]
pub fn rand_in_circle(center: (f32, f32), radius: f32) -> (f32, f32) {
    let mut rng = thread_rng();
    let pi = 180.0_f32.to_radians();
    // random angle
    let random_num: f32 = rng.gen();
    let angle = 2.0 * pi * random_num;
    // random radius
    let random_num: f32 = rng.gen();
    let r = radius * random_num;

    (r * angle.cos() + center.0, r * angle.sin() + center.1)
}

#[pyfunction]
pub fn rand_on_circle(center: (f32, f32), radius: f32) -> (f32, f32) {
    let mut rng = thread_rng();
    let pi = 180.0_f32.to_radians();
    // random angle
    let random_num: f32 = rng.gen();
    let angle = 2.0 * pi * random_num;

    (
        radius * angle.cos() + center.0,
        radius * angle.sin() + center.1,
    )
}

#[pyfunction]
pub fn rand_on_line(pos1: (f32, f32), pos2: (f32, f32)) -> (f32, f32) {
    let mut rng = thread_rng();
    let u: f32 = rng.gen_range(0.0..1.0);

    lerp_vec(pos1, pos2, u)
}

#[pyfunction]
pub fn rand_angle_360_deg() -> f32 {
    let mut rng = thread_rng();
    let random_angle: f32 = rng.gen_range(0.0..360.0);

    random_angle
}

#[pyfunction]
pub fn rand_angle_spread_deg(angle: f32, half_angle_spread: f32) -> f32 {
    let mut rng = thread_rng();
    let s = rng.gen_range(-half_angle_spread..half_angle_spread);

    angle + s
}

#[pyfunction]
pub fn rand_vec_degree_spread(angle: f32, half_angle_spread: f32, length: f32) -> (f32,f32) {
    let a = rand_angle_spread_deg(angle,half_angle_spread);
    let vel = _Vec2::from_polar(a, length);
    vel.as_tuple()
    


}
#[pyfunction]
pub fn rand_vec_magnitude( angle:f32, lo_magnitude:f32,hi_magnitude: f32) -> (f32,f32) {
    let mut rng = thread_rng();
    let mag = rng.gen_range(lo_magnitude..hi_magnitude);
    let vel = _Vec2::from_polar(angle,mag);
    vel.as_tuple()
    
    
}


struct _Vec2 {
    x: f32,
    y: f32,
}
impl _Vec2{
    fn from_polar(angle: f32, radius: f32) -> _Vec2{
    let rads = angle.to_radians();
    _Vec2{x: radius * rads.cos(), y: radius * rads.sin()}
    }

    fn as_tuple(self) -> (f32,f32){
        (self.x,self.y)
    }
    fn __add__(self, other: _Vec2) -> _Vec2{
        _Vec2{x:self.x + other.x,y:self.y + other.y}

    }
    fn __sub__(self, other: _Vec2) -> _Vec2{
        _Vec2{x:self.x - other.x,y:self.y - other.y}
    }
    fn __mul__(self, other:_Vec2) -> _Vec2{
        _Vec2{x:self.x * other.x,y:self.y * other.y}
    }
    fn __truediv__(self, other:_Vec2) -> _Vec2{
        _Vec2{x:self.x / other.x, y:self.y / other.y}
    }
    fn length(self) -> f32{
        (self.x*self.x + self.y*self.y).sqrt()
    }
    fn dot(self,other:_Vec2)-> f32{
        self.x * other.x + self.y * other.y
    }
    fn rotated(self,angle: f32) -> _Vec2{
        let rads = angle.to_radians();
        let cosine = rads.cos();
        let sine = rads.sin();
        _Vec2{x:(self.x * cosine)- (self.y - sine),y:(self.y * cosine)-(self.x * sine)}

    }
    fn __repr__(self){
        format!("Vec2({},{})",self.x,self.y);
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clamp() {
        let mut result = clamp(1.2, 1.0, 2.0);
        assert!(result == 1.2);

        result = clamp(2.5, 1.0, 2.0);
        assert!(result == 2.0);

        result = clamp(0.8, 1.0, 2.0);
        assert!(result == 1.0);
    }
}
