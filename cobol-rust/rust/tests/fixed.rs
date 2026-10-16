//! The default mode: each test pins one bug the original has (see the golden
//! sessions for the original behaviour) and checks the fix.

use std::fs;
use std::io::Cursor;
use std::path::PathBuf;

use banco::banco::{Banco, Mode};
use banco::files::Files;
use banco::records::{Cuenta, Movimiento, Tarjeta};

struct Bank {
    dir: PathBuf,
}

impl Bank {
    fn new(name: &str) -> Bank {
        let dir = std::env::temp_dir().join(format!("banco-fixed-{}-{name}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        Bank { dir }
    }

    /// One run of the program with these answers typed in.
    fn session(&self, answers: &[&str]) -> String {
        let input: String = answers.iter().map(|a| format!("{a}\n")).collect();
        let mut out = Vec::new();
        Banco::new(
            Cursor::new(input),
            &mut out,
            Files::new(&self.dir),
            Mode::Fixed,
        )
        .run()
        .unwrap();
        String::from_utf8(out).unwrap()
    }

    fn files(&self) -> Files {
        Files::new(&self.dir)
    }
}

impl Drop for Bank {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.dir);
    }
}

fn registro<'a>(nombre: &'a str, dni: &'a str) -> Vec<&'a str> {
    vec![
        "1",
        nombre,
        "Prueba",
        dni,
        "correo@x.es",
        "600000000",
        "Calle 1",
        "Madrid",
        "28001",
    ]
}

#[test]
fn logging_out_goes_back_to_the_main_menu() {
    let bank = Bank::new("logout");
    let mut answers = registro("Ana", "12345678Z");
    answers.extend(["2", "1001", "0", "4", "0"]);
    let out = bank.session(&answers);
    let after_logout = out.split("Cerrar Sesion").last().unwrap();
    assert!(after_logout.contains("INFORMACION DEL SISTEMA"), "{out}");
}

#[test]
fn ids_and_ibans_carry_on_after_a_restart() {
    let bank = Bank::new("restart");
    let mut first = registro("Ana", "12345678Z");
    first.extend(["2", "1001", "1", "1", "0", "0"]);
    bank.session(&first);

    let mut second = registro("Luis", "87654321X");
    second.extend(["2", "1002", "1", "1", "0", "0"]);
    let out = bank.session(&second);
    assert!(out.contains("Su ID de cliente es: 00001002"), "{out}");

    let cuentas = bank.files().read::<Cuenta>().unwrap();
    let ids: Vec<u64> = cuentas.iter().map(|c| c.id).collect();
    assert_eq!(ids, [100001, 100002]);
    assert_eq!(&cuentas[0].iban, b"ES9100000000010000000000");
    assert_eq!(&cuentas[1].iban, b"ES9100000000020000000000");
}

#[test]
fn a_transfer_reaches_the_other_customer_and_shows_in_their_history() {
    let bank = Bank::new("transfer");
    let mut answers = registro("Ana", "12345678Z");
    answers.extend(registro("Luis", "87654321X"));
    // Ana: account 100001, 100.00 in, 40.00 out to Luis's account 100002.
    answers.extend(["2", "1001", "1", "1", "4", "100001", "100", "0"]);
    answers.extend(["2", "1002", "1", "1", "0"]);
    answers.extend([
        "2",
        "1001",
        "6",
        "100001",
        "ES9100000000020000000000",
        "40",
        "0",
    ]);
    answers.extend(["2", "1002", "7", "0", "0"]);
    let out = bank.session(&answers);

    let cuentas = bank.files().read::<Cuenta>().unwrap();
    assert_eq!(cuentas[0].saldo, 6000);
    assert_eq!(cuentas[1].saldo, 4000);

    let movs = bank.files().read::<Movimiento>().unwrap();
    let recibida = movs
        .iter()
        .find(|m| m.descripcion.starts_with(b"Transferencia recibida"))
        .unwrap();
    assert_eq!(recibida.cliente_id, 1002);
    assert!(out
        .rsplit("HISTORIAL DE MOVIMIENTOS")
        .next()
        .unwrap()
        .contains("Transferencia recibida"));
}

#[test]
fn customers_only_see_and_change_their_own_cards() {
    let bank = Bank::new("cards");
    let mut answers = registro("Ana", "12345678Z");
    answers.extend(registro("Luis", "87654321X"));
    answers.extend(["2", "1001", "1", "1", "8", "100001", "0"]);
    answers.extend([
        "2", "1002", "9", "1", "9", "2", "10001", "9", "4", "10001", "0000", "0", "0",
    ]);
    let out = bank.session(&answers);

    // Ana's card exists, so this only prints if Luis's list leaves it out.
    assert!(out.contains("No tiene tarjetas"), "{out}");
    assert_eq!(out.matches("Tarjeta no encontrada").count(), 2, "{out}");
    let tarjeta = &bank.files().read::<Tarjeta>().unwrap()[0];
    assert_eq!(&tarjeta.estado, b"A");
    assert_ne!(&tarjeta.pin, b"0000");
}

#[test]
fn a_pin_must_be_four_digits() {
    let bank = Bank::new("pin");
    let mut answers = registro("Ana", "12345678Z");
    answers.extend([
        "2", "1001", "1", "1", "8", "100001", "9", "4", "10001", "12a", "9", "4", "10001", "4321",
        "0", "0",
    ]);
    let out = bank.session(&answers);
    assert!(out.contains("PIN invalido"), "{out}");
    assert!(out.contains("PIN cambiado exitosamente"), "{out}");
    assert_eq!(&bank.files().read::<Tarjeta>().unwrap()[0].pin, b"4321");
}

#[test]
fn a_deleted_customer_cannot_log_in() {
    let bank = Bank::new("deleted");
    let mut answers = registro("Ana", "12345678Z");
    answers.extend(["3", "admin", "admin123", "1", "3", "1001", "0"]);
    answers.extend(["2", "1001", "0"]);
    let out = bank.session(&answers);
    assert!(out.contains("Cliente eliminado"), "{out}");
    let last_login = out.rsplit("LOGIN DE CLIENTE").next().unwrap();
    assert!(last_login.contains("Cliente no encontrado"), "{out}");
    assert!(!last_login.contains("MENU CLIENTE"), "{out}");
}

#[test]
fn running_out_of_input_closes_the_program() {
    let bank = Bank::new("eof");
    let out = bank.session(&["4"]);
    assert!(out.ends_with("Hasta pronto.\n \n"), "{out}");
}
