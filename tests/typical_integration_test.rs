use study_crate::find_median;
use study_crate::find_mode;
use study_crate::find_mode_generic;

#[test]
fn median_meets_mode_in_the_middle() {
    let input = vec![1, 2, 3, 3, 3, 4, 5];

    assert_eq!(find_median(&input), find_mode(&input).unwrap());
    assert_eq!(find_median(&input), *find_mode_generic(&input).unwrap());
}
