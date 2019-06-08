use crate::inter::*;
#[derive(Clone, Debug)]
pub struct ProgramHeader {
    p_type:u32,
    p_offset:u32,
    p_vaddr:u32,
    p_paddr:u32,
    p_filesz:u32,
    p_memsz:u32,
    p_flags:u32,
    p_align:u32,
}
impl ProgramHeader{
    pub fn new(file:&[u8])->ProgramHeader{
        ProgramHeader{
            p_type : u8_to_u32(&file[0x0..0x4]),
            p_offset : u8_to_u32(&file[0x4..0x8]),
            p_vaddr : u8_to_u32(&file[0x8..0xC]),
            p_paddr : u8_to_u32(&file[0xC..0x10]),
            p_filesz : u8_to_u32(&file[0x10..0x14]),
            p_memsz : u8_to_u32(&file[0x14..0x18]),
            p_flags : u8_to_u32(&file[0x18..0x1C]),
            p_align : u8_to_u32(&file[0x1C..0x20]),
        }
    }
    pub fn get_p_paddr(&self)->u32{
        self.p_paddr
    }
    pub fn get_p_offset(&self)->u32{
        self.p_offset
    }
    pub fn get_p_memsz(&self)->u32{
        self.p_memsz
    }
}