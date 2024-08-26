use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub description: String,
    pub funding_goal: u64,
    pub tokens_sold: u64,
    pub success_threshold: u64,
    pub is_successful: bool,
}

impl Project {
    pub fn new(name: &str, description: &str, funding_goal: u64, success_threshold: u64) -> Self {
        Project {
            name: name.to_string(),
            description: description.to_string(),
            funding_goal,
            tokens_sold: 0,
            success_threshold,
            is_successful: false,
        }
    }

    pub fn buy_tokens(&mut self, amount: u64) {
        self.tokens_sold += amount;
        if self.tokens_sold >= self.success_threshold {
            self.is_successful = true;
        }
    }

    pub fn refund_tokens(&mut self, amount: u64) -> bool {
        if self.tokens_sold >= amount {
            self.tokens_sold -= amount;
            true
        } else {
            false
        }
    }
}
