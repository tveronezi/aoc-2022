pub(crate) fn group_max(values: &'_ str) -> impl Iterator<Item = usize> + '_ {
    values
        .split("\n\n")
        .map(|v| v.trim())
        .filter(|v| !v.is_empty())
        .map(|v| {
            v.lines()
                .map(|v| v.trim())
                .filter(|v| !v.is_empty())
                .filter_map(|v| v.parse::<usize>().ok())
                .sum()
        })
}
