#!/usr/bin/env python3
"""Generate bahilizator.kicad_sch in valid KiCad 10 format.

Each symbol MUST have:
1. lib_symbols entry with pins
2. symbol instance with (unit 1), pin uuids, properties
"""
import uuid

def u():
    return str(uuid.uuid4())

G = 2.54  # grid

# Symbol definitions: name -> {properties, pins}
# pin: (name, number, type, direction, x_offset, y_offset)
SYMBOLS = {
    "Device:R": {
        "ref_prefix": "R",
        "pins": [
            ("1", "1", "passive", 0, 3.81, 90),
            ("2", "2", "passive", 0, -3.81, 270),
        ]
    },
    "Device:C_Small": {
        "ref_prefix": "C",
        "pins": [
            ("1", "1", "passive", 0, 2.54, 90),
            ("2", "2", "passive", 0, -2.54, 270),
        ]
    },
    "Device:L_Small": {
        "ref_prefix": "L",
        "pins": [
            ("1", "1", "passive", 0, 2.54, 90),
            ("2", "2", "passive", 0, -2.54, 270),
        ]
    },
    "Device:Crystal": {
        "ref_prefix": "Y",
        "pins": [
            ("1", "1", "passive", -3.81, 0, 0),
            ("2", "2", "passive", 3.81, 0, 180),
        ]
    },
    "Device:LED": {
        "ref_prefix": "D",
        "pins": [
            ("A", "1", "passive", 0, 3.81, 90),
            ("K", "2", "passive", 0, -3.81, 270),
        ]
    },
    "Transistor_BJT:BC547": {
        "ref_prefix": "Q",
        "pins": [
            ("B", "1", "input", -2.54, 0, 0),
            ("E", "2", "passive", 2.54, 2.54, 0),
            ("C", "3", "passive", 2.54, -2.54, 0),
        ]
    },
    "Interface_Expansion:PCF8574P": {
        "ref_prefix": "U",
        "pins": [
            ("A0","1","input",0,-7.62,270),("A1","2","input",-2.54,-7.62,270),
            ("A2","3","input",-5.08,-7.62,270),("P0","4","bidirectional",7.62,0,0),
            ("P1","5","bidirectional",7.62,-2.54,0),("P2","6","bidirectional",7.62,-5.08,0),
            ("P3","7","bidirectional",7.62,-7.62,0),("P4","8","bidirectional",7.62,-10.16,0),
            ("P5","9","bidirectional",7.62,-12.7,0),("P6","10","bidirectional",7.62,-15.24,0),
            ("P7","11","bidirectional",7.62,-17.78,0),
            ("SCL","12","input",-7.62,-7.62,270),("SDA","13","bidirectional",-7.62,-5.08,270),
            ("INT","14","output",-7.62,-2.54,270),
            ("VDD","15","power_in",5.08,-7.62,270),("GND","16","power_in",5.08,7.62,90),
        ]
    },
    "Memory_EEPROM:AT24CS02-SSHM": {
        "ref_prefix": "U",
        "pins": [
            ("A0","1","input",0,-7.62,270),("A1","2","input",-2.54,-7.62,270),
            ("A2","3","input",-5.08,-7.62,270),("GND","4","power_in",-7.62,0,270),
            ("SDA","5","bidirectional",7.62,0,0),("SCL","6","input",7.62,-2.54,0),
            ("WP","7","input",0,7.62,90),("VCC","8","power_in",-7.62,7.62,90),
        ]
    },
    "MCU_ST_STM32F1:STM32F103C8Tx": {
        "ref_prefix": "U",
        "pins": [
            ("VBAT","1","power_in",-10.16,-27.94,0),("PC13","2","bidirectional",-10.16,-25.4,0),
            ("PC14","3","bidirectional",-10.16,-22.86,0),("PC15","4","bidirectional",-10.16,-20.32,0),
            ("PD0","5","bidirectional",-10.16,-17.78,0),("PD1","6","bidirectional",-10.16,-15.24,0),
            ("NRST","7","input",-10.16,-12.7,0),("VSSA","8","power_in",-10.16,-10.16,0),
            ("VDDA","9","power_in",-10.16,-7.62,0),("PA0","10","bidirectional",-10.16,-5.08,0),
            ("PA1","11","bidirectional",-10.16,-2.54,0),("PA2","12","bidirectional",-10.16,0,0),
            ("PA3","13","bidirectional",-10.16,2.54,0),("PA4","14","bidirectional",-10.16,5.08,0),
            ("PA5","15","bidirectional",-10.16,7.62,0),("PA6","16","bidirectional",-10.16,10.16,0),
            ("PA7","17","bidirectional",-10.16,12.7,0),("PB0","18","bidirectional",-10.16,15.24,0),
            ("PB1","19","bidirectional",-10.16,17.78,0),("PB2","20","bidirectional",-10.16,20.32,0),
            ("PB10","21","bidirectional",-10.16,22.86,0),("PB11","22","bidirectional",-10.16,25.4,0),
            ("VSS","23","power_in",-10.16,27.94,0),
            ("PA8","29","bidirectional",10.16,-27.94,0),("PA9","30","bidirectional",10.16,-25.4,0),
            ("PA10","31","bidirectional",10.16,-22.86,0),("PA11","32","bidirectional",10.16,-20.32,0),
            ("PA12","33","bidirectional",10.16,-17.78,0),("PA13","34","bidirectional",10.16,-15.24,0),
            ("VDD","24","power_in",10.16,-12.7,0),("VDD","37","power_in",10.16,-10.16,0),
            ("VDD","48","power_in",10.16,-7.62,0),
            ("PA14","35","bidirectional",10.16,-5.08,0),("PA15","36","bidirectional",10.16,-2.54,0),
            ("PB3","37","bidirectional",10.16,0,0),("PB4","38","bidirectional",10.16,2.54,0),
            ("PB5","39","bidirectional",10.16,5.08,0),("PB6","40","bidirectional",10.16,7.62,0),
            ("PB7","41","bidirectional",10.16,10.16,0),("BOOT0","42","input",10.16,12.7,0),
            ("PB8","43","bidirectional",10.16,15.24,0),("PB9","44","bidirectional",10.16,17.78,0),
            ("PB10","45","bidirectional",10.16,20.32,0),("PB11","46","bidirectional",10.16,22.86,0),
            ("PB12","47","bidirectional",10.16,25.4,0),("PB13","48","bidirectional",10.16,27.94,0),
            ("PB14","49","bidirectional",10.16,30.48,0),("PB15","50","bidirectional",10.16,33.02,0),
        ]
    },
    "Connector_Generic:Conn_01x04": {
        "ref_prefix": "J",
        "pins": [("1","1","passive",-3.81,0,0),("2","2","passive",-1.27,0,0),
                 ("3","3","passive",1.27,0,0),("4","4","passive",3.81,0,0)]
    },
    "Connector_Generic:Conn_01x06": {
        "ref_prefix": "J",
        "pins": [("1","1","passive",-6.35,0,0),("2","2","passive",-3.81,0,0),
                 ("3","3","passive",-1.27,0,0),("4","4","passive",1.27,0,0),
                 ("5","5","passive",3.81,0,0),("6","6","passive",6.35,0,0)]
    },
    "Connector_Generic:Conn_01x08": {
        "ref_prefix": "J",
        "pins": [("1","1","passive",-8.89,0,0),("2","2","passive",-6.35,0,0),
                 ("3","3","passive",-3.81,0,0),("4","4","passive",-1.27,0,0),
                 ("5","5","passive",1.27,0,0),("6","6","passive",3.81,0,0),
                 ("7","7","passive",6.35,0,0),("8","8","passive",8.89,0,0)]
    },
}

# Power symbols
POWER_SYMS = ["VDD", "+5V", "+12V", "GND", "VDDA"]
for pwr in POWER_SYMS:
    ptype = "power_out" if pwr != "GND" else "power_out"
    SYMBOLS[f"power:{pwr}"] = {
        "ref_prefix": "#PWR",
        "pins": [("out", "1", ptype, 0, 0, 0)]
    }

# Component instances: (lib_id, value, ref, x, y)
INSTANCES = [
    ("MCU_ST_STM32F1:STM32F103C8Tx", "STM32F103C8T6", "U1", 25*G, 20*G),
    ("Device:C_Small", "100nF", "C1", 2*G, 2*G),
    ("Device:C_Small", "100nF", "C2", 4*G, 2*G),
    ("Device:C_Small", "100nF", "C3", 6*G, 2*G),
    ("Device:C_Small", "4.7uF", "C4", 8*G, 2*G),
    ("Device:C_Small", "100nF", "C5", 10*G, 2*G),
    ("Device:L_Small", "10uH", "L1", 12*G, 2*G),
    ("Device:R", "10k", "R1", 2*G, 5*G),
    ("Device:R", "10k", "R2", 4*G, 5*G),
    ("Device:Crystal", "8MHz", "Y1", 8*G, 5*G),
    ("Device:C_Small", "22pF", "C6", 6*G, 7*G),
    ("Device:C_Small", "22pF", "C7", 10*G, 7*G),
    ("Device:R", "4.7k", "R3", 40*G, 2*G),
    ("Device:R", "4.7k", "R4", 42*G, 2*G),
    ("Interface_Expansion:PCF8574P", "PCF8574P", "U2", 45*G, 5*G),
    ("Memory_EEPROM:AT24CS02-SSHM", "AT24C08", "U3", 45*G, 12*G),
    ("Transistor_BJT:BC547", "BC547", "Q1", 2*G, 25*G),
    ("Device:R", "10k", "R5", 2*G, 22*G),
    ("Device:R", "1k", "R6", 2*G, 30*G),
    ("Device:R", "2k", "R7", 2*G, 32*G),
    ("Transistor_BJT:BC547", "BC547", "Q2", 12*G, 25*G),
    ("Device:R", "10k", "R8", 12*G, 22*G),
    ("Device:R", "10k", "R9", 12*G, 28*G),
    ("Transistor_BJT:BC547", "BC547", "Q3", 14*G, 25*G),
    ("Device:R", "10k", "R10", 14*G, 22*G),
    ("Device:R", "10k", "R11", 14*G, 28*G),
    ("Transistor_BJT:BC547", "BC547", "Q4", 16*G, 25*G),
    ("Device:R", "10k", "R12", 16*G, 22*G),
    ("Device:R", "10k", "R13", 16*G, 28*G),
    ("Transistor_BJT:BC547", "BC547", "Q5", 18*G, 25*G),
    ("Device:R", "10k", "R14", 18*G, 22*G),
    ("Device:R", "10k", "R15", 18*G, 28*G),
    ("Transistor_BJT:BC547", "BC547", "Q6", 20*G, 25*G),
    ("Device:R", "10k", "R16", 20*G, 22*G),
    ("Device:R", "10k", "R17", 20*G, 28*G),
    ("Transistor_BJT:BC547", "BC547", "Q7", 22*G, 25*G),
    ("Device:R", "10k", "R18", 22*G, 22*G),
    ("Device:R", "10k", "R19", 22*G, 28*G),
    ("Device:R", "10k", "R20", 24*G, 22*G),
    ("Transistor_BJT:BC547", "BC547", "Q8", 30*G, 25*G),
    ("Device:R", "10k", "R21", 30*G, 22*G),
    ("Transistor_BJT:BC547", "BC547", "Q9", 33*G, 25*G),
    ("Device:R", "10k", "R22", 33*G, 22*G),
    ("Device:R", "10k", "R23", 42*G, 22*G),
    ("Device:R", "10k", "R24", 44*G, 22*G),
    ("Device:R", "10k", "R25", 46*G, 22*G),
    ("Device:R", "10k", "R26", 48*G, 22*G),
    ("Device:R", "10k", "R27", 42*G, 28*G),
    ("Device:R", "10k", "R28", 44*G, 28*G),
    ("Device:R", "4.7k", "R29", 15*G, 2*G),
    ("Device:R", "1k", "R30", 38*G, 25*G),
    ("Device:LED", "RED", "D1", 38*G, 28*G),
    ("Device:R", "1k", "R31", 40*G, 25*G),
    ("Device:LED", "GREEN", "D2", 40*G, 28*G),
    ("Device:R", "10k", "R32", 26*G, 22*G),
    ("Connector_Generic:Conn_01x04", "SWD", "J1", 42*G, 5*G),
    ("Connector_Generic:Conn_01x06", "SIM800L", "J2", 2*G, 35*G),
    ("Connector_Generic:Conn_01x08", "NRI_G13", "J3", 16*G, 32*G),
]

# Power flag instances
for i, pwr in enumerate(POWER_SYMS):
    INSTANCES.append((f"power:{pwr}", pwr, f"#PWR{i+1:03d}", i*3*G, 0))

L = []

# Header
L.append('(kicad_sch')
L.append('\t(version 20250114)')
L.append('\t(generator "eeschema")')
L.append('\t(generator_version "10.0")')
L.append(f'\t(uuid "{u()}")')
L.append('\t(paper "A3")')
L.append('\t(title_block')
L.append('\t\t(title "Bahilizator v2.0 - STM32F103C8T6 Bluepill")')
L.append('\t\t(date "2026-04-17")')
L.append('\t\t(rev "2.0")')
L.append('\t)')

# lib_symbols — must contain ALL symbols used
L.append('\t(lib_symbols')

used_libs = set(inst[0] for inst in INSTANCES)
for lib_id in sorted(used_libs):
    if lib_id not in SYMBOLS:
        continue
    sym = SYMBOLS[lib_id]
    lib, name = lib_id.split(":")
    L.append(f'\t\t(symbol "{lib_id}"')
    L.append('\t\t\t(pin_numbers')
    L.append('\t\t\t\t(hide yes)')
    L.append('\t\t\t)')
    L.append('\t\t\t(pin_names')
    L.append('\t\t\t\t(offset 0)')
    L.append('\t\t\t)')
    L.append('\t\t\t(exclude_from_sim no)')
    L.append('\t\t\t(in_bom yes)')
    L.append('\t\t\t(on_board yes)')
    L.append(f'\t\t\t(property "Reference" "{sym["ref_prefix"]}"')
    L.append('\t\t\t\t(at 0 0 0)')
    L.append('\t\t\t\t(effects')
    L.append('\t\t\t\t\t(font')
    L.append('\t\t\t\t\t\t(size 1.27 1.27)')
    L.append('\t\t\t\t\t)')
    L.append('\t\t\t\t)')
    L.append('\t\t\t)')
    L.append(f'\t\t\t(property "Value" "{name}"')
    L.append('\t\t\t\t(at 0 0 90)')
    L.append('\t\t\t\t(effects')
    L.append('\t\t\t\t\t(font')
    L.append('\t\t\t\t\t\t(size 1.27 1.27)')
    L.append('\t\t\t\t\t)')
    L.append('\t\t\t\t)')
    L.append('\t\t\t)')
    L.append('\t\t\t(property "Footprint" ""')
    L.append('\t\t\t\t(at 0 0 0)')
    L.append('\t\t\t\t(effects')
    L.append('\t\t\t\t\t(font')
    L.append('\t\t\t\t\t\t(size 1.27 1.27)')
    L.append('\t\t\t\t\t)')
    L.append('\t\t\t\t\t(hide yes)')
    L.append('\t\t\t\t)')
    L.append('\t\t\t)')
    L.append('\t\t\t(property "Datasheet" "~"')
    L.append('\t\t\t\t(at 0 0 0)')
    L.append('\t\t\t\t(effects')
    L.append('\t\t\t\t\t(font')
    L.append('\t\t\t\t\t\t(size 1.27 1.27)')
    L.append('\t\t\t\t\t)')
    L.append('\t\t\t\t\t(hide yes)')
    L.append('\t\t\t\t)')
    L.append('\t\t\t)')
    # Symbol unit with pins
    L.append(f'\t\t\t(symbol "{name}_1_1"')
    for pname, pnum, ptype, px, py, pdir in sym["pins"]:
        L.append(f'\t\t\t\t\t(pin {ptype} line')
        L.append(f'\t\t\t\t\t\t(at {px} {py} {pdir})')
        L.append(f'\t\t\t\t\t\t(length 2.54)')
        L.append(f'\t\t\t\t\t\t(name "{pname}"')
        L.append('\t\t\t\t\t\t\t(effects')
        L.append('\t\t\t\t\t\t\t\t(font')
        L.append('\t\t\t\t\t\t\t\t\t(size 1.27 1.27)')
        L.append('\t\t\t\t\t\t\t\t)')
        L.append('\t\t\t\t\t\t\t)')
        L.append('\t\t\t\t\t\t)')
        L.append(f'\t\t\t\t\t\t(number "{pnum}"')
        L.append('\t\t\t\t\t\t\t(effects')
        L.append('\t\t\t\t\t\t\t\t(font')
        L.append('\t\t\t\t\t\t\t\t\t(size 1.27 1.27)')
        L.append('\t\t\t\t\t\t\t\t)')
        L.append('\t\t\t\t\t\t\t)')
        L.append('\t\t\t\t\t\t)')
        L.append('\t\t\t\t\t)')
    L.append('\t\t\t)')
    L.append('\t\t)')

L.append('\t)')  # end lib_symbols

# Symbol instances
for lib_id, value, ref, x, y in INSTANCES:
    if lib_id not in SYMBOLS:
        continue
    sym = SYMBOLS[lib_id]
    lib, name = lib_id.split(":")
    L.append('\t(symbol')
    L.append(f'\t\t(lib_id "{lib_id}")')
    L.append(f'\t\t(at {x} {y} 0)')
    L.append('\t\t(unit 1)')
    L.append('\t\t(exclude_from_sim no)')
    L.append('\t\t(in_bom yes)')
    L.append('\t\t(on_board yes)')
    L.append('\t\t(dnp no)')
    L.append(f'\t\t(uuid "{u()}")')
    L.append(f'\t\t(property "Reference" "{ref}"')
    L.append(f'\t\t\t(at {x} {y - 2.54} 0)')
    L.append('\t\t\t(effects')
    L.append('\t\t\t\t(font')
    L.append('\t\t\t\t\t(size 1.27 1.27)')
    L.append('\t\t\t\t)')
    L.append('\t\t\t)')
    L.append('\t\t)')
    L.append(f'\t\t(property "Value" "{value}"')
    L.append(f'\t\t\t(at {x} {y + 2.54} 0)')
    L.append('\t\t\t(effects')
    L.append('\t\t\t\t(font')
    L.append('\t\t\t\t\t(size 1.27 1.27)')
    L.append('\t\t\t\t)')
    L.append('\t\t\t)')
    L.append('\t\t)')
    # Pin uuids
    for pname, pnum, ptype, px, py, pdir in sym["pins"]:
        L.append(f'\t\t(pin "{pname}"')
        L.append(f'\t\t\t(uuid "{u()}")')
        L.append('\t\t)')
    L.append('\t)')

L.append(')')  # end kicad_sch

with open('bahilizator.kicad_sch', 'w') as f:
    f.write('\n'.join(L))

print(f"Generated: {len(L)} lines")
print(f"lib_symbols: {len(used_libs)}, instances: {len(INSTANCES)}")