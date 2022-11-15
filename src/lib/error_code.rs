use serde::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ErrorCode {
    code: u32,
}

impl ErrorCode {
    pub fn new(code: u32) -> Self {
        Self { code }
    }

    pub fn to_u32(self) -> u32 {
        self.code
    }
}
