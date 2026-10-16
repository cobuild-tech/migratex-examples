\# 🏦 Banco\_en\_COBOL



!\[License](https://img.shields.io/badge/License-AGPL-blue.svg)

!\[COBOL](https://img.shields.io/badge/Language-COBOL-blue)

!\[Compiler](https://img.shields.io/badge/Compiler-GnuCOBOL-green)

!\[Status](https://img.shields.io/badge/Status-Stable-success)

!\[Version](https://img.shields.io/badge/Version-v1.0.0-orange)

!\[Open Source](https://img.shields.io/badge/Open%20Source-Yes-brightgreen)



An educational banking management system written entirely in \*\*COBOL\*\*, developed to demonstrate that modern, structured applications can still be built using one of the oldest programming languages still in use today.



This project is intended for students, developers, and anyone interested in learning COBOL through a practical example.



\---



\# Table of Contents



\* Introduction

\* Features

\* Why this project?

\* Technologies

\* Requirements

\* Installation

\* Compilation

\* Running the program

\* Administrator Account

\* Project Structure

\* Data Files

\* How it Works

\* Available Operations

\* Screenshots

\* Roadmap

\* Limitations

\* FAQ

\* Contributing

\* Version History

\* Project Statistics

\* License

\* Author

\* Acknowledgements



\---



\# Introduction



Banco\_en\_COBOL is a complete banking simulation developed as an educational open-source project.



Unlike small COBOL examples that usually demonstrate only isolated concepts, this repository aims to provide a larger and more realistic application featuring authentication, account management, transactions, persistent storage and administration tools.



The primary objective is \*\*learning\*\*.



Anyone interested in COBOL can inspect the source code, understand how the program works internally, modify it, and use it as a foundation for their own projects.



\---



\# Features



Current functionality includes:



\* User registration

\* User authentication

\* Administrator authentication

\* Bank account creation

\* Deposits

\* Withdrawals

\* Transfers

\* Transaction history

\* User management

\* Employee management

\* Administrator management

\* Persistent data storage

\* Console interface

\* File-based database

\* Multiple data files

\* Open Source



Not currently available:



\* Network support

\* Client/server architecture

\* Database server

\* GUI interface

\* Importing data

\* Exporting data



\---



\# Why this project?



Many COBOL repositories available online are either:



\* Very small

\* Academic exercises

\* Simple examples

\* Incomplete



This project attempts to provide something closer to a real application while remaining understandable for beginners.



It was also developed as a learning experience, proving that large console applications can still be created entirely in COBOL.



\---



\# Technologies



| Component | Value             |

| --------- | ----------------- |

| Language  | COBOL             |

| Compiler  | GnuCOBOL          |

| Version   | 1.0.0             |

| Interface | Console           |

| Database  | File-based (.dat) |

| License   | GNU AGPL          |

| Platform  | Windows (tested)  |



Linux and macOS compatibility has not yet been verified.



\---



\# Requirements



To compile the project you need:



\* GnuCOBOL

\* A C compiler supported by GnuCOBOL

\* The project source code

\* All supplied libraries

\* All included `.dat` files



\---



\# Installation



Clone the repository.



```bash

git clone https://github.com/Mel_Roses/Banco_en_COBOL.git

```



Open the project folder.



Make sure every file remains inside the same directory.



Do not remove any library or data file.



\---



\# Compilation



Compile using:



```bash

cobc -x banco.cob

```



To specify another executable name:



```bash

cobc -x banco.cob -o MyBank.exe

```



\---



\# Running



Run from Windows Explorer:



```

Banco.exe

```



or



```

MyBank.exe

```



You can also execute it directly from the terminal.



\---



\# Administrator Account



Default administrator credentials:



Username



```

admin

```



Password



```

admin123

```



Normal users can be created from inside the application.



\---



\# Project Structure



Example:



```

Banco\_en\_COBOL/



│

├── banco.cob

├── README.md

├── \*.dll

├── \*.dat

│

└── Libraries

```



\---



\# Data Files



The project stores information using \*\*five persistent data files\*\*.



These files contain application data such as:



\* Users

\* Accounts

\* Transactions

\* Employees

\* Other internal information



\*\*Important\*\*



These files are required.



Deleting any of them will prevent the program from working correctly.



\---



\# Available Operations



Users can:



\* Register

\* Log in

\* Create accounts

\* Deposit money

\* Withdraw money

\* Transfer funds

\* View transaction history



Administrators can:



\* Manage users

\* Manage employees

\* Manage administrators

\* Supervise stored information



\---



\# Internal Design



Instead of using SQL or NoSQL databases, this project stores all information directly inside binary data files.



This approach follows the traditional way many historical COBOL systems handled persistent information.



Each operation updates the corresponding data file immediately.



\---



\# Educational Purpose



This repository demonstrates concepts such as:



\* File handling

\* Record management

\* Authentication

\* Console menus

\* Modular programming

\* Data persistence

\* Banking logic

\* Structured programming



It is intended as a reference project for people beginning their COBOL journey.



\---



\# Screenshots



Example placeholders.



```

\[ Main Menu Screenshot ]



```



```

\[ Login Screen Screenshot ]



```



```

\[ Banking Menu Screenshot ]



```



```

\[ Transaction Example ]



```



\---



\# Roadmap



Future planned improvements include:



\* Graphical interface

\* Network support

\* Multi-user environment

\* Client/server communication

\* Better file organization

\* Improved documentation

\* Version 2.0



\---



\# Limitations



Current limitations include:



\* Windows is the only tested platform.

\* No graphical interface.

\* No online banking.

\* No server support.

\* Import/export functionality is unavailable.

\* All users are stored locally.



\---



\# FAQ



\## Is this a real banking system?



No.



It is an educational project.



\---



\## Is it open source?



Yes.



The project is licensed under the GNU Affero General Public License.



\---



\## Can I modify it?



Yes.



You are free to study, modify and redistribute it according to the AGPL license.



\---



\## Does it use SQL?



No.



It stores information using COBOL data files.



\---



\## Is it beginner friendly?



Yes.



The source code was written with learning in mind.



\---



\## Does it require internet?



No.



Everything runs locally.



\---



\# Project Statistics



Approximate statistics.



| Item             | Value       |

| ---------------- | ----------- |

| Language         | COBOL       |

| Source Files     | 1           |

| Data Files       | 5           |

| Lines of Code    | 1,583       |

| Size             | \~10.7 MB    |

| Development Time | \~2.5 Months |

| Developers       | 1           |



\---



\# Version History



\## v1.0.0



\* Initial public release

\* Registration system

\* Login system

\* Account management

\* Transactions

\* Administration

\* Persistent storage



\---



\# Contributing



Contributions are welcome.



Possible ways to contribute:



\* Report bugs

\* Improve documentation

\* Refactor code

\* Optimize performance

\* Add comments

\* Suggest new banking features



\---



\# Learning COBOL



If you are learning COBOL, this project may help you understand:



\* Program structure

\* Divisions

\* Sections

\* Paragraphs

\* File handling

\* Sequential records

\* Working-Storage usage

\* Business-oriented programming



Studying complete projects is often easier than reading isolated code snippets.



\---



\# License



This project is licensed under the \*\*GNU Affero General Public License (AGPL)\*\*.



See the LICENSE file for more information.



\---



\# Author



Developed by \*\*Siaro\*\*



GitHub:



https://github.com/Siaro01



\---



\# Acknowledgements



Thanks to:



\* The GnuCOBOL community

\* Everyone keeping COBOL alive

\* Open-source contributors

\* Anyone interested in learning classic programming languages



\---



\# Final Notes



Banco\_en\_COBOL began as a personal learning project and gradually evolved into a complete educational banking simulation.



Although it is not intended for production environments, it demonstrates that COBOL remains capable of supporting structured applications with persistent storage, authentication, account management and administrative tools.



If this repository helps someone learn COBOL or inspires them to build their own project, then it has achieved its purpose.



