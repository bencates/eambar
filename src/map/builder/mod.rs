mod simple;

pub use simple::SimpleMapBuilder;

use super::Map;

pub trait MapBuilder {
    fn build(self) -> Map;
}

// impl dyn MapBuilder {
//     pub fn new() -> Box<Self> {
//         Box::new(SimpleMapBuilder)
//     }
// }
