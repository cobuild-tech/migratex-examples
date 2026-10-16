\# 🏦 Banco\_en\_COBOL



> "Because why learn COBOL with a calculator when you can learn it by running a bank?"



!\[COBOL](https://img.shields.io/badge/Made%20with-COBOL-blue)

!\[GnuCOBOL](https://img.shields.io/badge/Compiler-GnuCOBOL-green)

!\[Open Source](https://img.shields.io/badge/Open%20Source-Yes-brightgreen)

!\[License](https://img.shields.io/badge/License-AGPL-blue)

!\[Status](https://img.shields.io/badge/Still%20alive-Yes-success)



\---



\# Welcome!



Hi! 👋



This is \*\*Banco\_en\_COBOL\*\*, a banking system written completely in COBOL.



No SQL.



No fancy frameworks.



No cloud.



Just good old COBOL, data files and a lot of patience.



The whole project started because I wanted to learn COBOL... and then I thought:



> "What if I just build an entire bank?"



So... that's exactly what happened.



\---



\# What's inside?



Quite a lot actually.



You can:



\* Create users

\* Login

\* Create bank accounts

\* Deposit money

\* Withdraw money

\* Transfer money

\* View transaction history

\* Manage employees

\* Manage administrators

\* Save everything permanently



Basically...



It's a tiny bank running inside your terminal.



\---



\# Why?



Most COBOL repositories look like this:



```cobol

DISPLAY "HELLO WORLD".

STOP RUN.

```



...or maybe they calculate someone's salary.



That's cool...



But I wanted something bigger.



Something that actually feels like a real application.



So I built this.



\---



\# Is this a real bank?



Please don't try to store your life savings here.



This is an educational project.



No internet.



No servers.



No million-dollar transactions.



Just learning.



\---



\# Features



✔ User registration



✔ Login



✔ Multiple bank accounts



✔ Deposits



✔ Withdrawals



✔ Transfers



✔ Employee management



✔ Administrator management



✔ Persistent data



✔ Console menus



✔ Open Source



\---



\# What it doesn't have (yet)



Not yet, anyway.



\* GUI

\* Online banking

\* Multiplayer (yes, I know that's not how banks work 😂)

\* Server

\* Network support

\* Import/export



Maybe in Version 2.



\---



\# Requirements



You'll need:



\* GnuCOBOL

\* The project files

\* The DLLs

\* The `.dat` files



Don't delete random files.



Seriously.



The program needs them.



\---



\# Compile



Easy.



```bash

cobc -x banco.cob

```



Want another executable name?



```bash

cobc -x banco.cob -o AwesomeBank.exe

```



Done.



\---



\# Run



Double click.



Or use the terminal.



Whatever makes you happier.



\---



\# Admin Account



Default credentials:



Username



```

admin

```



Password



```

admin123

```



Maybe change them.



Or don't.



It's your computer.



\---



\# Project Structure



```

Banco\_en\_COBOL/



│

├── banco.cob

├── \*.dat

├── \*.dll

├── README.md

└── Libraries/

```



Simple.



No 700 nested folders.



No 900 dependencies.



\---



\# The Data Files



There are \*\*five\*\* data files.



They're basically the project's database.



Delete one...



...and the bank forgets how to be a bank.



So don't.



\---



\# How big is it?



Some stats:



| Thing            |          Value |

| ---------------- | -------------: |

| Lines of code    |          1,583 |

| Source files     |              1 |

| Data files       |              5 |

| Development time |    \~2.5 months |

| Developers       |              1 |

| Motivation       | Way too much ☕ |



\---



\# Why COBOL?



Because everyone says:



> "COBOL is dead."



Meanwhile...



Banks still use it.



Governments still use it.



Insurance companies still use it.



So I wanted to see what building something from scratch actually feels like.



Turns out...



It's fun.



\---



\# Funny facts



This project taught me:



\* How file storage works.

\* Why debugging old languages is... interesting.

\* That reading 1,500+ lines of COBOL is possible.

\* That semicolons aren't always the problem.



\---



\# Future Plans



Maybe someday...



\* GUI

\* Networking

\* Real client/server support

\* Better menus

\* Better documentation

\* Version 2.0



We'll see.



\---



\# FAQ



\### Is this production ready?



Absolutely not.



\---



\### Can I learn COBOL from it?



That's literally why it exists.



\---



\### Can I modify it?



Of course.



It's open source.



\---



\### Can I break it?



Probably.



Please open an Issue if you do.



\---



\### Why only one `.cob` file?



Because I kept adding features...



...and somehow it reached 1,583 lines before I noticed.



\---



\### Does it use SQL?



No.



Old-school data files.



Just like the classics.



\---



\### Is COBOL actually fun?



I wasn't expecting it...



But yes.



\---



\# If you find bugs...



Congratulations.



You found something I missed.



Open an Issue and I'll take a look.



\---



\# Contributing



Pull Requests?



Sure.



Bug reports?



Absolutely.



Ideas?



Always welcome.



Even if it's just:



> "Hey, you could organize this code a little better."



You're probably right.



\---



\# Credits



Made by \*\*Siaro\*\*



GitHub:



https://github.com/Siaro01



Special thanks to:



\* GnuCOBOL

\* Everyone keeping COBOL alive

\* Coffee ☕

\* Stack Overflow

\* Whoever decided COBOL should still exist in 2026



\---



\# Final Thoughts



This started as:



> "I'll learn a little COBOL."



Then became:



> "I'll make a login system."



Then:



> "Maybe bank accounts."



Then:



> "Transfers would be cool..."



Two and a half months later...



...I somehow had an entire banking simulator.



If you're here because you're curious about COBOL, I hope this project helps.



If you're here because you think COBOL is boring...



Give it a chance.



It might surprise you.



And if nothing else...



You'll at least have built a bank in a programming language older than the Moon landing.



