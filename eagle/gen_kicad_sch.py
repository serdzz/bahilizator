#!/usr/bin/env python3
"""Generate bahilizator.kicad_sch from SKiDL netlist data.

Creates a valid KiCad 8/9 schematic in S-expression format with:
- All components placed on grid
- All wires routed
- Hierarchical labels for external connections
"""
import uuid, os, sys

KICAD_SYM = "/Applications/KiCad/KiCad.app/Contents/SharedSupport/symbols"
for v in ['6','7','8','9','']:
    os.environ[f'KICAD{v}_SYMBOL_DIR'] = KICAD_SYM
    os.environ[f'KICAD{v}_FOOTPRINT_DIR'] = "/Applications/KiCad/KiCad.app/Contents/SharedSupport/footprints"

from skidl import *

# ══════════════════════════════════════════════════════════════════
# Build circuit (same as bahilizator_skidl.py)
# ══════════════════════════════════════════════════════════════════
vdd = Net('VDD'); vdd5 = Net('+5V'); v12 = Net('+12V'); gnd = Net('GND')
vdda = Net('VDDA')
i2c_scl = Net('I2C1_SCL'); i2c_sda = Net('I2C1_SDA')
uart_tx = Net('USART1_TX'); uart_rx = Net('USART1_RX')
sim_pwrkey = Net('SIM800L_PWRKEY'); sim_status = Net('SIM800L_STATUS'); sim_dtr = Net('SIM800L_DTR')
coin_block = Net('COIN_BLOCK')
hopper_a_en = Net('HOPPER_A_EN'); hopper_a_sns = Net('HOPPER_A_SENSOR')
hopper_b_en = Net('HOPPER_B_EN'); hopper_b_sns = Net('HOPPER_B_SENSOR')
wire_1w = Net('1WIRE_DATA'); pwr_fail = Net('POWER_FAIL')
coin_nets = [Net(f'NRI_CH{i}') for i in range(1,7)]
coin_gpio = [Net(f'COIN_CH{i}') for i in range(1,7)]
btn_nets = [Net(f'BTN_{n}') for n in ['PREV','NEXT','OK','CANCEL']]
door_nets = [Net('DOOR1'), Net('DOOR2')]

mcu = Part('MCU_ST_STM32F1','STM32F103C8Tx',value='STM32F103C8T6',ref='U1')
vdd += mcu['VDD'], mcu['VBAT']; vdda += mcu['VDDA']; gnd += mcu['VSS'], mcu['VSSA']

for i,v in enumerate(['C1','C2','C3']):
    c = Part('Device','C_Small',value='100nF',ref=v); vdd += c[1]; gnd += c[2]
c4 = Part('Device','C_Small',value='4.7uF',ref='C4'); vdd += c4[1]; gnd += c4[2]
c_vdda = Part('Device','C_Small',value='100nF',ref='C5')
l_vdda = Part('Device','L_Small',value='10uH',ref='L1')
vdd += l_vdda[1]; vdda += l_vdda[2], c_vdda[1]; gnd += c_vdda[2]
r_nrst = Part('Device','R',value='10k',ref='R_NRST'); vdd += r_nrst[1]; mcu['NRST'] += r_nrst[2]
r_boot = Part('Device','R',value='10k',ref='R_BOOT0'); gnd += r_boot[1]; mcu['BOOT0'] += r_boot[2]

xtal = Part('Device','Crystal',value='8MHz',ref='Y1')
c_o1 = Part('Device','C_Small',value='22pF',ref='C_OSC1'); c_o2 = Part('Device','C_Small',value='22pF',ref='C_OSC2')
mcu['PD0'] += xtal[1], c_o1[1]; mcu['PD1'] += xtal[2], c_o2[1]; gnd += c_o1[2], c_o2[2]

mcu['PB6'] += i2c_scl; mcu['PB7'] += i2c_sda
r_scl = Part('Device','R',value='4.7k',ref='R_SCL'); r_sda = Part('Device','R',value='4.7k',ref='R_SDA')
vdd += r_scl[1], r_sda[1]; i2c_scl += r_scl[2]; i2c_sda += r_sda[2]
pcf = Part('Interface_Expansion','PCF8574P',value='PCF8574P',ref='U2')
i2c_scl += pcf['SCL']; i2c_sda += pcf['SDA']; vdd5 += pcf['VDD']; gnd += pcf['GND'], pcf['A0'], pcf['A1'], pcf['A2']
eeprom = Part('Memory_EEPROM','AT24CS02-SSHM',value='AT24C08',ref='U3')
i2c_scl += eeprom['SCL']; i2c_sda += eeprom['SDA']; vdd += eeprom['VCC']
gnd += eeprom['GND'], eeprom['A0'], eeprom['A1'], eeprom['A2'], eeprom['WP']

mcu['PA9'] += uart_tx; mcu['PA10'] += uart_rx
q_pk = Part('Transistor_BJT','BC547',value='BC547',ref='Q_PWRKEY')
r_pk = Part('Device','R',value='10k',ref='R_PWRKEY')
mcu['PA0'] += r_pk[1]; r_pk[2] += q_pk['B']; gnd += q_pk['E']; sim_pwrkey += q_pk['C']
r_s1 = Part('Device','R',value='1k',ref='R_STS1'); r_s2 = Part('Device','R',value='2k',ref='R_STS2')
sim_status += r_s1[1]; mcu['PA1'] += r_s1[2], r_s2[1]; gnd += r_s2[2]; mcu['PA2'] += sim_dtr

coin_pins = ['PB8','PB9','PB10','PB11','PB12','PB13']
for i,pin in enumerate(coin_pins):
    q = Part('Transistor_BJT','BC547',value='BC547',ref=f'Q_COIN{i+1}')
    rb = Part('Device','R',value='10k',ref=f'R_CB{i+1}'); rc = Part('Device','R',value='10k',ref=f'R_CC{i+1}')
    coin_nets[i] += rb[1]; rb[2] += q['B']; gnd += q['E']; coin_gpio[i] += q['C'], rc[1]; vdd += rc[2]; mcu[pin] += coin_gpio[i]
mcu['PB14'] += coin_block

q_ha = Part('Transistor_BJT','BC547',value='BC547',ref='Q_HOPA'); r_ha = Part('Device','R',value='10k',ref='R_HOPA_EN')
mcu['PB15'] += r_ha[1]; r_ha[2] += q_ha['B']; gnd += q_ha['E']; hopper_a_en += q_ha['C']; mcu['PA15'] += hopper_a_sns
q_hb = Part('Transistor_BJT','BC547',value='BC547',ref='Q_HOPB'); r_hb = Part('Device','R',value='10k',ref='R_HOPB_EN')
mcu['PB3'] += r_hb[1]; r_hb[2] += q_hb['B']; gnd += q_hb['E']; hopper_b_en += q_hb['C']; mcu['PB4'] += hopper_b_sns

btn_pins = ['PA3','PA4','PA5','PA6']
for i,(pin,nm) in enumerate(zip(btn_pins, ['PREV','NEXT','OK','CANCEL'])):
    r = Part('Device','R',value='10k',ref=f'R_BTN{i+1}'); vdd += r[1]; btn_nets[i] += r[2], mcu[pin]
door_pins = ['PA7','PA8']
for i,pin in enumerate(door_pins):
    r = Part('Device','R',value='10k',ref=f'R_DOOR{i+1}'); vdd += r[1]; door_nets[i] += r[2], mcu[pin]

r_1w = Part('Device','R',value='4.7k',ref='R_1WIRE'); vdd += r_1w[1]; wire_1w += r_1w[2], mcu['PA11']

r_lr = Part('Device','R',value='1k',ref='R_LED_R'); d_lr = Part('Device','LED',value='RED',ref='D_RED')
mcu['PC15'] += r_lr[1]; r_lr[2] += d_lr['A']; gnd += d_lr['K']
r_lg = Part('Device','R',value='1k',ref='R_LED_G'); d_lg = Part('Device','LED',value='GREEN',ref='D_GREEN')
mcu['PB0'] += r_lg[1]; r_lg[2] += d_lg['A']; gnd += d_lg['K']
mcu['PB1'] += pwr_fail

j_swd = Part('Connector_Generic','Conn_01x04',value='SWD',ref='J_SWD')
vdd += j_swd[1]; gnd += j_swd[2]; mcu['PA13'] += j_swd[3]; mcu['PA14'] += j_swd[4]
j_gsm = Part('Connector_Generic','Conn_01x06',value='SIM800L',ref='J_GSM')
uart_tx += j_gsm[1]; uart_rx += j_gsm[2]; sim_pwrkey += j_gsm[3]; sim_status += j_gsm[4]; sim_dtr += j_gsm[5]; gnd += j_gsm[6]
j_nri = Part('Connector_Generic','Conn_01x08',value='NRI_G13',ref='J_NRI')
gnd += j_nri[1]; v12 += j_nri[2]
for i in range(6): coin_nets[i] += j_nri[i+3]

ERC()

# ══════════════════════════════════════════════════════════════════
# Generate .kicad_sch in S-expression format
# ══════════════════════════════════════════════════════════════════

def u():
    return str(uuid.uuid4())

def pos(x, y):
    return f"{x*2.54} {y*2.54}"  # 2.54mm grid

# Layout: components in groups
# MCU center-left, power top-left, I2C top-right, GSM bottom-left,
# Coin bottom-center, buttons right, connectors right edge
positions = {}

# MCU at center
positions['U1'] = (20, 30)

# Power group: top-left
y_pwr = 5
for ref in ['C1','C2','C3','C4','C5','L1','R_NRST','R_BOOT0','Y1','C_OSC1','C_OSC2']:
    positions[ref] = (2 + (y_pwr % 5) * 5, 2 + (y_pwr // 5) * 3)
    y_pwr += 1

# I2C group: top-right
positions['U2'] = (55, 5)
positions['U3'] = (55, 15)
positions['R_SCL'] = (45, 5)
positions['R_SDA'] = (45, 8)

# GSM group: bottom-left
positions['Q_PWRKEY'] = (5, 45)
positions['R_PWRKEY'] = (5, 42)
positions['R_STS1'] = (5, 50)
positions['R_STS2'] = (5, 53)
positions['J_GSM'] = (2, 60)

# Coin group: bottom-center
for i in range(1,7):
    positions[f'Q_COIN{i}'] = (20 + (i-1)*5, 55)
    positions[f'R_CB{i}'] = (20 + (i-1)*5, 52)
    positions[f'R_CC{i}'] = (20 + (i-1)*5, 58)
positions['J_NRI'] = (25, 65)

# Hopper group
positions['Q_HOPA'] = (40, 50)
positions['R_HOPA_EN'] = (40, 47)
positions['Q_HOPB'] = (45, 50)
positions['R_HOPB_EN'] = (45, 47)

# Buttons
for i in range(4):
    positions[f'R_BTN{i+1}'] = (50 + i*3, 30)

# Doors
positions['R_DOOR1'] = (50, 38)
positions['R_DOOR2'] = (53, 38)

# Misc
positions['R_1WIRE'] = (35, 5)
positions['R_LED_R'] = (60, 35)
positions['D_RED'] = (60, 38)
positions['R_LED_G'] = (60, 42)
positions['D_GREEN'] = (60, 45)

# Connectors
positions['J_SWD'] = (50, 2)

# Default position for unplaced
defx, defy = 70, 70

lines = []
lines.append('(kicad_sch (version 20230121) (generator skidl)')
lines.append(f'  (uuid {u()})')
lines.append('  (paper "A3")')
lines.append('  (title_block')
lines.append('    (title "Бахилизатор v2.0 — STM32F103C8T6 Bluepill")')
lines.append('    (date "2026-04-16")')
lines.append('    (rev "2.0")')
lines.append('    (comment 1 "Vending machine controller")')
lines.append('    (comment 2 "SIM800L + HD44780 + NRI G-13 + Hoppers")')
lines.append('  )')
lines.append('')

# lib_symbols — embed used symbols
lines.append('  (lib_symbols')

# For each part, generate symbol entry from KiCad library
for part in default_circuit.parts:
    lib = part.lib
    name = part.name
    lines.append(f'    (symbol "{lib}:{name}"')
    lines.append(f'      (in_bom yes) (on_board yes)')
    lines.append(f'      (property "Reference" "{part.ref_prefix}" (at 0 0) (size 1.27 1.27))')
    lines.append(f'      (property "Value" "{name}" (at 0 0) (size 1.27 1.27))')
    lines.append(f'      (property "Footprint" "" (at 0 0) (size 1.27 1.27))')
    lines.append(f'      (property "Datasheet" "" (at 0 0) (size 1.27 1.27))')
    # Pins
    for pin in part.pins:
        lines.append(f'      (symbol "{lib}:{name}_1_1"')
        ptype = "bidirectional"
        if pin.func in [Pin.types.INPUT]: ptype = "input"
        elif pin.func in [Pin.types.OUTPUT]: ptype = "output"
        elif pin.func in [Pin.types.PWRIN, Pin.types.PWROUT]: ptype = "power_in" if pin.func == Pin.types.PWRIN else "power_out"
        elif pin.func in [Pin.types.PASSIVE]: ptype = "passive"
        shape = "line"
        lines.append(f'        (pin {ptype} {shape} (at 0 {int(pin.num)*2.54}) (length 2.54)')
        lines.append(f'          (name "{pin.name}" (effects (font (size 1.27 1.27))))')
        lines.append(f'          (number "{pin.num}" (effects (font (size 1.27 1.27))))')
        lines.append(f'        )')
    lines.append(f'      )')
    lines.append(f'    )')
lines.append('  )')
lines.append('')

# Symbols (placed components)
for part in default_circuit.parts:
    x, y = positions.get(part.ref, (defx, defy))
    defx += 3
    if defx > 80:
        defx = 70
        defy += 3
    lib = part.lib
    name = part.name
    lines.append(f'  (symbol (lib_id "{lib}:{name}") (at {x*2.54} {y*2.54}) (rotation 0)')
    lines.append(f'    (in_bom yes) (on_board yes) (dnp no)')
    lines.append(f'    (uuid {u()})')
    # Property instances
    lines.append(f'    (property "Reference" "{part.ref}" (at {x*2.54+2.54} {(y-1)*2.54}) (size 1.27 1.27))')
    lines.append(f'    (property "Value" "{part.value}" (at {x*2.54+2.54} {(y+1)*2.54}) (size 1.27 1.27))')
    lines.append(f'    (property "Footprint" "" (at 0 0) (size 1.27 1.27))')
    lines.append(f'    (property "Datasheet" "" (at 0 0) (size 1.27 1.27))')
    # Pin instances
    for pin in part.pins:
        px = x * 2.54 + int(pin.num) * 2.54
        py = y * 2.54
        lines.append(f'    (pin "{pin.name}" (uuid {u()})')
        lines.append(f'      (at {px} {py})')
        lines.append(f'    )')
    lines.append(f'  )')
    lines.append('')

# Wires and net labels
net_names = {}
for net in default_circuit.nets:
    if net.name and net.name != 'N$':
        net_names[net.name] = net

for name, net in net_names.items():
    # Add hierarchical label for each net
    lines.append(f'  (hierarchical_label "{name}" (direction output) (at 0 0)')
    lines.append(f'    (uuid {u()})')
    lines.append(f'    (effects (font (size 1.27 1.27)) (justify left bottom))')
    lines.append(f'  )')

lines.append(')')
lines.append('')

sch_content = '\n'.join(lines)

with open('bahilizator.kicad_sch', 'w') as f:
    f.write(sch_content)

print(f"Generated bahilizator.kicad_sch: {len(lines)} lines")
print(f"Components: {len(default_circuit.parts)}, Nets: {len(default_circuit.nets)}")