use rand::{self, Rng};
use toodee::TooDee;

#[derive(Debug, Clone)]
pub struct Sandpile {
    pub x: usize,
    pub y: usize,
    pub cells: TooDee<usize>,
    probability_to_topple: f32,
    pub is_completely_toppled: bool,
    pub num_steps: usize,
    pub num_topples: usize,
}

impl Sandpile {
    pub fn new(x: usize, y: usize) -> Self {
        // first and last rows are "invisible" to make computations easier
        let x_offseted = x;
        let y_offseted = y;

        Sandpile {
            x: x_offseted,
            y: y_offseted,
            cells: TooDee::init(x_offseted, y_offseted, 0),
            probability_to_topple: 1.0,
            is_completely_toppled: false,
            num_steps: 0,
            num_topples: 0,
        }
    }

    pub fn set_value_at(&mut self, value: usize, coordinate: (usize, usize)) {
        self.cells[(coordinate)] = value;
    }

    pub fn get_value_at(&self, coordinate: (usize, usize)) -> usize {
        self.cells[(coordinate)]
    }

    pub fn set_probailitiy(&mut self, value: f32) {
        self.probability_to_topple = value.clamp(0.001, 1.0);
    }

    pub fn len_x(&self) -> usize {
        self.x
    }

    pub fn len_y(&self) -> usize {
        self.y
    }

    pub fn topple(&mut self, value: usize, x: usize, y: usize) {
        let x_compare = x as isize;
        let y_compare = y as isize;
        let x_max = (self.x - 1) as isize;
        let y_max = (self.y - 1) as isize;

        // increase step count
        self.num_steps += 1;
        if x_compare + 1 <= x_max
            && y_compare + 1 <= y_max
            && x_compare - 1 >= 0
            && y_compare - 1 >= 0
        {
            // add topple amount
            self.cells[(x, y)] += value;

            if self.cells[(x, y)] >= 4 {
                // increase topples count
                self.num_topples += 1;

                let multiples = self.cells[(x, y)] / 4;
                self.cells[(x, y)] -= 4 * multiples;

                self.topple(multiples, x - 1, y);
                self.topple(multiples, x, y - 1);
                self.topple(multiples, x + 1, y);
                self.topple(multiples, x, y + 1);
            }

            self.is_completely_toppled = true;
        }
    }

    pub fn topple_torus(&mut self, value: usize, x: usize, y: usize) {
        let x_max = self.x - 1;
        let y_max = self.y - 1;

        // increase step count
        self.num_steps += 1;

        // add topple amount
        self.cells[(x, y)] += value;

        if self.cells[(x, y)] >= 4 {
            // increase topples count
            self.num_topples += 1;

            let multiples = self.cells[(x, y)] / 4;
            self.cells[(x, y)] -= 4 * multiples;

            // check bound and wrap around if at border
            if x == 0 {
                self.topple_torus(multiples, x_max, y);
            } else {
                self.topple_torus(multiples, x - 1, y);
            }

            if y == 0 {
                self.topple_torus(multiples, x, y_max);
            } else {
                self.topple_torus(multiples, x, y - 1);
            }

            if x == x_max {
                self.topple_torus(multiples, 0, y);
            } else {
                self.topple_torus(multiples, x + 1, y);
            }

            if y == y_max {
                self.topple_torus(multiples, x, 0)
            } else {
                self.topple_torus(multiples, x, y + 1);
            }
        }

        self.is_completely_toppled = true;
    }

    pub fn topple_sandpile(&mut self) {
        let mut been_toppled = false;

        // use old algorithm if probability is 1 since it is way more effecient
        if self.probability_to_topple == 1.0 {
            for i in 1..(self.x - 1) {
                for j in 1..(self.y - 1) {
                    // increase step count
                    self.num_steps += 1;

                    // most efficitient algorithm for big piles
                    if self.cells[(i, j)] >= 8 {
                        // increase topples count
                        self.num_topples += 1;

                        let multiples = self.cells[(i, j)] / 4;

                        // reduce pile that's too big
                        self.cells[(i, j)] -= 4 * multiples;
                        been_toppled = true;
                        // move grains to neighbouring cells
                        self.cells[(i - 1, j)] += multiples;
                        self.cells[(i, j - 1)] += multiples;
                        self.cells[(i + 1, j)] += multiples;
                        self.cells[(i, j + 1)] += multiples;
                    }

                    // less division and multiplication for small piles
                    if self.cells[(i, j)] > 3 && self.cells[(i, j)] < 8 {
                        // increase topples count
                        self.num_topples += 1;

                        // reduce pile that's too big
                        self.cells[(i, j)] -= 4;
                        been_toppled = true;
                        // move grains to neighbouring cells
                        self.cells[(i - 1, j)] += 1;
                        self.cells[(i, j - 1)] += 1;
                        self.cells[(i + 1, j)] += 1;
                        self.cells[(i, j + 1)] += 1;
                    }
                }
            }
        } else {
            // create a random number tread for this topple stage
            let mut rng = rand::thread_rng();

            for i in 1..(self.x - 1) {
                for j in 1..(self.y - 1) {
                    if self.cells[(i, j)] > 3 {
                        // count how many grains have been moved
                        let mut moved_grain_counter: usize = 0;

                        // create random number between 0 and 1
                        // when the random number is higher than the probability threshold, dont't move sandgrain
                        let random: [f32; 4] = rng.gen();

                        // move grains to neighbouring cells if threshold is not met
                        for counter in 0..random.len() {
                            if random[counter] < self.probability_to_topple {
                                match counter {
                                    0 => {
                                        self.cells[(i - 1, j)] += 1;
                                    }
                                    1 => {
                                        self.cells[(i, j - 1)] += 1;
                                    }
                                    2 => {
                                        self.cells[(i + 1, j)] += 1;
                                    }
                                    3 => {
                                        self.cells[(i, j + 1)] += 1;
                                    }
                                    _ => (),
                                }
                                moved_grain_counter += 1;
                            }
                        }

                        // reduce grains that have been moved
                        self.cells[(i, j)] -= moved_grain_counter;

                        // even if no grains have been moved
                        // with really low probability values it could take a few passes until a grain will be moved
                        // setting this bool to true means piles with more than 3 grains have been found
                        been_toppled = true;
                    }
                }
            }
        }

        if !been_toppled {
            self.is_completely_toppled = true;
        }
    }

    pub fn topple_torus_naive(&mut self) {
        let mut been_toppled = false;

        // use old algorithm if probability is 1 since it is way more effecient
        if self.probability_to_topple == 1.0 {
            for i in 0..self.x {
                for j in 0..self.y {
                    // increase step count
                    self.num_steps += 1;

                    // most efficitient algorithm for big piles
                    if self.cells[(i, j)] >= 4 {
                        // increase topples count
                        self.num_topples += 1;

                        let multiples = self.cells[(i, j)] / 4;

                        // reduce pile that's too big
                        self.cells[(i, j)] -= 4 * multiples;
                        been_toppled = true;

                        if i == 0 {
                            self.cells[(self.x - 1, j)] += multiples;
                        } else {
                            self.cells[(i - 1, j)] += multiples;
                        }

                        if j == 0 {
                            self.cells[(i, self.y - 1)] += multiples;
                        } else {
                            self.cells[(i, j - 1)] += multiples;
                        }

                        if i == self.x - 1 {
                            self.cells[(0, j)] += multiples;
                        } else {
                            self.cells[(i + 1, j)] += multiples;
                        }

                        if j == self.y - 1 {
                            self.cells[(i, 0)] += multiples;
                        } else {
                            self.cells[(i, j + 1)] += multiples;
                        }
                    }
                }
            }
        }

        if !been_toppled {
            self.is_completely_toppled = true;
        }
    }

    pub fn reset(&mut self) {
        for i in 0..self.x {
            for j in 0..self.y {
                self.cells[(i, j)] = 0;
            }
        }
        self.is_completely_toppled = false;
    }
}
