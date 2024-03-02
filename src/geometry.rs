use pyo3::prelude::*;

pub fn are_polygons_intersecting_native(
    poly_a: &Vec<(f32, f32)>,
    poly_b: &Vec<(f32, f32)>,
) -> bool {
    // If either polygon is empty, we should just return False
    if poly_a.is_empty() || poly_b.is_empty() {
        return false;
    }
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

            for point in polygons[0] {
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

            for point in polygons[1] {
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

#[pyfunction]
pub fn are_polygons_intersecting(poly_a: Vec<(f32, f32)>, poly_b: Vec<(f32, f32)>) -> bool {
    // If either polygon is empty, we should just return False
    if poly_a.is_empty() || poly_b.is_empty() {
        return false;
    }
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

#[pyfunction]
pub fn is_point_in_box(p: (f32, f32), q: (f32, f32), r: (f32, f32)) -> bool {
    (q.0 <= p.0.max(r.0)) && (q.0 >= p.0.min(r.0)) && (q.1 <= p.1.max(r.1)) && (q.1 >= p.1.min(r.1))
}

#[pyfunction]
pub fn get_triangle_orientation(p: (f32, f32), q: (f32, f32), r: (f32, f32)) -> i32 {
    let val: f32 = ((q.1 - p.1) * (r.0 - q.0)) - ((q.0 - p.0) * (r.1 - q.1));
    if val == 0.0 {
        0 //collinear
    } else if val > 0.0 {
        1 // clockwise
    } else {
        2 // counter clockwise
    }
}

#[pyfunction]
pub fn are_lines_intersecting(
    p1: (f32, f32),
    q1: (f32, f32),
    p2: (f32, f32),
    q2: (f32, f32),
) -> bool {
    let o1 = get_triangle_orientation(p1, q1, p2);
    let o2 = get_triangle_orientation(p1, q1, q2);
    let o3 = get_triangle_orientation(p2, q2, p1);
    let o4 = get_triangle_orientation(p2, q2, q1);
    // General case
    ((o1 != o2) && (o3 != o4))
    // p1, q1 and p2 are collinear and p2 lies on segment p1q1
    || ((o1 == 0) && is_point_in_box(p1, p2, q1))
    // p1, q1 and p2 are collinear and q2 lies on segment p1q1
    || ((o2 == 0) && is_point_in_box(p1, q2, q1))
    // p2, q2 and p1 are collinear and p1 lies on segment p2q2
    || ((o3 == 0) && is_point_in_box(p2, p1, q2))
    // p2, q2 and q1 are collinear and q1 lies on segment p2q2
    || ((o4 == 0) && is_point_in_box(p2, q1, q2))
}

#[pyfunction]
pub fn is_point_in_polygon(x: f32, y: f32, polygon: Vec<(f32, f32)>) -> bool {
    let p = (x, y);
    let n = polygon.len();

    // There must be at least 3 vertices
    // in polygon
    if n < 3 {
        return false;
    }

    // Create a point for line segment
    // from p to infinite
    let extreme = (f32::MAX, p.1);

    // To count number of points in polygon
    // whose y-coordinate is equal to
    // y-coordinate of the point
    let mut decrease = 0;
    let mut count = 0;
    let mut i = 0;

    loop {
        let next_item = (i + 1) % n;

        if polygon[i].1 == p.1 {
            decrease += 1;
        }

        // Check if the line segment from 'p' to
        // 'extreme' intersects with the line
        // segment from 'polygon[i]' to 'polygon[next]'
        if are_lines_intersecting(polygon[i], polygon[next_item], p, extreme) {
            // If the point 'p' is collinear with line
            // segment 'i-next', then check if it lies
            // on segment. If it lies, return true, otherwise false
            if get_triangle_orientation(polygon[i], p, polygon[next_item]) == 0 {
                return !is_point_in_box(polygon[i], p, polygon[next_item]);
            }

            count += 1
        }

        i = next_item;

        if i == 0 {
            break;
        }
    }

    // Reduce the count by decrease amount
    // as these points would have been added twice
    count -= decrease;

    // Return true if count is odd, false otherwise
    count % 2 == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_are_polygons_intersecting() {
        let mut poly_a: Vec<(f32, f32)> = vec![(0.0, 0.0), (0.0, 50.0), (50.0, 50.0), (50.0, 0.0)];
        let mut poly_b: Vec<(f32, f32)> =
            vec![(25.0, 25.0), (25.0, 75.0), (75.0, 75.0), (75.0, 25.0)];
        let mut result = are_polygons_intersecting(poly_a, poly_b);
        assert!(result);

        poly_a = vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
        poly_b = vec![(5.0, 5.0), (6.0, 5.0), (6.0, 6.0), (5.0, 6.0)];
        result = are_polygons_intersecting(poly_a, poly_b);
        assert!(!result);
    }

    #[test]
    fn test_empty_polygons_intersecting() {
        let poly_a: Vec<(f32, f32)> = vec![];
        let poly_b: Vec<(f32, f32)> = vec![];
        let result = are_polygons_intersecting(poly_a, poly_b);
        assert!(!result);
    }

    #[test]
    fn test_is_point_in_box() {
        // point inside
        let mut result = is_point_in_box((0.0, 0.0), (50.0, 50.0), (100.0, 100.0));
        assert!(result);

        //point outside
        result = is_point_in_box((0.0, 0.0), (-1.0, -1.0), (100.0, 100.0));
        assert!(!result);
    }

    #[test]
    fn test_get_triangle_orientation_colinear() {
        // collinear
        let result = get_triangle_orientation((0.0, 0.0), (0.0, 1.0), (0.0, 2.0));
        assert_eq!(result, 0);
    }

    #[test]
    fn test_get_triangle_orientation_clockwise() {
        let result = get_triangle_orientation((0.0, 0.0), (0.0, 1.0), (1.0, 1.0));
        assert_eq!(result, 1);
    }

    #[test]
    fn test_get_triangle_orientation_counterclockwise() {
        let result = get_triangle_orientation((1.0, 1.0), (0.0, 1.0), (0.0, 0.0));
        assert_eq!(result, 2);
    }

    #[test]
    fn test_are_lines_intersecting_colinear() {
        let result = are_lines_intersecting((0.0, 0.0), (1.0, 1.0), (0.0, 0.0), (1.0, 1.0));
        assert!(result);
    }

    #[test]
    fn test_are_lines_intersecting() {
        let result = are_lines_intersecting((0.0, 0.0), (1.0, 1.0), (0.0, 1.0), (1.0, 0.0));
        assert!(result);
    }

    #[test]
    fn test_are_lines_intersecting_parallel() {
        let result = are_lines_intersecting((0.0, 0.0), (0.0, 1.0), (1.0, 0.0), (1.0, 1.0));
        assert!(!result);
    }

    #[test]
    fn test_point_in_rectangle() {
        let polygon = vec![(0.0, 0.0), (0.0, 50.0), (50.0, 50.0), (50.0, 0.0)];
        let result = is_point_in_polygon(25.0, 25.0, polygon);
        assert!(result);
    }

    #[test]
    fn test_point_not_in_rectangle() {
        let polygon = vec![(0.0, 0.0), (0.0, 50.0), (50.0, 50.0), (50.0, 0.0)];
        let result = is_point_in_polygon(100.0, 100.0, polygon);
        assert!(!result);
    }

    #[test]
    fn test_point_not_in_empty_polygon() {
        let polygon = vec![];
        let result = is_point_in_polygon(25.0, 25.0, polygon);
        assert!(!result);
    }
}
