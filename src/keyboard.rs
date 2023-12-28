pub mod keyboard {
    pub fn get() -> u8{
        #![feature(asm)]
        use std::arch::asm;
        let mut data: u8;
        let mut status: u8;
    
        unsafe {
            asm!(
                "mov dx, 0x64",
                "mov al, 0xFE",
                "out dx, al",
                "in al, dx",
                "test al, 1",
                "mov dx, 0x60",
                "in al, dx",
                "mov [data], al",
                in("dx") 0x60,
                out("al") data,
            );
        }

        data
    }
}