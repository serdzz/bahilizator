<?xml version="1.0" encoding="utf-8"?>
<!DOCTYPE eagle SYSTEM "eagle.dtd">
<eagle version="9.6.2">
<drawing>
<settings>
<setting alwaysvectorfont="no"/>
<setting verticaltext="up"/>
</settings>
<grid distance="0.1" unitdist="inch" unit="inch" style="lines" multiple="1" display="no" altdistance="0.01" altunitdist="inch" altunit="inch"/>
<layers>
<layer number="1" name="Top" color="4" fill="1" visible="yes" active="yes"/>
<layer number="16" name="Bottom" color="1" fill="1" visible="yes" active="yes"/>
<layer number="17" name="Pads" color="2" fill="1" visible="yes" active="yes"/>
<layer number="18" name="Vias" color="2" fill="1" visible="yes" active="yes"/>
<layer number="19" name="Unrouted" color="6" fill="1" visible="yes" active="yes"/>
<layer number="20" name="Dimension" color="15" fill="1" visible="yes" active="yes"/>
<layer number="21" name="tPlace" color="7" fill="1" visible="yes" active="yes"/>
<layer number="22" name="bPlace" color="7" fill="1" visible="yes" active="yes"/>
<layer number="25" name="tNames" color="7" fill="1" visible="yes" active="yes"/>
<layer number="27" name="tValues" color="7" fill="1" visible="yes" active="yes"/>
<layer number="51" name="tDocu" color="7" fill="1" visible="yes" active="yes"/>
<layer number="94" name="Symbols" color="4" fill="1" visible="yes" active="yes"/>
<layer number="95" name="Names" color="7" fill="1" visible="yes" active="yes"/>
<layer number="96" name="Values" color="7" fill="1" visible="yes" active="yes"/>
<layer number="97" name="Info" color="7" fill="1" visible="yes" active="yes"/>
</layers>
<schematic xreflabel="%F%N/%S.%C%R" xrefpart="%F%N/%S.%C%R">
<libraries>
<!-- STM32F103C8T6 library -->
<library name="stm32f103">
<packages>
<package name="LQFP48">
<description>STM32F103C8T6 LQFP-48</description>
<wire x1="-3.5" y1="-3.5" x2="3.5" y2="-3.5" width="0.15" layer="21"/>
<wire x1="3.5" y1="-3.5" x2="3.5" y2="3.5" width="0.15" layer="21"/>
<wire x1="3.5" y1="3.5" x2="-3.5" y2="3.5" width="0.15" layer="21"/>
<wire x1="-3.5" y1="3.5" x2="-3.5" y2="-3.5" width="0.15" layer="21"/>
<circle x="-2.8" y="2.8" radius="0.3" width="0.15" layer="21"/>
<smd name="1" x="-4.2" y="2.75" dx="1.5" dy="0.3" layer="1"/>
<smd name="2" x="-4.2" y="2.25" dx="1.5" dy="0.3" layer="1"/>
<smd name="3" x="-4.2" y="1.75" dx="1.5" dy="0.3" layer="1"/>
<smd name="4" x="-4.2" y="1.25" dx="1.5" dy="0.3" layer="1"/>
<smd name="5" x="-4.2" y="0.75" dx="1.5" dy="0.3" layer="1"/>
<smd name="6" x="-4.2" y="0.25" dx="1.5" dy="0.3" layer="1"/>
<smd name="7" x="-4.2" y="-0.25" dx="1.5" dy="0.3" layer="1"/>
<smd name="8" x="-4.2" y="-0.75" dx="1.5" dy="0.3" layer="1"/>
<smd name="9" x="-4.2" y="-1.25" dx="1.5" dy="0.3" layer="1"/>
<smd name="10" x="-4.2" y="-1.75" dx="1.5" dy="0.3" layer="1"/>
<smd name="11" x="-4.2" y="-2.25" dx="1.5" dy="0.3" layer="1"/>
<smd name="12" x="-4.2" y="-2.75" dx="1.5" dy="0.3" layer="1"/>
<smd name="13" x="-2.75" y="-4.2" dx="0.3" dy="1.5" layer="1"/>
<smd name="14" x="-2.25" y="-4.2" dx="0.3" dy="1.5" layer="1"/>
<smd name="15" x="-1.75" y="-4.2" dx="0.3" dy="1.5" layer="1"/>
<smd name="16" x="-1.25" y="-4.2" dx="0.3" dy="1.5" layer="1"/>
<smd name="17" x="-0.75" y="-4.2" dx="0.3" dy="1.5" layer="1"/>
<smd name="18" x="-0.25" y="-4.2" dx="0.3" dy="1.5" layer="1"/>
<smd name="19" x="0.25" y="-4.2" dx="0.3" dy="1.5" layer="1"/>
<smd name="20" x="0.75" y="-4.2" dx="0.3" dy="1.5" layer="1"/>
<smd name="21" x="1.25" y="-4.2" dx="0.3" dy="1.5" layer="1"/>
<smd name="22" x="1.75" y="-4.2" dx="0.3" dy="1.5" layer="1"/>
<smd name="23" x="2.25" y="-4.2" dx="0.3" dy="1.5" layer="1"/>
<smd name="24" x="2.75" y="-4.2" dx="0.3" dy="1.5" layer="1"/>
<smd name="25" x="4.2" y="-2.75" dx="1.5" dy="0.3" layer="1"/>
<smd name="26" x="4.2" y="-2.25" dx="1.5" dy="0.3" layer="1"/>
<smd name="27" x="4.2" y="-1.75" dx="1.5" dy="0.3" layer="1"/>
<smd name="28" x="4.2" y="-1.25" dx="1.5" dy="0.3" layer="1"/>
<smd name="29" x="4.2" y="-0.75" dx="1.5" dy="0.3" layer="1"/>
<smd name="30" x="4.2" y="-0.25" dx="1.5" dy="0.3" layer="1"/>
<smd name="31" x="4.2" y="0.25" dx="1.5" dy="0.3" layer="1"/>
<smd name="32" x="4.2" y="0.75" dx="1.5" dy="0.3" layer="1"/>
<smd name="33" x="4.2" y="1.25" dx="1.5" dy="0.3" layer="1"/>
<smd name="34" x="4.2" y="1.75" dx="1.5" dy="0.3" layer="1"/>
<smd name="35" x="4.2" y="2.25" dx="1.5" dy="0.3" layer="1"/>
<smd name="36" x="4.2" y="2.75" dx="1.5" dy="0.3" layer="1"/>
<smd name="37" x="2.75" y="4.2" dx="0.3" dy="1.5" layer="1"/>
<smd name="38" x="2.25" y="4.2" dx="0.3" dy="1.5" layer="1"/>
<smd name="39" x="1.75" y="4.2" dx="0.3" dy="1.5" layer="1"/>
<smd name="40" x="1.25" y="4.2" dx="0.3" dy="1.5" layer="1"/>
<smd name="41" x="0.75" y="4.2" dx="0.3" dy="1.5" layer="1"/>
<smd name="42" x="0.25" y="4.2" dx="0.3" dy="1.5" layer="1"/>
<smd name="43" x="-0.25" y="4.2" dx="0.3" dy="1.5" layer="1"/>
<smd name="44" x="-0.75" y="4.2" dx="0.3" dy="1.5" layer="1"/>
<smd name="45" x="-1.25" y="4.2" dx="0.3" dy="1.5" layer="1"/>
<smd name="46" x="-1.75" y="4.2" dx="0.3" dy="1.5" layer="1"/>
<smd name="47" x="-2.25" y="4.2" dx="0.3" dy="1.5" layer="1"/>
<smd name="48" x="-2.75" y="4.2" dx="0.3" dy="1.5" layer="1"/>
<text x="-3" y="5" size="1.27" layer="25">&gt;NAME</text>
<text x="-3" y="-6" size="1.27" layer="27">&gt;VALUE</text>
</package>
</packages>
<symbols>
<symbol name="STM32F103C8T6">
<description>STM32F103C8T6 Cortex-M3 72MHz, 64KB Flash, 20KB SRAM</description>
<wire x1="-30.48" y1="38.1" x2="30.48" y2="38.1" width="0.254" layer="94"/>
<wire x1="30.48" y1="38.1" x2="30.48" y2="-38.1" width="0.254" layer="94"/>
<wire x1="30.48" y1="-38.1" x2="-30.48" y2="-38.1" width="0.254" layer="94"/>
<wire x1="-30.48" y1="-38.1" x2="-30.48" y2="38.1" width="0.254" layer="94"/>
<text x="-30.48" y="40.64" size="1.778" layer="95">&gt;NAME</text>
<text x="-30.48" y="-40.64" size="1.778" layer="96">&gt;VALUE</text>
<!-- Power pins -->
<pin name="VDD" x="-33.02" y="35.56" length="short" direction="pwr"/>
<pin name="VDDA" x="-33.02" y="33.02" length="short" direction="pwr"/>
<pin name="VSS" x="-33.02" y="27.94" length="short" direction="pwr"/>
<pin name="VSSA" x="-33.02" y="25.4" length="short" direction="pwr"/>
<pin name="VBAT" x="-33.02" y="30.48" length="short" direction="pwr"/>
<pin name="NRST" x="-33.02" y="22.86" length="short" direction="in"/>
<!-- PORT A -->
<pin name="PA0" x="-33.02" y="17.78" length="short"/>
<pin name="PA1" x="-33.02" y="15.24" length="short"/>
<pin name="PA2" x="-33.02" y="12.7" length="short"/>
<pin name="PA3" x="-33.02" y="10.16" length="short"/>
<pin name="PA4" x="-33.02" y="7.62" length="short"/>
<pin name="PA5" x="-33.02" y="5.08" length="short"/>
<pin name="PA6" x="-33.02" y="2.54" length="short"/>
<pin name="PA7" x="-33.02" y="0" length="short"/>
<pin name="PA8" x="-33.02" y="-2.54" length="short"/>
<pin name="PA9/USART1_TX" x="-33.02" y="-5.08" length="short"/>
<pin name="PA10/USART1_RX" x="-33.02" y="-7.62" length="short"/>
<pin name="PA11" x="-33.02" y="-10.16" length="short"/>
<pin name="PA12" x="-33.02" y="-12.7" length="short"/>
<pin name="PA13/SWDIO" x="-33.02" y="-15.24" length="short"/>
<pin name="PA14/SWCLK" x="-33.02" y="-17.78" length="short"/>
<pin name="PA15" x="-33.02" y="-20.32" length="short"/>
<!-- PORT B -->
<pin name="PB0" x="33.02" y="17.78" length="short" rot="R180"/>
<pin name="PB1" x="33.02" y="15.24" length="short" rot="R180"/>
<pin name="PB6/I2C1_SCL" x="33.02" y="12.7" length="short" rot="R180"/>
<pin name="PB7/I2C1_SDA" x="33.02" y="10.16" length="short" rot="R180"/>
<pin name="PB8" x="33.02" y="7.62" length="short" rot="R180"/>
<pin name="PB9" x="33.02" y="5.08" length="short" rot="R180"/>
<pin name="PB10" x="33.02" y="2.54" length="short" rot="R180"/>
<pin name="PB11" x="33.02" y="0" length="short" rot="R180"/>
<pin name="PB12" x="33.02" y="-2.54" length="short" rot="R180"/>
<pin name="PB13" x="33.02" y="-5.08" length="short" rot="R180"/>
<pin name="PB14" x="33.02" y="-7.62" length="short" rot="R180"/>
<pin name="PB15" x="33.02" y="-10.16" length="short" rot="R180"/>
<!-- PORT C -->
<pin name="PC13" x="33.02" y="-15.24" length="short" rot="R180"/>
<pin name="PC14" x="33.02" y="-17.78" length="short" rot="R180"/>
<pin name="PC15" x="33.02" y="-20.32" length="short" rot="R180"/>
<!-- Boot/OSC -->
<pin name="BOOT0" x="-33.02" y="-25.4" length="short" direction="in"/>
<pin name="OSC_IN" x="-33.02" y="-27.94" length="short" direction="in"/>
<pin name="OSC_OUT" x="-33.02" y="-30.48" length="short" direction="out"/>
</symbol>
</symbols>
<devicesets>
<deviceset name="STM32F103C8T6" prefix="U">
<description>STM32F103C8T6 Cortex-M3, 64KB Flash, 20KB SRAM, LQFP-48</description>
<gates>
<gate name="G$1" symbol="STM32F103C8T6" x="0" y="0"/>
</gates>
<devices>
<device name="" package="LQFP48">
<connects>
<!-- Pin mapping: LQFP48 pin → symbol pin -->
<connect gate="G$1" pin="VDD" pad="1 24 36 48"/>
<connect gate="G$1" pin="VDDA" pad="9"/>
<connect gate="G$1" pin="VSS" pad="23 35 47"/>
<connect gate="G$1" pin="VSSA" pad="8"/>
<connect gate="G$1" pin="VBAT" pad="1"/>
<connect gate="G$1" pin="NRST" pad="7"/>
<connect gate="G$1" pin="PA0" pad="10"/>
<connect gate="G$1" pin="PA1" pad="11"/>
<connect gate="G$1" pin="PA2" pad="12"/>
<connect gate="G$1" pin="PA3" pad="13"/>
<connect gate="G$1" pin="PA4" pad="14"/>
<connect gate="G$1" pin="PA5" pad="15"/>
<connect gate="G$1" pin="PA6" pad="16"/>
<connect gate="G$1" pin="PA7" pad="17"/>
<connect gate="G$1" pin="PA8" pad="29"/>
<connect gate="G$1" pin="PA9/USART1_TX" pad="30"/>
<connect gate="G$1" pin="PA10/USART1_RX" pad="31"/>
<connect gate="G$1" pin="PA11" pad="32"/>
<connect gate="G$1" pin="PA12" pad="33"/>
<connect gate="G$1" pin="PA13/SWDIO" pad="34"/>
<connect gate="G$1" pin="PA14/SWCLK" pad="37"/>
<connect gate="G$1" pin="PA15" pad="38"/>
<connect gate="G$1" pin="PB0" pad="18"/>
<connect gate="G$1" pin="PB1" pad="19"/>
<connect gate="G$1" pin="PB6/I2C1_SCL" pad="42"/>
<connect gate="G$1" pin="PB7/I2C1_SDA" pad="43"/>
<connect gate="G$1" pin="PB8" pad="45"/>
<connect gate="G$1" pin="PB9" pad="46"/>
<connect gate="G$1" pin="PB10" pad="21"/>
<connect gate="G$1" pin="PB11" pad="22"/>
<connect gate="G$1" pin="PB12" pad="25"/>
<connect gate="G$1" pin="PB13" pad="26"/>
<connect gate="G$1" pin="PB14" pad="27"/>
<connect gate="G$1" pin="PB15" pad="28"/>
<connect gate="G$1" pin="PC13" pad="2"/>
<connect gate="G$1" pin="PC14" pad="3"/>
<connect gate="G$1" pin="PC15" pad="4"/>
<connect gate="G$1" pin="BOOT0" pad="44"/>
<connect gate="G$1" pin="OSC_IN" pad="5"/>
<connect gate="G$1" pin="OSC_OUT" pad="6"/>
</connects>
<technologies>
<technology name=""/>
</technologies>
</device>
</devices>
</deviceset>
</devicesets>
</library>
<!-- Passive components library -->
<library name="rcl">
<packages>
<package name="R0805">
<description>&lt;b&gt;RESISTOR&lt;/b&gt;&lt;p&gt;</description>
<wire x1="-0.41" y1="0.635" x2="0.41" y2="0.635" width="0.1524" layer="51"/>
<wire x1="-0.41" y1="-0.635" x2="0.41" y2="-0.635" width="0.1524" layer="51"/>
<smd name="1" x="-0.95" y="0" dx="1.3" dy="1.5" layer="1"/>
<smd name="2" x="0.95" y="0" dx="1.3" dy="1.5" layer="1"/>
<text x="-0.635" y="1.27" size="1.27" layer="25">&gt;NAME</text>
<text x="-0.635" y="-2.54" size="1.27" layer="27">&gt;VALUE</text>
</package>
<package name="C0805">
<description>&lt;b&gt;CAPACITOR&lt;/b&gt;&lt;p&gt;</description>
<wire x1="-1.973" y1="0.983" x2="1.973" y2="0.983" width="0.0508" layer="39"/>
<wire x1="1.973" y1="-0.983" x2="-1.973" y2="-0.983" width="0.0508" layer="39"/>
<smd name="1" x="-0.95" y="0" dx="1.3" dy="1.5" layer="1"/>
<smd name="2" x="0.95" y="0" dx="1.3" dy="1.5" layer="1"/>
<text x="-1.27" y="1.27" size="1.27" layer="25">&gt;NAME</text>
<text x="-1.27" y="-2.54" size="1.27" layer="27">&gt;VALUE</text>
</package>
</packages>
<symbols>
<symbol name="R">
<wire x1="-2.54" y1="0" x2="-2.413" y2="0" width="0.2032" layer="94"/>
<wire x1="-2.413" y1="0" x2="-2.159" y2="1.016" width="0.2032" layer="94"/>
<wire x1="-2.159" y1="1.016" x2="-1.651" y2="-1.016" width="0.2032" layer="94"/>
<wire x1="-1.651" y1="-1.016" x2="-1.143" y2="1.016" width="0.2032" layer="94"/>
<wire x1="-1.143" y1="1.016" x2="-0.635" y2="-1.016" width="0.2032" layer="94"/>
<wire x1="-0.635" y1="-1.016" x2="-0.127" y2="1.016" width="0.2032" layer="94"/>
<wire x1="-0.127" y1="1.016" x2="0.381" y2="-1.016" width="0.2032" layer="94"/>
<wire x1="0.381" y1="-1.016" x2="0.889" y2="1.016" width="0.2032" layer="94"/>
<wire x1="0.889" y1="1.016" x2="1.397" y2="-1.016" width="0.2032" layer="94"/>
<wire x1="1.397" y1="-1.016" x2="1.905" y2="0" width="0.2032" layer="94"/>
<wire x1="1.905" y1="0" x2="2.54" y2="0" width="0.2032" layer="94"/>
<text x="-3.81" y="1.4986" size="1.778" layer="95">&gt;NAME</text>
<text x="-3.81" y="-3.302" size="1.778" layer="96">&gt;VALUE</text>
<pin name="1" x="-5.08" y="0" visible="off" length="short" direction="pas"/>
<pin name="2" x="5.08" y="0" visible="off" length="short" direction="pas" rot="R180"/>
</symbol>
<symbol name="C">
<wire x1="0" y1="2.54" x2="0" y2="2.032" width="0.1524" layer="94"/>
<wire x1="0" y1="0" x2="0" y2="0.508" width="0.1524" layer="94"/>
<wire x1="0" y1="0.508" x2="-1.524" y2="0.508" width="0.254" layer="94"/>
<wire x1="0" y1="2.032" x2="-1.524" y2="2.032" width="0.254" layer="94"/>
<pin name="1" x="0" y="5.08" visible="off" length="short" direction="pas" rot="R270"/>
<pin name="2" x="0" y="-2.54" visible="off" length="short" direction="pas" rot="R90"/>
<text x="1.524" y="2.032" size="1.778" layer="95">&gt;NAME</text>
<text x="1.524" y="-0.508" size="1.778" layer="96">&gt;VALUE</text>
</symbol>
</symbols>
<devicesets>
<deviceset name="R" prefix="R" uservalue="yes">
<gates><gate name="G$1" symbol="R" x="0" y="0"/></gates>
<devices>
<device name="0805" package="R0805">
<connects>
<connect gate="G$1" pin="1" pad="1"/>
<connect gate="G$1" pin="2" pad="2"/>
</connects>
<technologies><technology name=""/></technologies>
</device>
</devices>
</deviceset>
<deviceset name="C" prefix="C" uservalue="yes">
<gates><gate name="G$1" symbol="C" x="0" y="0"/></gates>
<devices>
<device name="0805" package="C0805">
<connects>
<connect gate="G$1" pin="1" pad="1"/>
<connect gate="G$1" pin="2" pad="2"/>
</connects>
<technologies><technology name=""/></technologies>
</device>
</devices>
</deviceset>
</devicesets>
</library>
</libraries>
<parts>
<part name="U1" library="stm32f103" deviceset="STM32F103C8T6" value="STM32F103C8T6"/>
<!-- Decoupling capacitors -->
<part name="C1" library="rcl" deviceset="C" value="100nF"/>
<part name="C2" library="rcl" deviceset="C" value="100nF"/>
<part name="C3" library="rcl" deviceset="C" value="100nF"/>
<part name="C4" library="rcl" deviceset="C" value="4.7uF"/>
<!-- I2C pull-ups -->
<part name="R_SCL" library="rcl" deviceset="R" value="4.7k"/>
<part name="R_SDA" library="rcl" deviceset="R" value="4.7k"/>
<!-- 1-Wire pull-up -->
<part name="R_1W" library="rcl" deviceset="R" value="4.7k"/>
<!-- NPN transistors for NRI coin acceptor (6 channels) -->
<part name="Q1" library="transistor" deviceset="BC547" value="BC547"/>
<part name="Q2" library="transistor" deviceset="BC547" value="BC547"/>
<part name="Q3" library="transistor" deviceset="BC547" value="BC547"/>
<part name="Q4" library="transistor" deviceset="BC547" value="BC547"/>
<part name="Q5" library="transistor" deviceset="BC547" value="BC547"/>
<part name="Q6" library="transistor" deviceset="BC547" value="BC547"/>
<!-- NPN for GSM PWRKEY -->
<part name="Q_PWR" library="transistor" deviceset="BC547" value="BC547"/>
<!-- NPN base resistors -->
<part name="R_B1" library="rcl" deviceset="R" value="10k"/>
<part name="R_B2" library="rcl" deviceset="R" value="10k"/>
<part name="R_B3" library="rcl" deviceset="R" value="10k"/>
<part name="R_B4" library="rcl" deviceset="R" value="10k"/>
<part name="R_B5" library="rcl" deviceset="R" value="10k"/>
<part name="R_B6" library="rcl" deviceset="R" value="10k"/>
<!-- NRI collector pull-ups -->
<part name="R_C1" library="rcl" deviceset="R" value="10k"/>
<part name="R_C2" library="rcl" deviceset="R" value="10k"/>
<part name="R_C3" library="rcl" deviceset="R" value="10k"/>
<part name="R_C4" library="rcl" deviceset="R" value="10k"/>
<part name="R_C5" library="rcl" deviceset="R" value="10k"/>
<part name="R_C6" library="rcl" deviceset="R" value="10k"/>
<!-- Button pull-ups -->
<part name="R_BTN1" library="rcl" deviceset="R" value="10k"/>
<part name="R_BTN2" library="rcl" deviceset="R" value="10k"/>
<part name="R_BTN3" library="rcl" deviceset="R" value="10k"/>
<part name="R_BTN4" library="rcl" deviceset="R" value="10k"/>
<!-- LED resistors -->
<part name="R_LED_R" library="rcl" deviceset="R" value="1k"/>
<part name="R_LED_G" library="rcl" deviceset="R" value="1k"/>
<!-- GSM STATUS voltage divider -->
<part name="R_STS1" library="rcl" deviceset="R" value="1k"/>
<part name="R_STS2" library="rcl" deviceset="R" value="2k"/>
</parts>
<sheets>
<sheet>
<description>MCU + Power</description>
<plain>
<text x="0" y="0" size="2.54" layer="97">Бахилизатор v2.0 — STM32F103C8T6 Bluepill</text>
<text x="0" y="-2.54" size="1.778" layer="97">Схема 1 из 5: MCU + Питание</text>
</plain>
<instances>
<instance part="U1" gate="G$1" x="50.8" y="50.8"/>
</instances>
<nets>
<net name="VDD" class="0"/>
<net name="GND" class="0"/>
<net name="+3V3" class="0"/>
</nets>
</sheet>
<sheet>
<description>GSM SIM800L</description>
<plain>
<text x="0" y="0" size="2.54" layer="97">Схема 2: GSM SIM800L</text>
<text x="0" y="-2.54" size="1.778" layer="97">USART1: PA9(TX)→SIM800L RXD, PA10(RX)←SIM800L TXD</text>
<text x="0" y="-5.08" size="1.778" layer="97">PA0=PWRKEY(NPN), PA1=STATUS(делитель), PA2=DTR</text>
</plain>
<instances/>
<nets/>
</sheet>
<sheet>
<description>I2C — Display + EEPROM</description>
<plain>
<text x="0" y="0" size="2.54" layer="97">Схема 3: I2C — Дисплей + EEPROM</text>
<text x="0" y="-2.54" size="1.778" layer="97">I2C1: PB6(SCL), PB7(SDA) — 100kHz</text>
<text x="0" y="-5.08" size="1.778" layer="97">PCF8574(0x27) → HD44780, 24C08(0x50) EEPROM</text>
<text x="0" y="-7.62" size="1.778" layer="97">Pull-up 4.7kΩ на SDA/SCL к +3.3V</text>
</plain>
<instances/>
<nets/>
</sheet>
<sheet>
<description>Coin + Hopper</description>
<plain>
<text x="0" y="0" size="2.54" layer="97">Схема 4: NRI G-13.6000 + Хопперы</text>
<text x="0" y="-2.54" size="1.778" layer="97">Coin CH1-6: PB8-PB13 (NPN BC547, инверсия active-low→HIGH)</text>
<text x="0" y="-5.08" size="1.778" layer="97">Coin BLOCK: PB14 (прямое, active HIGH)</text>
<text x="0" y="-7.62" size="1.778" layer="97">Hopper A Enable: PB15, Sensor: TBD</text>
<text x="0" y="-10.16" size="1.778" layer="97">Hopper B Enable/Sensor: TBD</text>
</plain>
<instances/>
<nets/>
</sheet>
<sheet>
<description>Buttons + Doors + Misc</description>
<plain>
<text x="0" y="0" size="2.54" layer="97">Схема 5: Кнопки + Двери + Прочее</text>
<text x="0" y="-2.54" size="1.778" layer="97">PREV=PA3, NEXT=PA4, OK=PA5, CANCEL=PA6</text>
<text x="0" y="-5.08" size="1.778" layer="97">Door1=PA7, Door2=PA8</text>
<text x="0" y="-7.62" size="1.778" layer="97">iButton: PA11 (1-Wire, 4.7kΩ pull-up)</text>
<text x="0" y="-10.16" size="1.778" layer="97">LED_RED=PC15, LED_GREEN=PB0</text>
<text x="0" y="-12.7" size="1.778" layer="97">Power Fail: PB1</text>
</plain>
<instances/>
<nets/>
</sheet>
</sheets>
</schematic>
</drawing>
</eagle>