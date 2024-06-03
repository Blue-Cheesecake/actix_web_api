use serde::Serialize;

#[derive(Serialize)]
pub struct CommonSimpleMessageDto {
    pub message: String,
}

impl CommonSimpleMessageDto {
    pub fn from(message: &str) -> Self {
        Self {
            message: String::from(message),
        }
    }
}
