pub struct GameMap {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Vec<u8>>,
}

impl GameMap {
    pub fn default() -> Self {
        Self {
            width: 16,
            height: 16,
            grid: vec![
                vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                vec![1; 16],
                vec![1; 16],
                vec![1; 16],
                vec![1; 16],
                vec![1; 16],
                vec![1; 16],
                vec![1; 16],
                vec![1; 16],
                vec![1; 16],
                vec![1; 16],
                vec![1; 16],
                vec![1; 16],
            ],
        }
    }

    pub fn is_wall(&self, x: usize, y: usize) -> bool {
        self.gridy[y][x] != 0;
    }
}
