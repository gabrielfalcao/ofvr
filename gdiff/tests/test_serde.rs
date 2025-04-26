#![allow(confusable_idents, mixed_script_confusables)]

use gdiff::{AxisBoundary, Diff, Result};

#[test]
fn test_serialize_and_deserialize_diff() -> Result<()> {
    let a = vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
    let b = vec![0xFF, 0x88, 0xFF, 0xFF, 0x88, 0xFF, 0xFF, 0xFF];
    let diff = gdiff::diff(&a, &b, AxisBoundary::Len(2))?;

    let bytes = diff.to_flate_bytes()?;
    assert_eq!(bytes.len(), 48);
    assert_eq!(
        bytes,
        vec![
            133, 142, 49, 18, 0, 48, 4, 4, 35, 154, 60, 61, 191, 166, 65, 129, 67, 195, 140, 157,
            189, 123, 167, 31, 178, 45, 229, 128, 196, 207, 31, 236, 192, 132, 59, 46, 76, 161,
            141, 8, 7, 175, 61, 120, 116, 120, 130, 2
        ]
    );
    let des_bytes = Diff::from_deflate_bytes(&bytes)?;
    assert_eq!(diff, des_bytes);
    assert_eq!(bytes, des_bytes.to_flate_bytes()?);
    Ok(())
}
