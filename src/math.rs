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
    let xnew = (temp_x * c + temp_y * s) + cx;
    let ynew = (-temp_x * s + temp_y * c) + cy;

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

    // Add 360
    lerp(temp_start, temp_end, u).rem_euclid(360.0)
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
pub fn rand_vec_degree_spread(angle: f32, half_angle_spread: f32, length: f32) -> (f32, f32) {
    let a = rand_angle_spread_deg(angle, half_angle_spread);
    let vel = _Vec2::from_polar(a, length);
    vel.as_tuple()
}

#[pyfunction]
pub fn rand_vec_magnitude(angle: f32, lo_magnitude: f32, hi_magnitude: f32) -> (f32, f32) {
    let mut rng = thread_rng();
    let mag = rng.gen_range(lo_magnitude..hi_magnitude);
    let vel = _Vec2::from_polar(angle, mag);
    vel.as_tuple()
}

// This is only a subset of _Vec2 methods defined in arcade.math.py
struct _Vec2 {
    x: f32,
    y: f32,
}
#[allow(dead_code)]
impl _Vec2 {
    fn from_polar(angle: f32, radius: f32) -> _Vec2 {
        let rads = angle.to_radians();
        _Vec2 {
            x: radius * rads.cos(),
            y: radius * rads.sin(),
        }
    }

    fn as_tuple(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    fn __add__(&self, other: _Vec2) -> _Vec2 {
        _Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
    fn __sub__(&self, other: _Vec2) -> _Vec2 {
        _Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
    fn __mul__(&self, other: _Vec2) -> _Vec2 {
        _Vec2 {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
    fn __truediv__(&self, other: _Vec2) -> _Vec2 {
        _Vec2 {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
    fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    fn dot(&self, other: _Vec2) -> f32 {
        self.x * other.x + self.y * other.y
    }
    fn rotated(&self, angle: f32) -> _Vec2 {
        let rads = angle.to_radians();
        let cosine = rads.cos();
        let sine = rads.sin();
        _Vec2 {
            x: (self.x * cosine) - (self.y * sine),
            y: (self.y * cosine) + (self.x * sine),
        }
    }
    fn __repr__(&self) {
        format!("Vec2({},{})", self.x, self.y);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_eq::assert_float_eq;

    fn round_float(n: f32, decimals: u32) -> f32 {
        let nn = 10i32.pow(decimals) as f32;
        (n * nn).round() / nn
    }

    #[test]
    fn test_clamp() {
        let mut result = clamp(1.2, 1.0, 2.0);
        assert_eq!(result, 1.2);

        result = clamp(2.5, 1.0, 2.0);
        assert_eq!(result, 2.0);

        result = clamp(0.8, 1.0, 2.0);
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_lerp() {
        let mut result = lerp(2.0, 3.0, 1.0);
        assert_eq!(result, 3.0);

        result = lerp(0.8, 1.2, 0.0);
        assert_eq!(result, 0.8);

        result = lerp(2.0, 4.0, 0.5);
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_lerp_vec() {
        let mut result = lerp_vec((0.0, 2.0), (8.0, 4.0), 0.25);
        assert_eq!(result, (2.0, 2.5));

        result = lerp_vec((0.0, 2.0), (8.0, 4.0), -0.25);
        assert_eq!(result, (-2.0, 1.5));
    }

    #[test]
    fn test_lerp_angle_normal() {
        //normal
        let mut result = lerp_angle(0.0, 90.0, 0.5);
        assert_eq!(result, 45.0);

        //backwards
        result = lerp_angle(90.0, 0.0, 0.5);
        assert_eq!(result, 45.0)
    }

    #[test]
    fn test_lerp_angle_loop_around() {
        //forward
        let mut result = lerp_angle(355.0, 15.0, 0.5);
        assert_eq!(result, 5.0);

        //backward
        result = lerp_angle(10.0, 350.0, 0.5);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_lerp_angle_equal() {
        let mut result = lerp_angle(50.0, 50.0, 0.5);
        assert_eq!(result, 50.0);

        //effectively equal
        result = lerp_angle(50.0, 50.0 + 360.0, 0.5);
        assert_eq!(result, 50.0);

        result = lerp_angle(50.0 - 360.0, 50.0, 0.5);
        assert_eq!(result, 50.0);
    }

    #[test]
    fn test_get_distance() {
        let mut result = get_distance(0.0, 0.0, 0.0, 0.0);
        assert_eq!(result, 0.0);

        result = get_distance(0.0, 0.0, 3.0, 4.0);
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_get_angle_degrees() {
        //0 when x_diff = 0, y_diff = 0
        let mut result = get_angle_degrees(0.0, 0.0, 0.0, 0.0);
        assert_eq!(result, 0.0);

        result = get_angle_degrees(0.0, 0.0, 0.0, 3.0);
        assert_eq!(result, 90.0);

        result = get_angle_degrees(0.0, 0.0, 1.0, 1.0);
        assert_eq!(result, 45.0);
    }

    #[test]
    fn test_get_angle_radians() {
        let pi = std::f32::consts::PI;
        //0 when x_diff = 0, y_diff = 0
        let mut result = get_angle_radians(0.0, 0.0, 0.0, 0.0);
        assert_eq!(result, 0.0);

        result = get_angle_radians(0.0, 0.0, 0.0, 3.0);
        assert_eq!(result, pi / 2.0);

        result = get_angle_radians(0.0, 0.0, 1.0, 1.0);
        assert_eq!(result, pi / 4.0);
    }

    #[test]
    fn test_vec2() {
        let s = _Vec2 { x: 1.5, y: 2.5 };
        assert_eq!(s.x, 1.5);
        assert_eq!(s.y, 2.5);
    }

    #[test]
    fn test_from_polar_in_vec2() {
        let mut result = _Vec2::from_polar(0.0, 1.0);
        let s = (result.x, result.y);
        assert_eq!(s, (1.0, 0.0));

        result = _Vec2::from_polar(90.0, 1.0);
        assert_float_eq!(result.x, 0.0, abs <= 1.0e-3);
        assert_float_eq!(result.y, 1.0, abs <= 1.0e-3);

        result = _Vec2::from_polar(45.0, 2.0);
        assert_float_eq!(result.x, 2.0f32.sqrt(), abs <= 1.0e-3);
        assert_float_eq!(result.y, 2.0f32.sqrt(), abs <= 1.0e-3);
    }

    #[test]
    fn test_length_in_vec2() {
        let mut s = _Vec2 { x: 3.0, y: 4.0 };
        let mut result = s.length();
        assert_eq!(result, 5.0);

        s = _Vec2 { x: 0.0, y: 0.0 };
        result = s.length();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_dot_in_vec2() {
        let s = _Vec2 { x: 1.0, y: 1.0 };
        let result = s.dot(_Vec2 { x: 2.0, y: 3.0 });
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_rotated_in_vec2() {
        let mut s = _Vec2 { x: 1.0, y: 0.0 };
        let mut result = s.rotated(0.0);
        assert_eq!(result.x, 1.0);
        assert_eq!(result.y, 0.0);

        result = s.rotated(90.0);
        assert_float_eq!(result.x, 0.0, abs <= 1.0e-3);
        assert_float_eq!(result.y, 1.0, abs <= 1.0e-3);

        s = _Vec2 { x: 0.0, y: 0.0 };
        result = s.rotated(25.0);
        assert_eq!(result.x, 0.0);
        assert_eq!(result.y, 0.0);
    }

    #[test]
    fn test_rotate_point() {
        let mut x: f32 = 0.0;
        let mut y: f32 = 0.0;
        let mut cx: f32 = 0.0;
        let mut cy: f32 = 0.0;
        let mut angle: f32 = 0.0;
        let mut point: (f32, f32) = rotate_point(x, y, cx, cy, angle);
        assert_float_eq!(round_float(point.0, 2), 0.0, abs <= 1.0e-3);
        assert_float_eq!(round_float(point.1, 2), 0.0, abs <= 1.0e-3);

        x = 0.0;
        y = 0.0;
        cx = 0.0;
        cy = 0.0;
        angle = 90.0;
        point = rotate_point(x, y, cx, cy, angle);
        assert_float_eq!(round_float(point.0, 2), 0.0, abs <= 1.0e-3);
        assert_float_eq!(round_float(point.1, 2), 0.0, abs <= 1.0e-3);

        x = 50.0;
        y = 50.0;
        cx = 0.0;
        cy = 0.0;
        angle = 0.0;
        point = rotate_point(x, y, cx, cy, angle);
        assert_float_eq!(round_float(point.0, 2), 50.0, abs <= 1.0e-3);
        assert_float_eq!(round_float(point.1, 2), 50.0, abs <= 1.0e-3);

        x = 50.0;
        y = 0.0;
        cx = 0.0;
        cy = 0.0;
        angle = 90.0;
        point = rotate_point(x, y, cx, cy, angle);
        assert_float_eq!(round_float(point.0, 2), 0.0, abs <= 1.0e-3);
        assert_float_eq!(round_float(point.1, 2), -50.0, abs <= 1.0e-3);

        x = 20.0;
        y = 10.0;
        cx = 10.0;
        cy = 10.0;
        angle = 180.0;
        point = rotate_point(x, y, cx, cy, angle);
        assert_float_eq!(round_float(point.0, 2), 0.0, abs <= 1.0e-3);
        assert_float_eq!(round_float(point.1, 2), 10.0, abs <= 1.0e-3);
    }
}
