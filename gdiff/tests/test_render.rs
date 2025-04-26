use gdiff::*;

fn render_diff(anterior: Vec<u8>, current: Vec<u8>) -> Result<Vec<String>> {
    Ok(diff(&anterior, &current, AxisBoundary::Len(2))?
        .render()
        .split("\n")
        .map(|line| line.to_string())
        .collect::<Vec<String>>())
}
#[test]
fn test_binary_diff() -> Result<()> {
    let rendered = render_diff(vec![0x00, 0x00], vec![0x00, 0x02])?;
    assert_eq!(
        rendered,
        vec![
            "\x1b[0;31m-0x0000000\x1b[0m 0x00 \x1b[0;31m0x00\x1b[0m", //
            "\x1b[0;32m+0x0000000\x1b[0m 0x00 \x1b[0;32m0x02\x1b[0m", //
        ]
    );
    Ok(())
}

#[test]
fn test_binary_diff_multiple_changes() -> Result<()> {
    let anterior = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    let current = vec![0x00, 0x02, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00];
    let rendered = diff(&anterior, &current, AxisBoundary::Len(2))?
        .render()
        .split("\n")
        .map(|line| line.to_string())
        .collect::<Vec<String>>();
    assert_eq!(
        rendered,
        vec![
            "\x1b[0;31m-0x0000000\x1b[0m 0x00 \x1b[0;31m0x00\x1b[0m",
            "\x1b[0;32m+0x0000000\x1b[0m 0x00 \x1b[0;32m0x02\x1b[0m",
            " 0x0000001 0x00 0x00",
            "\x1b[0;31m-0x0000002\x1b[0m \x1b[0;31m0x00\x1b[0m 0x00",
            "\x1b[0;32m+0x0000002\x1b[0m \x1b[0;32m0x02\x1b[0m 0x00",
            " 0x0000003 0x00 0x00",
        ]
    );
    Ok(())
}
