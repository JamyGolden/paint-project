use std::{cmp::Ordering, mem::discriminant};

use implicit_clone::{sync::IArray, ImplicitClone};

use crate::components::pattern::GridCell;

#[derive(Clone, Debug, PartialEq, ImplicitClone)]
pub enum ActionType {
    Fill,
    Brush,
    Erase,
}

#[derive(Debug)]
pub enum HistoryError {
    InvalidGridCellPosition,
}

#[derive(Clone, Debug, PartialEq, ImplicitClone)]
pub enum Direction {
    Start(GridCell),
    Up(GridCell),
    Right(GridCell),
    Down(GridCell),
    Left(GridCell),
    UpRight(GridCell),
    DownRight(GridCell),
    DownLeft(GridCell),
    UpLeft(GridCell),
}

#[derive(Debug)]
pub struct History {
    pub path: IArray<Direction>,
    pub action_type: ActionType,
}

impl History {
    fn get_direction(
        start_grid_cell: GridCell,
        end_grid_cell: GridCell,
    ) -> Result<Direction, HistoryError> {
        let column_difference: usize =
            (start_grid_cell.0 as isize - end_grid_cell.0 as isize).unsigned_abs();
        let row_difference: usize =
            (start_grid_cell.1 as isize - end_grid_cell.1 as isize).unsigned_abs();

        if column_difference == 0 {
            match start_grid_cell.1.cmp(&end_grid_cell.1) {
                Ordering::Greater => Ok(Direction::Up(end_grid_cell)),
                Ordering::Less => Ok(Direction::Down(end_grid_cell)),
                Ordering::Equal => Err(HistoryError::InvalidGridCellPosition),
            }
        } else if row_difference == 0 {
            match start_grid_cell.0.cmp(&end_grid_cell.0) {
                Ordering::Greater => Ok(Direction::Left(end_grid_cell)),
                Ordering::Less => Ok(Direction::Right(end_grid_cell)),
                Ordering::Equal => Err(HistoryError::InvalidGridCellPosition),
            }
        } else if column_difference == 1 && row_difference == 1 {
            if start_grid_cell.0 < end_grid_cell.0 && start_grid_cell.1 < end_grid_cell.1 {
                Ok(Direction::DownRight(end_grid_cell))
            } else if start_grid_cell.0 < end_grid_cell.0 && start_grid_cell.1 > end_grid_cell.1 {
                Ok(Direction::UpRight(end_grid_cell))
            } else if start_grid_cell.0 > end_grid_cell.0 && start_grid_cell.1 > end_grid_cell.1 {
                Ok(Direction::UpLeft(end_grid_cell))
            } else if start_grid_cell.0 > end_grid_cell.0 && start_grid_cell.1 < end_grid_cell.1 {
                Ok(Direction::DownLeft(end_grid_cell))
            } else {
                Err(HistoryError::InvalidGridCellPosition)
            }
        } else {
            Err(HistoryError::InvalidGridCellPosition)
        }
    }

    // It's expected that each GridCell in raw_path is a maximum of 1 unit away from the last
    // GridCell
    pub fn from_raw_path(
        raw_path: Vec<GridCell>,
        action_type: ActionType,
    ) -> Result<Self, HistoryError> {
        let path = match action_type {
            ActionType::Brush | ActionType::Erase => {
                let mut final_path: Vec<Direction> = vec![];
                let mut prev_direction_option: Option<Direction> = None;
                let last_item_index = raw_path.len() - 1;

                for (i, grid_cell) in raw_path.iter().enumerate() {
                    let prev_path_option = if i > 0 { raw_path.get(i - 1) } else { None };

                    // If not the first item
                    if let (Some(prev_grid_cell), Some(prev_direction)) =
                        (prev_path_option, prev_direction_option)
                    {
                        let direction = History::get_direction(*prev_grid_cell, *grid_cell)?;

                        // Push current direction if final item
                        if i == last_item_index {
                            final_path.push(direction.clone());
                        // Otherwise push the previous item if the current direction is not equal
                        // to the previous direction.
                        } else if discriminant(&direction) != discriminant(&prev_direction) {
                            match prev_direction {
                                // Don't push the previous direction if it's Start since that
                                // has already been pushed
                                Direction::Start(_) => {}
                                _ => {
                                    final_path.push(prev_direction);
                                }
                            }
                        }

                        prev_direction_option = Some(direction);
                    // Otherwise push the starting direction if first item
                    } else {
                        let direction = Direction::Start(*grid_cell);
                        final_path.push(direction.clone());

                        prev_direction_option = Some(direction);
                    }
                }

                IArray::from(final_path)
            }
            _ => IArray::default(),
        };

        Ok(History { action_type, path })
    }
}
