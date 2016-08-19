#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate easy_util;

#[macro_use]
extern crate log;

extern crate rustc_serialize;
use rustc_serialize::json::Json;
use rustc_serialize::json::ToJson;

use std::collections::BTreeMap;

extern crate cons;
use cons::ErrCode;

extern crate util;

pub mod base;
pub mod validate;
pub mod draw;

use validate::ValidateFactory;
use draw::DrawFactory;
use draw::PrintFactory;

lazy_static! {
    pub static ref VF:ValidateFactory = ValidateFactory::new();
    pub static ref DF:DrawFactory = DrawFactory::new();
    pub static ref PF:PrintFactory = PrintFactory::new();
}
