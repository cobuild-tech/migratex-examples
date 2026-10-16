\# 🏦 Banco\_en\_COBOL



> \*"¿Aprender COBOL haciendo un `Hola Mundo`? Mejor aprendámoslo montando un banco entero."\*



!\[COBOL](https://img.shields.io/badge/Hecho%20con-COBOL-blue)

!\[GnuCOBOL](https://img.shields.io/badge/Compilador-GnuCOBOL-green)

!\[Open Source](https://img.shields.io/badge/Open%20Source-Sí-brightgreen)

!\[Licencia](https://img.shields.io/badge/Licencia-AGPL-blue)

!\[Estado](https://img.shields.io/badge/Estado-Sigue%20funcionando-success)



\---



\# ¡Hola! 👋



Bienvenido a \*\*Banco\_en\_COBOL\*\*.



Sí.



Es exactamente lo que parece.



Un banco escrito completamente en \*\*COBOL\*\*.



Y sí... probablemente había formas más fáciles de aprender este lenguaje.



Pero... ¿dónde está la gracia? 😅



\---



\# ¿Qué es esto?



Un sistema bancario de consola donde puedes hacer prácticamente todo lo que esperarías de un banco "pequeño":



\* Crear usuarios

\* Iniciar sesión

\* Crear cuentas bancarias

\* Ingresar dinero

\* Sacar dinero

\* Hacer transferencias

\* Ver el historial de movimientos

\* Administrar usuarios

\* Administrar empleados

\* Administrar administradores



Todo eso...



En COBOL.



\---



\# ¿Por qué hice esto?



Porque empecé aprendiendo COBOL y pensé:



> "Voy a hacer un programita sencillo."



Cinco minutos después:



> "Bueno... un sistema de login tampoco es tan difícil."



Una semana después:



> "¿Y si añado cuentas bancarias?"



Dos meses y medio más tarde...



Tenía un banco entero. 😂



\---



\# ¿Es un banco de verdad?



No.



No metas aquí tus ahorros.



No intentes comprar una casa con él.



No llames al soporte porque no existe.



Es simplemente un proyecto educativo para aprender COBOL.



\---



\# Lo que puedes hacer



✅ Registrarte



✅ Iniciar sesión



✅ Crear cuentas



✅ Ingresar dinero



✅ Retirar dinero



✅ Transferir dinero



✅ Consultar movimientos



✅ Administrar usuarios



✅ Administrar empleados



✅ Administrar administradores



Y todo se guarda automáticamente.



No hace falta volver a empezar cada vez que abres el programa.



\---



\# Lo que todavía NO puedes hacer



Todavía faltan algunas cosas:



❌ Interfaz gráfica



❌ Servidor



❌ Multijugador (aunque un banco multijugador suena curioso 😂)



❌ Conexión por Internet



❌ Importar datos



❌ Exportar datos



Pero ya llegarán.



(O eso espero.)



\---



\# ¿Qué necesitas?



Muy poco.



\* GnuCOBOL

\* Los archivos del proyecto

\* Las DLL

\* Los archivos `.dat`



Eso sí...



No borres archivos porque "parecen importantes".



Spoiler:



Lo son.



\---



\# Compilar



Tan sencillo como:



```bash

cobc -x banco.cob

```



¿Quieres otro nombre?



```bash

cobc -x banco.cob -o MiBanco.exe

```



Y listo.



\---



\# Ejecutar



Doble clic.



O consola.



Lo que prefieras.



Todo el programa funciona mediante texto.



Nada de ventanas.



Todo al estilo clásico.



\---



\# Usuario administrador



Si quieres entrar como administrador:



\*\*Usuario\*\*



```

admin

```



\*\*Contraseña\*\*



```

admin123

```



(No la olvides... aunque siempre puedes mirar aquí otra vez 😆)



\---



\# ¿Cómo está organizado?



Nada demasiado complicado.



```

Banco\_en\_COBOL/



│

├── banco.cob

├── \*.dll

├── \*.dat

├── README.md

└── Librerías/

```



Solo hay un archivo principal.



Sí.



\*\*Uno solo.\*\*



Y mide más de 1.500 líneas.



\---



\# Los archivos `.dat`



Estos cinco archivos son básicamente la memoria del banco.



Ahí se guarda todo.



Usuarios.



Cuentas.



Transferencias.



Y demás información.



Si borras uno...



El banco se enfada.



Y deja de funcionar.



\---



\# Algunas estadísticas



| Dato                 |            Valor |

| -------------------- | ---------------: |

| Lenguaje             |            COBOL |

| Líneas de código     |            1.583 |

| Archivos fuente      |                1 |

| Archivos `.dat`      |                5 |

| Tiempo de desarrollo | \~2 meses y medio |

| Personas trabajando  |                1 |

| Cafés consumidos     |     Demasiados ☕ |



\---



\# ¿Por qué COBOL?



Porque todo el mundo dice:



> "COBOL está muerto."



Mientras tanto...



\* Bancos usando COBOL.

\* Gobiernos usando COBOL.

\* Seguros usando COBOL.



Así que decidí aprenderlo.



Y ya que estaba...



Construí un banco.



\---



\# Cosas curiosas



Este proyecto me enseñó que:



\* COBOL es mucho más divertido de lo que parece.

\* Los archivos pueden hacer de base de datos.

\* Depurar COBOL puede convertirse en un deporte extremo.

\* 1.500 líneas pasan muy rápido cuando sigues añadiendo funciones.



\---



\# Próximamente...



La idea es seguir mejorándolo.



Entre otras cosas:



\* Interfaz gráfica 🖥️

\* Conexión por red 🌐

\* Cliente/Servidor

\* Mejor organización del código

\* Más documentación

\* Versión 2.0



\---



\# Preguntas que probablemente nadie hizo



\### ¿Funciona?



Sí.



Sorprendentemente.



\---



\### ¿Tiene bugs?



Que yo haya visto...



No.



Pero si encuentras alguno, avísame.



\---



\### ¿Es rápido?



Sí.



COBOL lleva décadas moviendo bancos de verdad.



\---



\### ¿Puedo modificar el código?



Claro.



Para eso es Open Source.



\---



\### ¿Usa SQL?



No.



Todo va con archivos.



A la vieja escuela.



\---



\### ¿Necesita Internet?



No.



Puedes usarlo completamente sin conexión.



\---



\### ¿Aprenderé COBOL?



Ese era el objetivo desde el principio.



\---



\# ¿Quieres ayudar?



Perfecto.



Puedes:



\* Encontrar errores.

\* Mejorar la documentación.

\* Añadir comentarios.

\* Optimizar el código.

\* Proponer nuevas funciones.



Toda ayuda es bienvenida.



\---



\# Créditos



Hecho por \*\*Siaro\*\*



GitHub:



https://github.com/Siaro01



Gracias a:



\* GnuCOBOL

\* La comunidad Open Source

\* Stack Overflow (como siempre 😅)

\* El café ☕

\* Y a todos los que siguen programando en un lenguaje de más de 60 años.



\---



\# Últimas palabras



Este proyecto empezó siendo una simple excusa para aprender COBOL.



Después se convirtió en un reto personal.



Y acabó siendo un banco completamente funcional escrito desde cero.



Puede que no sea el proyecto más moderno del mundo.



Puede que no tenga una interfaz espectacular.



Pero demuestra algo bastante curioso:



\*\*COBOL sigue siendo perfectamente capaz de construir aplicaciones completas.\*\*



Y, además...



Es bastante divertido cuando le das una oportunidad.



Si este proyecto consigue que alguien pierda el miedo a COBOL o simplemente despierte su curiosidad por este lenguaje, entonces habrá merecido totalmente la pena.



¡Gracias por pasarte por aquí! ❤️



