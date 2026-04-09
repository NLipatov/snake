use snake::grid::{Grid, GridCell, Point};
use snake::grid_geometry::GridGeometry;

fn point(x: i32, y: i32) -> Point {
    Point::new(x, y)
}

fn grid(width: i32, height: i32) -> Grid {
    Grid::new(GridGeometry::new(width, height))
}

#[test]
fn new_grid_sets_dimensions() {
    let grid = grid(8, 6);

    assert_eq!(grid.width(), 8);
    assert_eq!(grid.height(), 6);
}

#[test]
fn new_grid_marks_borders_as_walls() {
    let grid = grid(4, 4);

    assert_eq!(grid.cell(&point(0, 0)), &GridCell::Wall);
    assert_eq!(grid.cell(&point(3, 0)), &GridCell::Wall);
    assert_eq!(grid.cell(&point(0, 3)), &GridCell::Wall);
    assert_eq!(grid.cell(&point(3, 3)), &GridCell::Wall);
    assert_eq!(grid.cell(&point(0, 2)), &GridCell::Wall);
    assert_eq!(grid.cell(&point(2, 0)), &GridCell::Wall);
}

#[test]
fn new_grid_marks_inner_cells_as_empty() {
    let grid = grid(5, 5);

    assert_eq!(grid.cell(&point(1, 1)), &GridCell::Empty);
    assert_eq!(grid.cell(&point(2, 2)), &GridCell::Empty);
    assert_eq!(grid.cell(&point(3, 3)), &GridCell::Empty);
}

#[test]
#[should_panic(expected = "point should be withing grid bounds")]
fn cell_panics_for_points_outside_grid() {
    let grid = grid(5, 5);

    let _ = grid.cell(&point(-1, 0));
}

#[test]
fn change_cell_updates_cell_value() {
    let mut grid = grid(5, 5);

    grid.change_cell(&point(2, 2), GridCell::Food);

    assert_eq!(grid.cell(&point(2, 2)), &GridCell::Food);
}

#[test]
fn change_cell_can_overwrite_existing_value() {
    let mut grid = grid(5, 5);
    grid.change_cell(&point(2, 2), GridCell::Food);

    grid.change_cell(&point(2, 2), GridCell::Empty);

    assert_eq!(grid.cell(&point(2, 2)), &GridCell::Empty);
}

#[test]
fn change_cell_ignores_points_outside_grid() {
    let mut grid = grid(5, 5);

    grid.change_cell(&point(5, 5), GridCell::Food);

    assert!(!grid.in_bounds(&point(5, 5)));
    assert_eq!(grid.cell(&point(2, 2)), &GridCell::Empty);
}

#[test]
fn on_food_consumed_clears_food_cell() {
    let mut grid = grid(5, 5);
    grid.change_cell(&point(2, 2), GridCell::Food);

    grid.on_food_consumed(&point(2, 2));

    assert_eq!(grid.cell(&point(2, 2)), &GridCell::Empty);
}

#[test]
fn on_food_consumed_leaves_non_food_cell_unchanged() {
    let mut grid = grid(5, 5);

    grid.on_food_consumed(&point(2, 2));

    assert_eq!(grid.cell(&point(2, 2)), &GridCell::Empty);
    assert_eq!(grid.cell(&point(0, 0)), &GridCell::Wall);
}

#[test]
fn on_food_consumed_ignores_points_outside_grid() {
    let mut grid = grid(5, 5);

    grid.on_food_consumed(&point(-1, 0));
    grid.on_food_consumed(&point(5, 5));

    assert_eq!(grid.cell(&point(2, 2)), &GridCell::Empty);
    assert_eq!(grid.cell(&point(0, 0)), &GridCell::Wall);
}

#[test]
fn in_bounds_returns_true_for_points_inside_grid() {
    let grid = grid(5, 5);

    assert!(grid.in_bounds(&point(0, 0)));
    assert!(grid.in_bounds(&point(2, 2)));
    assert!(grid.in_bounds(&point(4, 4)));
}

#[test]
fn in_bounds_returns_false_for_points_outside_grid() {
    let grid = grid(5, 5);

    assert!(!grid.in_bounds(&point(-1, 0)));
    assert!(!grid.in_bounds(&point(0, -1)));
    assert!(!grid.in_bounds(&point(5, 0)));
    assert!(!grid.in_bounds(&point(0, 5)));
}
