use super::Modifier;

pub struct ShowEnd {}

impl ShowEnd {
    pub fn new() -> Self {
        Self {}
    }
}

impl Modifier for ShowEnd {
    fn modify<'a>(&self, buf: &mut Vec<u8>) -> Result<(), crate::error::RCatError> {
        let mut i: usize = 0;
        let mut len = buf.len();

        while i < len {
            match buf[i] {
                b'\n' => {
                    buf.insert(i, b'$');
                    i += 2;
                    len += 1;
                }
                b'\r' if i + 1 < len && buf[i + 1] == b'\n' => {
                    buf.insert(i, b'$');
                    i += 3;
                    len += 1;
                }
                _ => {
                    i += 1;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_succeed() {
        let _ = ShowEnd::new();
    }

    #[test_case::test_case(b"\n", b"$\n" ; "unix")]
    #[test_case::test_case(b"test\n", b"test$\n" ; "complex unix")]
    #[test_case::test_case(b"line1\nline2\nline3\nline4", b"line1$\nline2$\nline3$\nline4" ; "multiline unix")]
    #[test_case::test_case(b"\r\n", b"$\r\n" ; "windows")]
    #[test_case::test_case(b"test\r\n", b"test$\r\n" ; "complex windows")]
    #[test_case::test_case(b"line1\r\nline2\r\nline3\r\nline4", b"line1$\r\nline2$\r\nline3$\r\nline4" ; "multiline windows")]
    #[test_case::test_case(b"a", b"a" ; "no line ending")]
    #[test_case::test_case(b"\r", b"\r" ; "incomplete line ending")]
    fn modify(input: &[u8], expected: &[u8]) {
        let line_ending = ShowEnd::new();
        let mut buffer = input.to_vec();

        let result = line_ending.modify(&mut buffer);

        assert!(result.is_ok(), "[{}]", result.err().unwrap());
        assert_eq!(buffer, expected.to_vec());
    }
}
