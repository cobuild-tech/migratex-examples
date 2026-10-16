//! Sequential files, opened the way the COBOL program opens them.
//!
//! Every paragraph in the original opens a file, scans it from the start and
//! closes it again, so there is no state to keep between calls. `OPEN EXTEND`
//! then `WRITE` is [`Files::append`], `OPEN OUTPUT` is [`Files::replace`], and
//! the read-until-match-then-`REWRITE` loops are [`Files::rewrite_first`].

use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::PathBuf;

use crate::records::{Cliente, Config, Cuenta, Empleado, Movimiento, Record, Tarjeta};

pub struct Files {
    dir: PathBuf,
}

impl Files {
    pub fn new(dir: impl Into<PathBuf>) -> Files {
        Files { dir: dir.into() }
    }

    /// `VERIFICAR-CREAR-ARCHIVOS`. The original opens each file `EXTEND` and
    /// stops with status 35 when one is missing; this creates it instead.
    pub fn create_missing(&self) -> io::Result<()> {
        for name in [
            Cliente::FILE,
            Cuenta::FILE,
            Tarjeta::FILE,
            Movimiento::FILE,
            Config::FILE,
            Empleado::FILE,
        ] {
            OpenOptions::new()
                .create(true)
                .append(true)
                .open(self.dir.join(name))?;
        }
        Ok(())
    }

    /// Every complete record in the file, in order.
    pub fn read<R: Record>(&self) -> io::Result<Vec<R>> {
        let bytes = fs::read(self.dir.join(R::FILE))?;
        Ok(bytes.chunks_exact(R::LEN).map(R::decode).collect())
    }

    pub fn append<R: Record>(&self, record: &R) -> io::Result<()> {
        let mut f = OpenOptions::new()
            .append(true)
            .create(true)
            .open(self.dir.join(R::FILE))?;
        f.write_all(&record.encode())
    }

    pub fn replace<R: Record>(&self, records: &[R]) -> io::Result<()> {
        let bytes: Vec<u8> = records.iter().flat_map(Record::encode).collect();
        fs::write(self.dir.join(R::FILE), bytes)
    }

    /// Changes the first record that matches and writes it back in place.
    /// Returns whether one matched.
    pub fn rewrite_first<R: Record>(
        &self,
        matches: impl Fn(&R) -> bool,
        change: impl FnOnce(&mut R),
    ) -> io::Result<bool> {
        let mut records = self.read::<R>()?;
        match records.iter_mut().find(|r| matches(r)) {
            Some(r) => {
                change(r);
                self.replace(&records)?;
                Ok(true)
            }
            None => Ok(false),
        }
    }
}
