#[derive(Debug, Clone)]
pub struct Match {
    pub teams: String,
    pub competition: String,
    pub date: String,
    pub time: String,
    pub channels: Vec<String>,
}
