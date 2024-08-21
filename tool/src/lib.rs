pub mod crc;
pub mod reader;

pub fn bcd_encode(input: &str, size: usize) -> Vec<u8> {
    let digits = input
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as u8)
        .collect::<Vec<u8>>();

    let mut bcd_bytes: Vec<u8> = Vec::with_capacity((digits.len() + 1) / 2);

    for chunk in digits.chunks(2) {
        let high = chunk[0] << 4;
        let low = if chunk.len() > 1 { chunk[1] } else { 0 };
        bcd_bytes.push(high | low);
    }

    if bcd_bytes.len() < size {
        let mut padded_bcd = vec![0; size];
        padded_bcd[0..bcd_bytes.len()].copy_from_slice(&bcd_bytes);
        padded_bcd
    } else {
        bcd_bytes.truncate(size);
        bcd_bytes
    }
}

pub fn bcd_decode(input: &[u8]) -> String {
    let index = match input.iter().rposition(|&b| b != 0) {
        Some(index) => index,
        None => return "".to_string(),
    };
    input[..index + 1]
        .iter()
        .map(|&b| format!("{:02x}", b))
        .collect()
}
