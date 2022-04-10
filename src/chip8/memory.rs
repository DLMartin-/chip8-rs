const FOUR_KB: usize = 1024 * 4;

pub struct Memory {
    data: [u8; FOUR_KB],
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            data: [0xBA; FOUR_KB],
        }
    }

    pub fn print(&self) {
        for i in self.data {
            println!("{}", i);
        }
    }

    pub fn print_data_at(&self, location: u16) {
        let data = self.data[location as usize];
        println!("Data at location {}: {}", location, data);
    }
}

impl<Idx> std::ops::Index<Idx> for Memory
where
    Idx: std::slice::SliceIndex<[u8]>,
{
    type Output = Idx::Output;
    fn index(&self, index: Idx) -> &Self::Output {
        &self.data[index]
    }
}

pub fn write_into(data: &[u8], dest: &mut Memory, location: u16) {
    let begin = location as usize;
    let end = begin + data.len();
    dest.data[begin..end].copy_from_slice(data);
}

pub fn read_from(source: &Memory, location: u16) -> u8 {
    source.data[location as usize]
}
