
extern crate debug;
extern crate core;

mod sudoku {
    pub struct SudokuGrid {
        cells: [int, ..81]
    }

    /* 
     * Need to find out how to do this kind of thing:
     *
     * fn row_position_iterator(row: uint) -> &Iterator<(uint, uint)> {
     *     range(0u, 9u).map(|x| (row, x))
     * }
     */
    struct RowPositionIterator {
        row: uint,
        col: uint
    }

    impl RowPositionIterator {
        fn new(row: uint) -> RowPositionIterator {
            assert!(row < 9, "row ({}) must be < 9", row);
            RowPositionIterator { row: row, col: 0u }
        }
    }

    impl Iterator<(uint, uint)> for RowPositionIterator {
        fn next(&mut self) -> Option<(uint, uint)> {
            if self.col < 9 {
                let out = Some((self.row, self.col));
                self.col += 1;
                out
            } else {
                None
            }
        }
    }

    struct ColPositionIterator {
        row: uint,
        col: uint
    }

    impl ColPositionIterator {
        fn new(col: uint) -> ColPositionIterator {
            assert!(col < 9, "col ({}) must be < 9", col);
            ColPositionIterator { row: 0, col: col }
        }
    }
    
    impl Iterator<(uint, uint)> for ColPositionIterator {
        fn next(&mut self) -> Option<(uint, uint)> {
            if self.row < 9 {
                let out = Some((self.row, self.col));
                self.row += 1;
                out
            } else {
                None
            }
        }
    }

    struct BoxPositionIterator {
        row: uint,
        col: uint,
        pos: uint,
    }

    impl BoxPositionIterator {
        fn new(row: uint, col: uint) -> BoxPositionIterator {
            assert!(row % 3 == 0, "row ({}) must be divisible by 3", row);
            assert!(row < 9, "row ({}) must be < 9", row);
            assert!(col % 3 == 0, "col ({}) must be divisible by 3", col);
            assert!(col < 9, "col ({}) must be < 9", col);
            BoxPositionIterator { row: row, col: col, pos: 0 }
        }
    }
    
    impl Iterator<(uint, uint)> for BoxPositionIterator {
        fn next(&mut self) -> Option<(uint, uint)> {
            if self.pos < 9 {
                let out = Some((
                    self.row + self.pos / 3,
                    self.col + self.pos % 3
                ));
                self.pos += 1;
                out
            } else {
                None
            }
        }
    }
    
    pub fn show_iterator_outputs() {
        let mut iterator = RowPositionIterator::new(1u);
        println!("for iterator = {:?}", iterator);
        for pair in iterator {
            println!("iterator.next() => {:?}", pair);
        }

        let mut iterator = ColPositionIterator::new(1u);
        println!("for iterator = {:?}", iterator);
        for pair in iterator {
            println!("iterator.next() => {:?}", pair);
        }

        let mut iterator = BoxPositionIterator::new(3u, 0u);
        println!("for iterator = {:?}", iterator);
        for pair in iterator {
            println!("iterator.next() => {:?}", pair);
        }
    }

    fn is_complete(iter: ~[int]) -> bool {
        let mut symbols = [0, ..9];
        for val in iter.iter() {
            if 9 <= *val {
                fail!("bad symbol");
            }
            if 0 <= *val {
                symbols[*val as uint] += 1; 
            }
        }
        for counter in symbols.iter() {
            if *counter != 1 {
                return false;
            }
        }
        return true;
    }

    impl SudokuGrid {
        pub fn new() -> SudokuGrid {
            let cells = [-1, ..81];
            SudokuGrid { cells: cells }
        }

        pub fn get_cell(&self, pos: (uint, uint)) -> int {
            let (x, y) = pos;
            self.cells[x * 9 + y]
        }
        
        pub fn put_cell(&self, pos: (uint, uint), val: int) -> SudokuGrid {
            let (x, y) = pos;
            let mut cells = self.cells;
            cells[x * 9 + y] = val;
            SudokuGrid { cells: cells }
        }

        pub fn is_complete(&self) -> bool {
            if !self.is_complete_rows() {
                return false;
            }
            if !self.is_complete_cols() {
                return false;
            }
            if !self.is_complete_boxes() {
                return false;
            }
            return true;
        }

        fn is_complete_rows(&self) -> bool {
            for i in range(0u, 9u) {
                let mut iter = RowPositionIterator::new(i);
                let quack = iter.map(|pair| self.get_cell(pair));
                println!("quack = {:?}", quack);
                /*let quack2 = quac.collect();
                if !is_complete(quack) {
                    return false;
                }*/
				fail!("left off here");
            }
            true
        }

        fn is_complete_col(&self, col: uint) -> bool {
            for i in range(0, 9) {
                let mut occurrences = 0;
                for j in range(0u, 9u) {
                    if self.get_cell((j, col)) == i {
                        occurrences += 1;
                    }
                }
                if occurrences != 1 {
                    return false;
                }
            }
            return true;
        }

        fn is_complete_cols(&self) -> bool {
            for i in range(0u, 9u) {
                if !self.is_complete_col(i) {
                    return false;
                }
            }
            return true;
        }

        fn is_complete_boxes(&self) -> bool {
            fail!("not implemented");
        }
    }
}


fn main() {
    sudoku::show_iterator_outputs();

    let grid = sudoku::SudokuGrid::new();
    println!("grid.cell[0, 0] is {:d}", grid.get_cell((0, 0)));
    let grid2 = grid.put_cell((0, 0), 1);
    println!("grid2.cell[0, 0] is {:d}", grid2.get_cell((0, 0)));
    println!("grid.cell[0, 0] is {:d}", grid.get_cell((0, 0)));
    println!("grid.is_complete() => {:?}", grid.is_complete());
}

