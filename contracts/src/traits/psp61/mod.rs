#[openbrush::trait_definition]
pub trait PSP61 {
    #[ink(message)]
    fn supports_interface(&self, interface_id: u32) -> bool;
}
