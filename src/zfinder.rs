pub const JOKER: u8 = 0b01000000;

pub struct ZFinder {
    cursor: usize,
    pattern_size: usize,
    joined: Vec<u8>,
    z_arr: Vec<usize>,
}

impl ZFinder {
    pub fn new(data: &[u8], pattern: &[u8]) -> ZFinder {
        let mut joined = vec![];
        joined.extend(pattern);
        joined.push(u8::MAX);
        joined.extend(data);

        let mut result = ZFinder {
            cursor: 0,
            pattern_size: pattern.len(),
            joined,
            z_arr: vec![],
        };
        result.build_z_array();

        result
    }

    fn compare_parts(&self, first: usize, second: usize) -> usize {
        let mut cnt = 0;
        let max = std::cmp::max(first, second);

        // first >= second
        // joined = pattern$data

        while max + cnt < self.joined.len()
            && (self.joined[first + cnt] == self.joined[second + cnt]
                || self.joined[second + cnt] == JOKER)
        {
            cnt += 1;
        }

        cnt
    }

    fn build_z_array(&mut self) {
        self.z_arr = vec![0; self.joined.len()];
        let mut window_left = 0;
        let mut window_right = 0;

        for i in 1..self.joined.len() {
            if i > window_right {
                let cnt = self.compare_parts(i, 0);
                self.z_arr[i] = cnt;
                window_left = i;
                window_right = i + cnt - 1;
            } else if self.z_arr[i - window_left] < window_right - i + 1 {
                self.z_arr[i] = self.z_arr[i - window_left];
            } else {
                let cnt = self.compare_parts(window_right + 1, window_right - window_left + 1);
                self.z_arr[i] = window_right - i + 1 + cnt;
                window_left = i;
                window_right = window_right + self.z_arr[i] - 1;
            }
        }
    }

    pub fn find(&mut self) -> Result<usize, &str> {
        let start = std::cmp::max(self.cursor, self.pattern_size) + 1;

        for i in start..self.joined.len() {
            if self.z_arr[i] == self.pattern_size {
                self.cursor = i;
                return Ok(i - self.pattern_size - 1);
            }
        }

        Err("No occurrences")
    }

    pub fn find_all(&mut self) -> Result<Vec<usize>, &str> {
        let mut result = vec![];

        while let Ok(occurrence) = self.find() {
            result.push(occurrence);
        }

        match result.is_empty() {
            true => Err("No occurrences"),
            false => Ok(result),
        }
    }
}
