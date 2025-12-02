pub fn solve(input: &str) -> String {
    let [a_count, c_count, g_count, t_count] = input.chars().fold([0usize; 4], |mut acc, c| {
        acc[match c {
            'A' => 0,
            'C' => 1,
            'G' => 2,
            'T' => 3,
            _ => return acc,
        }] += 1;
        acc
    });
    format!("{a_count} {c_count} {g_count} {t_count}")
}
