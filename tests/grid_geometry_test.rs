use snake::grid::Point;
use snake::grid_geometry::GridGeometry;

fn point(x: i32, y: i32) -> Point {
    Point::new(x, y)
}

#[test]
fn geometry_exposes_dimensions() {
    let geometry = GridGeometry::new(8, 6);

    assert_eq!(geometry.width(), 8);
    assert_eq!(geometry.height(), 6);
}

#[test]
fn geometry_index_maps_point_into_linear_offset() {
    let geometry = GridGeometry::new(8, 6);

    assert_eq!(geometry.index(&point(0, 0)), Some(0));
    assert_eq!(geometry.index(&point(3, 2)), Some(19));
    assert_eq!(geometry.index(&point(7, 5)), Some(47));
}

#[test]
fn geometry_index_returns_none_for_points_outside_bounds() {
    let geometry = GridGeometry::new(8, 6);

    assert_eq!(geometry.index(&point(-1, 0)), None);
    assert_eq!(geometry.index(&point(0, -1)), None);
    assert_eq!(geometry.index(&point(8, 0)), None);
    assert_eq!(geometry.index(&point(0, 6)), None);
}

#[test]
fn geometry_in_bounds_matches_valid_and_invalid_points() {
    let geometry = GridGeometry::new(8, 6);

    assert!(geometry.in_bounds(&point(0, 0)));
    assert!(geometry.in_bounds(&point(7, 5)));
    assert!(!geometry.in_bounds(&point(-1, 0)));
    assert!(!geometry.in_bounds(&point(8, 5)));
}
