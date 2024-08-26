use crate::project::Project;

#[derive(Debug)]
pub struct User {
    pub name: String,
    pub tokens: u64,
}

impl User {
    pub fn new(name: &str, tokens: u64) -> Self {
        User {
            name: name.to_string(),
            tokens,
        }
    }

    pub fn buy_tokens(&mut self, project: &mut Project, amount: u64) -> bool {
        if self.tokens >= amount {
            self.tokens -= amount;
            project.buy_tokens(amount);
            true
        } else {
            false
        }
    }

    pub fn refund_tokens(&mut self, project: &mut Project, amount: u64) -> bool {
        if project.refund_tokens(amount) {
            self.tokens += amount;
            true
        } else {
            false
        }
    }
}
