#!/usr/bin/env python3
"""Generate bahilizator.kicad_pcb — KiCad 10 format, 2-layer board.

Strict S-expression format matching real KiCad output.
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

# Use tabs for indentation like real KiCad
T = '\t'

def prop(ref_name, value, x, y, rot, layer, hide=False):
    """Generate a property block in KiCad PCB format."""
    lines = []
    hide_str = '\n\t\t\t(hide yes)' if hide else ''
    lines.append(f'{T}{T}(property "{ref_name}" "{value}"')
    lines.append(f'{T}{T}{T}(at {x} {y} {rot})')
    lines.append(f'{T}{T}{T}(layer "{layer}")')
    lines.append(f'{T}{T}{T}(uuid "{u()}")')
    lines.append(f'{T}{T}{T}(effects')
    lines.append(f'{T}{T}{T}{T}(font')
    lines.append(f'{T}{T}{T}{T}{T}(size 1 1)')
    lines.append(f'{T}{T}{T}{T}{T}(thickness 0.15)')
    lines.append(f'{T}{T}{T}{T})')
    if hide:
        lines.append(f'{T}{T}{T}{T}(hide yes)')
    lines.append(f'{T}{T}{T})')
    lines.append(f'{T}{T})')
    return '\n'.join(lines)

L = []

# Header
L.append('(kicad_pcb')
L.append(f'{T}(version 20241229)')
L.append(f'{T}(generator "pcbnew")')
L.append(f'{T}(generator_version "10.0")')
L.append(f'{T}(general')
L.append(f'{T}{T}(thickness 1.6)')
L.append(f'{T})')
L.append(f'{T}(paper "A4")')
L.append(f'{T}(title_block')
L.append(f'{T}{T}(title "Bahilizator v2.0 - STM32F103C8T6 Bluepill")')
L.append(f'{T}{T}(date "2026-04-17")')
L.append(f'{T}{T}(rev "2.0")')
L.append(f'{T})')

# Layers
L.append(f'{T}(layers')
for num, name, tp in [(0,"F.Cu","signal"),(31,"B.Cu","signal"),
    (32,"B.Adhes","user"),(33,"F.Adhes","user"),(34,"B.Paste","user"),
    (35,"F.Paste","user"),(36,"B.SilkS","user"),(37,"F.SilkS","user"),
    (38,"B.Mask","user"),(39,"F.Mask","user"),(40,"Dwgs.User","user"),
    (41,"Cmts.User","user"),(42,"Eco1.User","user"),(43,"Eco2.User","user"),
    (44,"Edge.Cuts","user"),(45,"Margin","user"),(46,"B.CrtYd","user"),
    (47,"F.CrtYd","user"),(48,"B.Fab","user"),(49,"F.Fab","user")]:
    L.append(f'{T}{T}({num} "{name}" {tp})')
L.append(f'{T})')

# Setup
L.append(f'{T}(setup')
L.append(f'{T}{T}(pad_to_mask_clearance 0.05)')
L.append(f'{T}{T}(grid_origin 0 0)')
L.append(f'{T})')

# Net classes
L.append(f'{T}(net_class "Default" "Default"')
for k,v in [('clearance',0.2),('trace_width',0.25),('via_dia',0.8),('via_drill',0.4)]:
    L.append(f'{T}{T}({k} {v})')
L.append(f'{T}{T}(add_net "")')
for n in ALL_NETS:
    L.append(f'{T}{T}(add_net "{n}")')
L.append(f'{T})')
L.append(f'{T}(net_class "Power" "Power nets"')
for k,v in [('clearance',0.3),('trace_width',0.4),('via_dia',0.8),('via_drill',0.4)]:
    L.append(f'{T}{T}({k} {v})')
for n in POWER_NETS:
    L.append(f'{T}{T}(add_net "{n}")')
L.append(f'{T})')

# Nets
L.append(f'{T}(net 0 "")')
for i,n in enumerate(ALL_NETS, 1):
    L.append(f'{T}(net {i} "{n}")')

# Footprints
for ref,(x,y,rot) in sorted(PLACEMENT.items(), key=lambda kv: kv[0]):
    fp = get_fp(ref)
    is_smd = any(k in fp for k in ['SMD','QFP','SOIC','SOT','0805'])
    L.append(f'{T}(footprint "{fp}"')
    L.append(f'{T}{T}(layer "F.Cu")')
    L.append(f'{T}{T}(at {x} {y} {rot})')
    L.append(f'{T}{T}(attr {"smd" if is_smd else "through_hole"})')
    # Reference property — proper multi-line format
    L.append(prop("Reference", ref, 0, -1.5, 0, "F.SilkS"))
    # Value property
    L.append(prop("Value", ref, 0, 1.5, 0, "F.Fab"))
    # Footprint property (hidden)
    L.append(prop("Footprint", "", 0, 0, 0, "F.Fab", hide=True))
    # Datasheet property (hidden)
    L.append(prop("Datasheet", "", 0, 0, 0, "F.Fab", hide=True))
    L.append(f'{T})')

# Board outline
L.append(f'{T}(gr_rect')
L.append(f'{T}{T}(start 0 0)')
L.append(f'{T}{T}(end {BOARD_W} {BOARD_H})')
L.append(f'{T}{T}(stroke')
L.append(f'{T}{T}{T}(width 0.15)')
L.append(f'{T}{T}{T}(type solid)')
L.append(f'{T}{T})')
L.append(f'{T}{T}(fill none)')
L.append(f'{T}{T}(layer "Edge.Cuts")')
L.append(f'{T}{T}(uuid "{u()}")')
L.append(f'{T})')

# Silkscreen
L.append(f'{T}(gr_text "Bahilizator v2.0"')
L.append(f'{T}{T}(at 50 3 0)')
L.append(f'{T}{T}(layer "F.SilkS")')
L.append(f'{T}{T}(uuid "{u()}")')
L.append(f'{T}{T}(effects')
L.append(f'{T}{T}{T}(font')
L.append(f'{T}{T}{T}{T}(size 3 3)')
L.append(f'{T}{T}{T}{T}(thickness 0.5)')
L.append(f'{T}{T}{T})')
L.append(f'{T}{T})')
L.append(f'{T})')
L.append(f'{T}(gr_text "STM32F103C8T6"')
L.append(f'{T}{T}(at 50 7 0)')
L.append(f'{T}{T}(layer "F.SilkS")')
L.append(f'{T}{T}(uuid "{u()}")')
L.append(f'{T}{T}(effects')
L.append(f'{T}{T}{T}(font')
L.append(f'{T}{T}{T}{T}(size 1.5 1.5)')
L.append(f'{T}{T}{T}{T}(thickness 0.3)')
L.append(f'{T}{T}{T})')
L.append(f'{T}{T})')
L.append(f'{T})')

# Zones
for net_name, net_id, layer in [("GND", net_map["GND"], "B.Cu"), ("VDD", net_map["VDD"], "F.Cu")]:
    L.append(f'{T}(zone')
    L.append(f'{T}{T}(net {net_id})')
    L.append(f'{T}{T}(net_name "{net_name}")')
    L.append(f'{T}{T}(layer "{layer}")')
    L.append(f'{T}{T}(uuid "{u()}")')
    L.append(f'{T}{T}(hatch edge 0.508)')
    L.append(f'{T}{T}(connect_pads')
    L.append(f'{T}{T}{T}(clearance 0.508)')
    L.append(f'{T}{T})')
    L.append(f'{T}{T}(min_thickness 0.254)')
    L.append(f'{T}{T}(filled_areas_thickness no)')
    L.append(f'{T}{T}(fill')
    L.append(f'{T}{T}{T}(thermal_gap 0.508)')
    L.append(f'{T}{T}{T}(thermal_bridge_width 0.508)')
    L.append(f'{T}{T})')
    L.append(f'{T}{T}(polygon')
    L.append(f'{T}{T}{T}(pts')
    L.append(f'{T}{T}{T}{T}(xy 1 1) (xy {BOARD_W-1} 1) (xy {BOARD_W-1} {BOARD_H-1}) (xy 1 {BOARD_H-1})')
    L.append(f'{T}{T}{T})')
    L.append(f'{T}{T})')
    L.append(f'{T})')

# Mounting holes
for cx,cy in [(5,5),(BOARD_W-5,5),(5,BOARD_H-5),(BOARD_W-5,BOARD_H-5)]:
    L.append(f'{T}(footprint "MountingHole:MountingHole_Pad"')
    L.append(f'{T}{T}(layer "F.Cu")')
    L.append(f'{T}{T}(at {cx} {cy} 0)')
    L.append(f'{T}{T}(attr board_only exclude_from_pos_files exclude_from_bom)')
    L.append(prop("Reference", "REF**", 0, -1.5, 0, "F.SilkS"))
    L.append(prop("Value", "MountingHole_Pad", 0, 1.5, 0, "F.Fab"))
    L.append(f'{T}{T}(pad "1" thru_hole circle (at 0 0) (size 3.5 3.5) (drill 1.5)')
    L.append(f'{T}{T}{T}(layers "*.Cu" "*.Mask") (remove_unused_layers no)')
    L.append(f'{T}{T}{T}(net 0 "")')
    L.append(f'{T}{T})')
    L.append(f'{T})')

L.append(')')  # end kicad_pcb

with open('bahilizator.kicad_pcb', 'w') as f:
    f.write('\n'.join(L))

# Validate parentheses
depth = 0
for c in '\n'.join(L):
    if c == '(': depth += 1
    elif c == ')': depth -= 1
    if depth < 0:
        print("ERROR: Unbalanced parentheses!")
        break
else:
    print(f"OK: parens balanced (depth={depth})")

print(f"Generated: {len(L)} lines, {len(PLACEMENT)} footprints, {len(ALL_NETS)} nets")