pub fn solve(input: &str) {
    let [mut a_count, mut c_count, mut g_count, mut t_count] = [0, 0, 0, 0];
    input.chars().for_each(|c| match c {
        'A' => a_count += 1,
        'C' => c_count += 1,
        'G' => g_count += 1,
        'T' => t_count += 1,
        _ => {}
    });
    println!("{a_count} {c_count} {g_count} {t_count}")
}
