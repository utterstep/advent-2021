pub fn minmax<I, T>(iter: I) -> Option<(I::Item, I::Item)>
where
    I: Iterator<Item = T>,
    T: Ord + Copy,
{
    iter.fold(None, |mut minmax, item| {
        minmax.replace(match minmax {
            None => (item, item),
            Some((min, max)) => (min.min(item), max.max(item)),
        });

        minmax
    })
}
