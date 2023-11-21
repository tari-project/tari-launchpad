// Copyright 2023. The Tari Project
// SPDX-License-Identifier: BSD-3-Clause

use std::{fmt, fmt::Display};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
#[serde(transparent)]
pub struct TariFormat {
    value: u64,
}

impl TariFormat {
    pub fn as_u64(&self) -> u64 {
        self.value
    }
}

impl From<u64> for TariFormat {
    fn from(value: u64) -> Self {
        Self { value }
    }
}

impl Display for TariFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut value = self.value;
        let unit = if value < 1_000_000 { "μT" } else { "T" };
        let mut decimals = None;
        if value >= 1_000_000 {
            decimals = Some((value % 1_000_000).to_string());
            value /= 1_000_000;
        }
        let val_str = value
            .to_string()
            .as_bytes()
            .rchunks(3)
            .rev()
            .map(std::str::from_utf8)
            .collect::<Result<Vec<&str>, _>>()
            .unwrap()
            .join(",");

        let dec_str = match decimals {
            Some(dec) => format!(".{:0<03.3}", dec),
            None => String::new(),
        };
        write!(f, "{val_str}{dec_str} {unit}")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn format_whole_number() {
        let value = 1_234_567_891_000_000;
        let tari = TariFormat::from(value);
        assert_eq!(tari.to_string(), "1,234,567,891.000 T");
    }

    #[test]
    fn format_small_number() {
        let value = 123_456;
        let tari = TariFormat::from(value);
        assert_eq!(tari.to_string(), "123,456 μT");
    }
    #[test]
    fn format_big_number_w_frac() {
        let value = 1_234_567_890_222_333;
        let tari = TariFormat::from(value);
        assert_eq!(tari.to_string(), "1,234,567,890.222 T");
    }

    #[test]
    fn format_zero() {
        let value = 0;
        let tari = TariFormat::from(value);
        assert_eq!(tari.to_string(), "0 μT");
    }
}
