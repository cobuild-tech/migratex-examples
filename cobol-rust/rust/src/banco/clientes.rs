//! Registration, customer login, the customer menu and "Modificar Datos".

use std::io::{BufRead, Write};

use super::Banco;
use crate::display;
use crate::records::*;
use crate::term::Step;

impl<R: BufRead, W: Write> Banco<R, W> {
    pub(crate) fn registrar_cliente(&mut self) -> Step {
        display!(self.t, " ")?;
        display!(self.t, "REGISTRO DE CLIENTE NUEVO")?;
        display!(self.t, "========================")?;
        self.t.prompt("Nombre: ")?;
        self.cli.nombre = self.t.accept_x()?;
        self.t.prompt("Apellido: ")?;
        self.cli.apellido = self.t.accept_x()?;
        self.t.prompt("DNI (9 caracteres): ")?;
        self.cli.dni = self.t.accept_x()?;
        if !self.validar_dni() {
            display!(self.t, "DNI invalido")?;
            return Ok(());
        }
        if self.validar_dni_duplicado()? {
            display!(self.t, "Este DNI ya esta registrado")?;
            return Ok(());
        }
        self.t.prompt("Email: ")?;
        self.cli.email = self.t.accept_x()?;
        self.t.prompt("Telefono: ")?;
        self.cli.telefono = self.t.accept_x()?;
        self.t.prompt("Direccion: ")?;
        self.cli.direccion = self.t.accept_x()?;
        self.t.prompt("Ciudad: ")?;
        self.cli.ciudad = self.t.accept_x()?;
        self.t.prompt("Codigo Postal: ")?;
        self.cli.cp = self.t.accept_x()?;

        // GENERAR-ID-CLIENTE
        self.gen.cliente = ID_CLIENTE.store(self.gen.cliente as i128 + 1);
        self.cli.id = self.gen.cliente;

        // GUARDAR-CLIENTE
        self.f.append(&Cliente {
            fecha_reg: text(&self.fecha_actual),
            estado: *b"A",
            relleno: [SPACES; 50],
            ..self.cli.clone()
        })?;
        let id = ID_CLIENTE.show(self.cli.id);
        display!(self.t, " ")?;
        display!(self.t, "Cliente registrado exitosamente")?;
        display!(self.t, "Su ID de cliente es: ", id)?;
        Ok(())
    }

    /// `LENGTH(TRIM(WS-DNI-CLIENTE)) = 9`: the field is 9 bytes, so this only
    /// asks that it neither starts nor ends with a space. The letter and the
    /// digits are never checked.
    fn validar_dni(&self) -> bool {
        self.cli.dni[0] != b' ' && self.cli.dni[8] != b' '
    }

    /// Looks at every customer, deleted ones included.
    fn validar_dni_duplicado(&self) -> std::io::Result<bool> {
        Ok(self
            .f
            .read::<Cliente>()?
            .iter()
            .any(|c| c.dni == self.cli.dni))
    }

    pub(crate) fn login_cliente(&mut self) -> Step {
        display!(self.t, " ")?;
        display!(self.t, "LOGIN DE CLIENTE")?;
        display!(self.t, "================")?;
        self.t.prompt("ID de Cliente: ")?;
        self.cliente_actual = self.t.accept_n(ID_CLIENTE)?;
        if !self.buscar_cliente_por_id()? {
            display!(self.t, "Cliente no encontrado")?;
            return Ok(());
        }
        self.menu_cliente()
    }

    /// Finds the first record with the id and copies it into
    /// `WS-DATOS-CLIENTE-TEMP`. The original ignores `RC-ESTADO`, so a
    /// customer the administrator deleted can still log in; the fixed mode
    /// skips them.
    fn buscar_cliente_por_id(&mut self) -> std::io::Result<bool> {
        let fixed = self.fixed();
        let found = self
            .f
            .read::<Cliente>()?
            .into_iter()
            .find(|c| c.id == self.cliente_actual && (!fixed || c.estado == *b"A"));
        match found {
            Some(c) => {
                self.cli = c;
                Ok(true)
            }
            None => Ok(false),
        }
    }

    /// The loop shares `WS-MOSTRAR-MENU` with the main menu. In the original,
    /// "0. Cerrar Sesion" sets it to "N", the main menu sees "N" too, and the
    /// whole program closes instead of going back. The fixed mode resets the
    /// flag on the way out.
    fn menu_cliente(&mut self) -> Step {
        self.mostrar_menu = true;
        while self.mostrar_menu {
            let nombre = self.cli.nombre;
            display!(self.t, " ")?;
            display!(self.t, "MENU CLIENTE - ", nombre)?;
            display!(self.t, "========================")?;
            display!(self.t, "1.  Crear Cuenta")?;
            display!(self.t, "2.  Ver Cuentas")?;
            display!(self.t, "3.  Consultar Saldo")?;
            display!(self.t, "4.  Ingresar Dinero")?;
            display!(self.t, "5.  Retirar Dinero")?;
            display!(self.t, "6.  Transferir Dinero")?;
            display!(self.t, "7.  Historial de Movimientos")?;
            display!(self.t, "8.  Solicitar Tarjeta")?;
            display!(self.t, "9.  Gestionar Tarjetas")?;
            display!(self.t, "10. Modificar Datos")?;
            display!(self.t, "0.  Cerrar Sesion")?;
            display!(self.t, "========================")?;
            self.t.prompt("Seleccione opcion: ")?;
            self.opcion = self.t.accept_x()?;
            match &self.opcion {
                b"1 " => self.crear_cuenta()?,
                b"2 " => self.ver_cuentas_cliente()?,
                b"3 " => self.consultar_saldo()?,
                b"4 " => self.ingresar_dinero()?,
                b"5 " => self.retirar_dinero()?,
                b"6 " => self.transferir_dinero()?,
                b"7 " => self.historial_cliente()?,
                b"8 " => self.solicitar_tarjeta()?,
                b"9 " => self.gestionar_tarjetas()?,
                b"10" => self.modificar_datos_cliente()?,
                b"0 " => self.mostrar_menu = false,
                _ => display!(self.t, "Opcion invalida")?,
            }
        }
        if self.fixed() {
            self.mostrar_menu = true;
        }
        Ok(())
    }

    fn modificar_datos_cliente(&mut self) -> Step {
        display!(self.t, " ")?;
        display!(self.t, "MODIFICAR DATOS PERSONALES")?;
        display!(self.t, "=========================")?;
        display!(self.t, "1. Cambiar Email")?;
        display!(self.t, "2. Cambiar Telefono")?;
        display!(self.t, "3. Cambiar Direccion")?;
        display!(self.t, "4. Cambiar Ciudad")?;
        display!(self.t, "0. Volver")?;
        self.t.prompt("Seleccione: ")?;
        self.opcion = self.t.accept_x()?;
        match &self.opcion {
            b"1 " => {
                self.t.prompt("Nuevo email: ")?;
                self.cli.email = self.t.accept_x()?;
                self.actualizar_cliente()?;
            }
            b"2 " => {
                self.t.prompt("Nuevo telefono: ")?;
                self.cli.telefono = self.t.accept_x()?;
                self.actualizar_cliente()?;
            }
            b"3 " => {
                self.t.prompt("Nueva direccion: ")?;
                self.cli.direccion = self.t.accept_x()?;
                self.actualizar_cliente()?;
            }
            b"4 " => {
                self.t.prompt("Nueva ciudad: ")?;
                self.cli.ciudad = self.t.accept_x()?;
                self.actualizar_cliente()?;
            }
            b"0 " => {}
            _ => {
                display!(self.t, "Opcion invalida")?;
                return Ok(());
            }
        }
        // Printed after "0. Volver" too; kept.
        display!(self.t, "Datos actualizados")?;
        Ok(())
    }

    fn actualizar_cliente(&mut self) -> Step {
        let cli = self.cli.clone();
        self.f.rewrite_first(
            |c: &Cliente| c.id == self.cliente_actual,
            |c| {
                c.email = cli.email;
                c.telefono = cli.telefono;
                c.direccion = cli.direccion;
                c.ciudad = cli.ciudad;
            },
        )?;
        Ok(())
    }
}
