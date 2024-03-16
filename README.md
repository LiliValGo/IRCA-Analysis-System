# IRCA - Water Quality Classification Program in Rust

## Introduction
This program implements the IRCA (Índice de Riesgo de Calidad del Agua) water quality classification system as defined by Colombian [RESOLUCION 2115 DE 2007](https://www.udea.edu.co/wps/wcm/connect/udea/c46bea38-2c19-4942-8b74-6475d1a36625/Resoluci%C3%B3n+2115+de+2007.pdf?MOD=AJPERES). The IRCA is an indicator that evaluates the quality of water for human consumption based on the results of physicochemical and microbiological characteristics.

## Implementation
The program is implemented in Rust, a modern systems programming language. It uses the following libraries:

std::io: For user input and output
std::str: For parsing user input

## Usage
To use the program, simply input the value of the water quality feature you wish to classify. The program will then output the corresponding IRCA classification and risk level.
```IRCA CLASIFICATION

Please input the value feature: 12.5

You guessed: 12.5

LOW RISK 


