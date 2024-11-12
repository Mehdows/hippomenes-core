#[allow(non_snake_case)]
pub mod Interrupt1Config {
    #[allow(unused_imports)]
    use core::arch::asm;
    use hippomenes_derive::CSRAccess;
    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x404]
    pub enum Region0Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x404]
    pub enum Region0Width {}

    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x404]
    pub enum Region0Address {}

    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x405]
    pub enum Region1Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x405]
    pub enum Region1Width {}

    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x405]
    pub enum Region1Address {}
}
