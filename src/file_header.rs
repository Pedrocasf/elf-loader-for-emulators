use crate::inter::*;
#[derive(Clone, Debug)]
pub struct FileHeader {
    elf_ident:ElfIdent,
    e_type:u16,
    e_machine:u16,
    e_version:u32,
    e_entry:u32,
    e_phoff:u32,
    e_shoff:u32,
    e_flags:u32,
    e_ehsize:u16,
    e_phentsize:u16,
    e_phnum:u16,
    e_shentsize:u16,
    e_shnum:u16,
    e_shstrndx:u16,
}
impl FileHeader{
    pub fn new(file:&[u8])->FileHeader{
        FileHeader{
            elf_ident: ElfIdent::new(&file[0..9]),
            e_type: u8_to_u16(&file[0x10..0x12]),
            e_machine: u8_to_u16(&file[0x12..0x14]),
            e_version: u8_to_u32(&file[0x14..0x18]),
            e_entry: u8_to_u32(&file[0x18..0x1C]),
            e_phoff: u8_to_u32(&file[0x1C..0x20]),
            e_shoff:u8_to_u32(&file[0x20..0x24]),
            e_flags:u8_to_u32(&file[0x24..0x28]),
            e_ehsize:u8_to_u16(&file[0x28..0x2A]),
            e_phentsize:u8_to_u16(&file[0x2A..0x2C]),
            e_phnum:u8_to_u16(&file[0x2C..0x2E]),
            e_shentsize:u8_to_u16(&file[0x2E..0x30]),
            e_shnum:u8_to_u16(&file[0x30..0x32]),
            e_shstrndx:u8_to_u16(&file[0x32..0x34]),
        }
    }
    pub fn get_e_shoff(&self)->u32{
        self.e_shoff
    }
    pub fn get_e_shentsize(&self)->u16{
        self.e_shentsize
    }
    pub fn get_e_shnum(&self)->u16{
        self.e_shnum
    }
    pub fn get_e_entry(&self)->u32{
        self.e_entry
    }
}
#[derive(Clone, Debug)]
struct ElfIdent{
    ei_mag:u32,
    ei_class:u8,
    ei_data:u8,
    ei_version:u8,
    ei_osabi:u8,
    ei_abiversion:u8,
}
impl ElfIdent{
    pub fn new(ei:&[u8])->ElfIdent{
        ElfIdent{
            ei_mag:u8_to_u32(&ei[0..4]),
            ei_class:ei[4],
            ei_data:ei[5],
            ei_version:ei[6],
            ei_osabi:ei[7],
            ei_abiversion:ei[8],
        }
    }
}