#!/usr/bin/env python3
"""Бахилизатор v2.0 — KiCad Schematic Generator via SKiDL
STM32F103C8T6 Bluepill + SIM800L + HD44780 + NRI G-13 + Hoppers
"""
import os, sys

KICAD_SYM = "/Applications/KiCad/KiCad.app/Contents/SharedSupport/symbols"
KICAD_FP = "/Applications/KiCad/KiCad.app/Contents/SharedSupport/footprints"
KICAD_TPL = "/Applications/KiCad/KiCad.app/Contents/SharedSupport/template"
for v in ['6','7','8','9','']:
    os.environ[f'KICAD{v}_SYMBOL_DIR'] = KICAD_SYM
    os.environ[f'KICAD{v}_FOOTPRINT_DIR'] = KICAD_FP
os.environ['KICAD_TEMPLATE_DIR'] = KICAD_TPL

from skidl import *

# ── Nets ──────────────────────────────────────────────────────
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

# ── 1. MCU ────────────────────────────────────────────────────
mcu = Part('MCU_ST_STM32F1','STM32F103C8Tx',value='STM32F103C8T6',ref='U1')
vdd += mcu['VDD'], mcu['VBAT']; vdda += mcu['VDDA']; gnd += mcu['VSS'], mcu['VSSA']

# ── 2. Power ──────────────────────────────────────────────────
for i,v in enumerate(['C1','C2','C3']):
    c = Part('Device','C_Small',value='100nF',ref=v); vdd += c[1]; gnd += c[2]
c4 = Part('Device','C_Small',value='4.7uF',ref='C4'); vdd += c4[1]; gnd += c4[2]

c_vdda = Part('Device','C_Small',value='100nF',ref='C5')
l_vdda = Part('Device','L_Small',value='10uH',ref='L1')
vdd += l_vdda[1]; vdda += l_vdda[2], c_vdda[1]; gnd += c_vdda[2]

r_nrst = Part('Device','R',value='10k',ref='R_NRST'); vdd += r_nrst[1]; mcu['NRST'] += r_nrst[2]
r_boot = Part('Device','R',value='10k',ref='R_BOOT0'); gnd += r_boot[1]; mcu['BOOT0'] += r_boot[2]

# ── 3. Crystal 8MHz ───────────────────────────────────────────
xtal = Part('Device','Crystal',value='8MHz',ref='Y1')
c_o1 = Part('Device','C_Small',value='22pF',ref='C_OSC1')
c_o2 = Part('Device','C_Small',value='22pF',ref='C_OSC2')
mcu['PD0'] += xtal[1], c_o1[1]; mcu['PD1'] += xtal[2], c_o2[1]; gnd += c_o1[2], c_o2[2]

# ── 4. I2C ────────────────────────────────────────────────────
mcu['PB6'] += i2c_scl; mcu['PB7'] += i2c_sda
r_scl = Part('Device','R',value='4.7k',ref='R_SCL'); r_sda = Part('Device','R',value='4.7k',ref='R_SDA')
vdd += r_scl[1], r_sda[1]; i2c_scl += r_scl[2]; i2c_sda += r_sda[2]

pcf = Part('Interface_Expansion','PCF8574P',value='PCF8574P',ref='U2')
i2c_scl += pcf['SCL']; i2c_sda += pcf['SDA']; vdd5 += pcf['VDD']
gnd += pcf['GND'], pcf['A0'], pcf['A1'], pcf['A2']

# 24C08 — нет в KiCad lib, используем AT24C02 (pin-compatible, 8-pin SOIC/DIP)
eeprom = Part('Memory_EEPROM','AT24CS02-SSHM',value='AT24C08',ref='U3')
i2c_scl += eeprom['SCL']; i2c_sda += eeprom['SDA']; vdd += eeprom['VCC']
gnd += eeprom['GND'], eeprom['A0'], eeprom['A1'], eeprom['A2'], eeprom['WP']

# ── 5. GSM SIM800L ────────────────────────────────────────────
mcu['PA9'] += uart_tx; mcu['PA10'] += uart_rx
q_pk = Part('Transistor_BJT','BC547',value='BC547',ref='Q_PWRKEY')
r_pk = Part('Device','R',value='10k',ref='R_PWRKEY')
mcu['PA0'] += r_pk[1]; r_pk[2] += q_pk['B']; gnd += q_pk['E']; sim_pwrkey += q_pk['C']

r_s1 = Part('Device','R',value='1k',ref='R_STS1'); r_s2 = Part('Device','R',value='2k',ref='R_STS2')
sim_status += r_s1[1]; mcu['PA1'] += r_s1[2], r_s2[1]; gnd += r_s2[2]
mcu['PA2'] += sim_dtr

# ── 6. NRI G-13 Coin Acceptor ────────────────────────────────
coin_pins = ['PB8','PB9','PB10','PB11','PB12','PB13']
for i,pin in enumerate(coin_pins):
    q = Part('Transistor_BJT','BC547',value='BC547',ref=f'Q_COIN{i+1}')
    rb = Part('Device','R',value='10k',ref=f'R_CB{i+1}')
    rc = Part('Device','R',value='10k',ref=f'R_CC{i+1}')
    coin_nets[i] += rb[1]; rb[2] += q['B']; gnd += q['E']
    coin_gpio[i] += q['C'], rc[1]; vdd += rc[2]; mcu[pin] += coin_gpio[i]
mcu['PB14'] += coin_block

# ── 7. Hoppers ────────────────────────────────────────────────
q_ha = Part('Transistor_BJT','BC547',value='BC547',ref='Q_HOPA')
r_ha = Part('Device','R',value='10k',ref='R_HOPA_EN')
mcu['PB15'] += r_ha[1]; r_ha[2] += q_ha['B']; gnd += q_ha['E']; hopper_a_en += q_ha['C']
mcu['PA15'] += hopper_a_sns

q_hb = Part('Transistor_BJT','BC547',value='BC547',ref='Q_HOPB')
r_hb = Part('Device','R',value='10k',ref='R_HOPB_EN')
mcu['PB3'] += r_hb[1]; r_hb[2] += q_hb['B']; gnd += q_hb['E']; hopper_b_en += q_hb['C']
mcu['PB4'] += hopper_b_sns

# ── 8. Buttons ────────────────────────────────────────────────
btn_pins = ['PA3','PA4','PA5','PA6']
for i,(pin,nm) in enumerate(zip(btn_pins, ['PREV','NEXT','OK','CANCEL'])):
    r = Part('Device','R',value='10k',ref=f'R_BTN{i+1}')
    vdd += r[1]; btn_nets[i] += r[2], mcu[pin]

# ── 9. Doors ──────────────────────────────────────────────────
door_pins = ['PA7','PA8']
for i,pin in enumerate(door_pins):
    r = Part('Device','R',value='10k',ref=f'R_DOOR{i+1}')
    vdd += r[1]; door_nets[i] += r[2], mcu[pin]

# ── 10. 1-Wire iButton ───────────────────────────────────────
r_1w = Part('Device','R',value='4.7k',ref='R_1WIRE')
vdd += r_1w[1]; wire_1w += r_1w[2], mcu['PA11']

# ── 11. LEDs ──────────────────────────────────────────────────
r_lr = Part('Device','R',value='1k',ref='R_LED_R')
d_lr = Part('Device','LED',value='RED',ref='D_RED')
mcu['PC15'] += r_lr[1]; r_lr[2] += d_lr['A']; gnd += d_lr['K']
r_lg = Part('Device','R',value='1k',ref='R_LED_G')
d_lg = Part('Device','LED',value='GREEN',ref='D_GREEN')
mcu['PB0'] += r_lg[1]; r_lg[2] += d_lg['A']; gnd += d_lg['K']

# ── 12. Power Fail ────────────────────────────────────────────
mcu['PB1'] += pwr_fail

# ── 13. SWD Connector ────────────────────────────────────────
j_swd = Part('Connector_Generic','Conn_01x04',value='SWD',ref='J_SWD')
vdd += j_swd[1]; gnd += j_swd[2]; mcu['PA13'] += j_swd[3]; mcu['PA14'] += j_swd[4]

# ── 14. External Connectors ──────────────────────────────────
j_gsm = Part('Connector_Generic','Conn_01x06',value='SIM800L',ref='J_GSM')
uart_tx += j_gsm[1]; uart_rx += j_gsm[2]; sim_pwrkey += j_gsm[3]
sim_status += j_gsm[4]; sim_dtr += j_gsm[5]; gnd += j_gsm[6]

j_nri = Part('Connector_Generic','Conn_01x08',value='NRI_G13',ref='J_NRI')
gnd += j_nri[1]; v12 += j_nri[2]
for i in range(6): coin_nets[i] += j_nri[i+3]

# ── Generate ──────────────────────────────────────────────────
ERC()
generate_netlist()
generate_schematic(filepath='.', top_name='bahilizator')
print(f"Components: {len(default_circuit.parts)}, Nets: {len(default_circuit.nets)}")