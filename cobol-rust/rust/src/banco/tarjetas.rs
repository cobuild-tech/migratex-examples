//! Cards: request, list, block, unblock, change PIN.

use std::io::{BufRead, Write};

use super::Banco;
use crate::clock;
use crate::display;
use crate::pic::Pic;
use crate::records::*;
use crate::term::Step;

const ANIO: Pic = Pic::new(4, 0);

/// `STRING field DELIMITED BY SPACE`: the bytes before the first space.
fn hasta_espacio(field: &[u8]) -> &[u8] {
    let end = field.iter().position(|&b| b == b' ').unwrap_or(field.len());
    &field[..end]
}

impl<R: BufRead, W: Write> Banco<R, W> {
    pub(crate) fn solicitar_tarjeta(&mut self) -> Step {
        display!(self.t, " ")?;
        display!(self.t, "SOLICITUD DE TARJETA")?;
        display!(self.t, "====================")?;
        self.t.prompt("Ingrese ID de cuenta: ")?;
        self.cuenta_actual = self.t.accept_n(ID_CUENTA)?;
        if !self.buscar_cuenta_cliente()? {
            display!(self.t, "Cuenta no encontrada")?;
            return Ok(());
        }
        self.crear_tarjeta()?;
        display!(self.t, "Tarjeta creada exitosamente")?;
        Ok(())
    }

    /// Every part of the card comes from the clock, as in the original: the
    /// number is "4532" + today + "1234" (so every card issued on the same
    /// day has the same number), the PIN is the seconds and hundredths, and
    /// the CVV is the minutes and the first digit of the seconds.
    fn crear_tarjeta(&mut self) -> Step {
        self.gen.tarjeta = ID_TARJETA.store(self.gen.tarjeta as i128 + 1);
        self.tar.id = self.gen.tarjeta;

        let hoy = clock::now().date;
        self.tar.numero = text(&[&b"4532"[..], &hoy, b"1234"].concat());
        self.tar.pin = text(&clock::now().time[4..8]);
        self.tar.cvv = text(&clock::now().time[2..5]);

        let anio = ANIO.store(ANIO.decode(&self.fecha_actual[..4]) as i128 + 4);
        let caducidad = [&ANIO.show(anio)[..], &self.fecha_actual[4..6], b"01"].concat();
        self.tar.fecha_caducidad = text(&caducidad);

        // GUARDAR-TARJETA: "Ana Maria" + "Garcia Lopez" becomes "Ana Garcia".
        let titular = [
            hasta_espacio(&self.cli.nombre),
            b" ",
            hasta_espacio(&self.cli.apellido),
        ]
        .concat();
        self.tar.titular = text(&titular);
        self.f.append(&Tarjeta {
            id: self.tar.id,
            cuenta_id: self.cuenta_actual,
            numero: self.tar.numero,
            cvv: self.tar.cvv,
            pin: self.tar.pin,
            fecha_emision: text(&self.fecha_actual),
            fecha_caducidad: self.tar.fecha_caducidad,
            titular: self.tar.titular,
            estado: *b"A",
            limite: 300000, // 3000.00
            relleno: [SPACES; 50],
        })?;
        Ok(())
    }

    pub(crate) fn gestionar_tarjetas(&mut self) -> Step {
        display!(self.t, " ")?;
        display!(self.t, "GESTIONAR TARJETAS")?;
        display!(self.t, "==================")?;
        display!(self.t, "1. Ver Tarjetas")?;
        display!(self.t, "2. Bloquear Tarjeta")?;
        display!(self.t, "3. Desbloquear Tarjeta")?;
        display!(self.t, "4. Cambiar PIN")?;
        display!(self.t, "0. Volver")?;
        self.t.prompt("Seleccione: ")?;
        self.opcion = self.t.accept_x()?;
        match &self.opcion {
            b"1 " => self.ver_tarjetas(),
            b"2 " => self.cambiar_estado_tarjeta(*b"B", "Tarjeta bloqueada"),
            b"3 " => self.cambiar_estado_tarjeta(*b"A", "Tarjeta desbloqueada"),
            b"4 " => self.cambiar_pin_tarjeta(),
            b"0 " => Ok(()),
            _ => Ok(display!(self.t, "Opcion invalida")?),
        }
    }

    /// The account ids the logged-in customer owns.
    fn cuentas_propias(&self) -> std::io::Result<Vec<u64>> {
        Ok(self
            .f
            .read::<Cuenta>()?
            .into_iter()
            .filter(|c| c.cliente_id == self.cliente_actual)
            .map(|c| c.id)
            .collect())
    }

    /// The original lists every active or blocked card in the bank, whoever
    /// owns it. The fixed mode lists the customer's own.
    fn ver_tarjetas(&mut self) -> Step {
        display!(self.t, " ")?;
        display!(self.t, "TARJETAS DEL CLIENTE")?;
        display!(self.t, "===================")?;
        let propias = self.cuentas_propias()?;
        let fixed = self.fixed();
        let mut encontrado = false;
        for t in self.f.read::<Tarjeta>()? {
            if (t.estado == *b"A" || t.estado == *b"B")
                && (!fixed || propias.contains(&t.cuenta_id))
            {
                display!(self.t, " ")?;
                display!(self.t, "ID: ", ID_TARJETA.show(t.id))?;
                display!(self.t, "Numero: ", t.numero)?;
                display!(self.t, "Titular: ", t.titular)?;
                display!(self.t, "Caducidad: ", t.fecha_caducidad)?;
                display!(self.t, "Estado: ", t.estado)?;
                encontrado = true;
            }
        }
        if !encontrado {
            display!(self.t, "No tiene tarjetas")?;
        }
        Ok(())
    }

    /// The card whose id was typed, if the current mode lets this customer
    /// touch it. The original accepts any card in the bank.
    fn tarjeta_permitida(&self) -> std::io::Result<impl Fn(&Tarjeta) -> bool> {
        let id = self.tar.id;
        let propias = if self.fixed() {
            Some(self.cuentas_propias()?)
        } else {
            None
        };
        Ok(move |t: &Tarjeta| {
            t.id == id && propias.as_ref().is_none_or(|p| p.contains(&t.cuenta_id))
        })
    }

    /// `BLOQUEAR-TARJETA` and `DESBLOQUEAR-TARJETA`, which differ only in the
    /// state they write. The original says nothing when no card matches.
    fn cambiar_estado_tarjeta(&mut self, estado: Text<1>, mensaje: &str) -> Step {
        self.t.prompt("Ingrese ID de tarjeta: ")?;
        self.tar.id = self.t.accept_n(ID_TARJETA)?;
        let permitida = self.tarjeta_permitida()?;
        if self.f.rewrite_first(permitida, |t| t.estado = estado)? {
            display!(self.t, mensaje)?;
        } else if self.fixed() {
            display!(self.t, "Tarjeta no encontrada")?;
        }
        Ok(())
    }

    /// The original stores whatever was typed, letters and blanks included.
    /// The fixed mode wants four digits.
    fn cambiar_pin_tarjeta(&mut self) -> Step {
        self.t.prompt("Ingrese ID de tarjeta: ")?;
        self.tar.id = self.t.accept_n(ID_TARJETA)?;
        self.t.prompt("Ingrese nuevo PIN (4 digitos): ")?;
        self.tar.pin = self.t.accept_x()?;
        if self.fixed() && !self.tar.pin.iter().all(u8::is_ascii_digit) {
            display!(self.t, "PIN invalido")?;
            return Ok(());
        }
        let pin = self.tar.pin;
        let permitida = self.tarjeta_permitida()?;
        if self.f.rewrite_first(permitida, |t| t.pin = pin)? {
            display!(self.t, "PIN cambiado exitosamente")?;
        } else if self.fixed() {
            display!(self.t, "Tarjeta no encontrada")?;
        }
        Ok(())
    }
}
