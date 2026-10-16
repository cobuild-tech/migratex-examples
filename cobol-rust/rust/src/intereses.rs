//! `PROGRAM-ID. INTERESES.`: posts a month's interest to every active
//! savings and current account.

use std::io::{self, Write};

use crate::clock;
use crate::display;
use crate::files::Files;
use crate::pic::Pic;
use crate::records::*;
use crate::term::Term;

const CONTADOR: Pic = Pic::new(10, 0);
const TOTAL: Pic = Pic::new(15, 2);

/// `COMPUTE WS-INTERES = RCU-SALDO * WS-TASA / 100 / 12` into `PIC 9(12)V99`.
///
/// The balance is in cents and the rate in thousandths of a percent, so the
/// exact result in cents is `saldo * tasa / (100 * 12 * 1000)`. COBOL has no
/// `ROUNDED` here, so the extra decimals are cut off, not rounded: 2.5720...
/// becomes 2.57, and so does 2.5799. Integer division does the same.
pub fn interes_mensual(saldo: u64, tasa: u64) -> u64 {
    SALDO.store((u128::from(saldo) * u128::from(tasa) / 1_200_000) as i128)
}

pub fn run(files: &Files, out: impl Write) -> io::Result<()> {
    let mut t = Term::new(io::empty(), out);
    let now = clock::now();

    // CARGAR-TASAS: the defaults banco.cob writes on first run.
    let (mut ahorros, mut corriente) = (2500, 500);
    if let Some(c) = files.read::<Config>()?.into_iter().next() {
        ahorros = c.interes_ahorros;
        corriente = c.interes_corriente;
    }

    // CARGAR-ULTIMO-MOVIMIENTO
    let mut ultimo = files
        .read::<Movimiento>()?
        .iter()
        .map(|m| m.id)
        .fold(1_000_000, u64::max);

    display!(t, "=====================================")?;
    display!(t, "   ABONO MENSUAL DE INTERESES")?;
    display!(t, "=====================================")?;
    display!(t, "Fecha: ", now.date)?;
    display!(t, "Interes ahorros (%): ", PORCENTAJE.show(ahorros))?;
    display!(t, "Interes corriente (%): ", PORCENTAJE.show(corriente))?;
    display!(t, " ")?;

    // ABONAR-INTERESES
    let mut cuentas = files.read::<Cuenta>()?;
    let (mut leidas, mut abonadas, mut total) = (0u64, 0u64, 0u64);
    for c in cuentas.iter_mut() {
        leidas = CONTADOR.store(leidas as i128 + 1);
        if c.estado != *b"A" {
            continue;
        }
        let tasa = match &c.tipo {
            t if *t == text::<20>(b"AHORROS") => ahorros,
            t if *t == text::<20>(b"CORRIENTE") => corriente,
            _ => 0,
        };
        let interes = interes_mensual(c.saldo, tasa);
        if interes == 0 {
            continue;
        }
        let anterior = c.saldo;
        c.saldo = SALDO.store(c.saldo as i128 + interes as i128);
        abonadas = CONTADOR.store(abonadas as i128 + 1);
        total = TOTAL.store(total as i128 + interes as i128);
        ultimo = ID_MOVIMIENTO.store(ultimo as i128 + 1);
        files.append(&Movimiento {
            id: ultimo,
            cliente_id: c.cliente_id,
            cuenta_id: c.id,
            fecha: text(&now.date),
            hora: text(&now.time),
            tipo: text(b"INTERES"),
            importe: interes,
            saldo_anterior: anterior,
            saldo_posterior: c.saldo,
            cuenta_destino: [SPACES; 24],
            descripcion: text(b"Abono de intereses"),
            relleno: [SPACES; 50],
        })?;
        display!(
            t,
            "Cuenta ",
            ID_CUENTA.show(c.id),
            " ",
            c.tipo,
            " Saldo: ",
            SALDO.show(anterior),
            " Interes: ",
            SALDO.show(interes)
        )?;
    }
    files.replace(&cuentas)?;

    display!(t, " ")?;
    display!(t, "Cuentas leidas: ", CONTADOR.show(leidas))?;
    display!(t, "Cuentas abonadas: ", CONTADOR.show(abonadas))?;
    display!(t, "Total intereses: ", TOTAL.show(total))?;
    t.flush()
}

#[cfg(test)]
mod tests {
    use super::interes_mensual;

    #[test]
    fn interest_is_cut_off_not_rounded() {
        // 1234.57 at 2.5% a year: 2.5720... a month.
        assert_eq!(interes_mensual(123457, 2500), 257);
        // 1234.56 at 2.5%: 2.5719..., still 2.57.
        assert_eq!(interes_mensual(123456, 2500), 257);
        // 4.79 at 2.5%: 0.00997..., rounds to 0.01 but cuts to 0.00.
        assert_eq!(interes_mensual(479, 2500), 0);
    }
}
