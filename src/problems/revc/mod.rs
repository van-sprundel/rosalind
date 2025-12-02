pub fn solve(input: &str) -> String {
    fn map(c: &mut u8) {
        match c {
            b'A' => *c = b'T',
            b'T' => *c = b'A',
            b'C' => *c = b'G',
            b'G' => *c = b'C',
            _ => {}
        };
    }

    let mut input = input.to_owned();
    unsafe {
        input.as_bytes_mut().iter_mut().for_each(map);
    }

    input.chars().rev().collect()
}
