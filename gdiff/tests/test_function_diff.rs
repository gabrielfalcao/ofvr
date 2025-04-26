use gdiff::*;

#[test]
fn test_binary_diff() {
    let anterior = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    let current = vec![0x00, 0x02, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00];
    assert_eq!(
        diff(&anterior, &current, AxisBoundary::Len(2)).expect("diff"),
        Diff {
            sequence: vec![
                DiffUnit {
                    x: 0,
                    y: 0,
                    anterior: vec![0],
                    current: vec![0]
                },
                DiffUnit {
                    x: 1,
                    y: 0,
                    anterior: vec![0],
                    current: vec![2]
                },
                DiffUnit {
                    x: 0,
                    y: 1,
                    anterior: vec![0],
                    current: vec![0]
                },
                DiffUnit {
                    x: 1,
                    y: 1,
                    anterior: vec![0],
                    current: vec![0]
                },
                DiffUnit {
                    x: 0,
                    y: 2,
                    anterior: vec![0],
                    current: vec![2]
                },
                DiffUnit {
                    x: 1,
                    y: 2,
                    anterior: vec![0],
                    current: vec![0]
                },
                DiffUnit {
                    x: 0,
                    y: 3,
                    anterior: vec![0],
                    current: vec![0]
                },
                DiffUnit {
                    x: 1,
                    y: 3,
                    anterior: vec![0],
                    current: vec![0]
                }
            ],
            settings: DiffSettings {
                axis_boundary: AxisBoundary::Len(2),
            }
        }
    )
}

#[test]
fn test_binary_diff_odd() {
    let anterior = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    let current = vec![0x00, 0x00, 0x02];
    assert_eq!(
        diff(&anterior, &current, AxisBoundary::Len(2)).expect("diff"),
        Diff {
            sequence: vec![
                DiffUnit {
                    x: 0,
                    y: 0,
                    anterior: vec![0],
                    current: vec![0]
                },
                DiffUnit {
                    x: 1,
                    y: 0,
                    anterior: vec![0],
                    current: vec![0]
                },
                DiffUnit {
                    x: 0,
                    y: 1,
                    anterior: vec![0],
                    current: vec![2]
                },
                DiffUnit {
                    x: 1,
                    y: 1,
                    anterior: vec![0],
                    current: vec![]
                },
                DiffUnit {
                    x: 0,
                    y: 2,
                    anterior: vec![0],
                    current: vec![]
                },
                DiffUnit {
                    x: 1,
                    y: 2,
                    anterior: vec![0],
                    current: vec![]
                }
            ],
            settings: DiffSettings {
                axis_boundary: AxisBoundary::Len(2),
            },
        }
    )
}
