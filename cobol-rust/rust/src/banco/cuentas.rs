//! Accounts, deposits, withdrawals, transfers and the movement history.

use std::io::{BufRead, Write};

use super::Banco;
use crate::display;
use crate::records::*;
use crate::term::Step;

impl<R: BufRead, W: Write> Banco<R, W> {
    pub(crate) fn crear_cuenta(&mut self) -> Step {
        display!(self.t, " ")?;
        display!(self.t, "CREAR NUEVA CUENTA")?;
        display!(self.t, "==================")?;
        display!(self.t, "Tipo de cuenta:")?;
        display!(self.t, "1. Cuenta Corriente")?;
        display!(self.t, "2. Cuenta de Ahorros")?;
        display!(self.t, "3. Cuenta Nomina")?;
        self.t.prompt("Seleccione: ")?;
        self.opcion = self.t.accept_x()?;
        self.cta.tipo = match &self.opcion {
            b"1 " => text(b"CORRIENTE"),
            b"2 " => text(b"AHORROS"),
            b"3 " => text(b"NOMINA"),
            _ => {
                display!(self.t, "Tipo invalido")?;
                return Ok(());
            }
        };
        self.gen.cuenta = ID_CUENTA.store(self.gen.cuenta as i128 + 1);
        self.cta.id = self.gen.cuenta;
        self.generar_iban();
        self.f.append(&Cuenta {
            id: self.cta.id,
            cliente_id: self.cliente_actual,
            iban: self.cta.iban,
            saldo: 0,
            tipo: self.cta.tipo,
            fecha_apertura: text(&self.fecha_actual),
            fecha_cierre: [SPACES; 10],
            estado: *b"A",
            limite_diario: self.param.limite_diario,
            limite_mensual: self.param.limite_mensual,
            comision: self.param.comision,
            relleno: [SPACES; 50],
        })?;
        let iban = self.cta.iban;
        display!(self.t, " ")?;
        display!(self.t, "Cuenta creada exitosamente")?;
        display!(self.t, "IBAN: ", iban)?;
        Ok(())
    }

    /// The original builds the IBAN with
    /// `STRING "ES91" WS-IBAN-NUM-EDIT DELIMITED BY SPACE "0000000000"`,
    /// where `WS-IBAN-NUM-EDIT` is `PIC Z(10)`. Zero suppression fills the
    /// front of that field with spaces, `DELIMITED BY SPACE` stops at the
    /// first one, and the number never makes it in: every account gets
    /// `ES910000000000`. The fixed mode puts the 10-digit sequence number in.
    fn generar_iban(&mut self) {
        self.gen.iban = ID_CUENTA.store(self.gen.iban as i128 + 1);
        let mut iban = b"ES91".to_vec();
        if self.fixed() {
            iban.extend_from_slice(format!("{:010}", self.gen.iban).as_bytes());
        } else {
            let edited = if self.gen.iban == 0 {
                " ".repeat(10)
            } else {
                format!("{:>10}", self.gen.iban)
            };
            iban.extend(edited.bytes().take_while(|&b| b != b' '));
        }
        iban.extend_from_slice(b"0000000000");
        self.cta.iban = text(&iban);
    }

    pub(crate) fn ver_cuentas_cliente(&mut self) -> Step {
        display!(self.t, " ")?;
        display!(self.t, "CUENTAS DEL CLIENTE")?;
        display!(self.t, "==================")?;
        let mut encontrado = false;
        for c in self.f.read::<Cuenta>()? {
            if c.cliente_id == self.cliente_actual && c.estado == *b"A" {
                display!(self.t, " ")?;
                display!(self.t, "ID Cuenta: ", ID_CUENTA.show(c.id))?;
                display!(self.t, "IBAN: ", c.iban)?;
                display!(self.t, "Tipo: ", c.tipo)?;
                display!(self.t, "Saldo: ", SALDO.show(c.saldo))?;
                display!(self.t, "Estado: ", c.estado)?;
                encontrado = true;
            }
        }
        if !encontrado {
            display!(self.t, "No tiene cuentas registradas")?;
        }
        Ok(())
    }

    pub(crate) fn consultar_saldo(&mut self) -> Step {
        display!(self.t, " ")?;
        display!(self.t, "CONSULTAR SALDO")?;
        display!(self.t, "===============")?;
        self.t.prompt("Ingrese ID de cuenta: ")?;
        self.cuenta_actual = self.t.accept_n(ID_CUENTA)?;
        if !self.buscar_cuenta_cliente()? {
            display!(self.t, "Cuenta no encontrada")?;
            return Ok(());
        }
        let saldo = SALDO.show(self.cta.saldo);
        display!(self.t, " ")?;
        display!(self.t, "Saldo actual: ", saldo)?;
        Ok(())
    }

    /// First account with this id that belongs to the logged-in customer,
    /// copied into `WS-DATOS-CUENTA-TEMP`. Closed accounts are not skipped.
    pub(crate) fn buscar_cuenta_cliente(&mut self) -> std::io::Result<bool> {
        let found = self
            .f
            .read::<Cuenta>()?
            .into_iter()
            .find(|c| c.id == self.cuenta_actual && c.cliente_id == self.cliente_actual);
        match found {
            Some(c) => {
                self.cta = c;
                Ok(true)
            }
            None => Ok(false),
        }
    }

    pub(crate) fn ingresar_dinero(&mut self) -> Step {
        display!(self.t, " ")?;
        display!(self.t, "INGRESO DE DINERO")?;
        display!(self.t, "=================")?;
        self.t.prompt("Ingrese ID de cuenta: ")?;
        self.cuenta_actual = self.t.accept_n(ID_CUENTA)?;
        if !self.buscar_cuenta_cliente()? {
            display!(self.t, "Cuenta no encontrada")?;
            return Ok(());
        }
        self.t.prompt("Cantidad a ingresar: ")?;
        self.entrada_decimal = self.t.accept_n(SALDO)?;
        if self.entrada_decimal == 0 {
            display!(self.t, "Cantidad invalida")?;
            return Ok(());
        }
        // REALIZAR-INGRESO
        self.mov.saldo_anterior = self.cta.saldo;
        self.cta.saldo = SALDO.store(self.cta.saldo as i128 + self.entrada_decimal as i128);
        self.mov.saldo_posterior = self.cta.saldo;
        self.actualizar_cuenta()?;
        self.mov.tipo = text(b"INGRESO");
        self.mov.importe = self.entrada_decimal;
        self.mov.cuenta_destino = [SPACES; 24];
        self.mov.descripcion = text(b"Ingreso de efectivo");
        self.registrar_movimiento(self.cliente_actual)?;
        display!(self.t, "Ingreso realizado exitosamente")?;
        Ok(())
    }

    /// Writes `WS-SALDO-CUENTA` to the first record with `WS-ID-CUENTA`. It
    /// matches on the id alone, so when a restart has handed the same id to
    /// two accounts (see `cargar_contadores`), the original updates the
    /// older one, whoever owns it.
    fn actualizar_cuenta(&mut self) -> Step {
        let (id, saldo) = (self.cta.id, self.cta.saldo);
        self.f
            .rewrite_first(|c: &Cuenta| c.id == id, |c| c.saldo = saldo)?;
        Ok(())
    }

    pub(crate) fn retirar_dinero(&mut self) -> Step {
        display!(self.t, " ")?;
        display!(self.t, "RETIRADA DE DINERO")?;
        display!(self.t, "==================")?;
        self.t.prompt("Ingrese ID de cuenta: ")?;
        self.cuenta_actual = self.t.accept_n(ID_CUENTA)?;
        if !self.buscar_cuenta_cliente()? {
            display!(self.t, "Cuenta no encontrada")?;
            return Ok(());
        }
        self.t.prompt("Cantidad a retirar: ")?;
        self.entrada_decimal = self.t.accept_n(SALDO)?;
        if self.entrada_decimal == 0 {
            display!(self.t, "Cantidad invalida")?;
            return Ok(());
        }
        if self.cta.saldo < self.entrada_decimal {
            display!(self.t, "Saldo insuficiente")?;
            return Ok(());
        }
        // REALIZAR-RETIRADA
        self.mov.saldo_anterior = self.cta.saldo;
        self.cta.saldo = SALDO.store(self.cta.saldo as i128 - self.entrada_decimal as i128);
        self.mov.saldo_posterior = self.cta.saldo;
        self.actualizar_cuenta()?;
        self.mov.tipo = text(b"RETIRADA");
        self.mov.importe = self.entrada_decimal;
        self.mov.cuenta_destino = [SPACES; 24];
        self.mov.descripcion = text(b"Retirada de efectivo");
        self.registrar_movimiento(self.cliente_actual)?;
        display!(self.t, "Retirada realizada exitosamente")?;
        Ok(())
    }

    pub(crate) fn transferir_dinero(&mut self) -> Step {
        display!(self.t, " ")?;
        display!(self.t, "TRANSFERENCIA DE DINERO")?;
        display!(self.t, "=======================")?;
        self.t.prompt("Ingrese ID cuenta origen: ")?;
        self.cuenta_actual = self.t.accept_n(ID_CUENTA)?;
        if !self.buscar_cuenta_cliente()? {
            display!(self.t, "Cuenta origen no encontrada")?;
            return Ok(());
        }
        self.transf.cuenta_origen_id = self.cta.id;
        self.transf.saldo_origen_antes = self.cta.saldo;
        self.t.prompt("Ingrese IBAN destino: ")?;
        self.transf.iban_destino = self.t.accept_x()?;
        if !self.buscar_cuenta_por_iban()? {
            display!(self.t, "Cuenta destino no encontrada")?;
            return Ok(());
        }
        self.transf.cuenta_destino_id = self.cta.id;
        self.t.prompt("Cantidad a transferir: ")?;
        self.transf.importe = self.t.accept_n(SALDO)?;
        if self.transf.importe == 0 {
            display!(self.t, "Cantidad invalida")?;
            return Ok(());
        }
        self.cta.saldo = self.transf.saldo_origen_antes;
        if self.transf.saldo_origen_antes < self.transf.importe {
            display!(self.t, "Saldo insuficiente")?;
            return Ok(());
        }
        self.procesar_transferencia()?;
        display!(self.t, "Transferencia realizada exitosamente")?;
        Ok(())
    }

    /// First account with this IBAN, any owner, any state. With the
    /// original's IBANs that is always the oldest account in the file.
    fn buscar_cuenta_por_iban(&mut self) -> std::io::Result<bool> {
        let iban = self.transf.iban_destino;
        match self
            .f
            .read::<Cuenta>()?
            .into_iter()
            .find(|c| c.iban == iban)
        {
            Some(c) => {
                self.cta.id = c.id;
                self.cta.saldo = c.saldo;
                self.transf.cliente_destino_id = c.cliente_id;
                Ok(true)
            }
            None => Ok(false),
        }
    }

    fn procesar_transferencia(&mut self) -> Step {
        let importe = self.transf.importe as i128;
        let antes = self.transf.saldo_origen_antes as i128;

        // ACTUALIZAR-SALDO-ORIGEN
        self.cta.saldo = SALDO.store(antes - importe);
        let (origen, saldo) = (self.transf.cuenta_origen_id, self.cta.saldo);
        self.f
            .rewrite_first(|c: &Cuenta| c.id == origen, |c| c.saldo = saldo)?;

        // ACTUALIZAR-SALDO-DESTINO
        let destino = self.transf.cuenta_destino_id;
        self.f.rewrite_first(
            |c: &Cuenta| c.id == destino,
            |c| c.saldo = SALDO.store(c.saldo as i128 + importe),
        )?;

        // REGISTRAR-MOVIMIENTO-ORIGEN
        self.cta.id = origen;
        self.cuenta_actual = origen;
        self.mov.tipo = text(b"TRANSFERENCIA");
        self.mov.importe = self.transf.importe;
        self.mov.saldo_anterior = self.transf.saldo_origen_antes;
        self.mov.saldo_posterior = SALDO.store(antes - importe);
        self.mov.cuenta_destino = self.transf.iban_destino;
        self.mov.descripcion = text(b"Transferencia enviada");
        self.registrar_movimiento(self.cliente_actual)?;

        // REGISTRAR-MOVIMIENTO-DESTINO
        self.cta.id = destino;
        self.cuenta_actual = destino;
        self.mov.tipo = text(b"TRANSFERENCIA");
        self.mov.importe = self.transf.importe;
        if let Some(c) = self
            .f
            .read::<Cuenta>()?
            .into_iter()
            .find(|c| c.id == destino)
        {
            self.mov.saldo_posterior = c.saldo;
            self.mov.saldo_anterior = SALDO.store(c.saldo as i128 - importe);
        }
        self.mov.cuenta_destino = [SPACES; 24];
        self.mov.descripcion = text(b"Transferencia recibida");
        // The original files the receiving side under WS-CLIENTE-ACTUAL, the
        // sender, so it never shows in the receiver's history and shows twice
        // in the sender's.
        let receptor = if self.fixed() {
            self.transf.cliente_destino_id
        } else {
            self.cliente_actual
        };
        self.registrar_movimiento(receptor)
    }

    pub(crate) fn historial_cliente(&mut self) -> Step {
        display!(self.t, " ")?;
        display!(self.t, "HISTORIAL DE MOVIMIENTOS")?;
        display!(self.t, "========================")?;
        let mut encontrado = false;
        for m in self.f.read::<Movimiento>()? {
            if m.cliente_id == self.cliente_actual {
                display!(self.t, " ")?;
                display!(self.t, "ID: ", ID_MOVIMIENTO.show(m.id))?;
                display!(self.t, "Fecha: ", m.fecha, " Hora: ", m.hora)?;
                display!(self.t, "Tipo: ", m.tipo)?;
                display!(self.t, "Importe: ", SALDO.show(m.importe))?;
                display!(self.t, "Saldo: ", SALDO.show(m.saldo_posterior))?;
                display!(self.t, "Descripcion: ", m.descripcion)?;
                encontrado = true;
            }
        }
        if !encontrado {
            display!(self.t, "No hay movimientos registrados")?;
        }
        Ok(())
    }
}
