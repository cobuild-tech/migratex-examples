       IDENTIFICATION DIVISION.
       PROGRAM-ID. INTERESES.
      * Monthly interest posting for the Banco data files.
      * Not part of the upstream app: added for the MigrateX example
      * so the migration covers a batch job as well as the menus.
       ENVIRONMENT DIVISION.
       INPUT-OUTPUT SECTION.
       FILE-CONTROL.
           SELECT CUENTAS-FILE ASSIGN TO "cuentas.dat"
               ORGANIZATION IS SEQUENTIAL.
           SELECT MOVIMIENTOS-FILE ASSIGN TO "movimientos.dat"
               ORGANIZATION IS SEQUENTIAL.
           SELECT CONFIG-FILE ASSIGN TO "config.dat"
               ORGANIZATION IS SEQUENTIAL.
       DATA DIVISION.
       FILE SECTION.
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
       WORKING-STORAGE SECTION.
       01 WS-FECHA-ACTUAL          PIC X(8).
       01 WS-HORA-ACTUAL           PIC X(8).
       01 WS-INTERES-AHORROS       PIC 9(3)V999 VALUE 2.5.
       01 WS-INTERES-CORRIENTE     PIC 9(3)V999 VALUE 0.5.
       01 WS-TASA                  PIC 9(3)V999.
       01 WS-INTERES               PIC 9(12)V99.
       01 WS-SALDO-ANTERIOR        PIC 9(12)V99.
       01 WS-CONTADOR-ID-MOVIMIENTO PIC 9(12) VALUE 1000000.
       01 WS-CUENTAS-LEIDAS        PIC 9(10) VALUE 0.
       01 WS-CUENTAS-ABONADAS      PIC 9(10) VALUE 0.
       01 WS-TOTAL-INTERESES       PIC 9(15)V99 VALUE 0.
       PROCEDURE DIVISION.
       MAIN.
           ACCEPT WS-FECHA-ACTUAL FROM DATE YYYYMMDD
           ACCEPT WS-HORA-ACTUAL FROM TIME
           PERFORM CARGAR-TASAS
           PERFORM CARGAR-ULTIMO-MOVIMIENTO
           DISPLAY "====================================="
           DISPLAY "   ABONO MENSUAL DE INTERESES"
           DISPLAY "====================================="
           DISPLAY "Fecha: " WS-FECHA-ACTUAL
           DISPLAY "Interes ahorros (%): "   WS-INTERES-AHORROS
           DISPLAY "Interes corriente (%): " WS-INTERES-CORRIENTE
           DISPLAY " "
           PERFORM ABONAR-INTERESES
           DISPLAY " "
           DISPLAY "Cuentas leidas: "   WS-CUENTAS-LEIDAS
           DISPLAY "Cuentas abonadas: " WS-CUENTAS-ABONADAS
           DISPLAY "Total intereses: "  WS-TOTAL-INTERESES
           STOP RUN.
       CARGAR-TASAS.
           OPEN INPUT CONFIG-FILE
           READ CONFIG-FILE
               NOT AT END
                   MOVE RCF-INTERES-AHORROS   TO WS-INTERES-AHORROS
                   MOVE RCF-INTERES-CORRIENTE TO WS-INTERES-CORRIENTE
           END-READ
           CLOSE CONFIG-FILE.
       CARGAR-ULTIMO-MOVIMIENTO.
           OPEN INPUT MOVIMIENTOS-FILE
           PERFORM UNTIL 1 = 0
               READ MOVIMIENTOS-FILE
                   AT END
                       EXIT PERFORM
                   NOT AT END
                       IF RM-ID > WS-CONTADOR-ID-MOVIMIENTO
                           MOVE RM-ID TO WS-CONTADOR-ID-MOVIMIENTO
                       END-IF
               END-READ
           END-PERFORM
           CLOSE MOVIMIENTOS-FILE.
       ABONAR-INTERESES.
           OPEN I-O CUENTAS-FILE
           OPEN EXTEND MOVIMIENTOS-FILE
           PERFORM UNTIL 1 = 0
               READ CUENTAS-FILE
                   AT END
                       EXIT PERFORM
                   NOT AT END
                       ADD 1 TO WS-CUENTAS-LEIDAS
                       IF RCU-ESTADO = "A"
                           PERFORM ABONAR-CUENTA
                       END-IF
               END-READ
           END-PERFORM
           CLOSE MOVIMIENTOS-FILE
           CLOSE CUENTAS-FILE.
       ABONAR-CUENTA.
           EVALUATE RCU-TIPO
               WHEN "AHORROS"
                   MOVE WS-INTERES-AHORROS   TO WS-TASA
               WHEN "CORRIENTE"
                   MOVE WS-INTERES-CORRIENTE TO WS-TASA
               WHEN OTHER
                   MOVE 0                    TO WS-TASA
           END-EVALUATE
      * No ROUNDED: COBOL cuts the extra decimals off.
           COMPUTE WS-INTERES = RCU-SALDO * WS-TASA / 100 / 12
           IF WS-INTERES > 0
               MOVE RCU-SALDO        TO WS-SALDO-ANTERIOR
               ADD WS-INTERES        TO RCU-SALDO
               REWRITE REGISTRO-CUENTA
               ADD 1                 TO WS-CUENTAS-ABONADAS
               ADD WS-INTERES        TO WS-TOTAL-INTERESES
               ADD 1                 TO WS-CONTADOR-ID-MOVIMIENTO
               MOVE WS-CONTADOR-ID-MOVIMIENTO TO RM-ID
               MOVE RCU-CLIENTE-ID   TO RM-CLIENTE-ID
               MOVE RCU-ID           TO RM-CUENTA-ID
               MOVE WS-FECHA-ACTUAL  TO RM-FECHA
               MOVE WS-HORA-ACTUAL   TO RM-HORA
               MOVE "INTERES"        TO RM-TIPO
               MOVE WS-INTERES       TO RM-IMPORTE
               MOVE WS-SALDO-ANTERIOR TO RM-SALDO-ANTERIOR
               MOVE RCU-SALDO        TO RM-SALDO-POSTERIOR
               MOVE SPACES           TO RM-CUENTA-DESTINO
               MOVE "Abono de intereses" TO RM-DESCRIPCION
               MOVE SPACES           TO RM-RELLENO
               WRITE REGISTRO-MOVIMIENTO
               DISPLAY "Cuenta " RCU-ID " " RCU-TIPO
                   " Saldo: " WS-SALDO-ANTERIOR
                   " Interes: " WS-INTERES
           END-IF.
