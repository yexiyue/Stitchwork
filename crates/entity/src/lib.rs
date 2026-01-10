pub mod customer;
pub mod order;
pub mod payroll;
pub mod payroll_record;
pub mod piece_record;
pub mod process;
pub mod register_code;
pub mod share;
pub mod user;
pub mod workshop;

pub mod prelude {
    pub use super::customer::Entity as Customer;
    pub use super::order::Entity as Order;
    pub use super::payroll::Entity as Payroll;
    pub use super::payroll_record::Entity as PayrollRecord;
    pub use super::piece_record::Entity as PieceRecord;
    pub use super::process::Entity as Process;
    pub use super::register_code::Entity as RegisterCode;
    pub use super::share::Entity as Share;
    pub use super::user::Entity as User;
    pub use super::workshop::Entity as Workshop;
}
