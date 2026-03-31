use snake::grid::{Grid, GridCell};

#[test]
fn new_grid_sets_dimensions() {
    let grid = Grid::new(8, 6);

    assert_eq!(grid.width(), 8);
    assert_eq!(grid.height(), 6);
}

#[test]
fn new_grid_marks_borders_as_walls() {
    let grid = Grid::new(4, 4);

    assert_eq!(grid.cell(0, 0), &GridCell::Wall);
    assert_eq!(grid.cell(3, 0), &GridCell::Wall);
    assert_eq!(grid.cell(0, 3), &GridCell::Wall);
    assert_eq!(grid.cell(3, 3), &GridCell::Wall);
    assert_eq!(grid.cell(0, 2), &GridCell::Wall);
    assert_eq!(grid.cell(2, 0), &GridCell::Wall);
}

#[test]
fn new_grid_marks_inner_cells_as_empty() {
    let grid = Grid::new(5, 5);

    assert_eq!(grid.cell(1, 1), &GridCell::Empty);
    assert_eq!(grid.cell(2, 2), &GridCell::Empty);
    assert_eq!(grid.cell(3, 3), &GridCell::Empty);
}

#[test]
fn change_cell_updates_cell_value() {
    let mut grid = Grid::new(5, 5);

    grid.change_cell(2, 2, GridCell::Food);

    assert_eq!(grid.cell(2, 2), &GridCell::Food);
}

#[test]
fn on_food_consumed_clears_food_cell() {
    let mut grid = Grid::new(5, 5);
    grid.change_cell(2, 2, GridCell::Food);

    grid.on_food_consumed(2, 2);

    assert_eq!(grid.cell(2, 2), &GridCell::Empty);
}

#[test]
fn on_food_consumed_leaves_non_food_cell_unchanged() {
    let mut grid = Grid::new(5, 5);

    grid.on_food_consumed(2, 2);

    assert_eq!(grid.cell(2, 2), &GridCell::Empty);
    assert_eq!(grid.cell(0, 0), &GridCell::Wall);
}
