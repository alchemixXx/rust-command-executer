use std::collections::{HashMap, HashSet};

pub struct ExecutionResult {
    pub failed: HashMap<String, String>,
    pub succeed: HashSet<String>,
    pub technical: HashSet<String>,
}
