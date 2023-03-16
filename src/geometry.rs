use pyo3::prelude::*;

#[pyfunction]
pub fn are_polygons_intersecting(poly_a: Vec<(f32, f32)>, poly_b: Vec<(f32, f32)>) -> bool {
    let polygons = [poly_a, poly_b];
    for polygon in &polygons {
        for i1 in 0..polygon.len() {
            let i2 = (i1 + 1) % polygon.len();
            let projection_1 = polygon[i1];
            let projection_2 = polygon[i2];

            let normal = (
                projection_2.1 - projection_1.1,
                projection_1.0 - projection_2.0,
            );

            let mut min_a: Option<f32> = None;
            let mut max_a: Option<f32> = None;
            let mut min_b: Option<f32> = None;
            let mut max_b: Option<f32> = None;

            for point in &polygons[0] {
                let projected = normal.0 * point.0 + normal.1 * point.1;
                match min_a {
                    Some(x) if projected < x => min_a = Some(projected),
                    Some(_x) => {}
                    None => min_a = Some(projected),
                }
                match max_a {
                    Some(x) if projected > x => max_a = Some(projected),
                    Some(_x) => {}
                    None => max_a = Some(projected),
                }
            }

            for point in &polygons[1] {
                let projected = normal.0 * point.0 + normal.1 * point.1;
                match min_b {
                    Some(x) if projected < x => min_b = Some(projected),
                    Some(_x) => {}
                    None => min_b = Some(projected),
                }
                match max_b {
                    Some(x) if projected > x => max_b = Some(projected),
                    Some(_x) => {}
                    None => max_b = Some(projected),
                }
            }

            if max_a <= min_b || max_b <= min_a {
                return false;
            }
        }
    }
    true
}
