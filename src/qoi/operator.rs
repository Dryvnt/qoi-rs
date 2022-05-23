#[derive(Debug)]
pub enum Operator {
    // QOI_OP_INDEX
    Index(u8),
    // QOI_OP_DIFF and QOI_OP_LUMA
    Diff {
        diff_red: u8,
        diff_green: u8,
        diff_blue: u8,
    },
    // QOI_OP_RUN
    Run(u8),
    // QOI_OP_RGB
    Rgb(u8, u8, u8),
    // QOI_OP_RGBA
    Rgba(u8, u8, u8, u8),
}

pub struct OperatorIter<'a> {
    input: &'a [u8],
}

impl<'a> OperatorIter<'a> {
    pub fn new(input: &'a [u8]) -> Self {
        Self { input }
    }

    fn take<const N: usize>(&mut self) -> Option<[u8; N]> {
        if self.input.len() < N {
            return None;
        }

        let out: [u8; N] = self.input[..N].try_into().unwrap();
        self.input = &self.input[N..];
        Some(out)
    }
}

impl<'a> Iterator for OperatorIter<'a> {
    type Item = Operator;

    fn next(&mut self) -> Option<Self::Item> {
        // End of stream
        if let [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, ..] = self.input {
            return None;
        }

        let [op] = self.take()?;

        match (op & 0b1100_0000) >> 6 {
            // QOI_OP_INDEX
            0b00 => Some(Operator::Index(op & 0b0011_1111)),
            // QOI_OP_DIFF
            0b01 => {
                let r = (op & 0b0011_0000) >> 4;
                let g = (op & 0b0000_1100) >> 2;
                let b = op & 0b0000_0011;

                let diff_red = r.wrapping_sub(2);
                let diff_green = g.wrapping_sub(2);
                let diff_blue = b.wrapping_sub(2);

                Some(Operator::Diff {
                    diff_red,
                    diff_green,
                    diff_blue,
                })
            }
            // QOI_OP_LUMA
            0b10 => {
                let g = op & 0b0011_1111;

                let [next] = self.take()?;
                let r = (next & 0b1111_0000) >> 4;
                let b = next & 0b0000_1111;

                let diff_green = g.wrapping_sub(32);
                let diff_red = r.wrapping_add(diff_green).wrapping_sub(8);
                let diff_blue = b.wrapping_add(diff_green).wrapping_sub(8);

                Some(Operator::Diff {
                    diff_red,
                    diff_green,
                    diff_blue,
                })
            }
            _ => match op & 0b0011_1111 {
                // QOI_OP_RGB
                0b0011_1110 => {
                    let [r, g, b] = self.take()?;
                    Some(Operator::Rgb(r, g, b))
                }
                // QOI_OP_RGBA
                0b0011_1111 => {
                    let [r, g, b, a] = self.take()?;
                    Some(Operator::Rgba(r, g, b, a))
                }
                // QOI_OP_RUN
                l => Some(Operator::Run(l.wrapping_add(1))),
            },
        }
    }
}
