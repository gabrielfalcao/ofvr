use gdiff::{AxisBoundary, Diff};

fn quick_diff(old: &[u8], new: &[u8]) -> Diff {
    gdiff::diff(&old, &new, AxisBoundary::Len(2)).expect("diff")
}

#[test]
fn test_diff_get_current_version() {
    let old = vec![0xFF, 0x00, 0xFF, 0x00];
    let new = vec![0xFF, 0x88];
    let diff = quick_diff(&old, &new);
    assert_eq!(new, diff.current_version());
    assert_eq!(old, diff.anterior_version());
}


#[test]
fn test_diff_update() {
    let data0 = vec![0xFF, 0x00, 0xFF, 0x00];
    let data1 = vec![0xFF, 0x88];
    let data2 = vec![0xF1, 0x47, 0x41, 0x25];
    let mut diff = quick_diff(&data0, &data1);
    assert_eq!(data1, diff.current_version());
    assert_eq!(data0, diff.anterior_version());

    diff.update(&data2).expect("diff update");
    assert_eq!(data2, diff.current_version());
    assert_eq!(data1, diff.anterior_version());
}
