pub fn levenshtein_dist(a: &str, b: &str) -> usize {
    let m = a.len();
    let n = b.len();

    if m == 0 || n == 0 {
        return usize::max(m, n);
    }

    let mut d = vec![vec![0; n + 1]; m + 1];

    for i in 1..m + 1 {
        d[i][0] = i;
    }

    for j in 1..n + 1 {
        d[0][j] = j;
    }

    let mut cost;
    for j in 0..n {
        for i in 0..m {
            cost = match a.as_bytes()[i] == b.as_bytes()[j] {
                true => 0,
                false => 1,
            };

            d[i + 1][j + 1] = usize::min(
                usize::min(d[i][j + 1] + 1, d[i + 1][j] + 1),
                d[i][j] + cost
            );
        }
    }

    d[m][n]
}

#[cfg(test)]
mod tests {
    use crate::algorithms::levenshtein_dist;

    #[test]
    fn levenshtein_test() {
        assert_eq!(levenshtein_dist("book", ""), 4);
        assert_eq!(levenshtein_dist("", "back"), 4);
        assert_eq!(levenshtein_dist("book", "back"), 2);
        assert_eq!(levenshtein_dist("brook", "bottomless"), 8);
        assert_eq!(levenshtein_dist("abc678!pstyu", "5klhs;e5phim;seigsnsrjo4o4"), 24);
        assert_eq!(levenshtein_dist("DFfsERTETET$RT", "dfFS:DLGFSdf;ldflgfgldfgsdFRADF"), 27);
    }
}
