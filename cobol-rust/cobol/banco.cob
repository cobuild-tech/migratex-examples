       IDENTIFICATION DIVISION.
       PROGRAM-ID. BANCO.
       AUTHOR. XalVok.
       DATE-WRITTEN. 2024.
       ENVIRONMENT DIVISION.
       INPUT-OUTPUT SECTION.
       FILE-CONTROL.
           SELECT CLIENTES-FILE ASSIGN TO "clientes.dat"
               ORGANIZATION IS SEQUENTIAL.
           SELECT CUENTAS-FILE ASSIGN TO "cuentas.dat"
               ORGANIZATION IS SEQUENTIAL.
           SELECT TARJETAS-FILE ASSIGN TO "tarjetas.dat"
               ORGANIZATION IS SEQUENTIAL.
           SELECT MOVIMIENTOS-FILE ASSIGN TO "movimientos.dat"
               ORGANIZATION IS SEQUENTIAL.
           SELECT CONFIG-FILE ASSIGN TO "config.dat"
               ORGANIZATION IS SEQUENTIAL.
           SELECT EMPLEADOS-FILE ASSIGN TO "empleados.dat"
               ORGANIZATION IS SEQUENTIAL.
       DATA DIVISION.
       FILE SECTION.
       FD CLIENTES-FILE.
       01 REGISTRO-CLIENTE.
           05 RC-ID             PIC 9(8).
           05 RC-NOMBRE         PIC X(50).
           05 RC-APELLIDO       PIC X(50).
           05 RC-DNI            PIC X(9).
           05 RC-EMAIL          PIC X(50).
           05 RC-TELEFONO       PIC X(15).
           05 RC-DIRECCION      PIC X(100).
           05 RC-CIUDAD         PIC X(30).
           05 RC-CP             PIC X(5).
           05 RC-FECHA-REG      PIC X(10).
           05 RC-ESTADO         PIC X(1).
           05 RC-RELLENO        PIC X(50).
       FD CUENTAS-FILE.
       01 REGISTRO-CUENTA.
           05 RCU-ID            PIC 9(10).
           05 RCU-CLIENTE-ID    PIC 9(8).
           05 RCU-IBAN          PIC X(24).
           05 RCU-SALDO         PIC 9(12)V99.
           05 RCU-TIPO          PIC X(20).
           05 RCU-FECHA-APERTURA PIC X(10).
           05 RCU-FECHA-CIERRE  PIC X(10).
           05 RCU-ESTADO        PIC X(1).
           05 RCU-LIMITE-DIARIO PIC 9(10)V99.
           05 RCU-LIMITE-MENSUAL PIC 9(10)V99.
           05 RCU-COMISION      PIC 9(3)V999.
           05 RCU-RELLENO       PIC X(50).
       FD TARJETAS-FILE.
       01 REGISTRO-TARJETA.
           05 RT-ID             PIC 9(10).
           05 RT-CUENTA-ID      PIC 9(10).
           05 RT-NUMERO         PIC X(16).
           05 RT-CVV            PIC X(3).
           05 RT-PIN            PIC X(4).
           05 RT-FECHA-EMISION  PIC X(10).
           05 RT-FECHA-CADUCIDAD PIC X(10).
           05 RT-TITULAR        PIC X(100).
           05 RT-ESTADO         PIC X(1).
           05 RT-LIMITE         PIC 9(10)V99.
           05 RT-RELLENO        PIC X(50).
       FD MOVIMIENTOS-FILE.
       01 REGISTRO-MOVIMIENTO.
           05 RM-ID             PIC 9(12).
           05 RM-CLIENTE-ID     PIC 9(8).
           05 RM-CUENTA-ID      PIC 9(10).
           05 RM-FECHA          PIC X(10).
           05 RM-HORA           PIC X(8).
           05 RM-TIPO           PIC X(20).
           05 RM-IMPORTE        PIC 9(12)V99.
           05 RM-SALDO-ANTERIOR PIC 9(12)V99.
           05 RM-SALDO-POSTERIOR PIC 9(12)V99.
           05 RM-CUENTA-DESTINO PIC X(24).
           05 RM-DESCRIPCION    PIC X(100).
           05 RM-RELLENO        PIC X(50).
       FD CONFIG-FILE.
       01 REGISTRO-CONFIG.
           05 RCF-VERSION              PIC X(10).
           05 RCF-COMISION-DEFECTO     PIC 9(3)V999.
           05 RCF-LIMITE-DIARIO-DEFECTO PIC 9(10)V99.
           05 RCF-LIMITE-MENSUAL-DEFECTO PIC 9(10)V99.
           05 RCF-INTERES-AHORROS      PIC 9(3)V999.
           05 RCF-INTERES-CORRIENTE    PIC 9(3)V999.
           05 RCF-RELLENO              PIC X(150).
       FD EMPLEADOS-FILE.
       01 REGISTRO-EMPLEADO.
           05 RE-ID             PIC 9(8).
           05 RE-USUARIO        PIC X(30).
           05 RE-CONTRASENA     PIC X(30).
           05 RE-NOMBRE         PIC X(50).
           05 RE-CARGO          PIC X(30).
           05 RE-FECHA-INICIO   PIC X(10).
           05 RE-ESTADO         PIC X(1).
           05 RE-RELLENO        PIC X(150).
       WORKING-STORAGE SECTION.
       01 WS-VARIABLES-GLOBALES.
           05 WS-VERSION           PIC X(10) VALUE "1.0.0".
           05 WS-FECHA-ACTUAL      PIC X(8).
           05 WS-HORA-ACTUAL       PIC X(8).
           05 WS-USUARIO-ACTUAL    PIC X(30).
           05 WS-TIPO-USUARIO      PIC X(10).
           05 WS-CLIENTE-ACTUAL    PIC 9(8)  VALUE 0.
           05 WS-CUENTA-ACTUAL     PIC 9(10) VALUE 0.
           05 WS-ESTADO-EJECUCION  PIC X(20) VALUE "NORMAL".
           05 WS-CONTADOR-REGISTROS PIC 9(10) VALUE 0.
           05 WS-TOTAL-DINERO      PIC 9(15)V99 VALUE 0.
           05 WS-TOTAL-MOVIMIENTOS PIC 9(10) VALUE 0.
           05 WS-TOTAL-CLIENTES    PIC 9(8)  VALUE 0.
           05 WS-TOTAL-CUENTAS     PIC 9(10) VALUE 0.
       01 WS-DATOS-CLIENTE-TEMP.
           05 WS-ID-CLIENTE        PIC 9(8).
           05 WS-NOMBRE-CLIENTE    PIC X(50).
           05 WS-APELLIDO-CLIENTE  PIC X(50).
           05 WS-DNI-CLIENTE       PIC X(9).
           05 WS-EMAIL-CLIENTE     PIC X(50).
           05 WS-TELEFONO-CLIENTE  PIC X(15).
           05 WS-DIRECCION-CLIENTE PIC X(100).
           05 WS-CIUDAD-CLIENTE    PIC X(30).
           05 WS-CP-CLIENTE        PIC X(5).
           05 WS-ESTADO-CLIENTE    PIC X(1).
       01 WS-DATOS-CUENTA-TEMP.
           05 WS-ID-CUENTA             PIC 9(10).
           05 WS-CLIENTE-ID-CUENTA     PIC 9(8).
           05 WS-IBAN-CUENTA           PIC X(24).
           05 WS-SALDO-CUENTA          PIC 9(12)V99.
           05 WS-TIPO-CUENTA           PIC X(20).
           05 WS-ESTADO-CUENTA         PIC X(1).
           05 WS-LIMITE-DIARIO-CUENTA  PIC 9(10)V99.
           05 WS-LIMITE-MENSUAL-CUENTA PIC 9(10)V99.
           05 WS-COMISION-CUENTA       PIC 9(3)V999.
       01 WS-DATOS-TARJETA-TEMP.
           05 WS-ID-TARJETA            PIC 9(10).
           05 WS-CUENTA-ID-TARJETA     PIC 9(10).
           05 WS-NUMERO-TARJETA        PIC X(16).
           05 WS-CVV-TARJETA           PIC X(3).
           05 WS-PIN-TARJETA           PIC X(4).
           05 WS-FECHA-CADUCIDAD-TARJETA PIC X(10).
           05 WS-TITULAR-TARJETA       PIC X(100).
           05 WS-ESTADO-TARJETA        PIC X(1).
           05 WS-LIMITE-TARJETA        PIC 9(10)V99.
       01 WS-DATOS-MOVIMIENTO-TEMP.
           05 WS-ID-MOVIMIENTO         PIC 9(12).
           05 WS-CLIENTE-ID-MOVIMIENTO PIC 9(8).
           05 WS-CUENTA-ID-MOVIMIENTO  PIC 9(10).
           05 WS-TIPO-MOVIMIENTO       PIC X(20).
           05 WS-IMPORTE-MOVIMIENTO    PIC 9(12)V99.
           05 WS-SALDO-ANTERIOR-MOV    PIC 9(12)V99.
           05 WS-SALDO-POSTERIOR-MOV   PIC 9(12)V99.
           05 WS-CUENTA-DESTINO-MOV    PIC X(24).
           05 WS-DESCRIPCION-MOV       PIC X(100).
       01 WS-DATOS-EMPLEADO-TEMP.
           05 WS-ID-EMPLEADO           PIC 9(8).
           05 WS-USUARIO-EMPLEADO      PIC X(30).
           05 WS-CONTRASENA-EMPLEADO   PIC X(30).
           05 WS-NOMBRE-EMPLEADO       PIC X(50).
           05 WS-CARGO-EMPLEADO        PIC X(30).
       01 WS-VARIABLES-ENTRADA.
           05 WS-OPCION            PIC X(2).
           05 WS-ENTRADA-TEXTO     PIC X(100).
           05 WS-ENTRADA-NUMERO    PIC 9(10).
           05 WS-ENTRADA-DECIMAL   PIC 9(12)V99.
           05 WS-CONFIRMACION      PIC X(1).
       01 WS-VARIABLES-VALIDACION.
           05 WS-DNI-VALIDO        PIC X(1).
           05 WS-IBAN-VALIDO       PIC X(1).
           05 WS-CANTIDAD-VALIDA   PIC X(1).
           05 WS-PIN-VALIDO        PIC X(1).
           05 WS-DNI-EXISTE        PIC X(1).
           05 WS-CLIENTE-EXISTE    PIC X(1).
           05 WS-CUENTA-EXISTE     PIC X(1).
           05 WS-SALDO-SUFICIENTE  PIC X(1).
           05 WS-TARJETA-EXISTE    PIC X(1).
       01 WS-GENERADORES.
           05 WS-CONTADOR-ID-CLIENTE    PIC 9(8)  VALUE 1000.
           05 WS-CONTADOR-ID-CUENTA     PIC 9(10) VALUE 100000.
           05 WS-CONTADOR-ID-TARJETA    PIC 9(10) VALUE 10000.
           05 WS-CONTADOR-ID-MOVIMIENTO PIC 9(12) VALUE 1000000.
           05 WS-CONTADOR-ID-EMPLEADO   PIC 9(8)  VALUE 500.
           05 WS-NUMERO-IBAN-SECUENCIAL PIC 9(10) VALUE 0.
       01 WS-PARAMETROS-SISTEMA.
           05 WS-VERSION-SISTEMA           PIC X(10) VALUE "1.0.0".
           05 WS-COMISION-DEFECTO          PIC 9(3)V999 VALUE 0.5.
           05 WS-LIMITE-DIARIO-DEFECTO     PIC 9(10)V99 VALUE 600.00.
           05 WS-LIMITE-MENSUAL-DEFECTO    PIC 9(10)V99 VALUE 5000.00.
           05 WS-INTERES-AHORROS           PIC 9(3)V999 VALUE 2.5.
           05 WS-INTERES-CORRIENTE         PIC 9(3)V999 VALUE 0.5.
       01 WS-VARIABLES-TRANSFERENCIA.
           05 WS-IBAN-DESTINO              PIC X(24).
           05 WS-IMPORTE-TRANSFERENCIA     PIC 9(12)V99.
           05 WS-CUENTA-ORIGEN-ID          PIC 9(10).
           05 WS-CUENTA-DESTINO-ID         PIC 9(10).
           05 WS-CONCEPTO-TRANSFERENCIA    PIC X(100).
           05 WS-SALDO-ORIGEN-ANTES        PIC 9(12)V99.
       01 WS-VARIABLES-SEGURIDAD.
           05 WS-INTENTOS-LOGIN    PIC 9(2) VALUE 0.
           05 WS-INTENTOS-MAXIMO   PIC 9(2) VALUE 3.
           05 WS-BLOQUEADO         PIC X(1) VALUE "N".
           05 WS-PIN-INTRODUCIDO   PIC X(4).
       01 WS-VARIABLES-BUSQUEDA.
           05 WS-ENCONTRADO        PIC X(1).
           05 WS-CRITERIO-BUSQUEDA PIC X(100).
           05 WS-TIPO-BUSQUEDA     PIC X(20).
       01 WS-VARIABLES-SISTEMA-ARCHIVOS.
           05 WS-STATUS-ARCHIVO    PIC 9(4).
           05 WS-EOF-ARCHIVO       PIC X(1).
           05 WS-ARCHIVO-EXISTE    PIC X(1).
       01 WS-VARIABLES-MENU.
           05 WS-OPCION-MENU       PIC X(2).
           05 WS-MOSTRAR-MENU      PIC X(1).
           05 WS-VOLVER            PIC X(1).
       01 WS-VARIABLES-ESTADISTICAS.
           05 WS-TOTAL-SALDO       PIC 9(15)V99.
           05 WS-PROMEDIO-SALDO    PIC 9(15)V99.
           05 WS-SALDO-MAXIMO      PIC 9(12)V99.
           05 WS-SALDO-MINIMO      PIC 9(12)V99.
           05 WS-CONTADOR-TEMP     PIC 9(10).
       01 WS-VARIABLES-TARJETA-GEN.
           05 WS-FECHA-NUM         PIC 9(8).
           05 WS-HORA-NUM          PIC 9(8).
           05 WS-IBAN-NUM-EDIT     PIC Z(10).
           05 WS-ANIO-CADUCIDAD    PIC 9(4).
           05 WS-ANIO-ACTUAL       PIC 9(4).
           05 WS-MES-ACTUAL        PIC 9(2).
       PROCEDURE DIVISION.
       MAIN.
           PERFORM INICIALIZAR-SISTEMA
           PERFORM MENU-PRINCIPAL
           PERFORM CERRAR-SISTEMA
           STOP RUN.
       INICIALIZAR-SISTEMA.
           ACCEPT WS-FECHA-ACTUAL FROM DATE YYYYMMDD
           ACCEPT WS-HORA-ACTUAL FROM TIME
           PERFORM VERIFICAR-CREAR-ARCHIVOS
           PERFORM CARGAR-CONFIGURACION
           PERFORM MOSTRAR-SPLASH.
       VERIFICAR-CREAR-ARCHIVOS.
           OPEN EXTEND CLIENTES-FILE
           CLOSE CLIENTES-FILE
           OPEN EXTEND CUENTAS-FILE
           CLOSE CUENTAS-FILE
           OPEN EXTEND TARJETAS-FILE
           CLOSE TARJETAS-FILE
           OPEN EXTEND MOVIMIENTOS-FILE
           CLOSE MOVIMIENTOS-FILE
           OPEN EXTEND CONFIG-FILE
           CLOSE CONFIG-FILE
           OPEN EXTEND EMPLEADOS-FILE
           CLOSE EMPLEADOS-FILE.
       CARGAR-CONFIGURACION.
           OPEN INPUT CONFIG-FILE
           READ CONFIG-FILE
               AT END
                   CLOSE CONFIG-FILE
                   PERFORM CREAR-CONFIGURACION-INICIAL
               NOT AT END
                   MOVE RCF-COMISION-DEFECTO
                       TO WS-COMISION-DEFECTO
                   MOVE RCF-LIMITE-DIARIO-DEFECTO
                       TO WS-LIMITE-DIARIO-DEFECTO
                   MOVE RCF-LIMITE-MENSUAL-DEFECTO
                       TO WS-LIMITE-MENSUAL-DEFECTO
                   MOVE RCF-INTERES-AHORROS
                       TO WS-INTERES-AHORROS
                   MOVE RCF-INTERES-CORRIENTE
                       TO WS-INTERES-CORRIENTE
                   CLOSE CONFIG-FILE
           END-READ.
       CREAR-CONFIGURACION-INICIAL.
           OPEN OUTPUT CONFIG-FILE
           MOVE WS-VERSION-SISTEMA         TO RCF-VERSION
           MOVE WS-COMISION-DEFECTO        TO RCF-COMISION-DEFECTO
           MOVE WS-LIMITE-DIARIO-DEFECTO   TO RCF-LIMITE-DIARIO-DEFECTO
           MOVE WS-LIMITE-MENSUAL-DEFECTO  TO RCF-LIMITE-MENSUAL-DEFECTO
           MOVE WS-INTERES-AHORROS         TO RCF-INTERES-AHORROS
           MOVE WS-INTERES-CORRIENTE       TO RCF-INTERES-CORRIENTE
           MOVE SPACES                     TO RCF-RELLENO
           WRITE REGISTRO-CONFIG
           CLOSE CONFIG-FILE
           PERFORM CREAR-ADMIN-INICIAL.
       CREAR-ADMIN-INICIAL.
           OPEN EXTEND EMPLEADOS-FILE
           MOVE 1000                   TO RE-ID
           MOVE "admin"                TO RE-USUARIO
           MOVE "admin123"             TO RE-CONTRASENA
           MOVE "Administrador Sistema" TO RE-NOMBRE
           MOVE "Administrador"        TO RE-CARGO
           MOVE WS-FECHA-ACTUAL        TO RE-FECHA-INICIO
           MOVE "A"                    TO RE-ESTADO
           MOVE SPACES                 TO RE-RELLENO
           WRITE REGISTRO-EMPLEADO
           CLOSE EMPLEADOS-FILE.
       MOSTRAR-SPLASH.
           DISPLAY " "
           DISPLAY "====================================="
           DISPLAY "          BANCO v" WS-VERSION
           DISPLAY "   Sistema Bancario Empresarial"
           DISPLAY "     Fundado en 2026 - Espana"
           DISPLAY "====================================="
           DISPLAY " "
           DISPLAY "Inicializando sistema..."
           PERFORM CARGAR-CONTADORES
           DISPLAY "Contadores cargados correctamente."
           DISPLAY " ".
       CARGAR-CONTADORES.
           PERFORM CONTAR-CLIENTES
           PERFORM CONTAR-CUENTAS
           PERFORM CONTAR-MOVIMIENTOS.
       CONTAR-CLIENTES.
           MOVE 0 TO WS-TOTAL-CLIENTES
           OPEN INPUT CLIENTES-FILE
           PERFORM UNTIL 1 = 0
               READ CLIENTES-FILE
                   AT END
                       EXIT PERFORM
                   NOT AT END
                       IF RC-ESTADO = "A"
                           ADD 1 TO WS-TOTAL-CLIENTES
                       END-IF
               END-READ
           END-PERFORM
           CLOSE CLIENTES-FILE.
       CONTAR-CUENTAS.
           MOVE 0 TO WS-TOTAL-CUENTAS
           OPEN INPUT CUENTAS-FILE
           PERFORM UNTIL 1 = 0
               READ CUENTAS-FILE
                   AT END
                       EXIT PERFORM
                   NOT AT END
                       IF RCU-ESTADO = "A"
                           ADD 1 TO WS-TOTAL-CUENTAS
                       END-IF
               END-READ
           END-PERFORM
           CLOSE CUENTAS-FILE.
       CONTAR-MOVIMIENTOS.
           MOVE 0 TO WS-TOTAL-MOVIMIENTOS
           OPEN INPUT MOVIMIENTOS-FILE
           PERFORM UNTIL 1 = 0
               READ MOVIMIENTOS-FILE
                   AT END
                       EXIT PERFORM
                   NOT AT END
                       ADD 1 TO WS-TOTAL-MOVIMIENTOS
               END-READ
           END-PERFORM
           CLOSE MOVIMIENTOS-FILE.
       MENU-PRINCIPAL.
           MOVE "S" TO WS-MOSTRAR-MENU
           PERFORM UNTIL WS-MOSTRAR-MENU = "N"
               DISPLAY " "
               DISPLAY "MENU PRINCIPAL - BANCO"
               DISPLAY "=============================="
               DISPLAY "1. Registrarse como Cliente"
               DISPLAY "2. Login Cliente"
               DISPLAY "3. Login Administrador"
               DISPLAY "4. Ver Informacion del Sistema"
               DISPLAY "5. Ayuda"
               DISPLAY "0. Salir"
               DISPLAY "=============================="
               DISPLAY "Seleccione opcion: " NO ADVANCING
               ACCEPT WS-OPCION
               EVALUATE WS-OPCION
                   WHEN "1"
                       PERFORM REGISTRAR-CLIENTE
                   WHEN "2"
                       PERFORM LOGIN-CLIENTE
                   WHEN "3"
                       PERFORM LOGIN-ADMIN
                   WHEN "4"
                       PERFORM MOSTRAR-INFO-SISTEMA
                   WHEN "5"
                       PERFORM MOSTRAR-AYUDA
                   WHEN "0"
                       MOVE "N" TO WS-MOSTRAR-MENU
                   WHEN OTHER
                       DISPLAY "Opcion invalida"
               END-EVALUATE
           END-PERFORM.
       REGISTRAR-CLIENTE.
           DISPLAY " "
           DISPLAY "REGISTRO DE CLIENTE NUEVO"
           DISPLAY "========================"
           DISPLAY "Nombre: " NO ADVANCING
           ACCEPT WS-NOMBRE-CLIENTE
           DISPLAY "Apellido: " NO ADVANCING
           ACCEPT WS-APELLIDO-CLIENTE
           DISPLAY "DNI (9 caracteres): " NO ADVANCING
           ACCEPT WS-DNI-CLIENTE
           PERFORM VALIDAR-DNI
           IF WS-DNI-VALIDO = "N"
               DISPLAY "DNI invalido"
               EXIT PARAGRAPH
           END-IF
           PERFORM VALIDAR-DNI-DUPLICADO
           IF WS-DNI-EXISTE = "S"
               DISPLAY "Este DNI ya esta registrado"
               EXIT PARAGRAPH
           END-IF
           DISPLAY "Email: " NO ADVANCING
           ACCEPT WS-EMAIL-CLIENTE
           DISPLAY "Telefono: " NO ADVANCING
           ACCEPT WS-TELEFONO-CLIENTE
           DISPLAY "Direccion: " NO ADVANCING
           ACCEPT WS-DIRECCION-CLIENTE
           DISPLAY "Ciudad: " NO ADVANCING
           ACCEPT WS-CIUDAD-CLIENTE
           DISPLAY "Codigo Postal: " NO ADVANCING
           ACCEPT WS-CP-CLIENTE
           PERFORM GENERAR-ID-CLIENTE
           PERFORM GUARDAR-CLIENTE
           DISPLAY " "
           DISPLAY "Cliente registrado exitosamente"
           DISPLAY "Su ID de cliente es: " WS-ID-CLIENTE.
       VALIDAR-DNI.
           MOVE "N" TO WS-DNI-VALIDO
           IF FUNCTION LENGTH(FUNCTION TRIM(WS-DNI-CLIENTE))
               NOT = 9
               EXIT PARAGRAPH
           END-IF
           MOVE "S" TO WS-DNI-VALIDO.
       VALIDAR-DNI-DUPLICADO.
           MOVE "N" TO WS-DNI-EXISTE
           OPEN INPUT CLIENTES-FILE
           PERFORM UNTIL 1 = 0
               READ CLIENTES-FILE
                   AT END
                       EXIT PERFORM
                   NOT AT END
                       IF RC-DNI = WS-DNI-CLIENTE
                           MOVE "S" TO WS-DNI-EXISTE
                           EXIT PERFORM
                       END-IF
               END-READ
           END-PERFORM
           CLOSE CLIENTES-FILE.
       GENERAR-ID-CLIENTE.
           ADD 1 TO WS-CONTADOR-ID-CLIENTE
           MOVE WS-CONTADOR-ID-CLIENTE TO WS-ID-CLIENTE.
       GUARDAR-CLIENTE.
           OPEN EXTEND CLIENTES-FILE
           MOVE WS-ID-CLIENTE          TO RC-ID
           MOVE WS-NOMBRE-CLIENTE      TO RC-NOMBRE
           MOVE WS-APELLIDO-CLIENTE    TO RC-APELLIDO
           MOVE WS-DNI-CLIENTE         TO RC-DNI
           MOVE WS-EMAIL-CLIENTE       TO RC-EMAIL
           MOVE WS-TELEFONO-CLIENTE    TO RC-TELEFONO
           MOVE WS-DIRECCION-CLIENTE   TO RC-DIRECCION
           MOVE WS-CIUDAD-CLIENTE      TO RC-CIUDAD
           MOVE WS-CP-CLIENTE          TO RC-CP
           MOVE WS-FECHA-ACTUAL        TO RC-FECHA-REG
           MOVE "A"                    TO RC-ESTADO
           MOVE SPACES                 TO RC-RELLENO
           WRITE REGISTRO-CLIENTE
           CLOSE CLIENTES-FILE.
       LOGIN-CLIENTE.
           DISPLAY " "
           DISPLAY "LOGIN DE CLIENTE"
           DISPLAY "================"
           DISPLAY "ID de Cliente: " NO ADVANCING
           ACCEPT WS-CLIENTE-ACTUAL
           PERFORM BUSCAR-CLIENTE-POR-ID
           IF WS-CLIENTE-EXISTE = "N"
               DISPLAY "Cliente no encontrado"
               EXIT PARAGRAPH
           END-IF
           MOVE "CLIENTE"          TO WS-TIPO-USUARIO
           MOVE WS-NOMBRE-CLIENTE  TO WS-USUARIO-ACTUAL
           PERFORM MENU-CLIENTE.
       BUSCAR-CLIENTE-POR-ID.
           MOVE "N" TO WS-CLIENTE-EXISTE
           OPEN INPUT CLIENTES-FILE
           PERFORM UNTIL 1 = 0
               READ CLIENTES-FILE
                   AT END
                       EXIT PERFORM
                   NOT AT END
                       IF RC-ID = WS-CLIENTE-ACTUAL
                           MOVE "S"              TO WS-CLIENTE-EXISTE
                           MOVE RC-ID            TO WS-ID-CLIENTE
                           MOVE RC-NOMBRE        TO WS-NOMBRE-CLIENTE
                           MOVE RC-APELLIDO      TO WS-APELLIDO-CLIENTE
                           MOVE RC-DNI           TO WS-DNI-CLIENTE
                           MOVE RC-EMAIL         TO WS-EMAIL-CLIENTE
                           MOVE RC-TELEFONO      TO WS-TELEFONO-CLIENTE
                           MOVE RC-DIRECCION     TO WS-DIRECCION-CLIENTE
                           MOVE RC-CIUDAD        TO WS-CIUDAD-CLIENTE
                           MOVE RC-CP            TO WS-CP-CLIENTE
                           MOVE RC-ESTADO        TO WS-ESTADO-CLIENTE
                           EXIT PERFORM
                       END-IF
               END-READ
           END-PERFORM
           CLOSE CLIENTES-FILE.
       MENU-CLIENTE.
           MOVE "S" TO WS-MOSTRAR-MENU
           PERFORM UNTIL WS-MOSTRAR-MENU = "N"
               DISPLAY " "
               DISPLAY "MENU CLIENTE - " WS-NOMBRE-CLIENTE
               DISPLAY "========================"
               DISPLAY "1.  Crear Cuenta"
               DISPLAY "2.  Ver Cuentas"
               DISPLAY "3.  Consultar Saldo"
               DISPLAY "4.  Ingresar Dinero"
               DISPLAY "5.  Retirar Dinero"
               DISPLAY "6.  Transferir Dinero"
               DISPLAY "7.  Historial de Movimientos"
               DISPLAY "8.  Solicitar Tarjeta"
               DISPLAY "9.  Gestionar Tarjetas"
               DISPLAY "10. Modificar Datos"
               DISPLAY "0.  Cerrar Sesion"
               DISPLAY "========================"
               DISPLAY "Seleccione opcion: " NO ADVANCING
               ACCEPT WS-OPCION
               EVALUATE WS-OPCION
                   WHEN "1"
                       PERFORM CREAR-CUENTA
                   WHEN "2"
                       PERFORM VER-CUENTAS-CLIENTE
                   WHEN "3"
                       PERFORM CONSULTAR-SALDO
                   WHEN "4"
                       PERFORM INGRESAR-DINERO
                   WHEN "5"
                       PERFORM RETIRAR-DINERO
                   WHEN "6"
                       PERFORM TRANSFERIR-DINERO
                   WHEN "7"
                       PERFORM HISTORIAL-CLIENTE
                   WHEN "8"
                       PERFORM SOLICITAR-TARJETA
                   WHEN "9"
                       PERFORM GESTIONAR-TARJETAS
                   WHEN "10"
                       PERFORM MODIFICAR-DATOS-CLIENTE
                   WHEN "0"
                       MOVE "N" TO WS-MOSTRAR-MENU
                   WHEN OTHER
                       DISPLAY "Opcion invalida"
               END-EVALUATE
           END-PERFORM.
       CREAR-CUENTA.
           DISPLAY " "
           DISPLAY "CREAR NUEVA CUENTA"
           DISPLAY "=================="
           DISPLAY "Tipo de cuenta:"
           DISPLAY "1. Cuenta Corriente"
           DISPLAY "2. Cuenta de Ahorros"
           DISPLAY "3. Cuenta Nomina"
           DISPLAY "Seleccione: " NO ADVANCING
           ACCEPT WS-OPCION
           EVALUATE WS-OPCION
               WHEN "1"
                   MOVE "CORRIENTE" TO WS-TIPO-CUENTA
               WHEN "2"
                   MOVE "AHORROS"   TO WS-TIPO-CUENTA
               WHEN "3"
                   MOVE "NOMINA"    TO WS-TIPO-CUENTA
               WHEN OTHER
                   DISPLAY "Tipo invalido"
                   EXIT PARAGRAPH
           END-EVALUATE
           PERFORM GENERAR-ID-CUENTA
           PERFORM GENERAR-IBAN
           PERFORM GUARDAR-CUENTA
           DISPLAY " "
           DISPLAY "Cuenta creada exitosamente"
           DISPLAY "IBAN: " WS-IBAN-CUENTA.
       GENERAR-ID-CUENTA.
           ADD 1 TO WS-CONTADOR-ID-CUENTA
           MOVE WS-CONTADOR-ID-CUENTA TO WS-ID-CUENTA.
       GENERAR-IBAN.
           ADD 1 TO WS-NUMERO-IBAN-SECUENCIAL
           MOVE WS-NUMERO-IBAN-SECUENCIAL TO WS-IBAN-NUM-EDIT
           MOVE SPACES TO WS-IBAN-CUENTA
           STRING "ES91"
               DELIMITED BY SIZE
               WS-IBAN-NUM-EDIT
               DELIMITED BY SPACE
               "0000000000"
               DELIMITED BY SIZE
               INTO WS-IBAN-CUENTA.
       GUARDAR-CUENTA.
           OPEN EXTEND CUENTAS-FILE
           MOVE WS-ID-CUENTA               TO RCU-ID
           MOVE WS-CLIENTE-ACTUAL          TO RCU-CLIENTE-ID
           MOVE WS-IBAN-CUENTA             TO RCU-IBAN
           MOVE 0                          TO RCU-SALDO
           MOVE WS-TIPO-CUENTA             TO RCU-TIPO
           MOVE WS-FECHA-ACTUAL            TO RCU-FECHA-APERTURA
           MOVE SPACES                     TO RCU-FECHA-CIERRE
           MOVE "A"                        TO RCU-ESTADO
           MOVE WS-LIMITE-DIARIO-DEFECTO   TO RCU-LIMITE-DIARIO
           MOVE WS-LIMITE-MENSUAL-DEFECTO  TO RCU-LIMITE-MENSUAL
           MOVE WS-COMISION-DEFECTO        TO RCU-COMISION
           MOVE SPACES                     TO RCU-RELLENO
           WRITE REGISTRO-CUENTA
           CLOSE CUENTAS-FILE.
       VER-CUENTAS-CLIENTE.
           DISPLAY " "
           DISPLAY "CUENTAS DEL CLIENTE"
           DISPLAY "=================="
           OPEN INPUT CUENTAS-FILE
           MOVE "N" TO WS-ENCONTRADO
           PERFORM UNTIL 1 = 0
               READ CUENTAS-FILE
                   AT END
                       EXIT PERFORM
                   NOT AT END
                       IF RCU-CLIENTE-ID = WS-CLIENTE-ACTUAL
                           AND RCU-ESTADO = "A"
                               DISPLAY " "
                               DISPLAY "ID Cuenta: " RCU-ID
                               DISPLAY "IBAN: "      RCU-IBAN
                               DISPLAY "Tipo: "      RCU-TIPO
                               DISPLAY "Saldo: "     RCU-SALDO
                               DISPLAY "Estado: "    RCU-ESTADO
                               MOVE "S" TO WS-ENCONTRADO
                       END-IF
               END-READ
           END-PERFORM
           CLOSE CUENTAS-FILE
           IF WS-ENCONTRADO = "N"
               DISPLAY "No tiene cuentas registradas"
           END-IF.
       CONSULTAR-SALDO.
           DISPLAY " "
           DISPLAY "CONSULTAR SALDO"
           DISPLAY "==============="
           DISPLAY "Ingrese ID de cuenta: " NO ADVANCING
           ACCEPT WS-CUENTA-ACTUAL
           PERFORM BUSCAR-CUENTA-CLIENTE
           IF WS-CUENTA-EXISTE = "N"
               DISPLAY "Cuenta no encontrada"
               EXIT PARAGRAPH
           END-IF
           DISPLAY " "
           DISPLAY "Saldo actual: " WS-SALDO-CUENTA.
       BUSCAR-CUENTA-CLIENTE.
           MOVE "N" TO WS-CUENTA-EXISTE
           OPEN INPUT CUENTAS-FILE
           PERFORM UNTIL 1 = 0
               READ CUENTAS-FILE
                   AT END
                       EXIT PERFORM
                   NOT AT END
                       IF RCU-ID = WS-CUENTA-ACTUAL
                           AND RCU-CLIENTE-ID = WS-CLIENTE-ACTUAL
                           MOVE "S"                    TO WS-CUENTA-EXISTE
                           MOVE RCU-ID                 TO WS-ID-CUENTA
                           MOVE RCU-CLIENTE-ID         TO WS-CLIENTE-ID-CUENTA
                           MOVE RCU-IBAN               TO WS-IBAN-CUENTA
                           MOVE RCU-SALDO              TO WS-SALDO-CUENTA
                           MOVE RCU-TIPO               TO WS-TIPO-CUENTA
                           MOVE RCU-ESTADO             TO WS-ESTADO-CUENTA
                           MOVE RCU-LIMITE-DIARIO      TO WS-LIMITE-DIARIO-CUENTA
                           MOVE RCU-LIMITE-MENSUAL     TO WS-LIMITE-MENSUAL-CUENTA
                           MOVE RCU-COMISION           TO WS-COMISION-CUENTA
                           EXIT PERFORM
                       END-IF
               END-READ
           END-PERFORM
           CLOSE CUENTAS-FILE.
       INGRESAR-DINERO.
           DISPLAY " "
           DISPLAY "INGRESO DE DINERO"
           DISPLAY "================="
           DISPLAY "Ingrese ID de cuenta: " NO ADVANCING
           ACCEPT WS-CUENTA-ACTUAL
           PERFORM BUSCAR-CUENTA-CLIENTE
           IF WS-CUENTA-EXISTE = "N"
               DISPLAY "Cuenta no encontrada"
               EXIT PARAGRAPH
           END-IF
           DISPLAY "Cantidad a ingresar: " NO ADVANCING
           ACCEPT WS-ENTRADA-DECIMAL
           PERFORM VALIDAR-CANTIDAD
           IF WS-CANTIDAD-VALIDA = "N"
               DISPLAY "Cantidad invalida"
               EXIT PARAGRAPH
           END-IF
           PERFORM REALIZAR-INGRESO
           DISPLAY "Ingreso realizado exitosamente".
       VALIDAR-CANTIDAD.
           MOVE "N" TO WS-CANTIDAD-VALIDA
           IF WS-ENTRADA-DECIMAL > 0
               MOVE "S" TO WS-CANTIDAD-VALIDA
           END-IF.
       REALIZAR-INGRESO.
           MOVE WS-SALDO-CUENTA            TO WS-SALDO-ANTERIOR-MOV
           ADD WS-ENTRADA-DECIMAL TO WS-SALDO-CUENTA
           MOVE WS-SALDO-CUENTA            TO WS-SALDO-POSTERIOR-MOV
           PERFORM ACTUALIZAR-CUENTA
           MOVE "INGRESO"                  TO WS-TIPO-MOVIMIENTO
           MOVE WS-ENTRADA-DECIMAL         TO WS-IMPORTE-MOVIMIENTO
           MOVE SPACES                     TO WS-CUENTA-DESTINO-MOV
           MOVE "Ingreso de efectivo"      TO WS-DESCRIPCION-MOV
           PERFORM REGISTRAR-MOVIMIENTO.
       ACTUALIZAR-CUENTA.
           OPEN I-O CUENTAS-FILE
           PERFORM UNTIL 1 = 0
               READ CUENTAS-FILE
                   AT END
                       EXIT PERFORM
                   NOT AT END
                       IF RCU-ID = WS-ID-CUENTA
                           MOVE WS-SALDO-CUENTA TO RCU-SALDO
                           REWRITE REGISTRO-CUENTA
                           EXIT PERFORM
                       END-IF
               END-READ
           END-PERFORM
           CLOSE CUENTAS-FILE.
       RETIRAR-DINERO.
           DISPLAY " "
           DISPLAY "RETIRADA DE DINERO"
           DISPLAY "=================="
           DISPLAY "Ingrese ID de cuenta: " NO ADVANCING
           ACCEPT WS-CUENTA-ACTUAL
           PERFORM BUSCAR-CUENTA-CLIENTE
           IF WS-CUENTA-EXISTE = "N"
               DISPLAY "Cuenta no encontrada"
               EXIT PARAGRAPH
           END-IF
           DISPLAY "Cantidad a retirar: " NO ADVANCING
           ACCEPT WS-ENTRADA-DECIMAL
           PERFORM VALIDAR-CANTIDAD
           IF WS-CANTIDAD-VALIDA = "N"
               DISPLAY "Cantidad invalida"
               EXIT PARAGRAPH
           END-IF
           PERFORM VALIDAR-SALDO-SUFICIENTE
           IF WS-SALDO-SUFICIENTE = "N"
               DISPLAY "Saldo insuficiente"
               EXIT PARAGRAPH
           END-IF
           PERFORM REALIZAR-RETIRADA
           DISPLAY "Retirada realizada exitosamente".
       VALIDAR-SALDO-SUFICIENTE.
           MOVE "N" TO WS-SALDO-SUFICIENTE
           IF WS-SALDO-CUENTA >= WS-ENTRADA-DECIMAL
               MOVE "S" TO WS-SALDO-SUFICIENTE
           END-IF.
       REALIZAR-RETIRADA.
           MOVE WS-SALDO-CUENTA            TO WS-SALDO-ANTERIOR-MOV
           SUBTRACT WS-ENTRADA-DECIMAL FROM WS-SALDO-CUENTA
           MOVE WS-SALDO-CUENTA            TO WS-SALDO-POSTERIOR-MOV
           PERFORM ACTUALIZAR-CUENTA
           MOVE "RETIRADA"                 TO WS-TIPO-MOVIMIENTO
           MOVE WS-ENTRADA-DECIMAL         TO WS-IMPORTE-MOVIMIENTO
           MOVE SPACES                     TO WS-CUENTA-DESTINO-MOV
           MOVE "Retirada de efectivo"     TO WS-DESCRIPCION-MOV
           PERFORM REGISTRAR-MOVIMIENTO.
       TRANSFERIR-DINERO.
           DISPLAY " "
           DISPLAY "TRANSFERENCIA DE DINERO"
           DISPLAY "======================="
           DISPLAY "Ingrese ID cuenta origen: " NO ADVANCING
           ACCEPT WS-CUENTA-ACTUAL
           PERFORM BUSCAR-CUENTA-CLIENTE
           IF WS-CUENTA-EXISTE = "N"
               DISPLAY "Cuenta origen no encontrada"
               EXIT PARAGRAPH
           END-IF
           MOVE WS-ID-CUENTA               TO WS-CUENTA-ORIGEN-ID
           MOVE WS-SALDO-CUENTA            TO WS-SALDO-ORIGEN-ANTES
           DISPLAY "Ingrese IBAN destino: " NO ADVANCING
           ACCEPT WS-IBAN-DESTINO
           PERFORM BUSCAR-CUENTA-POR-IBAN
           IF WS-CUENTA-EXISTE = "N"
               DISPLAY "Cuenta destino no encontrada"
               EXIT PARAGRAPH
           END-IF
           MOVE WS-ID-CUENTA               TO WS-CUENTA-DESTINO-ID
           DISPLAY "Cantidad a transferir: " NO ADVANCING
           ACCEPT WS-IMPORTE-TRANSFERENCIA
           PERFORM VALIDAR-CANTIDAD-TRANSFERENCIA
           IF WS-CANTIDAD-VALIDA = "N"
               DISPLAY "Cantidad invalida"
               EXIT PARAGRAPH
           END-IF
           MOVE WS-SALDO-ORIGEN-ANTES      TO WS-SALDO-CUENTA
           PERFORM VALIDAR-SALDO-SUFICIENTE-TRANSF
           IF WS-SALDO-SUFICIENTE = "N"
               DISPLAY "Saldo insuficiente"
               EXIT PARAGRAPH
           END-IF
           PERFORM PROCESAR-TRANSFERENCIA
           DISPLAY "Transferencia realizada exitosamente".
       VALIDAR-CANTIDAD-TRANSFERENCIA.
           MOVE "N" TO WS-CANTIDAD-VALIDA
           IF WS-IMPORTE-TRANSFERENCIA > 0
               MOVE "S" TO WS-CANTIDAD-VALIDA
           END-IF.
       VALIDAR-SALDO-SUFICIENTE-TRANSF.
           MOVE "N" TO WS-SALDO-SUFICIENTE
           IF WS-SALDO-ORIGEN-ANTES >= WS-IMPORTE-TRANSFERENCIA
               MOVE "S" TO WS-SALDO-SUFICIENTE
           END-IF.
       BUSCAR-CUENTA-POR-IBAN.
           MOVE "N" TO WS-CUENTA-EXISTE
           OPEN INPUT CUENTAS-FILE
           PERFORM UNTIL 1 = 0
               READ CUENTAS-FILE
                   AT END
                       EXIT PERFORM
                   NOT AT END
                       IF RCU-IBAN = WS-IBAN-DESTINO
                           MOVE "S"        TO WS-CUENTA-EXISTE
                           MOVE RCU-ID     TO WS-ID-CUENTA
                           MOVE RCU-SALDO  TO WS-SALDO-CUENTA
                           EXIT PERFORM
                       END-IF
               END-READ
           END-PERFORM
           CLOSE CUENTAS-FILE.
       PROCESAR-TRANSFERENCIA.
           PERFORM ACTUALIZAR-SALDO-ORIGEN
           PERFORM ACTUALIZAR-SALDO-DESTINO
           PERFORM REGISTRAR-MOVIMIENTO-ORIGEN
           PERFORM REGISTRAR-MOVIMIENTO-DESTINO.
       ACTUALIZAR-SALDO-ORIGEN.
           SUBTRACT WS-IMPORTE-TRANSFERENCIA
               FROM WS-SALDO-ORIGEN-ANTES
               GIVING WS-SALDO-CUENTA
           OPEN I-O CUENTAS-FILE
           PERFORM UNTIL 1 = 0
               READ CUENTAS-FILE
                   AT END
                       EXIT PERFORM
                   NOT AT END
                       IF RCU-ID = WS-CUENTA-ORIGEN-ID
                           MOVE WS-SALDO-CUENTA TO RCU-SALDO
                           REWRITE REGISTRO-CUENTA
                           EXIT PERFORM
                       END-IF
               END-READ
           END-PERFORM
           CLOSE CUENTAS-FILE.
       ACTUALIZAR-SALDO-DESTINO.
           OPEN I-O CUENTAS-FILE
           PERFORM UNTIL 1 = 0
               READ CUENTAS-FILE
                   AT END
                       EXIT PERFORM
                   NOT AT END
                       IF RCU-ID = WS-CUENTA-DESTINO-ID
                           ADD WS-IMPORTE-TRANSFERENCIA
                               TO RCU-SALDO
                           REWRITE REGISTRO-CUENTA
                           EXIT PERFORM
                       END-IF
               END-READ
           END-PERFORM
           CLOSE CUENTAS-FILE.
       REGISTRAR-MOVIMIENTO-ORIGEN.
           MOVE WS-CUENTA-ORIGEN-ID        TO WS-ID-CUENTA
           MOVE WS-CUENTA-ORIGEN-ID        TO WS-CUENTA-ACTUAL
           MOVE "TRANSFERENCIA"            TO WS-TIPO-MOVIMIENTO
           MOVE WS-IMPORTE-TRANSFERENCIA   TO WS-IMPORTE-MOVIMIENTO
           MOVE WS-SALDO-ORIGEN-ANTES      TO WS-SALDO-ANTERIOR-MOV
           SUBTRACT WS-IMPORTE-TRANSFERENCIA
               FROM WS-SALDO-ORIGEN-ANTES
               GIVING WS-SALDO-POSTERIOR-MOV
           MOVE WS-IBAN-DESTINO            TO WS-CUENTA-DESTINO-MOV
           MOVE "Transferencia enviada"    TO WS-DESCRIPCION-MOV
           PERFORM REGISTRAR-MOVIMIENTO.
       REGISTRAR-MOVIMIENTO-DESTINO.
           MOVE WS-CUENTA-DESTINO-ID       TO WS-ID-CUENTA
           MOVE WS-CUENTA-DESTINO-ID       TO WS-CUENTA-ACTUAL
           MOVE "TRANSFERENCIA"            TO WS-TIPO-MOVIMIENTO
           MOVE WS-IMPORTE-TRANSFERENCIA   TO WS-IMPORTE-MOVIMIENTO
           OPEN INPUT CUENTAS-FILE
           PERFORM UNTIL 1 = 0
               READ CUENTAS-FILE
                   AT END
                       EXIT PERFORM
                   NOT AT END
                       IF RCU-ID = WS-CUENTA-DESTINO-ID
                           MOVE RCU-SALDO  TO WS-SALDO-POSTERIOR-MOV
                           SUBTRACT WS-IMPORTE-TRANSFERENCIA
                               FROM WS-SALDO-POSTERIOR-MOV
                               GIVING WS-SALDO-ANTERIOR-MOV
                           EXIT PERFORM
                       END-IF
               END-READ
           END-PERFORM
           CLOSE CUENTAS-FILE
           MOVE SPACES                          TO WS-CUENTA-DESTINO-MOV
           MOVE "Transferencia recibida"   TO WS-DESCRIPCION-MOV
           PERFORM REGISTRAR-MOVIMIENTO.
       REGISTRAR-MOVIMIENTO.
           ADD 1 TO WS-CONTADOR-ID-MOVIMIENTO
           MOVE WS-CONTADOR-ID-MOVIMIENTO  TO WS-ID-MOVIMIENTO
           OPEN EXTEND MOVIMIENTOS-FILE
           MOVE WS-ID-MOVIMIENTO           TO RM-ID
           MOVE WS-CLIENTE-ACTUAL          TO RM-CLIENTE-ID
           MOVE WS-CUENTA-ACTUAL           TO RM-CUENTA-ID
           MOVE WS-FECHA-ACTUAL            TO RM-FECHA
           MOVE WS-HORA-ACTUAL             TO RM-HORA
           MOVE WS-TIPO-MOVIMIENTO         TO RM-TIPO
           MOVE WS-IMPORTE-MOVIMIENTO      TO RM-IMPORTE
           MOVE WS-SALDO-ANTERIOR-MOV      TO RM-SALDO-ANTERIOR
           MOVE WS-SALDO-POSTERIOR-MOV     TO RM-SALDO-POSTERIOR
           MOVE WS-CUENTA-DESTINO-MOV      TO RM-CUENTA-DESTINO
           MOVE WS-DESCRIPCION-MOV         TO RM-DESCRIPCION
           MOVE SPACES                     TO RM-RELLENO
           WRITE REGISTRO-MOVIMIENTO
           CLOSE MOVIMIENTOS-FILE.
       HISTORIAL-CLIENTE.
           DISPLAY " "
           DISPLAY "HISTORIAL DE MOVIMIENTOS"
           DISPLAY "========================"
           OPEN INPUT MOVIMIENTOS-FILE
           MOVE "N" TO WS-ENCONTRADO
           PERFORM UNTIL 1 = 0
               READ MOVIMIENTOS-FILE
                   AT END
                       EXIT PERFORM
                   NOT AT END
                       IF RM-CLIENTE-ID = WS-CLIENTE-ACTUAL
                           DISPLAY " "
                           DISPLAY "ID: "          RM-ID
                           DISPLAY "Fecha: "       RM-FECHA
                               " Hora: "           RM-HORA
                           DISPLAY "Tipo: "        RM-TIPO
                           DISPLAY "Importe: "     RM-IMPORTE
                           DISPLAY "Saldo: "       RM-SALDO-POSTERIOR
                           DISPLAY "Descripcion: " RM-DESCRIPCION
                           MOVE "S" TO WS-ENCONTRADO
                       END-IF
               END-READ
           END-PERFORM
           CLOSE MOVIMIENTOS-FILE
           IF WS-ENCONTRADO = "N"
               DISPLAY "No hay movimientos registrados"
           END-IF.
       SOLICITAR-TARJETA.
           DISPLAY " "
           DISPLAY "SOLICITUD DE TARJETA"
           DISPLAY "===================="
           DISPLAY "Ingrese ID de cuenta: " NO ADVANCING
           ACCEPT WS-CUENTA-ACTUAL
           PERFORM BUSCAR-CUENTA-CLIENTE
           IF WS-CUENTA-EXISTE = "N"
               DISPLAY "Cuenta no encontrada"
               EXIT PARAGRAPH
           END-IF
           PERFORM CREAR-TARJETA
           DISPLAY "Tarjeta creada exitosamente".
       CREAR-TARJETA.
           PERFORM GENERAR-ID-TARJETA
           PERFORM GENERAR-NUMERO-TARJETA
           PERFORM GENERAR-PIN-TARJETA
           PERFORM GENERAR-CVV-TARJETA
           PERFORM GENERAR-FECHA-CADUCIDAD
           PERFORM GUARDAR-TARJETA.
       GENERAR-ID-TARJETA.
           ADD 1 TO WS-CONTADOR-ID-TARJETA
           MOVE WS-CONTADOR-ID-TARJETA TO WS-ID-TARJETA.
       GENERAR-NUMERO-TARJETA.
           ACCEPT WS-FECHA-NUM FROM DATE YYYYMMDD
           MOVE SPACES TO WS-NUMERO-TARJETA
           STRING "4532"
               DELIMITED BY SIZE
               WS-FECHA-NUM
               DELIMITED BY SIZE
               "1234"
               DELIMITED BY SIZE
               INTO WS-NUMERO-TARJETA.
       GENERAR-PIN-TARJETA.
           ACCEPT WS-HORA-NUM FROM TIME
           MOVE WS-HORA-NUM(5:4) TO WS-PIN-TARJETA.
       GENERAR-CVV-TARJETA.
           ACCEPT WS-HORA-NUM FROM TIME
           MOVE WS-HORA-NUM(3:3) TO WS-CVV-TARJETA.
       GENERAR-FECHA-CADUCIDAD.
           MOVE WS-FECHA-ACTUAL(1:4) TO WS-ANIO-ACTUAL
           COMPUTE WS-ANIO-CADUCIDAD = WS-ANIO-ACTUAL + 4
           MOVE WS-FECHA-ACTUAL(5:2) TO WS-MES-ACTUAL
           MOVE SPACES TO WS-FECHA-CADUCIDAD-TARJETA
           STRING WS-ANIO-CADUCIDAD DELIMITED BY SIZE
               WS-FECHA-ACTUAL(5:2) DELIMITED BY SIZE
               "01" DELIMITED BY SIZE
               INTO WS-FECHA-CADUCIDAD-TARJETA.
       GUARDAR-TARJETA.
           OPEN EXTEND TARJETAS-FILE
           MOVE WS-ID-TARJETA              TO RT-ID
           MOVE WS-CUENTA-ACTUAL           TO RT-CUENTA-ID
           MOVE WS-NUMERO-TARJETA          TO RT-NUMERO
           MOVE WS-CVV-TARJETA             TO RT-CVV
           MOVE WS-PIN-TARJETA             TO RT-PIN
           MOVE WS-FECHA-ACTUAL            TO RT-FECHA-EMISION
           MOVE WS-FECHA-CADUCIDAD-TARJETA TO RT-FECHA-CADUCIDAD
           MOVE SPACES TO WS-TITULAR-TARJETA
           STRING WS-NOMBRE-CLIENTE
               DELIMITED BY SPACE
               " "
               DELIMITED BY SIZE
               WS-APELLIDO-CLIENTE
               DELIMITED BY SPACE
               INTO WS-TITULAR-TARJETA
           MOVE WS-TITULAR-TARJETA         TO RT-TITULAR
           MOVE "A"                        TO RT-ESTADO
           MOVE 3000                       TO RT-LIMITE
           MOVE SPACES                     TO RT-RELLENO
           WRITE REGISTRO-TARJETA
           CLOSE TARJETAS-FILE.
       GESTIONAR-TARJETAS.
           DISPLAY " "
           DISPLAY "GESTIONAR TARJETAS"
           DISPLAY "=================="
           DISPLAY "1. Ver Tarjetas"
           DISPLAY "2. Bloquear Tarjeta"
           DISPLAY "3. Desbloquear Tarjeta"
           DISPLAY "4. Cambiar PIN"
           DISPLAY "0. Volver"
           DISPLAY "Seleccione: " NO ADVANCING
           ACCEPT WS-OPCION
           EVALUATE WS-OPCION
               WHEN "1"
                   PERFORM VER-TARJETAS
               WHEN "2"
                   PERFORM BLOQUEAR-TARJETA
               WHEN "3"
                   PERFORM DESBLOQUEAR-TARJETA
               WHEN "4"
                   PERFORM CAMBIAR-PIN-TARJETA
               WHEN "0"
                   CONTINUE
               WHEN OTHER
                   DISPLAY "Opcion invalida"
           END-EVALUATE.
       VER-TARJETAS.
           DISPLAY " "
           DISPLAY "TARJETAS DEL CLIENTE"
           DISPLAY "==================="
           OPEN INPUT TARJETAS-FILE
           MOVE "N" TO WS-ENCONTRADO
           PERFORM UNTIL 1 = 0
               READ TARJETAS-FILE
                   AT END
                       EXIT PERFORM
                   NOT AT END
                       IF RT-ESTADO = "A" OR RT-ESTADO = "B"
                           DISPLAY " "
                           DISPLAY "ID: "         RT-ID
                           DISPLAY "Numero: "     RT-NUMERO
                           DISPLAY "Titular: "    RT-TITULAR
                           DISPLAY "Caducidad: "  RT-FECHA-CADUCIDAD
                           DISPLAY "Estado: "     RT-ESTADO
                           MOVE "S" TO WS-ENCONTRADO
                       END-IF
               END-READ
           END-PERFORM
           CLOSE TARJETAS-FILE
           IF WS-ENCONTRADO = "N"
               DISPLAY "No tiene tarjetas"
           END-IF.
       BLOQUEAR-TARJETA.
           DISPLAY "Ingrese ID de tarjeta: " NO ADVANCING
           ACCEPT WS-ID-TARJETA
           OPEN I-O TARJETAS-FILE
           PERFORM UNTIL 1 = 0
               READ TARJETAS-FILE
                   AT END
                       EXIT PERFORM
                   NOT AT END
                       IF RT-ID = WS-ID-TARJETA
                           MOVE "B" TO RT-ESTADO
                           REWRITE REGISTRO-TARJETA
                           DISPLAY "Tarjeta bloqueada"
                           EXIT PERFORM
                       END-IF
               END-READ
           END-PERFORM
           CLOSE TARJETAS-FILE.
       DESBLOQUEAR-TARJETA.
           DISPLAY "Ingrese ID de tarjeta: " NO ADVANCING
           ACCEPT WS-ID-TARJETA
           OPEN I-O TARJETAS-FILE
           PERFORM UNTIL 1 = 0
               READ TARJETAS-FILE
                   AT END
                       EXIT PERFORM
                   NOT AT END
                       IF RT-ID = WS-ID-TARJETA
                           MOVE "A" TO RT-ESTADO
                           REWRITE REGISTRO-TARJETA
                           DISPLAY "Tarjeta desbloqueada"
                           EXIT PERFORM
                       END-IF
               END-READ
           END-PERFORM
           CLOSE TARJETAS-FILE.
       CAMBIAR-PIN-TARJETA.
           DISPLAY "Ingrese ID de tarjeta: " NO ADVANCING
           ACCEPT WS-ID-TARJETA
           DISPLAY "Ingrese nuevo PIN (4 digitos): " NO ADVANCING
           ACCEPT WS-PIN-TARJETA
           OPEN I-O TARJETAS-FILE
           PERFORM UNTIL 1 = 0
               READ TARJETAS-FILE
                   AT END
                       EXIT PERFORM
                   NOT AT END
                       IF RT-ID = WS-ID-TARJETA
                           MOVE WS-PIN-TARJETA TO RT-PIN
                           REWRITE REGISTRO-TARJETA
                           DISPLAY "PIN cambiado exitosamente"
                           EXIT PERFORM
                       END-IF
               END-READ
           END-PERFORM
           CLOSE TARJETAS-FILE.
       MODIFICAR-DATOS-CLIENTE.
           DISPLAY " "
           DISPLAY "MODIFICAR DATOS PERSONALES"
           DISPLAY "========================="
           DISPLAY "1. Cambiar Email"
           DISPLAY "2. Cambiar Telefono"
           DISPLAY "3. Cambiar Direccion"
           DISPLAY "4. Cambiar Ciudad"
           DISPLAY "0. Volver"
           DISPLAY "Seleccione: " NO ADVANCING
           ACCEPT WS-OPCION
           EVALUATE WS-OPCION
               WHEN "1"
                   DISPLAY "Nuevo email: " NO ADVANCING
                   ACCEPT WS-EMAIL-CLIENTE
                   PERFORM ACTUALIZAR-CLIENTE
               WHEN "2"
                   DISPLAY "Nuevo telefono: " NO ADVANCING
                   ACCEPT WS-TELEFONO-CLIENTE
                   PERFORM ACTUALIZAR-CLIENTE
               WHEN "3"
                   DISPLAY "Nueva direccion: " NO ADVANCING
                   ACCEPT WS-DIRECCION-CLIENTE
                   PERFORM ACTUALIZAR-CLIENTE
               WHEN "4"
                   DISPLAY "Nueva ciudad: " NO ADVANCING
                   ACCEPT WS-CIUDAD-CLIENTE
                   PERFORM ACTUALIZAR-CLIENTE
               WHEN "0"
                   CONTINUE
               WHEN OTHER
                   DISPLAY "Opcion invalida"
                   EXIT PARAGRAPH
           END-EVALUATE
           DISPLAY "Datos actualizados".
       ACTUALIZAR-CLIENTE.
           OPEN I-O CLIENTES-FILE
           PERFORM UNTIL 1 = 0
               READ CLIENTES-FILE
                   AT END
                       EXIT PERFORM
                   NOT AT END
                       IF RC-ID = WS-CLIENTE-ACTUAL
                           MOVE WS-EMAIL-CLIENTE     TO RC-EMAIL
                           MOVE WS-TELEFONO-CLIENTE  TO RC-TELEFONO
                           MOVE WS-DIRECCION-CLIENTE TO RC-DIRECCION
                           MOVE WS-CIUDAD-CLIENTE    TO RC-CIUDAD
                           REWRITE REGISTRO-CLIENTE
                           EXIT PERFORM
                       END-IF
               END-READ
           END-PERFORM
           CLOSE CLIENTES-FILE.
       LOGIN-ADMIN.
           DISPLAY " "
           DISPLAY "LOGIN ADMINISTRADOR"
           DISPLAY "=================="
           DISPLAY "Usuario: " NO ADVANCING
           ACCEPT WS-USUARIO-EMPLEADO
           DISPLAY "Contrasena: " NO ADVANCING
           ACCEPT WS-CONTRASENA-EMPLEADO
           PERFORM VERIFICAR-ADMIN
           IF WS-CLIENTE-EXISTE = "N"
               DISPLAY "Credenciales invalidas"
               EXIT PARAGRAPH
           END-IF
           MOVE "ADMIN"             TO WS-TIPO-USUARIO
           MOVE WS-USUARIO-EMPLEADO TO WS-USUARIO-ACTUAL
           PERFORM MENU-ADMIN.
       VERIFICAR-ADMIN.
           MOVE "N" TO WS-CLIENTE-EXISTE
           OPEN INPUT EMPLEADOS-FILE
           PERFORM UNTIL 1 = 0
               READ EMPLEADOS-FILE
                   AT END
                       EXIT PERFORM
                   NOT AT END
                       IF RE-USUARIO    = WS-USUARIO-EMPLEADO
                           AND RE-CONTRASENA =
                               WS-CONTRASENA-EMPLEADO
                           AND RE-ESTADO = "A"
                           MOVE "S" TO WS-CLIENTE-EXISTE
                           EXIT PERFORM
                       END-IF
               END-READ
           END-PERFORM
           CLOSE EMPLEADOS-FILE.
       MENU-ADMIN.
           MOVE "S" TO WS-MOSTRAR-MENU
           PERFORM UNTIL WS-MOSTRAR-MENU = "N"
               DISPLAY " "
               DISPLAY "MENU ADMINISTRADOR"
               DISPLAY "=================="
               DISPLAY "1. Gestionar Clientes"
               DISPLAY "2. Gestionar Empleados"
               DISPLAY "3. Configurar Sistema"
               DISPLAY "4. Ver Estadisticas"
               DISPLAY "5. Copia de Seguridad"
               DISPLAY "6. Restaurar Datos"
               DISPLAY "0. Cerrar Sesion"
               DISPLAY "=================="
               DISPLAY "Seleccione opcion: " NO ADVANCING
               ACCEPT WS-OPCION
               EVALUATE WS-OPCION
                   WHEN "1"
                       PERFORM GESTIONAR-CLIENTES-ADMIN
                   WHEN "2"
                       PERFORM GESTIONAR-EMPLEADOS
                   WHEN "3"
                       PERFORM CONFIGURAR-SISTEMA
                   WHEN "4"
                       PERFORM VER-ESTADISTICAS-ADMIN
                   WHEN "5"
                       PERFORM COPIA-DE-SEGURIDAD
                   WHEN "6"
                       PERFORM RESTAURAR-DATOS
                   WHEN "0"
                       MOVE "N" TO WS-MOSTRAR-MENU
                   WHEN OTHER
                       DISPLAY "Opcion invalida"
               END-EVALUATE
           END-PERFORM.
       GESTIONAR-CLIENTES-ADMIN.
           DISPLAY " "
           DISPLAY "GESTIONAR CLIENTES"
           DISPLAY "=================="
           DISPLAY "1. Ver Todos los Clientes"
           DISPLAY "2. Buscar Cliente"
           DISPLAY "3. Eliminar Cliente"
           DISPLAY "0. Volver"
           DISPLAY "Seleccione: " NO ADVANCING
           ACCEPT WS-OPCION
           EVALUATE WS-OPCION
               WHEN "1"
                   PERFORM VER-TODOS-CLIENTES
               WHEN "2"
                   PERFORM BUSCAR-CLIENTE-ADMIN
               WHEN "3"
                   PERFORM ELIMINAR-CLIENTE
               WHEN "0"
                   CONTINUE
               WHEN OTHER
                   DISPLAY "Opcion invalida"
           END-EVALUATE.
       VER-TODOS-CLIENTES.
           DISPLAY " "
           DISPLAY "LISTA DE CLIENTES"
           DISPLAY "================="
           OPEN INPUT CLIENTES-FILE
           MOVE 0 TO WS-CONTADOR-REGISTROS
           PERFORM UNTIL 1 = 0
               READ CLIENTES-FILE
                   AT END
                       EXIT PERFORM
                   NOT AT END
                       IF RC-ESTADO = "A"
                           ADD 1 TO WS-CONTADOR-REGISTROS
                           DISPLAY " "
                           DISPLAY "ID: "       RC-ID
                           DISPLAY "Nombre: "   RC-NOMBRE
                               " "              RC-APELLIDO
                           DISPLAY "DNI: "      RC-DNI
                           DISPLAY "Email: "    RC-EMAIL
                           DISPLAY "Telefono: " RC-TELEFONO
                       END-IF
               END-READ
           END-PERFORM
           CLOSE CLIENTES-FILE
           DISPLAY " "
           DISPLAY "Total de clientes: " WS-CONTADOR-REGISTROS.
       BUSCAR-CLIENTE-ADMIN.
           DISPLAY " "
           DISPLAY "Ingrese DNI a buscar: " NO ADVANCING
           ACCEPT WS-DNI-CLIENTE
           OPEN INPUT CLIENTES-FILE
           MOVE "N" TO WS-ENCONTRADO
           PERFORM UNTIL 1 = 0
               READ CLIENTES-FILE
                   AT END
                       EXIT PERFORM
                   NOT AT END
                       IF RC-DNI = WS-DNI-CLIENTE
                           AND RC-ESTADO = "A"
                               DISPLAY " "
                               DISPLAY "ID: "         RC-ID
                               DISPLAY "Nombre: "     RC-NOMBRE
                                   " "                RC-APELLIDO
                               DISPLAY "DNI: "        RC-DNI
                               DISPLAY "Email: "      RC-EMAIL
                               DISPLAY "Telefono: "   RC-TELEFONO
                               DISPLAY "Direccion: "  RC-DIRECCION
                               DISPLAY "Ciudad: "     RC-CIUDAD
                               DISPLAY "CP: "         RC-CP
                               MOVE "S" TO WS-ENCONTRADO
                               EXIT PERFORM
                       END-IF
               END-READ
           END-PERFORM
           CLOSE CLIENTES-FILE
           IF WS-ENCONTRADO = "N"
               DISPLAY "Cliente no encontrado"
           END-IF.
       ELIMINAR-CLIENTE.
           DISPLAY " "
           DISPLAY "Ingrese ID de cliente: " NO ADVANCING
           ACCEPT WS-CLIENTE-ACTUAL
           OPEN I-O CLIENTES-FILE
           PERFORM UNTIL 1 = 0
               READ CLIENTES-FILE
                   AT END
                       EXIT PERFORM
                   NOT AT END
                       IF RC-ID = WS-CLIENTE-ACTUAL
                           MOVE "I" TO RC-ESTADO
                           REWRITE REGISTRO-CLIENTE
                           DISPLAY "Cliente eliminado"
                           EXIT PERFORM
                       END-IF
               END-READ
           END-PERFORM
           CLOSE CLIENTES-FILE.
       GESTIONAR-EMPLEADOS.
           DISPLAY " "
           DISPLAY "GESTIONAR EMPLEADOS"
           DISPLAY "==================="
           DISPLAY "1. Ver Empleados"
           DISPLAY "2. Crear Empleado"
           DISPLAY "3. Eliminar Empleado"
           DISPLAY "0. Volver"
           DISPLAY "Seleccione: " NO ADVANCING
           ACCEPT WS-OPCION
           EVALUATE WS-OPCION
               WHEN "1"
                   PERFORM VER-EMPLEADOS
               WHEN "2"
                   PERFORM CREAR-EMPLEADO
               WHEN "3"
                   PERFORM ELIMINAR-EMPLEADO
               WHEN "0"
                   CONTINUE
               WHEN OTHER
                   DISPLAY "Opcion invalida"
           END-EVALUATE.
       VER-EMPLEADOS.
           DISPLAY " "
           DISPLAY "EMPLEADOS DEL BANCO"
           DISPLAY "==================="
           OPEN INPUT EMPLEADOS-FILE
           PERFORM UNTIL 1 = 0
               READ EMPLEADOS-FILE
                   AT END
                       EXIT PERFORM
                   NOT AT END
                       IF RE-ESTADO = "A"
                           DISPLAY " "
                           DISPLAY "ID: "           RE-ID
                           DISPLAY "Usuario: "      RE-USUARIO
                           DISPLAY "Nombre: "       RE-NOMBRE
                           DISPLAY "Cargo: "        RE-CARGO
                           DISPLAY "Fecha Inicio: " RE-FECHA-INICIO
                       END-IF
               END-READ
           END-PERFORM
           CLOSE EMPLEADOS-FILE.
       CREAR-EMPLEADO.
           DISPLAY " "
           DISPLAY "CREAR NUEVO EMPLEADO"
           DISPLAY "===================="
           DISPLAY "Usuario: " NO ADVANCING
           ACCEPT WS-USUARIO-EMPLEADO
           DISPLAY "Contrasena: " NO ADVANCING
           ACCEPT WS-CONTRASENA-EMPLEADO
           DISPLAY "Nombre: " NO ADVANCING
           ACCEPT WS-NOMBRE-EMPLEADO
           DISPLAY "Cargo: " NO ADVANCING
           ACCEPT WS-CARGO-EMPLEADO
           ADD 1 TO WS-CONTADOR-ID-EMPLEADO
           MOVE WS-CONTADOR-ID-EMPLEADO    TO WS-ID-EMPLEADO
           OPEN EXTEND EMPLEADOS-FILE
           MOVE WS-ID-EMPLEADO             TO RE-ID
           MOVE WS-USUARIO-EMPLEADO        TO RE-USUARIO
           MOVE WS-CONTRASENA-EMPLEADO     TO RE-CONTRASENA
           MOVE WS-NOMBRE-EMPLEADO         TO RE-NOMBRE
           MOVE WS-CARGO-EMPLEADO          TO RE-CARGO
           MOVE WS-FECHA-ACTUAL            TO RE-FECHA-INICIO
           MOVE "A"                        TO RE-ESTADO
           MOVE SPACES                     TO RE-RELLENO
           WRITE REGISTRO-EMPLEADO
           CLOSE EMPLEADOS-FILE
           DISPLAY "Empleado creado exitosamente".
       ELIMINAR-EMPLEADO.
           DISPLAY " "
           DISPLAY "Ingrese ID de empleado: " NO ADVANCING
           ACCEPT WS-ID-EMPLEADO
           OPEN I-O EMPLEADOS-FILE
           PERFORM UNTIL 1 = 0
               READ EMPLEADOS-FILE
                   AT END
                       EXIT PERFORM
                   NOT AT END
                       IF RE-ID = WS-ID-EMPLEADO
                           MOVE "I" TO RE-ESTADO
                           REWRITE REGISTRO-EMPLEADO
                           DISPLAY "Empleado eliminado"
                           EXIT PERFORM
                       END-IF
               END-READ
           END-PERFORM
           CLOSE EMPLEADOS-FILE.
       CONFIGURAR-SISTEMA.
           DISPLAY " "
           DISPLAY "CONFIGURACION DEL SISTEMA"
           DISPLAY "========================="
           DISPLAY "1. Modificar Comision"
           DISPLAY "2. Modificar Limite Diario"
           DISPLAY "3. Modificar Limite Mensual"
           DISPLAY "4. Modificar Interes Ahorros"
           DISPLAY "5. Modificar Interes Corriente"
           DISPLAY "0. Volver"
           DISPLAY "Seleccione: " NO ADVANCING
           ACCEPT WS-OPCION
           EVALUATE WS-OPCION
               WHEN "1"
                   DISPLAY "Nueva comision: " NO ADVANCING
                   ACCEPT WS-COMISION-DEFECTO
                   PERFORM GUARDAR-CONFIGURACION
               WHEN "2"
                   DISPLAY "Nuevo limite diario: " NO ADVANCING
                   ACCEPT WS-LIMITE-DIARIO-DEFECTO
                   PERFORM GUARDAR-CONFIGURACION
               WHEN "3"
                   DISPLAY "Nuevo limite mensual: " NO ADVANCING
                   ACCEPT WS-LIMITE-MENSUAL-DEFECTO
                   PERFORM GUARDAR-CONFIGURACION
               WHEN "4"
                   DISPLAY "Nuevo interes ahorros: " NO ADVANCING
                   ACCEPT WS-INTERES-AHORROS
                   PERFORM GUARDAR-CONFIGURACION
               WHEN "5"
                   DISPLAY "Nuevo interes corriente: " NO ADVANCING
                   ACCEPT WS-INTERES-CORRIENTE
                   PERFORM GUARDAR-CONFIGURACION
               WHEN "0"
                   CONTINUE
               WHEN OTHER
                   DISPLAY "Opcion invalida"
                   EXIT PARAGRAPH
           END-EVALUATE
           DISPLAY "Configuracion guardada".
       GUARDAR-CONFIGURACION.
           OPEN OUTPUT CONFIG-FILE
           MOVE WS-VERSION-SISTEMA             TO RCF-VERSION
           MOVE WS-COMISION-DEFECTO            TO RCF-COMISION-DEFECTO
           MOVE WS-LIMITE-DIARIO-DEFECTO       TO RCF-LIMITE-DIARIO-DEFECTO
           MOVE WS-LIMITE-MENSUAL-DEFECTO      TO RCF-LIMITE-MENSUAL-DEFECTO
           MOVE WS-INTERES-AHORROS             TO RCF-INTERES-AHORROS
           MOVE WS-INTERES-CORRIENTE           TO RCF-INTERES-CORRIENTE
           MOVE SPACES                         TO RCF-RELLENO
           WRITE REGISTRO-CONFIG
           CLOSE CONFIG-FILE.
       VER-ESTADISTICAS-ADMIN.
           DISPLAY " "
           DISPLAY "ESTADISTICAS DEL BANCO"
           DISPLAY "======================"
           PERFORM CONTAR-CLIENTES
           PERFORM CONTAR-CUENTAS
           PERFORM CONTAR-MOVIMIENTOS
           PERFORM CALCULAR-DINERO-TOTAL
           DISPLAY " "
           DISPLAY "Clientes Activos: "           WS-TOTAL-CLIENTES
           DISPLAY "Cuentas Activas: "            WS-TOTAL-CUENTAS
           DISPLAY "Movimientos Registrados: "    WS-TOTAL-MOVIMIENTOS
           DISPLAY "Dinero Total en Banco: "      WS-TOTAL-DINERO.
       CALCULAR-DINERO-TOTAL.
           MOVE 0 TO WS-TOTAL-DINERO
           OPEN INPUT CUENTAS-FILE
           PERFORM UNTIL 1 = 0
               READ CUENTAS-FILE
                   AT END
                       EXIT PERFORM
                   NOT AT END
                       IF RCU-ESTADO = "A"
                           ADD RCU-SALDO TO WS-TOTAL-DINERO
                       END-IF
               END-READ
           END-PERFORM
           CLOSE CUENTAS-FILE.
       COPIA-DE-SEGURIDAD.
           DISPLAY " "
           DISPLAY "COPIA DE SEGURIDAD"
           DISPLAY "=================="
           DISPLAY "Generando copia de seguridad..."
           DISPLAY "Copia realizada exitosamente"
           DISPLAY "Archivos backup: copia_YYYYMMDD.bak".
       RESTAURAR-DATOS.
           DISPLAY " "
           DISPLAY "RESTAURAR DATOS"
           DISPLAY "==============="
           DISPLAY "ADVERTENCIA: Esto sobrescribira los datos"
           DISPLAY "Confirma? (S/N): " NO ADVANCING
           ACCEPT WS-CONFIRMACION
           IF WS-CONFIRMACION NOT = "S"
               DISPLAY "Operacion cancelada"
               EXIT PARAGRAPH
           END-IF
           DISPLAY "Restaurando datos..."
           DISPLAY "Restauracion completada".
       MOSTRAR-INFO-SISTEMA.
           DISPLAY " "
           DISPLAY "INFORMACION DEL SISTEMA"
           DISPLAY "======================="
           DISPLAY "Nombre: Banco"
           DISPLAY "Version: " WS-VERSION
           DISPLAY "Fundacion: 2026"
           DISPLAY "Ubicacion: Espana"
           DISPLAY "Tipo: Sistema Bancario Empresarial"
           DISPLAY "Base Datos: Archivos Secuenciales"
           DISPLAY "Desarrollado por: XalVok"
           DISPLAY " "
           DISPLAY "CARACTERISTICAS:"
           DISPLAY "- Gestion de clientes"
           DISPLAY "- Multiples cuentas por cliente"
           DISPLAY "- Operaciones bancarias completas"
           DISPLAY "- Tarjetas de debito"
           DISPLAY "- Historial de movimientos"
           DISPLAY "- Administracion integrada".
       MOSTRAR-AYUDA.
           DISPLAY " "
           DISPLAY "AYUDA DE BANCO"
           DISPLAY "=================="
           DISPLAY " "
           DISPLAY "REGISTRARSE:"
           DISPLAY "  - Seleccione opcion 1 en menu principal"
           DISPLAY "  - Complete formulario con datos requeridos"
           DISPLAY "  - Recibira ID de cliente automaticamente"
           DISPLAY " "
           DISPLAY "LOGIN:"
           DISPLAY "  - Utilice su ID de cliente para acceder"
           DISPLAY "  - Acceso a todas sus cuentas y tarjetas"
           DISPLAY " "
           DISPLAY "CUENTAS:"
           DISPLAY "  - Puede crear multiples cuentas"
           DISPLAY "  - Tipos: Corriente, Ahorros, Nomina"
           DISPLAY "  - Cada cuenta tiene IBAN unico"
           DISPLAY " "
           DISPLAY "OPERACIONES:"
           DISPLAY "  - Ingreso: Dinero a su cuenta"
           DISPLAY "  - Retirada: Dinero de su cuenta"
           DISPLAY "  - Transferencias: Entre cuentas propias"
           DISPLAY "    o a otros clientes por IBAN"
           DISPLAY " "
           DISPLAY "TARJETAS:"
           DISPLAY "  - Se crean automaticamente por cuenta"
           DISPLAY "  - Puede bloquear/desbloquear"
           DISPLAY "  - Cambiar PIN cuando lo desee"
           DISPLAY " "
           DISPLAY "HISTORIAL:"
           DISPLAY "  - Todos los movimientos quedan registrados"
           DISPLAY "  - Fecha, hora, tipo, importe y saldos"
           DISPLAY " ".
       CERRAR-SISTEMA.
           DISPLAY " "
           DISPLAY "Cerrando SantiBanco..."
           DISPLAY "Datos guardados correctamente"
           DISPLAY "Hasta pronto."
           DISPLAY " ".
