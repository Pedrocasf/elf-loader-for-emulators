use std::fmt;
pub mod section_header;
use crate::range::Range;
#[derive(Clone)]
pub struct Section{
    section_header:section_header::SectionHeader,
    range:Option<Range>,
    pub section_name:String,
    pub section:Vec<u8>,
}
impl Section{
    pub fn new(section_header:section_header::SectionHeader,section_name:String,section:&[u8])->Section{
        let range:Option<Range>;
        if section_header.get_sh_flags() & 2 == 2{
            let beg = section_header.get_sh_addr();
            let end = section_header.get_sh_size();
            range = Some(Range(beg as usize,end as usize));
        }else{
            range = None;
        }
        Section{
            section_header:section_header,
            range:range,
            section_name:section_name,
            section:section.to_vec(),
        }
    }
    pub fn get_section(&self)->&Vec<u8>{
        &self.section
    }
    pub fn get_range(&self)->Option<(usize,usize)>{
        if let Some(range) = &self.range{
            return Some((range.0,range.1));
        }else{
            return None;
        }
    }
    pub fn get_s(&self)->Vec<u8>{
        self.section.clone()
    }
    pub fn get_s_name(&self)->String{
        self.section_name.clone()
    }
    pub fn get_sh(&self)->section_header::SectionHeader{
        self.section_header
    }
    pub fn get_sh_offset(&self)->u32{
        self.section_header.get_sh_offset()
    }
    pub fn get_sh_type(&self)->u32{
        self.section_header.get_sh_type()
    }
    pub fn get_sh_name(&self)->u32{
        self.section_header.get_sh_name()
    }
    pub fn get_sh_size(&self)->u32{
        self.section_header.get_sh_size()
    }
    pub fn read8(&self,off:usize)->u8{
        self.section[off]
    }
    pub fn write8(&mut self, addr:usize, val:u8){
        self.section[addr]=val;
    }
    pub fn contains(&self,off:usize)->Option<usize>{
        if let Some(range) = &self.range{
            return range.contains(off);
        }else{
            return None;
        }
    }
    pub fn pop_name(&mut self){
        self.section_name.pop().expect("Null Name");
    }
}
impl fmt::Debug for Section {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Section Name:{:}, {:x?}, {:x?}",self.section_name,self.range,self.section_header)
    }
}