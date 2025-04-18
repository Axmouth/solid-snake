pub mod add;
pub mod debug;
pub mod decr;
pub mod function;
pub mod incr;
pub mod jump;
pub mod lessthan;
pub mod lessthanorequal;
pub mod load;
pub mod memory;
pub mod mov;
pub mod store;
pub mod subtract;

pub mod all {
    pub use super::*;

    pub use add::*;
    pub use debug::*;
    pub use decr::*;
    pub use function::*;
    pub use incr::*;
    pub use jump::*;
    pub use lessthan::*;
    pub use lessthanorequal::*;
    pub use load::*;
    pub use memory::*;
    pub use mov::*;
    pub use store::*;
    pub use subtract::*;
}
