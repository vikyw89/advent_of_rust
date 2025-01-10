pub struct LogQuery<'a> {
    logs: &'a Vec<String>,
}

impl<'a> LogQuery<'a> {
    pub fn new(logs: &'a Vec<String>) -> Self {
        LogQuery { logs }
    }

    pub fn search(&self, keyword: &str) -> Vec<&'a String> {
        self.logs
            .iter()
            .filter(|log| log.contains(keyword))
            .collect()
    }

    pub fn export_to_file(&self, keyword: &str, path: &str) -> std::io::Result<()> {
        // 🎁 Your code here! 🎁
        let searched_logs = self
            .search(keyword)
            .iter()
            .map(|log| log.as_str())
            .collect::<Vec<&str>>();

        std::fs::write(path, searched_logs.join("\n"))
    }
}

pub fn main() {}
