       IDENTIFICATION DIVISION.
       PROGRAM-ID. ACCEPTNUM.
      * Reads each input line once per numeric field type that
      * banco.cob accepts, and prints what ACCEPT stored. Used to
      * check the Rust parser against GnuCOBOL (see capture-golden.sh).
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01 WS-IMPORTE    PIC 9(12)V99.
       01 WS-ID-CUENTA  PIC 9(10).
       01 WS-ID-CLIENTE PIC 9(8).
       01 WS-COMISION   PIC 9(3)V999.
       01 WS-LINEAS     PIC 9(6).
       PROCEDURE DIVISION.
       MAIN.
           ACCEPT WS-LINEAS
           PERFORM WS-LINEAS TIMES
               ACCEPT WS-IMPORTE
               ACCEPT WS-ID-CUENTA
               ACCEPT WS-ID-CLIENTE
               ACCEPT WS-COMISION
               DISPLAY WS-IMPORTE "|" WS-ID-CUENTA "|" WS-ID-CLIENTE
                   "|" WS-COMISION
           END-PERFORM
           STOP RUN.
