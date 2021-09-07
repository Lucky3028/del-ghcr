pub trait TError {
    fn log(&self) {
        eprintln!("{}", self.get_log_as_str());
    }

    fn get_log_as_str(&self) -> String;
}
