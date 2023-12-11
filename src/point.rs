use std::ops::Sub;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl From<String> for Point {
    /// the string will be two numbers with a space between and a newline at the end
    ///
    /// example: `1234 1234\n`
    fn from(value: String) -> Self {
        let value = value.trim();
        let values = value
            .split(' ')
            .map(|coordinate| coordinate.parse::<f32>().unwrap())
            .collect::<Vec<f32>>();

        Self {
            x: values[0],
            y: values[1],
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
