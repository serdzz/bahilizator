#!/usr/bin/env python3
"""Generate minimal bahilizator.kicad_sch.

No embedded lib_symbols — KiCad resolves from installed libraries.
Symbol libs used: MCU_ST_STM32F1, Interface_Expansion, Memory_EEPROM,
Device, Transistor_BJT, Connector_Generic, Crystal, LED_THT
"""
import uuid

def u():
    return str(uuid.uuid4())

# Grid: 2.54mm = 100mil
G = 2.54

# Component definitions: (lib_id, value, x_grid, y_grid)
# Groups: MCU center, Power top-left, I2C top-right, etc.
COMPONENTS = [
    # MCU
    ('MCU_ST_STM32F1:STM32F103C8Tx', 'STM32F103C8T6', 25, 20),

    # Power / decoupling
    ('Device:C_Small', '100nF', 2, 2),
    ('Device:C_Small', '100nF', 4, 2),
    ('Device:C_Small', '100nF', 6, 2),
    ('Device:C_Small', '4.7uF', 8, 2),
    ('Device:C_Small', '100nF', 10, 2),
    ('Device:L_Small', '10uH', 12, 2),

    # Reset / Boot
    ('Device:R', '10k', 2, 5),
    ('Device:R', '10k', 4, 5),

    # Crystal
    ('Device:Crystal', '8MHz', 8, 5),
    ('Device:C_Small', '22pF', 6, 7),
    ('Device:C_Small', '22pF', 10, 7),

    # I2C pullups
    ('Device:R', '4.7k', 40, 2),
    ('Device:R', '4.7k', 42, 2),

    # PCF8574 + EEPROM
    ('Interface_Expansion:PCF8574P', 'PCF8574P', 45, 5),
    ('Memory_EEPROM:AT24CS02-SSHM', 'AT24C08', 45, 12),

    # SIM800L control
    ('Transistor_BJT:BC547', 'BC547', 2, 25),
    ('Device:R', '10k', 2, 22),
    ('Device:R', '1k', 2, 30),
    ('Device:R', '2k', 2, 32),

    # Coin acceptor channels (6x BC547 + resistors)
    ('Transistor_BJT:BC547', 'BC547', 12, 25),
    ('Device:R', '10k', 12, 22),
    ('Device:R', '10k', 12, 28),
    ('Transistor_BJT:BC547', 'BC547', 14, 25),
    ('Device:R', '10k', 14, 22),
    ('Device:R', '10k', 14, 28),
    ('Transistor_BJT:BC547', 'BC547', 16, 25),
    ('Device:R', '10k', 16, 22),
    ('Device:R', '10k', 16, 28),
    ('Transistor_BJT:BC547', 'BC547', 18, 25),
    ('Device:R', '10k', 18, 22),
    ('Device:R', '10k', 18, 28),
    ('Transistor_BJT:BC547', 'BC547', 20, 25),
    ('Device:R', '10k', 20, 22),
    ('Device:R', '10k', 20, 28),
    ('Transistor_BJT:BC547', 'BC547', 22, 25),
    ('Device:R', '10k', 22, 22),
    ('Device:R', '10k', 22, 28),

    # Coin block
    ('Device:R', '10k', 24, 22),

    # Hoppers
    ('Transistor_BJT:BC547', 'BC547', 30, 25),
    ('Device:R', '10k', 30, 22),
    ('Transistor_BJT:BC547', 'BC547', 33, 25),
    ('Device:R', '10k', 33, 22),

    # Buttons
    ('Device:R', '10k', 42, 22),
    ('Device:R', '10k', 44, 22),
    ('Device:R', '10k', 46, 22),
    ('Device:R', '10k', 48, 22),

    # Doors
    ('Device:R', '10k', 42, 28),
    ('Device:R', '10k', 44, 28),

    # 1-Wire
    ('Device:R', '4.7k', 15, 2),

    # LEDs
    ('Device:R', '1k', 38, 25),
    ('LED_THT:LED', 'RED', 38, 28),
    ('Device:R', '1k', 40, 25),
    ('LED_THT:LED', 'GREEN', 40, 28),

    # Power fail
    ('Device:R', '10k', 26, 22),

    # Connectors
    ('Connector_Generic:Conn_01x04', 'SWD', 42, 5),
    ('Connector_Generic:Conn_01x06', 'SIM800L', 2, 35),
    ('Connector_Generic:Conn_01x08', 'NRI_G13', 16, 32),
]

# Ref designators
REFS = [
    'U1',  # MCU
    'C1','C2','C3','C4','C5','L1',  # Power
    'R1','R2',  # Reset/Boot
    'Y1','C6','C7',  # Crystal
    'R3','R4',  # I2C pullups
    'U2','U3',  # PCF8574 + EEPROM
    'Q1','R5','R6','R7',  # SIM800L
    'Q2','R8','R9','Q3','R10','R11','Q4','R12','R13','Q5','R14','R15','Q6','R16','R17','Q7','R18','R19',  # Coin
    'R20',  # Coin block
    'Q8','R21','Q9','R22',  # Hoppers
    'R23','R24','R25','R26',  # Buttons
    'R27','R28',  # Doors
    'R29',  # 1-Wire
    'R30','D1','R31','D2',  # LEDs
    'R32',  # Power fail
    'J1','J2','J3',  # Connectors
]

L = []

# Header
L.append('(kicad_sch (version 20230121) (generator "skidl-gen")')
L.append(f'  (uuid "{u()}")')
L.append('  (paper "A3")')
L.append('  (title_block')
L.append('    (title "Бахилизатор v2.0 — STM32F103C8T6 Bluepill")')
L.append('    (date "2026-04-17") (rev "2.0")')
L.append('    (comment 1 "Vending machine controller")')
L.append('    (comment 2 "SIM800L, HD44780, NRI G-13, Hoppers")')
L.append('  )')

# lib_symbols — minimal, just declare we use these libs
# KiCad resolves symbols from global lib table
L.append('  (lib_symbols)')

# Symbols (placed components)
for i, (lib_id, value, gx, gy) in enumerate(COMPONENTS):
    ref = REFS[i] if i < len(REFS) else f'X{i}'
    x_mm = gx * G
    y_mm = gy * G
    L.append(f'  (symbol (lib_id "{lib_id}") (at {x_mm} {y_mm} 0)')
    L.append(f'    (in_bom yes) (on_board yes) (dnp no)')
    L.append(f'    (uuid "{u()}")')
    L.append(f'    (property "Reference" "{ref}" (at 0 {y_mm - G} 0)')
    L.append(f'      (effects (font (size 1.27 1.27))))')
    L.append(f'    (property "Value" "{value}" (at 0 {y_mm + G} 0)')
    L.append(f'      (effects (font (size 1.27 1.27))))')
    L.append(f'    (property "Footprint" "" (at 0 0 0)')
    L.append(f'      (effects (font (size 1.27 1.27)) hide))')
    L.append(f'    (property "Datasheet" "" (at 0 0 0)')
    L.append(f'      (effects (font (size 1.27 1.27)) hide))')
    L.append(f'  )')

# Power flags — needed for ERC
for name, x, y in [('VDD',0,0),('+5V',2,0),('+12V',4,0),('GND',6,0),('VDDA',8,0)]:
    L.append(f'  (power (lib_id "power:{name}") (at {x*G} {y*G} 0)')
    L.append(f'    (uuid "{u()}")')
    L.append(f'    (property "Reference" "#PWR" (at 0 0 0)')
    L.append(f'      (effects (font (size 1.27 1.27)) hide))')
    L.append(f'    (property "Value" "{name}" (at 0 0 0)')
    L.append(f'      (effects (font (size 1.27 1.27))))')
    L.append(f'  )')

# Wires and labels — simplified, just net labels
NET_NAMES = [
    'VDD','+5V','+12V','GND','VDDA',
    'I2C1_SCL','I2C1_SDA','USART1_TX','USART1_RX',
    'SIM800L_PWRKEY','SIM800L_STATUS','SIM800L_DTR',
    '1WIRE_DATA','POWER_FAIL','COIN_BLOCK',
    'HOPPER_A_EN','HOPPER_A_SENSOR','HOPPER_B_EN','HOPPER_B_SENSOR',
]
for i, name in enumerate(NET_NAMES):
    x = (i % 10) * 5 * G
    y = (40 + i // 10 * 3) * G
    L.append(f'  (label "{name}" (at {x} {y} 0)')
    L.append(f'    (effects (font (size 1.27 1.27)) (justify left bottom))')
    L.append(f'    (uuid "{u()}")')
    L.append(f'  )')

# Coin net labels
for i in range(1,7):
    name = f'NRI_CH{i}'
    x = 12 * G
    y = (32 + i) * G
    L.append(f'  (label "{name}" (at {x} {y} 0)')
    L.append(f'    (effects (font (size 1.27 1.27)) (justify left bottom))')
    L.append(f'    (uuid "{u()}")')
    L.append(f'  )')

L.append(')')  # end kicad_sch

with open('bahilizator.kicad_sch', 'w') as f:
    f.write('\n'.join(L))

print(f"Generated: {len(L)} lines, {len(COMPONENTS)} components, 5 power flags")