use crate::inter::*;
#[derive(Copy,Clone,Debug)]
pub struct SectionHeader{
    sh_name:u32,
    sh_type:u32,
    sh_flags:u32,
    sh_addr:u32,
    sh_offset:u32,
    sh_size:u32,
    sh_link:u32,
    sh_info:u32,
    sh_addralign:u32,
    sh_entsize:u32,
}
impl SectionHeader{
    pub fn new(val:&[u8])->SectionHeader{
        SectionHeader{
            sh_name:u8_to_u32(&val[0x0..0x4]),
            sh_type:u8_to_u32(&val[0x4..0x8]),
            sh_flags:u8_to_u32(&val[0x8..0xC]),
            sh_addr:u8_to_u32(&val[0xC..0x10]),
            sh_offset:u8_to_u32(&val[0x10..0x14]),
            sh_size:u8_to_u32(&val[0x14..0x18]),
            sh_link:u8_to_u32(&val[0x18..0x1C]),
            sh_info:u8_to_u32(&val[0x1C..0x20]),
            sh_addralign:u8_to_u32(&val[0x20..0x24]),
            sh_entsize:u8_to_u32(&val[0x24..0x28]),
        }
    }
    pub fn get_sh_offset(&self)->u32{
        self.sh_offset
    }
    pub fn get_sh_type(&self)->u32{
        self.sh_type
    }
    pub fn get_sh_name(&self)->u32{
        self.sh_name
    }
    pub fn get_sh_size(&self)->u32{
        self.sh_size
    }
    pub fn get_sh_flags(&self)->u32{
        self.sh_flags
    }
    pub fn get_sh_addr(&self)->u32{
        self.sh_addr
    }
}