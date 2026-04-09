use snake::grid::{Grid, GridCell, Point};

fn point(x: i32, y: i32) -> Point {
    Point::new(x, y)
}

#[test]
fn new_grid_sets_dimensions() {
    let grid = Grid::new(8, 6);

    assert_eq!(grid.width(), 8);
    assert_eq!(grid.height(), 6);
}

#[test]
fn new_grid_marks_borders_as_walls() {
    let grid = Grid::new(4, 4);

    assert_eq!(grid.cell(&point(0, 0)), Some(&GridCell::Wall));
    assert_eq!(grid.cell(&point(3, 0)), Some(&GridCell::Wall));
    assert_eq!(grid.cell(&point(0, 3)), Some(&GridCell::Wall));
    assert_eq!(grid.cell(&point(3, 3)), Some(&GridCell::Wall));
    assert_eq!(grid.cell(&point(0, 2)), Some(&GridCell::Wall));
    assert_eq!(grid.cell(&point(2, 0)), Some(&GridCell::Wall));
}

#[test]
fn new_grid_marks_inner_cells_as_empty() {
    let grid = Grid::new(5, 5);

    assert_eq!(grid.cell(&point(1, 1)), Some(&GridCell::Empty));
    assert_eq!(grid.cell(&point(2, 2)), Some(&GridCell::Empty));
    assert_eq!(grid.cell(&point(3, 3)), Some(&GridCell::Empty));
}

#[test]
fn cell_returns_none_for_points_outside_grid() {
    let grid = Grid::new(5, 5);

    assert_eq!(grid.cell(&point(-1, 0)), None);
    assert_eq!(grid.cell(&point(0, -1)), None);
    assert_eq!(grid.cell(&point(5, 0)), None);
    assert_eq!(grid.cell(&point(0, 5)), None);
}

#[test]
fn change_cell_updates_cell_value() {
    let mut grid = Grid::new(5, 5);

    grid.change_cell(&point(2, 2), GridCell::Food);

    assert_eq!(grid.cell(&point(2, 2)), Some(&GridCell::Food));
}

#[test]
fn change_cell_can_overwrite_existing_value() {
    let mut grid = Grid::new(5, 5);
    grid.change_cell(&point(2, 2), GridCell::Food);

    grid.change_cell(&point(2, 2), GridCell::Empty);

    assert_eq!(grid.cell(&point(2, 2)), Some(&GridCell::Empty));
}

#[test]
fn change_cell_ignores_points_outside_grid() {
    let mut grid = Grid::new(5, 5);

    grid.change_cell(&point(5, 5), GridCell::Food);

    assert_eq!(grid.cell(&point(5, 5)), None);
    assert_eq!(grid.cell(&point(2, 2)), Some(&GridCell::Empty));
}

#[test]
fn on_food_consumed_clears_food_cell() {
    let mut grid = Grid::new(5, 5);
    grid.change_cell(&point(2, 2), GridCell::Food);

    grid.on_food_consumed(&point(2, 2));

    assert_eq!(grid.cell(&point(2, 2)), Some(&GridCell::Empty));
}

#[test]
fn on_food_consumed_leaves_non_food_cell_unchanged() {
    let mut grid = Grid::new(5, 5);

    grid.on_food_consumed(&point(2, 2));

    assert_eq!(grid.cell(&point(2, 2)), Some(&GridCell::Empty));
    assert_eq!(grid.cell(&point(0, 0)), Some(&GridCell::Wall));
}

#[test]
fn on_food_consumed_ignores_points_outside_grid() {
    let mut grid = Grid::new(5, 5);

    grid.on_food_consumed(&point(-1, 0));
    grid.on_food_consumed(&point(5, 5));

    assert_eq!(grid.cell(&point(2, 2)), Some(&GridCell::Empty));
    assert_eq!(grid.cell(&point(0, 0)), Some(&GridCell::Wall));
}

#[test]
fn within_bounds_returns_true_for_points_inside_grid() {
    let grid = Grid::new(5, 5);

    assert!(grid.in_bounds(&point(0, 0)));
    assert!(grid.in_bounds(&point(2, 2)));
    assert!(grid.in_bounds(&point(4, 4)));
}

#[test]
fn within_bounds_returns_false_for_points_outside_grid() {
    let grid = Grid::new(5, 5);

    assert!(!grid.in_bounds(&point(-1, 0)));
    assert!(!grid.in_bounds(&point(0, -1)));
    assert!(!grid.in_bounds(&point(5, 0)));
    assert!(!grid.in_bounds(&point(0, 5)));
}
