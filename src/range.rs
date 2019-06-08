#[derive(Clone,Debug)]
pub struct Range(pub usize,pub usize);
impl Range{
    pub fn contains(&self, addr:usize) -> Option<usize>{
        let Range(start,length) = *self;
        if addr >= start && addr < start + length {
            Some(addr-start)
        }else{
            None
        }
    }
}