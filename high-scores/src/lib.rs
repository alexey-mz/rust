#[derive(Debug)]
pub struct HighScores <'a>{
    scores: &'a [u32]
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a[u32]) -> Self {
        Self {
            scores
        }
    }

    pub fn scores(&'a self) -> &[u32] {
        self.scores
    }

    pub fn latest(&'a self) -> Option<u32> {
        self.scores.iter().map(|&x| x).collect::<Vec<u32>>().pop()
    }

    pub fn personal_best(&'a self) -> Option<u32> {
        self.scores.iter().map(|&x| x).max()
    }

    pub fn personal_top_three(&'a self) -> Vec<u32> {
        let mut vec = self.scores().to_vec();
        vec.sort_by(|a, b| b.cmp(a));
        vec.truncate(3);
        vec
    }
}
