extern crate inter;

use std::any::Any;
use std::str;
use inter::Interconnect;

mod file_header;
mod program_header;
pub mod range;
pub mod section;

#[derive(Clone, Debug)]
pub struct Elf{
    file_header:file_header::FileHeader,
    program_header:program_header::ProgramHeader,
    pub sections:Vec<section::Section>,
}
impl Elf{
    pub fn new(file:&[u8])->Elf{
        let fh = file_header::FileHeader::new(&file);
        let sh_off = fh.get_e_shoff() as usize;
        let sh_num = fh.get_e_shnum();
        let sh_entsize = fh.get_e_shentsize();
        let mut shs:Vec<section::section_header::SectionHeader> = Vec::new();
        for i in 0..sh_num{
            let beg = sh_off+(i * sh_entsize) as usize;
            let end = beg + sh_entsize as usize;
            let sh = section::section_header::SectionHeader::new(&file[beg..end]);
            shs.push(sh); 
        }
        let mut name_table:(Option<section::section_header::SectionHeader>,Vec<u8>) = (None,Vec::new()); 
        for i in 0..sh_num{
            let current_sh = shs[i as usize];
            if current_sh.get_sh_type() == 3{
                let file_off = current_sh.get_sh_offset();
                let name_off = current_sh.get_sh_name();
                let toff = (file_off+name_off) as usize;
                let mut name:[u8;10] = [0;10];
                name.copy_from_slice(&file[toff..toff+10]);
                if name == [0x2E,0x73,0x68,0x73,0x74,0x72,0x74,0x61,0x62,0x00]{
                    let beg = file_off as usize;
                    let end = beg + current_sh.get_sh_size() as usize;
                    let mut sect:Vec<u8> = Vec::new();
                    sect.extend_from_slice(&file[beg..end]);
                    name_table = (Some(current_sh),sect);
                    break;
                }
            }
        }
        let mut sections:Vec<section::Section> = Vec::new();
        let (sh,table) = name_table;
        if let Some(sh) = sh{
            for i in 0..sh_num as usize{
                let mut s_name = shs[i].get_sh_name() as usize;
                let mut name:String = String::new();
                while{
                    let a:char = table[s_name] as char;
                    name.push(a);
                    s_name+=1;
                    a as u8 != 0
                }{} 
                let beg = shs[i].get_sh_offset() as usize;
                let end = beg + shs[i].get_sh_size() as usize;
                let mut section_vec:Vec<u8>;
                if shs[i].get_sh_type() != 8{
                    section_vec = Vec::new();
                    section_vec.extend_from_slice(&file[beg..end]);
                }else{
                    section_vec =vec!(0u8;end-beg);
                } 
                let section = section::Section::new(shs[i],name,&section_vec);
                sections.push(section);
            }
        }else{
            panic!("Malformed elf");
        }
        Elf{
            file_header:fh,
            program_header:program_header::ProgramHeader::new(&file[0x34..0x54]),
            sections:sections,
        }
    }
    pub fn get_p_paddr(&self)->u32{
        self.program_header.get_p_paddr()
    }
    pub fn get_p_offset(&self)->u32{
        self.program_header.get_p_offset()
    }
    pub fn get_p_memsz(&self)->u32{
        self.program_header.get_p_memsz()
    }
    pub fn get_e_entry(&self)->u32{
        self.file_header.get_e_entry()
    }
    pub fn pop_names(&mut self){
        for i in self.sections.iter_mut(){
            i.pop_name();
        }
    }
    pub fn get_section_data(&mut self,name:&str)->Vec<u8>{
        let index = self.sections.iter().position(|x| x.section_name == name).expect("Section was not found");
        self.sections[index].section.clone()
    }
    pub fn get_section(&self,name:&str)->section::Section{
        let index = self.sections.iter().position(|x| x.section_name == name).expect("Section was not found");
        self.sections[index].clone()
    }
    pub fn pop_section(&mut self,name:&str)->section::Section{
        let index = self.sections.iter().position(|x| x.section_name == name).expect("Section was not found");
        self.sections.remove(index)
    }
    pub fn push_section(&mut self,s:section::Section){
        self.sections.push(s);
    }
}
impl Interconnect for Elf{
    fn load8(&self, addr:usize)->Option<u8>{
        for s in self.sections.iter(){
            if let Some(offset) = s.contains(addr){
                return Some(s.read8(offset));
            }
        }
        None
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn store8(&mut self, addr:usize, val:u8){
        for s in self.sections.iter_mut(){
            if let Some(offset) = s.contains(addr){
                s.write8(offset,val);
            }
        }
    }
}
