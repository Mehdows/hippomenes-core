#[allow(non_snake_case)]
pub mod Interrupt0Config {
    #[allow(unused_imports)]
    use core::arch::asm;
    use hippomenes_derive::CSRAccess;
    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x500]
    pub enum Region0Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x500]
    pub enum Region0Width {}

    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x500]
    pub enum Region0Address {}
// ---------------------------------
    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x501]
    pub enum Region1Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x501]
    pub enum Region1Width {}

    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x501]
    pub enum Region1Address {}
// ------------------------------------
    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x502]
    pub enum Region2Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x502]
    pub enum Region2Width {}

    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x502]
    pub enum Region2Address {}
// --------------------------
    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x503]
    pub enum Region3Address {}

    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x503]
    pub enum Region3Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x503]
    pub enum Region3Width {}
}

pub mod Interrupt1Config {
    #[allow(unused_imports)]
    use core::arch::asm;
    use hippomenes_derive::CSRAccess;
    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x504]
    pub enum Region0Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x504]
    pub enum Region0Width {}

    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x504]
    pub enum Region0Address {}
// ---------------------------------
    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x505]
    pub enum Region1Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x505]
    pub enum Region1Width {}

    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x505]
    pub enum Region1Address {}
// ------------------------------------
    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x506]
    pub enum Region2Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x506]
    pub enum Region2Width {}

    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x506]
    pub enum Region2Address {}
// --------------------------
    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x507]
    pub enum Region3Address {}

    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x507]
    pub enum Region3Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x507]
    pub enum Region3Width {}
}

pub mod Interrupt2Config {
    #[allow(unused_imports)]
    use core::arch::asm;
    use hippomenes_derive::CSRAccess;
    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x508]
    pub enum Region0Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x508]
    pub enum Region0Width {}

    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x508]
    pub enum Region0Address {}
// ---------------------------------
    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x509]
    pub enum Region1Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x509]
    pub enum Region1Width {}

    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x509]
    pub enum Region1Address {}
// ------------------------------------
    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x50A]
    pub enum Region2Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x50A]
    pub enum Region2Width {}

    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x50A]
    pub enum Region2Address {}
// --------------------------
    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x50B]
    pub enum Region3Address {}

    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x50B]
    pub enum Region3Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x50B]
    pub enum Region3Width {}
}

pub mod Interrupt3Config {
    #[allow(unused_imports)]
    use core::arch::asm;
    use hippomenes_derive::CSRAccess;
    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x50C]
    pub enum Region0Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x50C]
    pub enum Region0Width {}

    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x50C]
    pub enum Region0Address {}
// ---------------------------------
    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x50D]
    pub enum Region1Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x50D]
    pub enum Region1Width {}

    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x50D]
    pub enum Region1Address {}
// ------------------------------------
    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x50E]
    pub enum Region2Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x50E]
    pub enum Region2Width {}

    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x50E]
    pub enum Region2Address {}
// --------------------------
    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x50F]
    pub enum Region3Address {}

    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x50F]
    pub enum Region3Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x50F]
    pub enum Region3Width {}
}

pub mod Interrupt4Config {
    #[allow(unused_imports)]
    use core::arch::asm;
    use hippomenes_derive::CSRAccess;
    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x510]
    pub enum Region0Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x510]
    pub enum Region0Width {}

    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x510]
    pub enum Region0Address {}
// ---------------------------------
    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x511]
    pub enum Region1Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x511]
    pub enum Region1Width {}

    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x511]
    pub enum Region1Address {}
// ------------------------------------
    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x512]
    pub enum Region2Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x512]
    pub enum Region2Width {}

    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x512]
    pub enum Region2Address {}
// --------------------------
    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x513]
    pub enum Region3Address {}

    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x513]
    pub enum Region3Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x513]
    pub enum Region3Width {}
}

pub mod Interrupt5Config {
    #[allow(unused_imports)]
    use core::arch::asm;
    use hippomenes_derive::CSRAccess;
    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x514]
    pub enum Region0Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x514]
    pub enum Region0Width {}

    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x514]
    pub enum Region0Address {}
// ---------------------------------
    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x515]
    pub enum Region1Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x515]
    pub enum Region1Width {}

    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x515]
    pub enum Region1Address {}
// ------------------------------------
    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x516]
    pub enum Region2Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x516]
    pub enum Region2Width {}

    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x516]
    pub enum Region2Address {}
// --------------------------
    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x517]
    pub enum Region3Address {}

    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x517]
    pub enum Region3Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x517]
    pub enum Region3Width {}
}

pub mod Interrupt6Config {
    #[allow(unused_imports)]
    use core::arch::asm;
    use hippomenes_derive::CSRAccess;
    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x518]
    pub enum Region0Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x518]
    pub enum Region0Width {}

    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x518]
    pub enum Region0Address {}
// ---------------------------------
    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x519]
    pub enum Region1Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x519]
    pub enum Region1Width {}

    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x519]
    pub enum Region1Address {}
// ------------------------------------
    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x51A]
    pub enum Region2Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x51A]
    pub enum Region2Width {}

    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x51A]
    pub enum Region2Address {}
// --------------------------
    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x51B]
    pub enum Region3Address {}

    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x51B]
    pub enum Region3Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x51B]
    pub enum Region3Width {}
}

pub mod Interrupt7Config {
    #[allow(unused_imports)]
    use core::arch::asm;
    use hippomenes_derive::CSRAccess;
    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x51C]
    pub enum Region0Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x51C]
    pub enum Region0Width {}

    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x51C]
    pub enum Region0Address {}
// ---------------------------------
    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x51D]
    pub enum Region1Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x51D]
    pub enum Region1Width {}

    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x51D]
    pub enum Region1Address {}
// ------------------------------------
    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x51E]
    pub enum Region2Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x51E]
    pub enum Region2Width {}

    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x51E]
    pub enum Region2Address {}
// --------------------------
    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x51F]
    pub enum Region3Address {}

    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x51F]
    pub enum Region3Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x51F]
    pub enum Region3Width {}
}

pub mod MPUConfig {
    #[allow(unused_imports)]
    use core::arch::asm;
    use hippomenes_derive::CSRAccess;
    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x520]
    pub enum Region0Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x520]
    pub enum Region0Width {}

    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x520]
    pub enum Region0Address {}
// ---------------------------------
    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x521]
    pub enum Region1Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x521]
    pub enum Region1Width {}

    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x521]
    pub enum Region1Address {}
// ------------------------------------
    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x522]
    pub enum Region2Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x522]
    pub enum Region2Width {}

    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x522]
    pub enum Region2Address {}
// --------------------------
    #[derive(CSRAccess)]
    #[width = 16]
    #[offset = 16]
    #[address = 0x523]
    pub enum Region3Address {}

    #[derive(CSRAccess)]
    #[width = 2]
    #[offset = 0]
    #[address = 0x523]
    pub enum Region3Permissions {
        Read = 1,
        Write = 2,
        ReadWrite = 3,
    }

    #[derive(CSRAccess)]
    #[width = 14]
    #[offset = 2]
    #[address = 0x523]
    pub enum Region3Width {}
}