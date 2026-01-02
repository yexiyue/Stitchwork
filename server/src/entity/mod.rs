pub mod customer;
pub mod order;
pub mod payroll;
pub mod piece_record;
pub mod process;
pub mod worker;

pub use customer::Entity as Customer;
pub use order::Entity as Order;
pub use payroll::Entity as Payroll;
pub use piece_record::Entity as PieceRecord;
pub use process::Entity as Process;
pub use worker::Entity as Worker;
