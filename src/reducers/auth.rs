use crate::redux::Reducer;

pub struct AuthSlice {
    pub token: String,
    pub token2: String
}

impl AuthSlice {
    pub fn new() -> Self {
        Self {
            token: "".to_string(),
            token2: "".to_string()
        }
    }
}

pub enum AuthAction {
    SetToken(String)
}

impl Reducer<AuthSlice> for AuthAction {
    fn reduce(self, slice: &mut AuthSlice) {
        match self {
            AuthAction::SetToken(t) => slice.token = t
        }
    }
}
