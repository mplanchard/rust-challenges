
fn is_increasing(seq: &[i32]) -> bool {
    seq.iter().fold(
        (0, true),
        |acc, i| {
            if acc.1 == false {acc} else {(*i, *i > acc.0)}
        }
    ).1
}


fn increasing_blocks(seq: &[i32]) -> Vec<&[i32]> {
    let mut indices = Vec::new();
    let mut start = 0;
    let mut prev = &seq[0];
    for (i, val) in seq.iter().enumerate().skip(1) {
        if val <= prev {
            indices.push((start, i));
            start = i;
        }
        prev = val;
    }

    if start == 0 { return vec![seq] };

    indices.push((start, seq.len()));
    indices.iter().map(
        |(start, end)| &seq[*start..*end]
    ).collect()
}


fn can_be_increasing(seq: &[&[i32]]) -> bool {
    if seq.len() == 2 {
        if seq[0].iter().rev().nth(1) < seq[1].iter().nth(1) {
            return true
        }
        else if seq[0].last() < seq[0].iter().nth(0) {
            return true
        }
    }
    false
}


fn almost_increasing(sequence: Vec<i32>) -> bool {
    let blocks = increasing_blocks(&sequence);
    match blocks.len() {
        0 => false,
        1 => true,
        i if i > 3 => false,
        _ => can_be_increasing(&blocks),
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_increasing_blocks_1() {
        let seq = vec![1, 2, 3];
        let blocks = increasing_blocks(&seq);
        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0], [1, 2, 3]);
    }

    #[test]
    fn test_increasing_blocks_2() {
        let seq = vec![1, 2, 3, 2, 3, 4];
        let blocks = increasing_blocks(&seq);
        assert_eq!(blocks.len(), 2);
        assert_eq!(blocks[0], [1, 2, 3]);
        assert_eq!(blocks[1], [2, 3, 4]);
    }

    #[test]
    fn test_increasing_blocks_3() {
        let seq = vec![1, 2, 3, 2, 2, 3, 4];
        let blocks = increasing_blocks(&seq);
        assert_eq!(blocks.len(), 3);
        assert_eq!(blocks[0], [1, 2, 3]);
        assert_eq!(blocks[1], [2]);
        assert_eq!(blocks[2], [2, 3, 4]);
    }

    #[test]
    fn directly_increasing() {
        assert!(almost_increasing(vec![1, 2, 3]))
    }

    #[test]
    fn not_increasing() {
        assert!(! almost_increasing(vec![3, 2, 1]))
    }

    #[test]
    fn almost_increasing_1() {
        assert!(almost_increasing(vec![1, 2, 2, 3]))
    }

}
