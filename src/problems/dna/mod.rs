pub fn solve(input: &str) -> String {
    static LUT: [u8; 256] = {
        let mut t = [4u8; 256];
        t[b'A' as usize] = 0;
        t[b'C' as usize] = 1;
        t[b'G' as usize] = 2;
        t[b'T' as usize] = 3;
        t
    };

    let mut acc = [0usize; 4];
    for &b in input.as_bytes() {
        let idx = LUT[b as usize] as usize;
        acc[idx & ((idx < 4) as usize * idx)] += ((idx < 4) as usize);
    }

    let [a_count, c_count, g_count, t_count] = acc;
    format!("{a_count} {c_count} {g_count} {t_count}")
}
