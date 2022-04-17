pub fn build_proverb(list: &[&str]) -> String {
    // no need to check for 0 length of list because we'll operate on
    // iterators over `list` (list.windows and list.iter)

    // see windows() on slices
    list.windows(2)
        .map(|window| format!("For want of a {} the {} was lost.", window[0], window[1]))
        // see chain() on interators
        .chain(
            list.iter()
                // see take() on iterators
                .take(1)
                .map(|x| format!("And all for the want of a {}.", x)),
        )
        .collect::<Vec<_>>()
        // the Vec gets converted to a slice implicitly and we can call join on it
        .join("\n")
}
