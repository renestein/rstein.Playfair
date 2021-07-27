use crate::globalconstants::*;
use crate::operationtype::*;
use std::fmt::Display;
pub struct PlayfairConfiguration {
    pub unused_char: char,
    pub replace_char_for_unused_char: char,
    pub surrogate_char: char,
    pub key: String,
    pub operation_type: OperationType,
}

impl PlayfairConfiguration {
    pub fn new() -> Self {
        PlayfairConfiguration {
            unused_char: UNUSED_CHAR,
            replace_char_for_unused_char: REPLACE_CHAR_FOR_UNUSED_CHAR,
            surrogate_char: CHAR_SURROGATE,
            key: String::new(),
            operation_type: OperationType::Decrypt,
        }
    }

    pub fn is_encrypt(&self) -> bool {
        self.operation_type == OperationType::Encrypt
    }

    pub fn is_decrypt(&self) -> bool {
        !self.is_encrypt()
    }
}

impl Display for PlayfairConfiguration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(f, "(unused_char='{}', replace_char_for_unused_char='{}', surrogate_char='{}', key='{}', operation_type='{:?}')", 
                    self.unused_char, 
                    self.replace_char_for_unused_char,
                    self.surrogate_char,
                    self.key,
                    self.operation_type
                )
            
            }
        }