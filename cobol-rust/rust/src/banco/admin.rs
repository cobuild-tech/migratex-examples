//! Administrator login and menu: customers, employees, settings, statistics.

use std::io::{BufRead, Write};

use super::{
    Banco, CONTADOR_REGISTROS, TOTAL_CLIENTES, TOTAL_CUENTAS, TOTAL_DINERO, TOTAL_MOVIMIENTOS,
};
use crate::display;
use crate::records::*;
use crate::term::Step;

impl<R: BufRead, W: Write> Banco<R, W> {
    pub(crate) fn login_admin(&mut self) -> Step {
        display!(self.t, " ")?;
        display!(self.t, "LOGIN ADMINISTRADOR")?;
        display!(self.t, "==================")?;
        self.t.prompt("Usuario: ")?;
        self.emp.usuario = self.t.accept_x()?;
        self.t.prompt("Contrasena: ")?;
        self.emp.contrasena = self.t.accept_x()?;
        // VERIFICAR-ADMIN: any active employee, passwords in plain text.
        let ok = self.f.read::<Empleado>()?.iter().any(|e| {
            e.usuario == self.emp.usuario
                && e.contrasena == self.emp.contrasena
                && e.estado == *b"A"
        });
        if !ok {
            display!(self.t, "Credenciales invalidas")?;
            return Ok(());
        }
        self.menu_admin()
    }

    /// Same shared `WS-MOSTRAR-MENU` as `menu_cliente`: logging out closes
    /// the program in the original.
    fn menu_admin(&mut self) -> Step {
        self.mostrar_menu = true;
        while self.mostrar_menu {
            display!(self.t, " ")?;
            display!(self.t, "MENU ADMINISTRADOR")?;
            display!(self.t, "==================")?;
            display!(self.t, "1. Gestionar Clientes")?;
            display!(self.t, "2. Gestionar Empleados")?;
            display!(self.t, "3. Configurar Sistema")?;
            display!(self.t, "4. Ver Estadisticas")?;
            display!(self.t, "5. Copia de Seguridad")?;
            display!(self.t, "6. Restaurar Datos")?;
            display!(self.t, "0. Cerrar Sesion")?;
            display!(self.t, "==================")?;
            self.t.prompt("Seleccione opcion: ")?;
            self.opcion = self.t.accept_x()?;
            match &self.opcion {
                b"1 " => self.gestionar_clientes_admin()?,
                b"2 " => self.gestionar_empleados()?,
                b"3 " => self.configurar_sistema()?,
                b"4 " => self.ver_estadisticas_admin()?,
                b"5 " => self.copia_de_seguridad()?,
                b"6 " => self.restaurar_datos()?,
                b"0 " => self.mostrar_menu = false,
                _ => display!(self.t, "Opcion invalida")?,
            }
        }
        if self.fixed() {
            self.mostrar_menu = true;
        }
        Ok(())
    }

    fn gestionar_clientes_admin(&mut self) -> Step {
        display!(self.t, " ")?;
        display!(self.t, "GESTIONAR CLIENTES")?;
        display!(self.t, "==================")?;
        display!(self.t, "1. Ver Todos los Clientes")?;
        display!(self.t, "2. Buscar Cliente")?;
        display!(self.t, "3. Eliminar Cliente")?;
        display!(self.t, "0. Volver")?;
        self.t.prompt("Seleccione: ")?;
        self.opcion = self.t.accept_x()?;
        match &self.opcion {
            b"1 " => self.ver_todos_clientes(),
            b"2 " => self.buscar_cliente_admin(),
            b"3 " => self.eliminar_cliente(),
            b"0 " => Ok(()),
            _ => Ok(display!(self.t, "Opcion invalida")?),
        }
    }

    fn ver_todos_clientes(&mut self) -> Step {
        display!(self.t, " ")?;
        display!(self.t, "LISTA DE CLIENTES")?;
        display!(self.t, "=================")?;
        let mut contador = 0u64;
        for c in self.f.read::<Cliente>()? {
            if c.estado == *b"A" {
                contador = CONTADOR_REGISTROS.store(contador as i128 + 1);
                display!(self.t, " ")?;
                display!(self.t, "ID: ", ID_CLIENTE.show(c.id))?;
                display!(self.t, "Nombre: ", c.nombre, " ", c.apellido)?;
                display!(self.t, "DNI: ", c.dni)?;
                display!(self.t, "Email: ", c.email)?;
                display!(self.t, "Telefono: ", c.telefono)?;
            }
        }
        display!(self.t, " ")?;
        display!(
            self.t,
            "Total de clientes: ",
            CONTADOR_REGISTROS.show(contador)
        )?;
        Ok(())
    }

    fn buscar_cliente_admin(&mut self) -> Step {
        display!(self.t, " ")?;
        self.t.prompt("Ingrese DNI a buscar: ")?;
        self.cli.dni = self.t.accept_x()?;
        let dni = self.cli.dni;
        match self
            .f
            .read::<Cliente>()?
            .into_iter()
            .find(|c| c.dni == dni && c.estado == *b"A")
        {
            Some(c) => {
                display!(self.t, " ")?;
                display!(self.t, "ID: ", ID_CLIENTE.show(c.id))?;
                display!(self.t, "Nombre: ", c.nombre, " ", c.apellido)?;
                display!(self.t, "DNI: ", c.dni)?;
                display!(self.t, "Email: ", c.email)?;
                display!(self.t, "Telefono: ", c.telefono)?;
                display!(self.t, "Direccion: ", c.direccion)?;
                display!(self.t, "Ciudad: ", c.ciudad)?;
                display!(self.t, "CP: ", c.cp)?;
            }
            None => display!(self.t, "Cliente no encontrado")?,
        }
        Ok(())
    }

    /// Marks the customer inactive. Their accounts and cards stay active,
    /// and nothing is printed when the id doesn't exist.
    fn eliminar_cliente(&mut self) -> Step {
        display!(self.t, " ")?;
        self.t.prompt("Ingrese ID de cliente: ")?;
        self.cliente_actual = self.t.accept_n(ID_CLIENTE)?;
        let id = self.cliente_actual;
        if self
            .f
            .rewrite_first(|c: &Cliente| c.id == id, |c| c.estado = *b"I")?
        {
            display!(self.t, "Cliente eliminado")?;
        }
        Ok(())
    }

    fn gestionar_empleados(&mut self) -> Step {
        display!(self.t, " ")?;
        display!(self.t, "GESTIONAR EMPLEADOS")?;
        display!(self.t, "===================")?;
        display!(self.t, "1. Ver Empleados")?;
        display!(self.t, "2. Crear Empleado")?;
        display!(self.t, "3. Eliminar Empleado")?;
        display!(self.t, "0. Volver")?;
        self.t.prompt("Seleccione: ")?;
        self.opcion = self.t.accept_x()?;
        match &self.opcion {
            b"1 " => self.ver_empleados(),
            b"2 " => self.crear_empleado(),
            b"3 " => self.eliminar_empleado(),
            b"0 " => Ok(()),
            _ => Ok(display!(self.t, "Opcion invalida")?),
        }
    }

    fn ver_empleados(&mut self) -> Step {
        display!(self.t, " ")?;
        display!(self.t, "EMPLEADOS DEL BANCO")?;
        display!(self.t, "===================")?;
        for e in self.f.read::<Empleado>()? {
            if e.estado == *b"A" {
                display!(self.t, " ")?;
                display!(self.t, "ID: ", ID_EMPLEADO.show(e.id))?;
                display!(self.t, "Usuario: ", e.usuario)?;
                display!(self.t, "Nombre: ", e.nombre)?;
                display!(self.t, "Cargo: ", e.cargo)?;
                display!(self.t, "Fecha Inicio: ", e.fecha_inicio)?;
            }
        }
        Ok(())
    }

    fn crear_empleado(&mut self) -> Step {
        display!(self.t, " ")?;
        display!(self.t, "CREAR NUEVO EMPLEADO")?;
        display!(self.t, "====================")?;
        self.t.prompt("Usuario: ")?;
        self.emp.usuario = self.t.accept_x()?;
        self.t.prompt("Contrasena: ")?;
        self.emp.contrasena = self.t.accept_x()?;
        self.t.prompt("Nombre: ")?;
        self.emp.nombre = self.t.accept_x()?;
        self.t.prompt("Cargo: ")?;
        self.emp.cargo = self.t.accept_x()?;
        self.gen.empleado = ID_EMPLEADO.store(self.gen.empleado as i128 + 1);
        self.emp.id = self.gen.empleado;
        self.f.append(&Empleado {
            fecha_inicio: text(&self.fecha_actual),
            estado: *b"A",
            relleno: [SPACES; 150],
            ..self.emp.clone()
        })?;
        display!(self.t, "Empleado creado exitosamente")?;
        Ok(())
    }

    /// Any employee, the last administrator included.
    fn eliminar_empleado(&mut self) -> Step {
        display!(self.t, " ")?;
        self.t.prompt("Ingrese ID de empleado: ")?;
        self.emp.id = self.t.accept_n(ID_EMPLEADO)?;
        let id = self.emp.id;
        if self
            .f
            .rewrite_first(|e: &Empleado| e.id == id, |e| e.estado = *b"I")?
        {
            display!(self.t, "Empleado eliminado")?;
        }
        Ok(())
    }

    /// Saves the new value to `config.dat`. New accounts pick up the limits
    /// and commission, but nothing in the program ever applies them.
    fn configurar_sistema(&mut self) -> Step {
        display!(self.t, " ")?;
        display!(self.t, "CONFIGURACION DEL SISTEMA")?;
        display!(self.t, "=========================")?;
        display!(self.t, "1. Modificar Comision")?;
        display!(self.t, "2. Modificar Limite Diario")?;
        display!(self.t, "3. Modificar Limite Mensual")?;
        display!(self.t, "4. Modificar Interes Ahorros")?;
        display!(self.t, "5. Modificar Interes Corriente")?;
        display!(self.t, "0. Volver")?;
        self.t.prompt("Seleccione: ")?;
        self.opcion = self.t.accept_x()?;
        match &self.opcion {
            b"1 " => {
                self.t.prompt("Nueva comision: ")?;
                self.param.comision = self.t.accept_n(PORCENTAJE)?;
                self.guardar_configuracion()?;
            }
            b"2 " => {
                self.t.prompt("Nuevo limite diario: ")?;
                self.param.limite_diario = self.t.accept_n(LIMITE)?;
                self.guardar_configuracion()?;
            }
            b"3 " => {
                self.t.prompt("Nuevo limite mensual: ")?;
                self.param.limite_mensual = self.t.accept_n(LIMITE)?;
                self.guardar_configuracion()?;
            }
            b"4 " => {
                self.t.prompt("Nuevo interes ahorros: ")?;
                self.param.interes_ahorros = self.t.accept_n(PORCENTAJE)?;
                self.guardar_configuracion()?;
            }
            b"5 " => {
                self.t.prompt("Nuevo interes corriente: ")?;
                self.param.interes_corriente = self.t.accept_n(PORCENTAJE)?;
                self.guardar_configuracion()?;
            }
            b"0 " => {}
            _ => {
                display!(self.t, "Opcion invalida")?;
                return Ok(());
            }
        }
        display!(self.t, "Configuracion guardada")?;
        Ok(())
    }

    fn ver_estadisticas_admin(&mut self) -> Step {
        display!(self.t, " ")?;
        display!(self.t, "ESTADISTICAS DEL BANCO")?;
        display!(self.t, "======================")?;
        self.contar_clientes()?;
        self.contar_cuentas()?;
        self.contar_movimientos()?;
        // CALCULAR-DINERO-TOTAL
        self.total_dinero = 0;
        for c in self.f.read::<Cuenta>()? {
            if c.estado == *b"A" {
                self.total_dinero = TOTAL_DINERO.store(self.total_dinero as i128 + c.saldo as i128);
            }
        }
        display!(self.t, " ")?;
        display!(
            self.t,
            "Clientes Activos: ",
            TOTAL_CLIENTES.show(self.total_clientes)
        )?;
        display!(
            self.t,
            "Cuentas Activas: ",
            TOTAL_CUENTAS.show(self.total_cuentas)
        )?;
        display!(
            self.t,
            "Movimientos Registrados: ",
            TOTAL_MOVIMIENTOS.show(self.total_movimientos)
        )?;
        display!(
            self.t,
            "Dinero Total en Banco: ",
            TOTAL_DINERO.show(self.total_dinero)
        )?;
        Ok(())
    }

    /// Prints that a backup was made. None is: kept as in the original and
    /// listed in the README.
    fn copia_de_seguridad(&mut self) -> Step {
        display!(self.t, " ")?;
        display!(self.t, "COPIA DE SEGURIDAD")?;
        display!(self.t, "==================")?;
        display!(self.t, "Generando copia de seguridad...")?;
        display!(self.t, "Copia realizada exitosamente")?;
        display!(self.t, "Archivos backup: copia_YYYYMMDD.bak")?;
        Ok(())
    }

    /// Same: asks for confirmation, then restores nothing.
    fn restaurar_datos(&mut self) -> Step {
        display!(self.t, " ")?;
        display!(self.t, "RESTAURAR DATOS")?;
        display!(self.t, "===============")?;
        display!(self.t, "ADVERTENCIA: Esto sobrescribira los datos")?;
        self.t.prompt("Confirma? (S/N): ")?;
        let confirmacion: Text<1> = self.t.accept_x()?;
        if confirmacion != *b"S" {
            display!(self.t, "Operacion cancelada")?;
            return Ok(());
        }
        display!(self.t, "Restaurando datos...")?;
        display!(self.t, "Restauracion completada")?;
        Ok(())
    }
}
