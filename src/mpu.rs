#[allow(non_snake_case)]
pub mod Interrupt0Config {
    #[allow(unused_imports)]
    use core::arch::asm;
    use hippomenes_derive::CSRAccess;
    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x400]
    pub enum Region0Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x400]
    pub enum Region0Width {}

    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x400]
    pub enum Region0Address {}
// ---------------------------------
    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x401]
    pub enum Region1Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x401]
    pub enum Region1Width {}

    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x401]
    pub enum Region1Address {}
// ------------------------------------
    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x402]
    pub enum Region2Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x402]
    pub enum Region2Width {}

    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x402]
    pub enum Region2Address {}
// --------------------------
    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x403]
    pub enum Region3Address {}

    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x403]
    pub enum Region3Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x403]
    pub enum Region3Width {}
}

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
// ---------------------------------
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
// ------------------------------------
    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x406]
    pub enum Region2Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x406]
    pub enum Region2Width {}

    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x406]
    pub enum Region2Address {}
// --------------------------
    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x407]
    pub enum Region3Address {}

    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x407]
    pub enum Region3Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x407]
    pub enum Region3Width {}
}

pub mod Interrupt2Config {
    #[allow(unused_imports)]
    use core::arch::asm;
    use hippomenes_derive::CSRAccess;
    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x408]
    pub enum Region0Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x408]
    pub enum Region0Width {}

    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x408]
    pub enum Region0Address {}
// ---------------------------------
    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x409]
    pub enum Region1Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x409]
    pub enum Region1Width {}

    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x409]
    pub enum Region1Address {}
// ------------------------------------
    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x40A]
    pub enum Region2Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x40A]
    pub enum Region2Width {}

    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x40A]
    pub enum Region2Address {}
// --------------------------
    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x40B]
    pub enum Region3Address {}

    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x40B]
    pub enum Region3Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x40B]
    pub enum Region3Width {}
}

pub mod Interrupt3Config {
    #[allow(unused_imports)]
    use core::arch::asm;
    use hippomenes_derive::CSRAccess;
    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x40C]
    pub enum Region0Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x40C]
    pub enum Region0Width {}

    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x40C]
    pub enum Region0Address {}
// ---------------------------------
    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x40D]
    pub enum Region1Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x40D]
    pub enum Region1Width {}

    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x40D]
    pub enum Region1Address {}
// ------------------------------------
    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x40E]
    pub enum Region2Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x40E]
    pub enum Region2Width {}

    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x40E]
    pub enum Region2Address {}
// --------------------------
    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x40F]
    pub enum Region3Address {}

    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x40F]
    pub enum Region3Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x40F]
    pub enum Region3Width {}
}

pub mod Interrupt4Config {
    #[allow(unused_imports)]
    use core::arch::asm;
    use hippomenes_derive::CSRAccess;
    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x410]
    pub enum Region0Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x410]
    pub enum Region0Width {}

    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x410]
    pub enum Region0Address {}
// ---------------------------------
    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x411]
    pub enum Region1Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x411]
    pub enum Region1Width {}

    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x411]
    pub enum Region1Address {}
// ------------------------------------
    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x412]
    pub enum Region2Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x412]
    pub enum Region2Width {}

    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x412]
    pub enum Region2Address {}
// --------------------------
    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x413]
    pub enum Region3Address {}

    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x413]
    pub enum Region3Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x413]
    pub enum Region3Width {}
}

pub mod Interrupt5Config {
    #[allow(unused_imports)]
    use core::arch::asm;
    use hippomenes_derive::CSRAccess;
    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x414]
    pub enum Region0Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x414]
    pub enum Region0Width {}

    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x414]
    pub enum Region0Address {}
// ---------------------------------
    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x415]
    pub enum Region1Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x415]
    pub enum Region1Width {}

    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x415]
    pub enum Region1Address {}
// ------------------------------------
    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x416]
    pub enum Region2Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x416]
    pub enum Region2Width {}

    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x416]
    pub enum Region2Address {}
// --------------------------
    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x417]
    pub enum Region3Address {}

    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x417]
    pub enum Region3Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x417]
    pub enum Region3Width {}
}

pub mod Interrupt6Config {
    #[allow(unused_imports)]
    use core::arch::asm;
    use hippomenes_derive::CSRAccess;
    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x418]
    pub enum Region0Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x418]
    pub enum Region0Width {}

    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x418]
    pub enum Region0Address {}
// ---------------------------------
    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x419]
    pub enum Region1Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x419]
    pub enum Region1Width {}

    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x419]
    pub enum Region1Address {}
// ------------------------------------
    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x41A]
    pub enum Region2Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x41A]
    pub enum Region2Width {}

    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x41A]
    pub enum Region2Address {}
// --------------------------
    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x41B]
    pub enum Region3Address {}

    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x41B]
    pub enum Region3Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x41B]
    pub enum Region3Width {}
}

pub mod Interrupt7Config {
    #[allow(unused_imports)]
    use core::arch::asm;
    use hippomenes_derive::CSRAccess;
    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x41C]
    pub enum Region0Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x41C]
    pub enum Region0Width {}

    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x41C]
    pub enum Region0Address {}
// ---------------------------------
    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x41D]
    pub enum Region1Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x41D]
    pub enum Region1Width {}

    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x41D]
    pub enum Region1Address {}
// ------------------------------------
    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x41E]
    pub enum Region2Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x41E]
    pub enum Region2Width {}

    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x41E]
    pub enum Region2Address {}
// --------------------------
    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x41F]
    pub enum Region3Address {}

    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x41F]
    pub enum Region3Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x41F]
    pub enum Region3Width {}
}

pub mod MPUConfig {
    #[allow(unused_imports)]
    use core::arch::asm;
    use hippomenes_derive::CSRAccess;
    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x420]
    pub enum Region0Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x420]
    pub enum Region0Width {}

    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x420]
    pub enum Region0Address {}
// ---------------------------------
    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x421]
    pub enum Region1Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x421]
    pub enum Region1Width {}

    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x421]
    pub enum Region1Address {}
// ------------------------------------
    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x422]
    pub enum Region2Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x422]
    pub enum Region2Width {}

    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x422]
    pub enum Region2Address {}
// --------------------------
    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x423]
    pub enum Region3Address {}

    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x423]
    pub enum Region3Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x423]
    pub enum Region3Width {}
}