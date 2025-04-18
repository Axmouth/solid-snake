
    pub fn reg_index(s: &str) -> u8 {
        s.trim_start_matches('R').parse::<u8>().unwrap()
    }
