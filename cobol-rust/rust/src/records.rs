//! The six record layouts from the `FILE SECTION`, byte for byte.
//!
//! The data files are GnuCOBOL `ORGANIZATION IS SEQUENTIAL`: fixed-length
//! records back to back, no separators. Each struct below decodes and encodes
//! one record exactly, so files written by the COBOL program and by this port
//! can be swapped freely.

use crate::pic::Pic;

/// `PIC X(n)`: text padded with spaces, kept as raw bytes.
pub type Text<const N: usize> = [u8; N];

/// `MOVE` into a `PIC X(n)`: copy from the left, cut or pad with spaces.
pub fn text<const N: usize>(src: &[u8]) -> Text<N> {
    let mut out = [b' '; N];
    let n = src.len().min(N);
    out[..n].copy_from_slice(&src[..n]);
    out
}

pub const SPACES: u8 = b' ';

pub trait Record: Sized {
    const FILE: &'static str;
    const LEN: usize;
    fn decode(bytes: &[u8]) -> Self;
    fn encode(&self) -> Vec<u8>;
}

struct Reader<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Reader { bytes, pos: 0 }
    }
    fn x<const N: usize>(&mut self) -> Text<N> {
        let out = text(&self.bytes[self.pos..self.pos + N]);
        self.pos += N;
        out
    }
    fn n(&mut self, pic: Pic) -> u64 {
        let v = pic.decode(&self.bytes[self.pos..self.pos + pic.size()]);
        self.pos += pic.size();
        v
    }
}

#[derive(Default)]
struct Writer(Vec<u8>);

impl Writer {
    fn x(&mut self, field: &[u8]) -> &mut Self {
        self.0.extend_from_slice(field);
        self
    }
    fn n(&mut self, pic: Pic, value: u64) -> &mut Self {
        pic.encode(value, &mut self.0);
        self
    }
}

// Numeric layouts shared by records and working storage.
pub const ID_CLIENTE: Pic = Pic::new(8, 0);
pub const ID_CUENTA: Pic = Pic::new(10, 0);
pub const ID_TARJETA: Pic = Pic::new(10, 0);
pub const ID_MOVIMIENTO: Pic = Pic::new(12, 0);
pub const ID_EMPLEADO: Pic = Pic::new(8, 0);
pub const SALDO: Pic = Pic::new(12, 2);
pub const LIMITE: Pic = Pic::new(10, 2);
pub const PORCENTAJE: Pic = Pic::new(3, 3);

/// `REGISTRO-CLIENTE`, 378 bytes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cliente {
    pub id: u64,
    pub nombre: Text<50>,
    pub apellido: Text<50>,
    pub dni: Text<9>,
    pub email: Text<50>,
    pub telefono: Text<15>,
    pub direccion: Text<100>,
    pub ciudad: Text<30>,
    pub cp: Text<5>,
    pub fecha_reg: Text<10>,
    pub estado: Text<1>,
    pub relleno: Text<50>,
}

impl Default for Cliente {
    fn default() -> Self {
        Cliente::decode(&[SPACES; Cliente::LEN])
    }
}

impl Record for Cliente {
    const FILE: &'static str = "clientes.dat";
    const LEN: usize = 378;
    fn decode(b: &[u8]) -> Self {
        let mut r = Reader::new(b);
        Cliente {
            id: r.n(ID_CLIENTE),
            nombre: r.x(),
            apellido: r.x(),
            dni: r.x(),
            email: r.x(),
            telefono: r.x(),
            direccion: r.x(),
            ciudad: r.x(),
            cp: r.x(),
            fecha_reg: r.x(),
            estado: r.x(),
            relleno: r.x(),
        }
    }
    fn encode(&self) -> Vec<u8> {
        let mut w = Writer::default();
        w.n(ID_CLIENTE, self.id)
            .x(&self.nombre)
            .x(&self.apellido)
            .x(&self.dni)
            .x(&self.email)
            .x(&self.telefono)
            .x(&self.direccion)
            .x(&self.ciudad)
            .x(&self.cp)
            .x(&self.fecha_reg)
            .x(&self.estado)
            .x(&self.relleno);
        w.0
    }
}

/// `REGISTRO-CUENTA`, 177 bytes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cuenta {
    pub id: u64,
    pub cliente_id: u64,
    pub iban: Text<24>,
    pub saldo: u64,
    pub tipo: Text<20>,
    pub fecha_apertura: Text<10>,
    pub fecha_cierre: Text<10>,
    pub estado: Text<1>,
    pub limite_diario: u64,
    pub limite_mensual: u64,
    pub comision: u64,
    pub relleno: Text<50>,
}

impl Default for Cuenta {
    fn default() -> Self {
        Cuenta::decode(&[SPACES; Cuenta::LEN])
    }
}

impl Record for Cuenta {
    const FILE: &'static str = "cuentas.dat";
    const LEN: usize = 177;
    fn decode(b: &[u8]) -> Self {
        let mut r = Reader::new(b);
        Cuenta {
            id: r.n(ID_CUENTA),
            cliente_id: r.n(ID_CLIENTE),
            iban: r.x(),
            saldo: r.n(SALDO),
            tipo: r.x(),
            fecha_apertura: r.x(),
            fecha_cierre: r.x(),
            estado: r.x(),
            limite_diario: r.n(LIMITE),
            limite_mensual: r.n(LIMITE),
            comision: r.n(PORCENTAJE),
            relleno: r.x(),
        }
    }
    fn encode(&self) -> Vec<u8> {
        let mut w = Writer::default();
        w.n(ID_CUENTA, self.id)
            .n(ID_CLIENTE, self.cliente_id)
            .x(&self.iban)
            .n(SALDO, self.saldo)
            .x(&self.tipo)
            .x(&self.fecha_apertura)
            .x(&self.fecha_cierre)
            .x(&self.estado)
            .n(LIMITE, self.limite_diario)
            .n(LIMITE, self.limite_mensual)
            .n(PORCENTAJE, self.comision)
            .x(&self.relleno);
        w.0
    }
}

/// `REGISTRO-TARJETA`, 226 bytes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tarjeta {
    pub id: u64,
    pub cuenta_id: u64,
    pub numero: Text<16>,
    pub cvv: Text<3>,
    pub pin: Text<4>,
    pub fecha_emision: Text<10>,
    pub fecha_caducidad: Text<10>,
    pub titular: Text<100>,
    pub estado: Text<1>,
    pub limite: u64,
    pub relleno: Text<50>,
}

impl Default for Tarjeta {
    fn default() -> Self {
        Tarjeta::decode(&[SPACES; Tarjeta::LEN])
    }
}

impl Record for Tarjeta {
    const FILE: &'static str = "tarjetas.dat";
    const LEN: usize = 226;
    fn decode(b: &[u8]) -> Self {
        let mut r = Reader::new(b);
        Tarjeta {
            id: r.n(ID_TARJETA),
            cuenta_id: r.n(ID_CUENTA),
            numero: r.x(),
            cvv: r.x(),
            pin: r.x(),
            fecha_emision: r.x(),
            fecha_caducidad: r.x(),
            titular: r.x(),
            estado: r.x(),
            limite: r.n(LIMITE),
            relleno: r.x(),
        }
    }
    fn encode(&self) -> Vec<u8> {
        let mut w = Writer::default();
        w.n(ID_TARJETA, self.id)
            .n(ID_CUENTA, self.cuenta_id)
            .x(&self.numero)
            .x(&self.cvv)
            .x(&self.pin)
            .x(&self.fecha_emision)
            .x(&self.fecha_caducidad)
            .x(&self.titular)
            .x(&self.estado)
            .n(LIMITE, self.limite)
            .x(&self.relleno);
        w.0
    }
}

/// `REGISTRO-MOVIMIENTO`, 284 bytes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Movimiento {
    pub id: u64,
    pub cliente_id: u64,
    pub cuenta_id: u64,
    pub fecha: Text<10>,
    pub hora: Text<8>,
    pub tipo: Text<20>,
    pub importe: u64,
    pub saldo_anterior: u64,
    pub saldo_posterior: u64,
    pub cuenta_destino: Text<24>,
    pub descripcion: Text<100>,
    pub relleno: Text<50>,
}

impl Default for Movimiento {
    fn default() -> Self {
        Movimiento::decode(&[SPACES; Movimiento::LEN])
    }
}

impl Record for Movimiento {
    const FILE: &'static str = "movimientos.dat";
    const LEN: usize = 284;
    fn decode(b: &[u8]) -> Self {
        let mut r = Reader::new(b);
        Movimiento {
            id: r.n(ID_MOVIMIENTO),
            cliente_id: r.n(ID_CLIENTE),
            cuenta_id: r.n(ID_CUENTA),
            fecha: r.x(),
            hora: r.x(),
            tipo: r.x(),
            importe: r.n(SALDO),
            saldo_anterior: r.n(SALDO),
            saldo_posterior: r.n(SALDO),
            cuenta_destino: r.x(),
            descripcion: r.x(),
            relleno: r.x(),
        }
    }
    fn encode(&self) -> Vec<u8> {
        let mut w = Writer::default();
        w.n(ID_MOVIMIENTO, self.id)
            .n(ID_CLIENTE, self.cliente_id)
            .n(ID_CUENTA, self.cuenta_id)
            .x(&self.fecha)
            .x(&self.hora)
            .x(&self.tipo)
            .n(SALDO, self.importe)
            .n(SALDO, self.saldo_anterior)
            .n(SALDO, self.saldo_posterior)
            .x(&self.cuenta_destino)
            .x(&self.descripcion)
            .x(&self.relleno);
        w.0
    }
}

/// `REGISTRO-CONFIG`, 202 bytes. The program only ever reads the first one.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Config {
    pub version: Text<10>,
    pub comision_defecto: u64,
    pub limite_diario_defecto: u64,
    pub limite_mensual_defecto: u64,
    pub interes_ahorros: u64,
    pub interes_corriente: u64,
    pub relleno: Text<150>,
}

impl Record for Config {
    const FILE: &'static str = "config.dat";
    const LEN: usize = 202;
    fn decode(b: &[u8]) -> Self {
        let mut r = Reader::new(b);
        Config {
            version: r.x(),
            comision_defecto: r.n(PORCENTAJE),
            limite_diario_defecto: r.n(LIMITE),
            limite_mensual_defecto: r.n(LIMITE),
            interes_ahorros: r.n(PORCENTAJE),
            interes_corriente: r.n(PORCENTAJE),
            relleno: r.x(),
        }
    }
    fn encode(&self) -> Vec<u8> {
        let mut w = Writer::default();
        w.x(&self.version)
            .n(PORCENTAJE, self.comision_defecto)
            .n(LIMITE, self.limite_diario_defecto)
            .n(LIMITE, self.limite_mensual_defecto)
            .n(PORCENTAJE, self.interes_ahorros)
            .n(PORCENTAJE, self.interes_corriente)
            .x(&self.relleno);
        w.0
    }
}

/// `REGISTRO-EMPLEADO`, 309 bytes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Empleado {
    pub id: u64,
    pub usuario: Text<30>,
    pub contrasena: Text<30>,
    pub nombre: Text<50>,
    pub cargo: Text<30>,
    pub fecha_inicio: Text<10>,
    pub estado: Text<1>,
    pub relleno: Text<150>,
}

impl Default for Empleado {
    fn default() -> Self {
        Empleado::decode(&[SPACES; Empleado::LEN])
    }
}

impl Record for Empleado {
    const FILE: &'static str = "empleados.dat";
    const LEN: usize = 309;
    fn decode(b: &[u8]) -> Self {
        let mut r = Reader::new(b);
        Empleado {
            id: r.n(ID_EMPLEADO),
            usuario: r.x(),
            contrasena: r.x(),
            nombre: r.x(),
            cargo: r.x(),
            fecha_inicio: r.x(),
            estado: r.x(),
            relleno: r.x(),
        }
    }
    fn encode(&self) -> Vec<u8> {
        let mut w = Writer::default();
        w.n(ID_EMPLEADO, self.id)
            .x(&self.usuario)
            .x(&self.contrasena)
            .x(&self.nombre)
            .x(&self.cargo)
            .x(&self.fecha_inicio)
            .x(&self.estado)
            .x(&self.relleno);
        w.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn round_trip<R: Record>() {
        // A record with a different byte in every position, digits where the
        // layout wants digits.
        let blank = R::decode(&vec![b'7'; R::LEN]).encode();
        assert_eq!(
            blank.len(),
            R::LEN,
            "{} encodes to the wrong length",
            R::FILE
        );
        assert_eq!(
            R::decode(&blank).encode(),
            blank,
            "{} does not round-trip",
            R::FILE
        );
    }

    #[test]
    fn every_layout_has_the_copybook_length_and_round_trips() {
        round_trip::<Cliente>();
        round_trip::<Cuenta>();
        round_trip::<Tarjeta>();
        round_trip::<Movimiento>();
        round_trip::<Config>();
        round_trip::<Empleado>();
    }

    #[test]
    fn text_pads_and_cuts_like_move() {
        assert_eq!(text::<4>(b"ab"), *b"ab  ");
        assert_eq!(text::<2>(b"abc"), *b"ab");
    }
}
