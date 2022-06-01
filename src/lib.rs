pub fn add_them(x: &u32, y: &u32) -> u64 {
    (*x as u64) + (*y as u64)
}

pub fn add_them2(x: u32, y: u32) -> u64 {
    (x as u64) + (y as u64)
}

pub fn add_them3(x: u32, y: &u32) -> u64 {
    (x as u64) + (*y as u64)
}
