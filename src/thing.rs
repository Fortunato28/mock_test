use mockall::automock;
use mockall_double::double;

pub struct Thing {
    pub some: u32,
}
#[automock]
impl Thing {
    pub fn foo(&self) -> u32 {
        3
    }
}
