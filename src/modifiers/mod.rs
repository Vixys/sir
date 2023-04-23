use crate::error::RCatError;

pub mod show_end;

pub trait Modifier {
    fn modify<'a>(&self, buf: &mut Vec<u8>) -> Result<(), RCatError>;
}

pub struct ModifierCollection {
    modifiers: Vec<Box<dyn Modifier>>,
}

impl ModifierCollection {
    pub fn new() -> Self {
        Self {
            modifiers: Vec::new(),
        }
    }

    pub fn add(&mut self, modifier: Box<dyn Modifier>) {
        self.modifiers.push(modifier);
    }
}

impl std::io::Write for ModifierCollection {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut to_write = Vec::from(buf);

        for modifier in &self.modifiers {
            modifier
                .modify(&mut to_write)
                .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;
        }

        std::io::stdout().write(&to_write)?;

        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
