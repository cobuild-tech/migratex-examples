\# 🏦 Banco\_en\_COBOL



!\[Licencia](https://img.shields.io/badge/Licencia-AGPL-blue.svg)

!\[Lenguaje](https://img.shields.io/badge/Lenguaje-COBOL-blue)

!\[Compilador](https://img.shields.io/badge/Compilador-GnuCOBOL-green)

!\[Estado](https://img.shields.io/badge/Estado-Estable-success)

!\[Versión](https://img.shields.io/badge/Versión-v1.0.0-orange)

!\[Open Source](https://img.shields.io/badge/Open%20Source-Sí-brightgreen)



\# Banco\_en\_COBOL



\*\*Banco\_en\_COBOL\*\* es un sistema de gestión bancaria desarrollado íntegramente en \*\*COBOL\*\* con fines educativos. El proyecto demuestra cómo es posible construir una aplicación relativamente completa utilizando uno de los lenguajes de programación más importantes de la historia de la informática.



Su principal objetivo es servir como referencia para estudiantes, desarrolladores y cualquier persona interesada en aprender COBOL mediante un proyecto real y de código abierto.



\---



\# Índice



\* Introducción

\* Objetivos

\* Características

\* Tecnologías utilizadas

\* Requisitos

\* Instalación

\* Compilación

\* Ejecución

\* Cuenta de administrador

\* Funcionalidades

\* Estructura del proyecto

\* Archivos de datos

\* Funcionamiento interno

\* Estadísticas

\* Capturas

\* Hoja de ruta

\* Limitaciones

\* Preguntas frecuentes

\* Contribuciones

\* Historial de versiones

\* Licencia

\* Autor

\* Agradecimientos



\---



\# Introducción



La mayoría de ejemplos de COBOL disponibles en Internet consisten en pequeños programas académicos o ejercicios básicos.



Este proyecto busca ofrecer algo diferente: una aplicación de mayor tamaño que permita estudiar la organización de un programa real, la gestión de archivos, la persistencia de datos, la autenticación de usuarios y la lógica de negocio utilizada en un sistema bancario simplificado.



Aunque no está pensado para uso en producción, el código ha sido desarrollado intentando mantener una estructura clara y fácil de comprender para quienes están aprendiendo el lenguaje.



\---



\# Objetivos



Los principales objetivos del proyecto son:



\* Facilitar el aprendizaje de COBOL mediante un proyecto práctico.

\* Mostrar cómo estructurar una aplicación de tamaño medio.

\* Servir como ejemplo de programación orientada al tratamiento de datos.

\* Demostrar el uso de archivos persistentes sin necesidad de bases de datos SQL.

\* Fomentar el aprendizaje de tecnologías clásicas que continúan utilizándose en numerosos sectores.



\---



\# Características



Actualmente el sistema incorpora las siguientes funciones:



\## Gestión de usuarios



\* Registro de usuarios.

\* Inicio de sesión.

\* Gestión de cuentas.



\## Gestión bancaria



\* Creación de cuentas bancarias.

\* Ingreso de dinero.

\* Retirada de fondos.

\* Transferencias entre cuentas.

\* Historial de movimientos.



\## Administración



\* Gestión de empleados.

\* Gestión de administradores.

\* Administración de usuarios.



\## Persistencia



\* Almacenamiento permanente de datos.

\* Uso de archivos `.dat`.

\* Conservación automática de la información entre ejecuciones.



\---



\# Tecnologías utilizadas



| Componente         | Valor           |

| ------------------ | --------------- |

| Lenguaje           | COBOL           |

| Compilador         | GnuCOBOL        |

| Versión            | 1.0.0           |

| Interfaz           | Consola         |

| Base de datos      | Archivos `.dat` |

| Licencia           | GNU AGPL        |

| Plataforma probada | Windows         |



La compatibilidad con Linux y macOS todavía no ha sido verificada.



\---



\# Requisitos



Para utilizar el proyecto es necesario disponer de:



\* GnuCOBOL instalado.

\* Todas las bibliotecas incluidas en el repositorio.

\* Todos los archivos `.dat`.

\* El código fuente completo.



Es importante mantener todos los archivos dentro de la misma carpeta.



\---



\# Instalación



Clonar el repositorio:



```bash

git clone https://github.com/Siaro01/Banco\_en\_COBOL.git

```



Entrar en la carpeta del proyecto y verificar que todos los archivos se encuentran presentes.



\---



\# Compilación



Compilar el proyecto:



```bash

cobc -x banco.cob

```



Compilar con otro nombre para el ejecutable:



```bash

cobc -x banco.cob -o MiBanco.exe

```



\---



\# Ejecución



El programa puede iniciarse:



\* Mediante doble clic sobre el ejecutable.

\* Desde la consola del sistema.



Toda la interacción se realiza mediante una interfaz de texto.



\---



\# Cuenta de administrador



El sistema incluye una cuenta administrativa predeterminada.



\*\*Usuario\*\*



```

admin

```



\*\*Contraseña\*\*



```

admin123

```



Los usuarios normales pueden crearse desde el propio programa.



\---



\# Funcionalidades



El proyecto permite realizar las siguientes operaciones:



\* Registro de usuarios.

\* Inicio de sesión.

\* Gestión de cuentas.

\* Creación de cuentas bancarias.

\* Depósitos.

\* Retiros.

\* Transferencias.

\* Consulta del historial.

\* Gestión de empleados.

\* Gestión de administradores.

\* Gestión de usuarios.



\---



\# Estructura del proyecto



```

Banco\_en\_COBOL/



│

├── banco.cob

├── README.md

├── \*.dll

├── \*.dat

└── Librerías/

```



El proyecto está formado principalmente por:



\* Un único archivo fuente COBOL.

\* Cinco archivos de datos.

\* Bibliotecas necesarias para la ejecución.

\* Documentación.



\---



\# Archivos de datos



La información se almacena utilizando cinco archivos `.dat`.



Estos archivos contienen la información persistente del sistema, incluyendo:



\* Usuarios.

\* Cuentas.

\* Transacciones.

\* Empleados.

\* Información interna del programa.



\*\*Importante\*\*



Eliminar cualquiera de estos archivos impedirá el funcionamiento correcto del sistema.



\---



\# Funcionamiento interno



A diferencia de aplicaciones modernas que utilizan bases de datos SQL o NoSQL, este proyecto almacena toda la información directamente en archivos de datos.



Cada operación realizada por el usuario actualiza inmediatamente la información correspondiente, manteniendo la persistencia entre ejecuciones.



Este enfoque reproduce la forma tradicional de trabajar de numerosos sistemas desarrollados originalmente en COBOL.



\---



\# Finalidad educativa



Este proyecto permite estudiar conceptos como:



\* Organización de programas COBOL.

\* Manejo de archivos.

\* Definición de registros.

\* Persistencia de información.

\* Menús de consola.

\* Autenticación.

\* Programación estructurada.

\* Lógica bancaria.



\---



\# Estadísticas del proyecto



| Concepto             |            Valor |

| -------------------- | ---------------: |

| Lenguaje             |            COBOL |

| Archivos fuente      |                1 |

| Archivos de datos    |                5 |

| Líneas de código     |            1.583 |

| Tamaño               |         \~10,7 MB |

| Tiempo de desarrollo | \~2 meses y medio |

| Desarrolladores      |                1 |



\---



\# Capturas



Espacio reservado para futuras capturas de pantalla.



```

\[ Menú principal ]

```



```

\[ Inicio de sesión ]

```



```

\[ Menú bancario ]

```



```

\[ Historial de transacciones ]

```



\---



\# Hoja de ruta



Entre las mejoras previstas se encuentran:



\* Interfaz gráfica.

\* Comunicación por red.

\* Arquitectura cliente-servidor.

\* Compatibilidad con múltiples equipos.

\* Mejor organización del código.

\* Documentación ampliada.

\* Versión 2.0.



\---



\# Limitaciones



Actualmente el proyecto presenta las siguientes limitaciones:



\* Solo se ha probado en Windows.

\* No dispone de interfaz gráfica.

\* No permite conexiones mediante red.

\* No funciona como servidor.

\* No incluye importación ni exportación de datos.

\* Todo el almacenamiento es local.



\---



\# Preguntas frecuentes



\## ¿Es un banco real?



No.



Es un proyecto educativo.



\---



\## ¿Puedo utilizarlo para aprender COBOL?



Sí.



Ese es precisamente su objetivo principal.



\---



\## ¿Puedo modificar el código?



Sí.



El proyecto es de código abierto y puede modificarse respetando los términos de la licencia AGPL.



\---



\## ¿Utiliza bases de datos SQL?



No.



Toda la información se almacena mediante archivos de datos propios.



\---



\## ¿Necesita conexión a Internet?



No.



Todo funciona completamente de forma local.



\---



\## ¿Es recomendable para principiantes?



Sí.



El proyecto fue desarrollado pensando en facilitar el aprendizaje del lenguaje.



\---



\# Contribuciones



Las contribuciones son bienvenidas.



Puedes colaborar de distintas maneras:



\* Reportando errores.

\* Mejorando la documentación.

\* Optimizando el código.

\* Añadiendo comentarios.

\* Proponiendo nuevas funcionalidades.

\* Corrigiendo errores existentes.



\---



\# Historial de versiones



\## Versión 1.0.0



\* Sistema de registro.

\* Sistema de autenticación.

\* Gestión bancaria.

\* Gestión administrativa.

\* Persistencia mediante archivos.

\* Primera versión pública.



\---



\# Licencia



Este proyecto se distribuye bajo la licencia \*\*GNU Affero General Public License (AGPL)\*\*.



Consulta el archivo `LICENSE` para obtener información detallada sobre las condiciones de uso.



\---



\# Autor



\*\*Siaro\*\*



GitHub:



https://github.com/Siaro01



\---



\# Agradecimientos



Mi agradecimiento a:



\* La comunidad de GnuCOBOL.

\* Todos los desarrolladores que mantienen vivo COBOL.

\* La comunidad de software libre.

\* Las personas interesadas en aprender lenguajes clásicos.



\---



\# Consideraciones finales



Banco\_en\_COBOL nació como un proyecto personal de aprendizaje y evolucionó hasta convertirse en una simulación bancaria funcional escrita íntegramente en COBOL.



Aunque no pretende sustituir soluciones profesionales, demuestra que este lenguaje continúa siendo perfectamente válido para desarrollar aplicaciones estructuradas con almacenamiento persistente, autenticación, gestión de datos y lógica de negocio.



Si este proyecto ayuda a otra persona a iniciarse en COBOL o le sirve como inspiración para desarrollar sus propias aplicaciones, habrá cumplido plenamente su propósito.



