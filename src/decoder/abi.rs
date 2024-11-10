use ethabi::{Contract, RawLog, Token};
use mongodb::bson::Document;
use std::sync::Arc;
use crate::error::Result;

#[derive(Clone)]
pub struct EventDecoder {
    contract: Arc<Contract>,
}

impl EventDecoder {
    pub fn new(contract: Arc<Contract>) -> Self {
        Self { contract }
    }

    pub fn decode_log(&self, raw_log: RawLog) -> Result<(String, Document)> {
        for event in self.contract.events() {
            if let Ok(decoded) = event.parse_log(raw_log.clone()) {
                let mut params = Document::new();
                for param in decoded.params {
                    let value = self.token_to_bson(param.value);
                    params.insert(param.name, value);
                }
                return Ok((event.name.clone(), params));
            }
        }
        Err(crate::error::Error::UnknownEvent)
    }

    fn token_to_bson(&self, token: Token) -> mongodb::bson::Bson {
        match token {
            Token::Address(addr) => mongodb::bson::Bson::String(format!("{:?}", addr)),
            Token::Uint(num) => mongodb::bson::Bson::String(num.to_string()),
            Token::Int(num) => mongodb::bson::Bson::String(num.to_string()),
            Token::Bool(b) => mongodb::bson::Bson::Boolean(b),
            Token::String(s) => mongodb::bson::Bson::String(s),
            Token::Bytes(b) => mongodb::bson::Bson::String(hex::encode(b)),
            _ => mongodb::bson::Bson::Null,
        }
    }
}
