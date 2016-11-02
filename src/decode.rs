const ALPHA: &'static [u8]
        = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

pub trait FromBase58 {
    fn from_base58(&self) -> Result<Vec<u8>, String>;
}

impl FromBase58 for str {
    fn from_base58(&self) -> Result<Vec<u8>, String> {
        let mut bytes = Vec::with_capacity(self.len() / 8 * 6);

        for c in self.bytes() {
            // Relies on the fact that the alphabet is sorted ASCII
            if let Ok(mut val) = ALPHA.binary_search(&c) {
                for byte in &mut bytes {
                    val += (*byte as usize) * 58;
                    *byte = (val & 0xFF) as u8;
                    val >>= 8;
                }

                while val > 0 {
                    bytes.push((val & 0xff) as u8);
                    val >>= 8
                }
            } else {
                return Err(format!("unexpected utf8 byte '{}'", c));
            }
        }

        for c in self.bytes() {
            if c == ALPHA[0] {
                bytes.push(0);
            } else {
                break;
            }
        }

        bytes.reverse();
        Ok(bytes)
    }
}

// Subset of test cases from https://github.com/cryptocoinjs/base-x/blob/master/test/fixtures.json
#[cfg(test)]
mod tests {
    use FromBase58;

    #[test]
    fn tests() {
        let tests = [
            (vec![], ""),
            (vec![0x61], "2g"),
            (vec![0x62, 0x62, 0x62], "a3gV"),
            (vec![0x63, 0x63, 0x63], "aPEr"),
            (vec![0x57, 0x2e, 0x47, 0x94], "3EFU7m"),
            (vec![0x10, 0xc8, 0x51, 0x1e], "Rt5zm"),
            (vec![0x51, 0x6b, 0x6f, 0xcd, 0x0f], "ABnLTmg"),
            (vec![0xbf, 0x4f, 0x89, 0x00, 0x1e, 0x67, 0x02, 0x74, 0xdd], "3SEo3LWLoPntC"),
            (vec![0xec, 0xac, 0x89, 0xca, 0xd9, 0x39, 0x23, 0xc0, 0x23, 0x21], "EJDM8drfXA6uyA"),
            (vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], "1111111111"),
            (vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff], "FPBt6CHo3fovdL"),
            (vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff], "NKioeUVktgzXLJ1B3t"),
            (vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff], "YcVfxkQb6JRzqk5kF2tNLv"),
            (vec![0x73, 0x69, 0x6d, 0x70, 0x6c, 0x79, 0x20, 0x61, 0x20, 0x6c, 0x6f, 0x6e, 0x67, 0x20, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67], "2cFupjhnEsSn59qHXstmK2ffpLv2"),
            (vec![0x00, 0xeb, 0x15, 0x23, 0x1d, 0xfc, 0xeb, 0x60, 0x92, 0x58, 0x86, 0xb6, 0x7d, 0x06, 0x52, 0x99, 0x92, 0x59, 0x15, 0xae, 0xb1, 0x72, 0xc0, 0x66, 0x47], "1NS17iag9jJgTHD1VXjvLCEnZuQ3rJDE9L"),
            (vec![0x00, 0x3c, 0x17, 0x6e, 0x65, 0x9b, 0xea, 0x0f, 0x29, 0xa3, 0xe9, 0xbf, 0x78, 0x80, 0xc1, 0x12, 0xb1, 0xb3, 0x1b, 0x4d, 0xc8, 0x26, 0x26, 0x81, 0x87], "16UjcYNBG9GTK4uq2f7yYEbuifqCzoLMGS"),
            (vec![0x80, 0x11, 0x84, 0xcd, 0x2c, 0xdd, 0x64, 0x0c, 0xa4, 0x2c, 0xfc, 0x3a, 0x09, 0x1c, 0x51, 0xd5, 0x49, 0xb2, 0xf0, 0x16, 0xd4, 0x54, 0xb2, 0x77, 0x40, 0x19, 0xc2, 0xb2, 0xd2, 0xe0, 0x85, 0x29, 0xfd, 0x20, 0x6e, 0xc9, 0x7e], "5Hx15HFGyep2CfPxsJKe2fXJsCVn5DEiyoeGGF6JZjGbTRnqfiD"),
        ];

        for &(ref val, s) in tests.iter() {
            assert_eq!(val, &(s.from_base58().unwrap()));
        }
    }
}