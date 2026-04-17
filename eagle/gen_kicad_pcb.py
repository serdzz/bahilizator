#!/usr/bin/env python3
"""Generate bahilizator.kicad_pcb — KiCad 10 format, 2-layer board.

KiCad 10 PCB format:
- Uses (footprint ...) not (module ...)
- (property "Reference" ...) not (fp_text reference ...)
- (version 20260206) required
- (net N "name") at top level, no (nets) wrapper
- Footprints reference library, pads come from library
"""
import uuid

def u():
    return str(uuid.uuid4())

BOARD_W, BOARD_H = 100.0, 80.0

POWER_NETS = ['VDD', '+5V', '+12V', 'GND', 'VDDA']
SIGNAL_NETS = ['I2C1_SCL','I2C1_SDA','USART1_TX','USART1_RX',
    'SIM800L_PWRKEY','SIM800L_STATUS','SIM800L_DTR',
    '1WIRE_DATA','POWER_FAIL','COIN_BLOCK',
    'HOPPER_A_EN','HOPPER_A_SENSOR','HOPPER_B_EN','HOPPER_B_SENSOR']
COIN_NETS = [f'COIN_CH{i}' for i in range(1,7)] + [f'NRI_CH{i}' for i in range(1,7)]
BTN_NETS = [f'BTN_{n}' for n in ['PREV','NEXT','OK','CANCEL']]
DOOR_NETS = ['DOOR1','DOOR2']
ALL_NETS = POWER_NETS + SIGNAL_NETS + COIN_NETS + BTN_NETS + DOOR_NETS
net_map = {n: i for i, n in enumerate(ALL_NETS, 1)}

FOOTPRINTS = {
    'U1': 'Package_QFP:LQFP-48_7x7mm_P0.5mm',
    'U2': 'Package_SO:SOIC-16_3.9x9.9mm_P1.27mm',
    'U3': 'Package_SOIC:SOIC-8_3.9x4.9mm_P1.27mm',
    'Y1': 'Crystal:Crystal_HC49-U_Vertical',
    'J_SWD': 'Connector_PinHeader_2.54mm:PinHeader_1x04_P2.54mm_Vertical',
    'J_GSM': 'Connector_PinHeader_2.54mm:PinHeader_1x06_P2.54mm_Vertical',
    'J_NRI': 'Connector_PinHeader_2.54mm:PinHeader_1x08_P2.54mm_Vertical',
}

def get_fp(ref):
    if ref in FOOTPRINTS: return FOOTPRINTS[ref]
    if ref.startswith('Q_'): return 'Package_TO_SOT_SMD:SOT-23'
    if ref.startswith('D_'): return 'LED_THT:LED_D3.0mm'
    if ref.startswith('L'): return 'Inductor_SMD:L_0805_2012Metric_Pad1.15x1.40mm_HandSolder'
    if ref.startswith('C_') or ref.startswith('C_OSC'): return 'Capacitor_SMD:C_0805_2012Metric_Pad1.18x1.45mm_HandSolder'
    return 'Resistor_SMD:R_0805_2012Metric_Pad1.20x1.40mm_HandSolder'

PLACEMENT = {
    'U1': (50,35,0),
    'C1':(15,10,0),'C2':(18,10,0),'C3':(21,10,0),'C4':(24,10,0),'C5':(27,10,0),'L1':(30,10,0),
    'R_NRST':(15,15,0),'R_BOOT0':(18,15,0),
    'Y1':(25,18,0),'C_OSC1':(22,18,90),'C_OSC2':(28,18,90),
    'U2':(80,10,0),'U3':(90,10,0),'R_SCL':(72,12,0),'R_SDA':(72,16,0),
    'Q_PWRKEY':(10,55,0),'R_PWRKEY':(10,50,0),'R_STS1':(10,60,0),'R_STS2':(10,65,0),'J_GSM':(5,70,0),
    'Q_COIN1':(25,55,0),'Q_COIN2':(30,55,0),'Q_COIN3':(35,55,0),'Q_COIN4':(40,55,0),'Q_COIN5':(45,55,0),'Q_COIN6':(50,55,0),
    'R_CB1':(25,50,0),'R_CB2':(30,50,0),'R_CB3':(35,50,0),'R_CB4':(40,50,0),'R_CB5':(45,50,0),'R_CB6':(50,50,0),
    'R_CC1':(25,60,0),'R_CC2':(30,60,0),'R_CC3':(35,60,0),'R_CC4':(40,60,0),'R_CC5':(45,60,0),'R_CC6':(50,60,0),
    'J_NRI':(30,70,0),
    'Q_HOPA':(60,55,0),'R_HOPA_EN':(60,50,0),'Q_HOPB':(65,55,0),'R_HOPB_EN':(65,50,0),
    'R_BTN1':(85,30,0),'R_BTN2':(85,35,0),'R_BTN3':(85,40,0),'R_BTN4':(85,45,0),
    'R_DOOR1':(85,55,0),'R_DOOR2':(85,60,0),
    'R_1WIRE':(35,10,0),
    'R_LED_R':(75,55,0),'D_RED':(75,60,0),'R_LED_G':(80,55,0),'D_GREEN':(80,60,0),
    'J_SWD':(70,5,0),
}

L = []

# Header — KiCad 10 format
L.append('(kicad_pcb (version 20241229) (generator "pcbnew")')
L.append('  (general (thickness 1.6))')
L.append('  (paper "A4")')
L.append('  (title_block')
L.append('    (title "Бахилизатор v2.0 — STM32F103C8T6 Bluepill")')
L.append('    (date "2026-04-16") (rev "2.0")')
L.append('  )')

# Layers
L.append('  (layers')
for num, name, tp in [(0,"F.Cu","signal"),(31,"B.Cu","signal"),
    (32,"B.Adhes","user"),(33,"F.Adhes","user"),(34,"B.Paste","user"),
    (35,"F.Paste","user"),(36,"B.SilkS","user"),(37,"F.SilkS","user"),
    (38,"B.Mask","user"),(39,"F.Mask","user"),(40,"Dwgs.User","user"),
    (41,"Cmts.User","user"),(42,"Eco1.User","user"),(43,"Eco2.User","user"),
    (44,"Edge.Cuts","user"),(45,"Margin","user"),(46,"B.CrtYd","user"),
    (47,"F.CrtYd","user"),(48,"B.Fab","user"),(49,"F.Fab","user")]:
    L.append(f'    ({num} "{name}" {tp})')
L.append('  )')

# Setup
L.append('  (setup')
L.append('    (pad_to_mask_clearance 0.05)')
L.append('    (grid_origin 0 0)')
L.append('  )')

# Net classes
L.append('  (net_class "Default" "Default"')
for k,v in [('clearance',0.2),('trace_width',0.25),('via_dia',0.8),('via_drill',0.4)]:
    L.append(f'    ({k} {v})')
L.append('    (add_net "")')
for n in ALL_NETS:
    L.append(f'    (add_net "{n}")')
L.append('  )')
L.append('  (net_class "Power" "Power nets"')
for k,v in [('clearance',0.3),('trace_width',0.4),('via_dia',0.8),('via_drill',0.4)]:
    L.append(f'    ({k} {v})')
for n in POWER_NETS:
    L.append(f'    (add_net "{n}")')
L.append('  )')

# Nets at top level (KiCad PCB format)
L.append('  (net 0 "")')
for i,n in enumerate(ALL_NETS, 1):
    L.append(f'  (net {i} "{n}")')

# Footprints — KiCad 10 format uses (footprint ...) with (property "Reference" ...)
for ref,(x,y,rot) in sorted(PLACEMENT.items(), key=lambda kv: kv[0]):
    fp = get_fp(ref)
    is_smd = any(k in fp for k in ['SMD','QFP','SOIC','SOT','0805'])
    L.append(f'  (footprint "{fp}"')
    L.append(f'    (layer "F.Cu")')
    L.append(f'    (at {x} {y} {rot})')
    L.append(f'    (attr {"smd" if is_smd else "through_hole"})')
    L.append(f'    (property "Reference" "{ref}"')
    L.append(f'      (at 0 -1.5 0)')
    L.append(f'      (layer "F.SilkS")')
    L.append(f'      (uuid "{u()}")')
    L.append(f'      (effects (font (size 1 1) (thickness 0.15))))')
    L.append(f'    )')
    L.append(f'    (property "Value" "{ref}"')
    L.append(f'      (at 0 1.5 0)')
    L.append(f'      (layer "F.Fab")')
    L.append(f'      (uuid "{u()}")')
    L.append(f'      (effects (font (size 1 1) (thickness 0.15))))')
    L.append(f'    )')
    L.append(f'  )')

# Board outline
L.append(f'  (gr_rect')
L.append(f'    (start 0 0) (end {BOARD_W} {BOARD_H})')
L.append(f'    (stroke (width 0.15) (type solid))')
L.append(f'    (fill none)')
L.append(f'    (layer "Edge.Cuts")')
L.append(f'    (uuid "{u()}")')
L.append(f'  )')

# Silkscreen labels
L.append(f'  (gr_text "Бахилизатор v2.0"')
L.append(f'    (at 50 3 0) (layer "F.SilkS")')
L.append(f'    (uuid "{u()}")')
L.append(f'    (effects (font (size 3 3) (thickness 0.5))))')
L.append(f'  )')
L.append(f'  (gr_text "STM32F103C8T6"')
L.append(f'    (at 50 7 0) (layer "F.SilkS")')
L.append(f'    (uuid "{u()}")')
L.append(f'    (effects (font (size 1.5 1.5) (thickness 0.3))))')
L.append(f'  )')

# Copper zones
L.append(f'  (zone (net {net_map["GND"]}) (net_name "GND") (layer "B.Cu") (uuid "{u()}")')
L.append(f'    (hatch edge 0.5)')
L.append(f'    (connect_pads (clearance 0.5))')
L.append(f'    (min_thickness 0.25)')
L.append(f'    (fill (mode solid) (thermal_gap 0.5) (thermal_bridge_width 0.5))')
L.append(f'    (polygon')
L.append(f'      (pts')
L.append(f'        (xy 1 1) (xy {BOARD_W-1} 1) (xy {BOARD_W-1} {BOARD_H-1}) (xy 1 {BOARD_H-1})')
L.append(f'      )')
L.append(f'    )')
L.append(f'  )')
L.append(f'  (zone (net {net_map["VDD"]}) (net_name "VDD") (layer "F.Cu") (uuid "{u()}")')
L.append(f'    (hatch edge 0.5)')
L.append(f'    (connect_pads (clearance 0.5))')
L.append(f'    (min_thickness 0.25)')
L.append(f'    (fill (mode solid) (thermal_gap 0.5) (thermal_bridge_width 0.5))')
L.append(f'    (polygon')
L.append(f'      (pts')
L.append(f'        (xy 1 1) (xy {BOARD_W-1} 1) (xy {BOARD_W-1} {BOARD_H-1}) (xy 1 {BOARD_H-1})')
L.append(f'      )')
L.append(f'    )')
L.append(f'  )')

# Mounting holes
for cx,cy in [(5,5),(BOARD_W-5,5),(5,BOARD_H-5),(BOARD_W-5,BOARD_H-5)]:
    L.append(f'  (footprint "MountingHole:MountingHole_Pad"')
    L.append(f'    (layer "F.Cu")')
    L.append(f'    (at {cx} {cy} 0)')
    L.append(f'    (attr board_only exclude_from_pos_files exclude_from_bom)')
    L.append(f'    (property "Reference" "REF**"')
    L.append(f'      (at 0 -1.5 0) (layer "F.SilkS")')
    L.append(f'      (uuid "{u()}")')
    L.append(f'      (effects (font (size 1 1) (thickness 0.15))))')
    L.append(f'    )')
    L.append(f'    (pad "1" thru_hole circle (at 0 0) (size 3.5 3.5) (drill 1.5)')
    L.append(f'      (layers "*.Cu" "*.Mask") (remove_unused_layers no)')
    L.append(f'      (net 0 "")')
    L.append(f'    )')
    L.append(f'  )')

L.append(')')  # end kicad_pcb

with open('bahilizator.kicad_pcb', 'w') as f:
    f.write('\n'.join(L))

print(f"Generated: {len(L)} lines, {len(PLACEMENT)} footprints, {len(ALL_NETS)} nets")