use anyhow::{bail, Result};

mod operator;
use operator::OperatorIter;

mod pixel;
use pixel::{process_operators, Rgb, Rgba};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum QoiChannels {
    Rgb = 3,
    Rgba = 4,
}

impl TryFrom<u8> for QoiChannels {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            3 => Ok(QoiChannels::Rgb),
            4 => Ok(QoiChannels::Rgba),
            n => bail!("invalid channel enum {}", n),
        }
    }
}

pub struct QoiHeader {
    pub width: u32,
    pub height: u32,
    pub channels: QoiChannels,
    pub colorspace: u8,
}

fn split_header(input: &[u8]) -> Result<(QoiHeader, &[u8])> {
    const HEADER_SIZE: usize = 4 + 4 + 4 + 1 + 1;
    if input.len() < HEADER_SIZE {
        bail!("input too small");
    }

    let (header_buf, body_buf) = input.split_at(HEADER_SIZE);

    let magic = &header_buf[0..4];
    if magic != b"qoif" {
        bail!("bad magic, got {:x?}, expected {:x?}", magic, b"qoif");
    }

    let width = u32::from_be_bytes(header_buf[4..8].try_into().unwrap());
    let height = u32::from_be_bytes(header_buf[8..12].try_into().unwrap());

    let channels = header_buf[12].try_into()?;
    let colorspace = header_buf[13];

    let header = QoiHeader {
        width,
        height,
        channels,
        colorspace,
    };

    Ok((header, body_buf))
}

pub fn process_qoi(input: &[u8]) -> Result<(QoiHeader, Vec<u8>)> {
    let (header, input) = split_header(input)?;

    let operators = OperatorIter::new(input);

    let output = match header.channels {
        QoiChannels::Rgb => process_operators::<_, Rgb>(operators),
        QoiChannels::Rgba => process_operators::<_, Rgba>(operators),
    };

    debug_assert_eq!(
        output.len(),
        header.channels as usize * header.width as usize * header.height as usize
    );

    Ok((header, output))
}
