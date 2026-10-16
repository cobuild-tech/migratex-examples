//! `PROGRAM-ID. BANCO.`: the menus and the paragraphs behind them.
//!
//! [`Banco`] holds the program's `WORKING-STORAGE`. The COBOL paragraphs share
//! those fields freely (a lookup leaves its result in the same fields the next
//! paragraph reads), and some of the program's behaviour comes from exactly
//! that, so the port keeps the fields and the order paragraphs touch them.
//! Each paragraph is a method with the paragraph's name.
//!
//! [`Mode::Compat`] reproduces the original, bugs included, and is what the
//! golden tests run against the COBOL output. [`Mode::Fixed`], the default,
//! fixes the bugs listed in the README; every difference is an
//! `if self.fixed()` in this module and its children.

mod admin;
mod clientes;
mod cuentas;
mod tarjetas;

use std::io::{BufRead, Write};

use crate::clock;
use crate::display;
use crate::files::Files;
use crate::pic::Pic;
use crate::records::*;
use crate::term::{Step, Stop, Term};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mode {
    /// Same output and data files as the COBOL program.
    Compat,
    /// The bugs listed in the README fixed.
    Fixed,
}

/// `WS-PARAMETROS-SISTEMA`
pub struct Parametros {
    pub version: Text<10>,
    pub comision: u64,
    pub limite_diario: u64,
    pub limite_mensual: u64,
    pub interes_ahorros: u64,
    pub interes_corriente: u64,
}

impl Default for Parametros {
    fn default() -> Self {
        Parametros {
            version: text(b"1.0.0"),
            comision: 500,          // 0.5
            limite_diario: 60000,   // 600.00
            limite_mensual: 500000, // 5000.00
            interes_ahorros: 2500,  // 2.5
            interes_corriente: 500, // 0.5
        }
    }
}

/// `WS-GENERADORES`: the next id is the counter plus one.
pub struct Contadores {
    pub cliente: u64,
    pub cuenta: u64,
    pub tarjeta: u64,
    pub movimiento: u64,
    pub empleado: u64,
    pub iban: u64,
}

impl Default for Contadores {
    fn default() -> Self {
        Contadores {
            cliente: 1000,
            cuenta: 100000,
            tarjeta: 10000,
            movimiento: 1000000,
            empleado: 500,
            iban: 0,
        }
    }
}

/// `WS-VARIABLES-TRANSFERENCIA`
#[derive(Default)]
pub struct Transferencia {
    pub iban_destino: Text<24>,
    pub importe: u64,
    pub cuenta_origen_id: u64,
    pub cuenta_destino_id: u64,
    pub saldo_origen_antes: u64,
    /// Not in the original: the receiving account's owner, so the fixed mode
    /// can file "Transferencia recibida" under them.
    pub cliente_destino_id: u64,
}

pub struct Banco<R, W> {
    pub(crate) t: Term<R, W>,
    pub(crate) f: Files,
    pub(crate) mode: Mode,

    pub(crate) fecha_actual: [u8; 8],
    pub(crate) hora_actual: [u8; 8],
    pub(crate) cliente_actual: u64,
    pub(crate) cuenta_actual: u64,
    pub(crate) total_clientes: u64,
    pub(crate) total_cuentas: u64,
    pub(crate) total_movimientos: u64,
    pub(crate) total_dinero: u64,

    /// `WS-DATOS-*-TEMP`: the record types double as the temp areas, since
    /// every temp field has the same picture as its record field.
    pub(crate) cli: Cliente,
    pub(crate) cta: Cuenta,
    pub(crate) tar: Tarjeta,
    pub(crate) mov: Movimiento,
    pub(crate) emp: Empleado,

    pub(crate) opcion: Text<2>,
    pub(crate) entrada_decimal: u64,
    pub(crate) gen: Contadores,
    pub(crate) param: Parametros,
    pub(crate) transf: Transferencia,

    /// `WS-MOSTRAR-MENU`. One flag for every menu loop: see `menu_cliente`.
    pub(crate) mostrar_menu: bool,
}

pub(crate) const TOTAL_CLIENTES: Pic = Pic::new(8, 0);
pub(crate) const TOTAL_CUENTAS: Pic = Pic::new(10, 0);
pub(crate) const TOTAL_MOVIMIENTOS: Pic = Pic::new(10, 0);
pub(crate) const TOTAL_DINERO: Pic = Pic::new(15, 2);
pub(crate) const CONTADOR_REGISTROS: Pic = Pic::new(10, 0);

impl<R: BufRead, W: Write> Banco<R, W> {
    pub fn new(input: R, out: W, files: Files, mode: Mode) -> Self {
        Banco {
            t: Term::new(input, out),
            f: files,
            mode,
            fecha_actual: [b' '; 8],
            hora_actual: [b' '; 8],
            cliente_actual: 0,
            cuenta_actual: 0,
            total_clientes: 0,
            total_cuentas: 0,
            total_movimientos: 0,
            total_dinero: 0,
            cli: Cliente::default(),
            cta: Cuenta::default(),
            tar: Tarjeta::default(),
            mov: Movimiento::default(),
            emp: Empleado::default(),
            opcion: [b' '; 2],
            entrada_decimal: 0,
            gen: Contadores::default(),
            param: Parametros::default(),
            transf: Transferencia::default(),
            mostrar_menu: true,
        }
    }

    pub(crate) fn fixed(&self) -> bool {
        self.mode == Mode::Fixed
    }

    /// `MAIN`. Running out of input closes the program like option 0.
    pub fn run(&mut self) -> std::io::Result<()> {
        let result = self
            .inicializar_sistema()
            .and_then(|_| self.menu_principal());
        match result {
            Ok(()) | Err(Stop::Eof) => {
                self.cerrar_sistema()?;
                self.t.flush()
            }
            Err(Stop::Io(e)) => Err(e),
        }
    }

    fn inicializar_sistema(&mut self) -> Step {
        let now = clock::now();
        self.fecha_actual = now.date;
        self.hora_actual = now.time;
        self.f.create_missing()?;
        self.cargar_configuracion()?;
        self.mostrar_splash()
    }

    fn cargar_configuracion(&mut self) -> Step {
        match self.f.read::<Config>()?.into_iter().next() {
            Some(c) => {
                self.param.comision = c.comision_defecto;
                self.param.limite_diario = c.limite_diario_defecto;
                self.param.limite_mensual = c.limite_mensual_defecto;
                self.param.interes_ahorros = c.interes_ahorros;
                self.param.interes_corriente = c.interes_corriente;
                Ok(())
            }
            None => self.crear_configuracion_inicial(),
        }
    }

    fn crear_configuracion_inicial(&mut self) -> Step {
        self.guardar_configuracion()?;
        self.f.append(&Empleado {
            id: 1000,
            usuario: text(b"admin"),
            contrasena: text(b"admin123"),
            nombre: text(b"Administrador Sistema"),
            cargo: text(b"Administrador"),
            fecha_inicio: text(&self.fecha_actual),
            estado: *b"A",
            relleno: [SPACES; 150],
        })?;
        Ok(())
    }

    pub(crate) fn guardar_configuracion(&mut self) -> Step {
        self.f.replace(&[Config {
            version: self.param.version,
            comision_defecto: self.param.comision,
            limite_diario_defecto: self.param.limite_diario,
            limite_mensual_defecto: self.param.limite_mensual,
            interes_ahorros: self.param.interes_ahorros,
            interes_corriente: self.param.interes_corriente,
            relleno: [SPACES; 150],
        }])?;
        Ok(())
    }

    fn mostrar_splash(&mut self) -> Step {
        let version = self.param.version;
        display!(self.t, " ")?;
        display!(self.t, "=====================================")?;
        display!(self.t, "          BANCO v", version)?;
        display!(self.t, "   Sistema Bancario Empresarial")?;
        display!(self.t, "     Fundado en 2026 - Espana")?;
        display!(self.t, "=====================================")?;
        display!(self.t, " ")?;
        display!(self.t, "Inicializando sistema...")?;
        self.cargar_contadores()?;
        display!(self.t, "Contadores cargados correctamente.")?;
        display!(self.t, " ")?;
        Ok(())
    }

    /// The original only counts records here. Its id counters always restart
    /// at their `VALUE`, so after a restart the next customer, account, card
    /// or movement gets an id that is already in the files. The fixed mode
    /// carries on from the highest id on file.
    fn cargar_contadores(&mut self) -> Step {
        self.contar_clientes()?;
        self.contar_cuentas()?;
        self.contar_movimientos()?;
        if self.fixed() {
            let g = &mut self.gen;
            let cuentas = self.f.read::<Cuenta>()?;
            g.cliente = self
                .f
                .read::<Cliente>()?
                .iter()
                .map(|r| r.id)
                .fold(g.cliente, u64::max);
            g.cuenta = cuentas.iter().map(|r| r.id).fold(g.cuenta, u64::max);
            g.iban = cuentas.len() as u64;
            g.tarjeta = self
                .f
                .read::<Tarjeta>()?
                .iter()
                .map(|r| r.id)
                .fold(g.tarjeta, u64::max);
            g.movimiento = self
                .f
                .read::<Movimiento>()?
                .iter()
                .map(|r| r.id)
                .fold(g.movimiento, u64::max);
            g.empleado = self
                .f
                .read::<Empleado>()?
                .iter()
                .map(|r| r.id)
                .fold(g.empleado, u64::max);
        }
        Ok(())
    }

    pub(crate) fn contar_clientes(&mut self) -> Step {
        let n = self
            .f
            .read::<Cliente>()?
            .iter()
            .filter(|c| c.estado == *b"A")
            .count();
        self.total_clientes = TOTAL_CLIENTES.store(n as i128);
        Ok(())
    }

    pub(crate) fn contar_cuentas(&mut self) -> Step {
        let n = self
            .f
            .read::<Cuenta>()?
            .iter()
            .filter(|c| c.estado == *b"A")
            .count();
        self.total_cuentas = TOTAL_CUENTAS.store(n as i128);
        Ok(())
    }

    pub(crate) fn contar_movimientos(&mut self) -> Step {
        let n = self.f.read::<Movimiento>()?.len();
        self.total_movimientos = TOTAL_MOVIMIENTOS.store(n as i128);
        Ok(())
    }

    fn menu_principal(&mut self) -> Step {
        self.mostrar_menu = true;
        while self.mostrar_menu {
            display!(self.t, " ")?;
            display!(self.t, "MENU PRINCIPAL - BANCO")?;
            display!(self.t, "==============================")?;
            display!(self.t, "1. Registrarse como Cliente")?;
            display!(self.t, "2. Login Cliente")?;
            display!(self.t, "3. Login Administrador")?;
            display!(self.t, "4. Ver Informacion del Sistema")?;
            display!(self.t, "5. Ayuda")?;
            display!(self.t, "0. Salir")?;
            display!(self.t, "==============================")?;
            self.t.prompt("Seleccione opcion: ")?;
            self.opcion = self.t.accept_x()?;
            // `EVALUATE WS-OPCION WHEN "1"` compares the whole PIC X(2),
            // padded with a space: "1" matches, "01" and "1x" don't.
            match &self.opcion {
                b"1 " => self.registrar_cliente()?,
                b"2 " => self.login_cliente()?,
                b"3 " => self.login_admin()?,
                b"4 " => self.mostrar_info_sistema()?,
                b"5 " => self.mostrar_ayuda()?,
                b"0 " => self.mostrar_menu = false,
                _ => display!(self.t, "Opcion invalida")?,
            }
        }
        Ok(())
    }

    /// `REGISTRAR-MOVIMIENTO`: appends `WS-DATOS-MOVIMIENTO-TEMP` for the
    /// current customer and account.
    ///
    /// The original stamps every movement with the date and time the program
    /// started (`WS-FECHA-ACTUAL`, `WS-HORA-ACTUAL`, set once in
    /// `INICIALIZAR-SISTEMA`). The fixed mode reads the clock.
    pub(crate) fn registrar_movimiento(&mut self, cliente_id: u64) -> Step {
        self.gen.movimiento = ID_MOVIMIENTO.store(self.gen.movimiento as i128 + 1);
        self.mov.id = self.gen.movimiento;
        let (fecha, hora) = if self.fixed() {
            let now = clock::now();
            (now.date, now.time)
        } else {
            (self.fecha_actual, self.hora_actual)
        };
        self.f.append(&Movimiento {
            id: self.mov.id,
            cliente_id,
            cuenta_id: self.cuenta_actual,
            fecha: text(&fecha),
            hora: text(&hora),
            tipo: self.mov.tipo,
            importe: self.mov.importe,
            saldo_anterior: self.mov.saldo_anterior,
            saldo_posterior: self.mov.saldo_posterior,
            cuenta_destino: self.mov.cuenta_destino,
            descripcion: self.mov.descripcion,
            relleno: [SPACES; 50],
        })?;
        Ok(())
    }

    fn mostrar_info_sistema(&mut self) -> Step {
        let version = self.param.version;
        for line in [
            " ",
            "INFORMACION DEL SISTEMA",
            "=======================",
            "Nombre: Banco",
        ] {
            display!(self.t, line)?;
        }
        display!(self.t, "Version: ", version)?;
        for line in [
            "Fundacion: 2026",
            "Ubicacion: Espana",
            "Tipo: Sistema Bancario Empresarial",
            "Base Datos: Archivos Secuenciales",
            "Desarrollado por: XalVok",
            " ",
            "CARACTERISTICAS:",
            "- Gestion de clientes",
            "- Multiples cuentas por cliente",
            "- Operaciones bancarias completas",
            "- Tarjetas de debito",
            "- Historial de movimientos",
            "- Administracion integrada",
        ] {
            display!(self.t, line)?;
        }
        Ok(())
    }

    fn mostrar_ayuda(&mut self) -> Step {
        for line in [
            " ",
            "AYUDA DE BANCO",
            "==================",
            " ",
            "REGISTRARSE:",
            "  - Seleccione opcion 1 en menu principal",
            "  - Complete formulario con datos requeridos",
            "  - Recibira ID de cliente automaticamente",
            " ",
            "LOGIN:",
            "  - Utilice su ID de cliente para acceder",
            "  - Acceso a todas sus cuentas y tarjetas",
            " ",
            "CUENTAS:",
            "  - Puede crear multiples cuentas",
            "  - Tipos: Corriente, Ahorros, Nomina",
            "  - Cada cuenta tiene IBAN unico",
            " ",
            "OPERACIONES:",
            "  - Ingreso: Dinero a su cuenta",
            "  - Retirada: Dinero de su cuenta",
            "  - Transferencias: Entre cuentas propias",
            "    o a otros clientes por IBAN",
            " ",
            "TARJETAS:",
            "  - Se crean automaticamente por cuenta",
            "  - Puede bloquear/desbloquear",
            "  - Cambiar PIN cuando lo desee",
            " ",
            "HISTORIAL:",
            "  - Todos los movimientos quedan registrados",
            "  - Fecha, hora, tipo, importe y saldos",
            " ",
        ] {
            display!(self.t, line)?;
        }
        Ok(())
    }

    fn cerrar_sistema(&mut self) -> std::io::Result<()> {
        display!(self.t, " ")?;
        display!(self.t, "Cerrando SantiBanco...")?;
        display!(self.t, "Datos guardados correctamente")?;
        display!(self.t, "Hasta pronto.")?;
        display!(self.t, " ")
    }
}
