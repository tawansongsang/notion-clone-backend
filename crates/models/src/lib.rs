use serde::{Deserialize, Serialize};

// the input to our `create_user` handler
#[derive(Deserialize)]
pub struct CreateUser {
    pub username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
pub struct User {
    pub id: u64,
    pub username: String,
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
