#!/bin/bash
export KICAD8_SYMBOL_DIR="/Applications/KiCad/KiCad.app/Contents/SharedSupport/symbols"
export KICAD9_SYMBOL_DIR="$KICAD8_SYMBOL_DIR"
export KICAD7_SYMBOL_DIR="$KICAD8_SYMBOL_DIR"
export KICAD6_SYMBOL_DIR="$KICAD8_SYMBOL_DIR"
export KICAD_SYMBOL_DIR="$KICAD8_SYMBOL_DIR"
export KICAD8_FOOTPRINT_DIR="/Applications/KiCad/KiCad.app/Contents/SharedSupport/footprints"
export KICAD9_FOOTPRINT_DIR="$KICAD8_FOOTPRINT_DIR"
export KICAD7_FOOTPRINT_DIR="$KICAD8_FOOTPRINT_DIR"
export KICAD_FOOTPRINT_DIR="$KICAD8_FOOTPRINT_DIR"
export KICAD_TEMPLATE_DIR="/Applications/KiCad/KiCad.app/Contents/SharedSupport/template"
cd "$(dirname "$0")"
python3 bahilizator_skidl.py