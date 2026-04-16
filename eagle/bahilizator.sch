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
<!-- ========== LIBRARY: STM32F103 ========== -->
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
<wire x1="-30.48" y1="38.1" x2="30.48" y2="38.1" width="0.254" layer="94"/>
<wire x1="30.48" y1="38.1" x2="30.48" y2="-38.1" width="0.254" layer="94"/>
<wire x1="30.48" y1="-38.1" x2="-30.48" y2="-38.1" width="0.254" layer="94"/>
<wire x1="-30.48" y1="-38.1" x2="-30.48" y2="38.1" width="0.254" layer="94"/>
<text x="-30.48" y="40.64" size="1.778" layer="95">&gt;NAME</text>
<text x="-30.48" y="-40.64" size="1.778" layer="96">&gt;VALUE</text>
<pin name="VDD" x="-33.02" y="35.56" length="short" direction="pwr"/>
<pin name="VDDA" x="-33.02" y="33.02" length="short" direction="pwr"/>
<pin name="VSS" x="-33.02" y="27.94" length="short" direction="pwr"/>
<pin name="VSSA" x="-33.02" y="25.4" length="short" direction="pwr"/>
<pin name="VBAT" x="-33.02" y="30.48" length="short" direction="pwr"/>
<pin name="NRST" x="-33.02" y="22.86" length="short" direction="in"/>
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
<pin name="PC13" x="33.02" y="-15.24" length="short" rot="R180"/>
<pin name="PC14" x="33.02" y="-17.78" length="short" rot="R180"/>
<pin name="PC15" x="33.02" y="-20.32" length="short" rot="R180"/>
<pin name="BOOT0" x="-33.02" y="-25.4" length="short" direction="in"/>
<pin name="OSC_IN" x="-33.02" y="-27.94" length="short" direction="in"/>
<pin name="OSC_OUT" x="-33.02" y="-30.48" length="short" direction="out"/>
</symbol>
</symbols>
<devicesets>
<deviceset name="STM32F103C8T6" prefix="U">
<description>STM32F103C8T6 Cortex-M3, LQFP-48</description>
<gates>
<gate name="G$1" symbol="STM32F103C8T6" x="0" y="0"/>
</gates>
<devices>
<device name="" package="LQFP48">
<connects>
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
</library>
<!-- ========== LIBRARY: Passive R/C/L ========== -->
<library name="rcl">
<packages>
<package name="R0805">
<wire x1="-0.41" y1="0.635" x2="0.41" y2="0.635" width="0.1524" layer="51"/>
<wire x1="-0.41" y1="-0.635" x2="0.41" y2="-0.635" width="0.1524" layer="51"/>
<smd name="1" x="-0.95" y="0" dx="1.3" dy="1.5" layer="1"/>
<smd name="2" x="0.95" y="0" dx="1.3" dy="1.5" layer="1"/>
<text x="-0.635" y="1.27" size="1.27" layer="25">&gt;NAME</text>
<text x="-0.635" y="-2.54" size="1.27" layer="27">&gt;VALUE</text>
</package>
<package name="C0805">
<wire x1="-1.973" y1="0.983" x2="1.973" y2="0.983" width="0.0508" layer="39"/>
<wire x1="1.973" y1="-0.983" x2="-1.973" y2="-0.983" width="0.0508" layer="39"/>
<smd name="1" x="-0.95" y="0" dx="1.3" dy="1.5" layer="1"/>
<smd name="2" x="0.95" y="0" dx="1.3" dy="1.5" layer="1"/>
<text x="-1.27" y="1.27" size="1.27" layer="25">&gt;NAME</text>
<text x="-1.27" y="-2.54" size="1.27" layer="27">&gt;VALUE</text>
</package>
<package name="C1206">
<wire x1="-2.473" y1="1.083" x2="2.473" y2="1.083" width="0.0508" layer="39"/>
<wire x1="2.473" y1="-1.083" x2="-2.473" y2="-1.083" width="0.0508" layer="39"/>
<smd name="1" x="-1.4" y="0" dx="1.6" dy="1.8" layer="1"/>
<smd name="2" x="1.4" y="0" dx="1.6" dy="1.8" layer="1"/>
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
<symbol name="L">
<wire x1="-2.54" y1="0" x2="-1.27" y2="0" width="0.254" layer="94"/>
<wire x1="-1.27" y1="0" x2="-1.27" y2="1.27" width="0.254" layer="94" curve="-180"/>
<wire x1="-1.27" y1="1.27" x2="-1.27" y2="2.54" width="0.254" layer="94" curve="-180"/>
<wire x1="-1.27" y1="2.54" x2="-1.27" y2="0" width="0.254" layer="94" curve="-180"/>
<wire x1="-1.27" y1="0" x2="1.27" y2="0" width="0.254" layer="94"/>
<wire x1="1.27" y1="0" x2="1.27" y2="1.27" width="0.254" layer="94" curve="-180"/>
<wire x1="1.27" y1="1.27" x2="1.27" y2="2.54" width="0.254" layer="94" curve="-180"/>
<wire x1="1.27" y1="2.54" x2="1.27" y2="0" width="0.254" layer="94" curve="-180"/>
<wire x1="1.27" y1="0" x2="2.54" y2="0" width="0.254" layer="94"/>
<text x="-2.54" y="3.81" size="1.778" layer="95">&gt;NAME</text>
<text x="-2.54" y="-2.54" size="1.778" layer="96">&gt;VALUE</text>
<pin name="1" x="-5.08" y="0" visible="off" length="short" direction="pas"/>
<pin name="2" x="5.08" y="0" visible="off" length="short" direction="pas" rot="R180"/>
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
<device name="1206" package="C1206">
<connects>
<connect gate="G$1" pin="1" pad="1"/>
<connect gate="G$1" pin="2" pad="2"/>
</connects>
<technologies><technology name=""/></technologies>
</device>
</devices>
</deviceset>
<deviceset name="L" prefix="L" uservalue="yes">
<gates><gate name="G$1" symbol="L" x="0" y="0"/></gates>
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
</library>
<!-- ========== LIBRARY: Transistor NPN ========== -->
<library name="transistor">
<packages>
<package name="SOT23">
<description>SOT-23-3</description>
<wire x1="1.4224" y1="0.6604" x2="1.4224" y2="-0.6604" width="0.1524" layer="51"/>
<wire x1="1.4224" y1="-0.6604" x2="-1.4224" y2="-0.6604" width="0.1524" layer="51"/>
<wire x1="-1.4224" y1="-0.6604" x2="-1.4224" y2="0.6604" width="0.1524" layer="51"/>
<wire x1="-1.4224" y1="0.6604" x2="1.4224" y2="0.6604" width="0.1524" layer="51"/>
<smd name="B" x="-0.95" y="-1.1" dx="0.8" dy="0.9" layer="1"/>
<smd name="E" x="0.95" y="-1.1" dx="0.8" dy="0.9" layer="1"/>
<smd name="C" x="0" y="1.1" dx="0.8" dy="0.9" layer="1"/>
<text x="-1.905" y="1.905" size="1.27" layer="25">&gt;NAME</text>
<text x="-1.905" y="-3.175" size="1.27" layer="27">&gt;VALUE</text>
</package>
</packages>
<symbols>
<symbol name="NPN">
<wire x1="2.54" y1="2.54" x2="0.508" y2="1.524" width="0.1524" layer="94"/>
<wire x1="0" y1="5.08" x2="0.508" y2="3.556" width="0.1524" layer="94"/>
<wire x1="0" y1="0" x2="0.508" y2="1.524" width="0.1524" layer="94"/>
<wire x1="0" y1="0" x2="2.032" y2="1.524" width="0.1524" layer="94"/>
<pin name="B" x="-2.54" y="0" visible="off" length="short" direction="pas"/>
<pin name="C" x="0" y="7.62" visible="off" length="short" direction="pas" rot="R270"/>
<pin name="E" x="0" y="-2.54" visible="off" length="short" direction="pas" rot="R90"/>
<text x="2.54" y="5.08" size="1.778" layer="95">&gt;NAME</text>
<text x="2.54" y="2.54" size="1.778" layer="96">&gt;VALUE</text>
</symbol>
</symbols>
<devicesets>
<deviceset name="BC847" prefix="Q">
<description>NPN BC847 SOT-23 (SMD BC547 equivalent)</description>
<gates><gate name="G$1" symbol="NPN" x="0" y="0"/></gates>
<devices>
<device name="" package="SOT23">
<connects>
<connect gate="G$1" pin="B" pad="B"/>
<connect gate="G$1" pin="C" pad="C"/>
<connect gate="G$1" pin="E" pad="E"/>
</connects>
<technologies><technology name=""/></technologies>
</device>
</devices>
</deviceset>
</library>
<!-- ========== LIBRARY: Crystal ========== -->
<library name="crystal">
<packages>
<package name="HC49_SMD">
<description>HC49/S SMD Crystal</description>
<wire x1="-3.048" y1="1.651" x2="3.048" y2="1.651" width="0.254" layer="21"/>
<wire x1="-3.048" y1="-1.651" x2="3.048" y2="-1.651" width="0.254" layer="21"/>
<smd name="1" x="-4.25" y="0" dx="3.5" dy="2.5" layer="1"/>
<smd name="2" x="4.25" y="0" dx="3.5" dy="2.5" layer="1"/>
<text x="-3.81" y="2.54" size="1.27" layer="25">&gt;NAME</text>
<text x="-3.81" y="-3.81" size="1.27" layer="27">&gt;VALUE</text>
</package>
</packages>
<symbols>
<symbol name="Q">
<wire x1="1.016" y1="0" x2="1.27" y2="0" width="0.1524" layer="94"/>
<wire x1="-1.27" y1="0" x2="-1.016" y2="0" width="0.1524" layer="94"/>
<rectangle x1="-1.016" y1="-2.54" x2="1.016" y2="2.54" layer="94"/>
<wire x1="-1.27" y1="1.27" x2="1.27" y2="1.27" width="0.1524" layer="94"/>
<wire x1="-1.27" y1="-1.27" x2="1.27" y2="-1.27" width="0.1524" layer="94"/>
<pin name="1" x="-5.08" y="0" visible="off" length="short" direction="pas"/>
<pin name="2" x="5.08" y="0" visible="off" length="short" direction="pas" rot="R180"/>
<text x="-2.54" y="3.81" size="1.778" layer="95">&gt;NAME</text>
<text x="-2.54" y="-5.08" size="1.778" layer="96">&gt;VALUE</text>
</symbol>
</symbols>
<devicesets>
<deviceset name="8MHZ" prefix="Y">
<description>8MHz Crystal HC49/S</description>
<gates><gate name="G$1" symbol="Q" x="0" y="0"/></gates>
<devices>
<device name="" package="HC49_SMD">
<connects>
<connect gate="G$1" pin="1" pad="1"/>
<connect gate="G$1" pin="2" pad="2"/>
</connects>
<technologies><technology name=""/></technologies>
</device>
</devices>
</deviceset>
</library>
<!-- ========== LIBRARY: Connectors ========== -->
<library name="con-jst-xh">
<packages>
<package name="JST-XH-02">
<description>JST-XH 2-pin header, top entry</description>
<wire x1="-3.7" y1="-2.35" x2="-3.7" y2="3.4" width="0.15" layer="21"/>
<wire x1="-3.7" y1="3.4" x2="3.7" y2="3.4" width="0.15" layer="21"/>
<wire x1="3.7" y1="3.4" x2="3.7" y2="-2.35" width="0.15" layer="21"/>
<wire x1="3.7" y1="-2.35" x2="-3.7" y2="-2.35" width="0.15" layer="21"/>
<pad name="1" x="-1.25" y="0" drill="0.8" diameter="1.4"/>
<pad name="2" x="1.25" y="0" drill="0.8" diameter="1.4"/>
<text x="-3.81" y="3.81" size="1.27" layer="25">&gt;NAME</text>
<text x="-3.81" y="-3.81" size="1.27" layer="27">&gt;VALUE</text>
</package>
<package name="JST-XH-03">
<description>JST-XH 3-pin header</description>
<wire x1="-4.95" y1="-2.35" x2="-4.95" y2="3.4" width="0.15" layer="21"/>
<wire x1="-4.95" y1="3.4" x2="4.95" y2="3.4" width="0.15" layer="21"/>
<wire x1="4.95" y1="3.4" x2="4.95" y2="-2.35" width="0.15" layer="21"/>
<wire x1="4.95" y1="-2.35" x2="-4.95" y2="-2.35" width="0.15" layer="21"/>
<pad name="1" x="-2.5" y="0" drill="0.8" diameter="1.4"/>
<pad name="2" x="0" y="0" drill="0.8" diameter="1.4"/>
<pad name="3" x="2.5" y="0" drill="0.8" diameter="1.4"/>
<text x="-5.08" y="3.81" size="1.27" layer="25">&gt;NAME</text>
<text x="-5.08" y="-3.81" size="1.27" layer="27">&gt;VALUE</text>
</package>
<package name="JST-XH-04">
<description>JST-XH 4-pin header</description>
<wire x1="-6.2" y1="-2.35" x2="-6.2" y2="3.4" width="0.15" layer="21"/>
<wire x1="-6.2" y1="3.4" x2="6.2" y2="3.4" width="0.15" layer="21"/>
<wire x1="6.2" y1="3.4" x2="6.2" y2="-2.35" width="0.15" layer="21"/>
<wire x1="6.2" y1="-2.35" x2="-6.2" y2="-2.35" width="0.15" layer="21"/>
<pad name="1" x="-3.75" y="0" drill="0.8" diameter="1.4"/>
<pad name="2" x="-1.25" y="0" drill="0.8" diameter="1.4"/>
<pad name="3" x="1.25" y="0" drill="0.8" diameter="1.4"/>
<pad name="4" x="3.75" y="0" drill="0.8" diameter="1.4"/>
<text x="-6.35" y="3.81" size="1.27" layer="25">&gt;NAME</text>
<text x="-6.35" y="-3.81" size="1.27" layer="27">&gt;VALUE</text>
</package>
<package name="JST-XH-05">
<description>JST-XH 5-pin header</description>
<wire x1="-7.45" y1="-2.35" x2="-7.45" y2="3.4" width="0.15" layer="21"/>
<wire x1="-7.45" y1="3.4" x2="7.45" y2="3.4" width="0.15" layer="21"/>
<wire x1="7.45" y1="3.4" x2="7.45" y2="-2.35" width="0.15" layer="21"/>
<wire x1="7.45" y1="-2.35" x2="-7.45" y2="-2.35" width="0.15" layer="21"/>
<pad name="1" x="-5" y="0" drill="0.8" diameter="1.4"/>
<pad name="2" x="-2.5" y="0" drill="0.8" diameter="1.4"/>
<pad name="3" x="0" y="0" drill="0.8" diameter="1.4"/>
<pad name="4" x="2.5" y="0" drill="0.8" diameter="1.4"/>
<pad name="5" x="5" y="0" drill="0.8" diameter="1.4"/>
<text x="-7.62" y="3.81" size="1.27" layer="25">&gt;NAME</text>
<text x="-7.62" y="-3.81" size="1.27" layer="27">&gt;VALUE</text>
</package>
<package name="JST-XH-10">
<description>JST-XH 10-pin header</description>
<wire x1="-14.95" y1="-2.35" x2="-14.95" y2="3.4" width="0.15" layer="21"/>
<wire x1="-14.95" y1="3.4" x2="14.95" y2="3.4" width="0.15" layer="21"/>
<wire x1="14.95" y1="3.4" x2="14.95" y2="-2.35" width="0.15" layer="21"/>
<wire x1="14.95" y1="-2.35" x2="-14.95" y2="-2.35" width="0.15" layer="21"/>
<pad name="1" x="-11.25" y="0" drill="0.8" diameter="1.4"/>
<pad name="2" x="-8.75" y="0" drill="0.8" diameter="1.4"/>
<pad name="3" x="-6.25" y="0" drill="0.8" diameter="1.4"/>
<pad name="4" x="-3.75" y="0" drill="0.8" diameter="1.4"/>
<pad name="5" x="-1.25" y="0" drill="0.8" diameter="1.4"/>
<pad name="6" x="1.25" y="0" drill="0.8" diameter="1.4"/>
<pad name="7" x="3.75" y="0" drill="0.8" diameter="1.4"/>
<pad name="8" x="6.25" y="0" drill="0.8" diameter="1.4"/>
<pad name="9" x="8.75" y="0" drill="0.8" diameter="1.4"/>
<pad name="10" x="11.25" y="0" drill="0.8" diameter="1.4"/>
<text x="-15.24" y="3.81" size="1.27" layer="25">&gt;NAME</text>
<text x="-15.24" y="-3.81" size="1.27" layer="27">&gt;VALUE</text>
</package>
</packages>
<symbols>
<symbol name="JST-XH-02">
<pin name="1" x="-5.08" y="2.54" visible="pad" length="short"/>
<pin name="2" x="-5.08" y="0" visible="pad" length="short"/>
<wire x1="-2.54" y1="5.08" x2="2.54" y2="5.08" width="0.254" layer="94"/>
<wire x1="2.54" y1="5.08" x2="2.54" y2="-2.54" width="0.254" layer="94"/>
<wire x1="2.54" y1="-2.54" x2="-2.54" y2="-2.54" width="0.254" layer="94"/>
<wire x1="-2.54" y1="-2.54" x2="-2.54" y2="5.08" width="0.254" layer="94"/>
<text x="-2.54" y="5.588" size="1.778" layer="95">&gt;NAME</text>
<text x="-2.54" y="-4.572" size="1.778" layer="96">&gt;VALUE</text>
</symbol>
<symbol name="JST-XH-03">
<pin name="1" x="-5.08" y="5.08" visible="pad" length="short"/>
<pin name="2" x="-5.08" y="2.54" visible="pad" length="short"/>
<pin name="3" x="-5.08" y="0" visible="pad" length="short"/>
<wire x1="-2.54" y1="7.62" x2="2.54" y2="7.62" width="0.254" layer="94"/>
<wire x1="2.54" y1="7.62" x2="2.54" y2="-2.54" width="0.254" layer="94"/>
<wire x1="2.54" y1="-2.54" x2="-2.54" y2="-2.54" width="0.254" layer="94"/>
<wire x1="-2.54" y1="-2.54" x2="-2.54" y2="7.62" width="0.254" layer="94"/>
<text x="-2.54" y="8.128" size="1.778" layer="95">&gt;NAME</text>
<text x="-2.54" y="-4.572" size="1.778" layer="96">&gt;VALUE</text>
</symbol>
<symbol name="JST-XH-04">
<pin name="1" x="-5.08" y="7.62" visible="pad" length="short"/>
<pin name="2" x="-5.08" y="5.08" visible="pad" length="short"/>
<pin name="3" x="-5.08" y="2.54" visible="pad" length="short"/>
<pin name="4" x="-5.08" y="0" visible="pad" length="short"/>
<wire x1="-2.54" y1="10.16" x2="2.54" y2="10.16" width="0.254" layer="94"/>
<wire x1="2.54" y1="10.16" x2="2.54" y2="-2.54" width="0.254" layer="94"/>
<wire x1="2.54" y1="-2.54" x2="-2.54" y2="-2.54" width="0.254" layer="94"/>
<wire x1="-2.54" y1="-2.54" x2="-2.54" y2="10.16" width="0.254" layer="94"/>
<text x="-2.54" y="10.668" size="1.778" layer="95">&gt;NAME</text>
<text x="-2.54" y="-4.572" size="1.778" layer="96">&gt;VALUE</text>
</symbol>
<symbol name="JST-XH-05">
<pin name="1" x="-5.08" y="10.16" visible="pad" length="short"/>
<pin name="2" x="-5.08" y="7.62" visible="pad" length="short"/>
<pin name="3" x="-5.08" y="5.08" visible="pad" length="short"/>
<pin name="4" x="-5.08" y="2.54" visible="pad" length="short"/>
<pin name="5" x="-5.08" y="0" visible="pad" length="short"/>
<wire x1="-2.54" y1="12.7" x2="2.54" y2="12.7" width="0.254" layer="94"/>
<wire x1="2.54" y1="12.7" x2="2.54" y2="-2.54" width="0.254" layer="94"/>
<wire x1="2.54" y1="-2.54" x2="-2.54" y2="-2.54" width="0.254" layer="94"/>
<wire x1="-2.54" y1="-2.54" x2="-2.54" y2="12.7" width="0.254" layer="94"/>
<text x="-2.54" y="13.208" size="1.778" layer="95">&gt;NAME</text>
<text x="-2.54" y="-4.572" size="1.778" layer="96">&gt;VALUE</text>
</symbol>
<symbol name="JST-XH-10">
<pin name="1" x="-5.08" y="22.86" visible="pad" length="short"/>
<pin name="2" x="-5.08" y="20.32" visible="pad" length="short"/>
<pin name="3" x="-5.08" y="17.78" visible="pad" length="short"/>
<pin name="4" x="-5.08" y="15.24" visible="pad" length="short"/>
<pin name="5" x="-5.08" y="12.7" visible="pad" length="short"/>
<pin name="6" x="-5.08" y="10.16" visible="pad" length="short"/>
<pin name="7" x="-5.08" y="7.62" visible="pad" length="short"/>
<pin name="8" x="-5.08" y="5.08" visible="pad" length="short"/>
<pin name="9" x="-5.08" y="2.54" visible="pad" length="short"/>
<pin name="10" x="-5.08" y="0" visible="pad" length="short"/>
<wire x1="-2.54" y1="25.4" x2="2.54" y2="25.4" width="0.254" layer="94"/>
<wire x1="2.54" y1="25.4" x2="2.54" y2="-2.54" width="0.254" layer="94"/>
<wire x1="2.54" y1="-2.54" x2="-2.54" y2="-2.54" width="0.254" layer="94"/>
<wire x1="-2.54" y1="-2.54" x2="-2.54" y2="25.4" width="0.254" layer="94"/>
<text x="-2.54" y="25.908" size="1.778" layer="95">&gt;NAME</text>
<text x="-2.54" y="-4.572" size="1.778" layer="96">&gt;VALUE</text>
</symbol>
</symbols>
<devicesets>
<deviceset name="JST-XH-02" prefix="J">
<gates><gate name="G$1" symbol="JST-XH-02" x="0" y="0"/></gates>
<devices>
<device name="" package="JST-XH-02">
<connects>
<connect gate="G$1" pin="1" pad="1"/>
<connect gate="G$1" pin="2" pad="2"/>
</connects>
<technologies><technology name=""/></technologies>
</device>
</devices>
</deviceset>
<deviceset name="JST-XH-03" prefix="J">
<gates><gate name="G$1" symbol="JST-XH-03" x="0" y="0"/></gates>
<devices>
<device name="" package="JST-XH-03">
<connects>
<connect gate="G$1" pin="1" pad="1"/>
<connect gate="G$1" pin="2" pad="2"/>
<connect gate="G$1" pin="3" pad="3"/>
</connects>
<technologies><technology name=""/></technologies>
</device>
</devices>
</deviceset>
<deviceset name="JST-XH-04" prefix="J">
<gates><gate name="G$1" symbol="JST-XH-04" x="0" y="0"/></gates>
<devices>
<device name="" package="JST-XH-04">
<connects>
<connect gate="G$1" pin="1" pad="1"/>
<connect gate="G$1" pin="2" pad="2"/>
<connect gate="G$1" pin="3" pad="3"/>
<connect gate="G$1" pin="4" pad="4"/>
</connects>
<technologies><technology name=""/></technologies>
</device>
</devices>
</deviceset>
<deviceset name="JST-XH-05" prefix="J">
<gates><gate name="G$1" symbol="JST-XH-05" x="0" y="0"/></gates>
<devices>
<device name="" package="JST-XH-05">
<connects>
<connect gate="G$1" pin="1" pad="1"/>
<connect gate="G$1" pin="2" pad="2"/>
<connect gate="G$1" pin="3" pad="3"/>
<connect gate="G$1" pin="4" pad="4"/>
<connect gate="G$1" pin="5" pad="5"/>
</connects>
<technologies><technology name=""/></technologies>
</device>
</devices>
</deviceset>
<deviceset name="JST-XH-10" prefix="J">
<gates><gate name="G$1" symbol="JST-XH-10" x="0" y="0"/></gates>
<devices>
<device name="" package="JST-XH-10">
<connects>
<connect gate="G$1" pin="1" pad="1"/>
<connect gate="G$1" pin="2" pad="2"/>
<connect gate="G$1" pin="3" pad="3"/>
<connect gate="G$1" pin="4" pad="4"/>
<connect gate="G$1" pin="5" pad="5"/>
<connect gate="G$1" pin="6" pad="6"/>
<connect gate="G$1" pin="7" pad="7"/>
<connect gate="G$1" pin="8" pad="8"/>
<connect gate="G$1" pin="9" pad="9"/>
<connect gate="G$1" pin="10" pad="10"/>
</connects>
<technologies><technology name=""/></technologies>
</device>
</devices>
</deviceset>
</library>
<!-- ========== LIBRARY: PCF8574 ========== -->
<library name="pcf8574">
<packages>
<package name="DIL16">
<description>DIP-16 PCF8574</description>
<wire x1="-10.16" y1="-1.27" x2="-10.16" y2="6.35" width="0.15" layer="21"/>
<wire x1="-10.16" y1="6.35" x2="10.16" y2="6.35" width="0.15" layer="21"/>
<wire x1="10.16" y1="6.35" x2="10.16" y2="-1.27" width="0.15" layer="21"/>
<wire x1="10.16" y1="-1.27" x2="-10.16" y2="-1.27" width="0.15" layer="21"/>
<pad name="1" x="-8.89" y="0" drill="0.8" diameter="1.4" shape="long" rot="R90"/>
<pad name="2" x="-6.35" y="0" drill="0.8" diameter="1.4" shape="long" rot="R90"/>
<pad name="3" x="-3.81" y="0" drill="0.8" diameter="1.4" shape="long" rot="R90"/>
<pad name="4" x="-1.27" y="0" drill="0.8" diameter="1.4" shape="long" rot="R90"/>
<pad name="5" x="1.27" y="0" drill="0.8" diameter="1.4" shape="long" rot="R90"/>
<pad name="6" x="3.81" y="0" drill="0.8" diameter="1.4" shape="long" rot="R90"/>
<pad name="7" x="6.35" y="0" drill="0.8" diameter="1.4" shape="long" rot="R90"/>
<pad name="8" x="8.89" y="0" drill="0.8" diameter="1.4" shape="long" rot="R90"/>
<pad name="16" x="-8.89" y="5.08" drill="0.8" diameter="1.4" shape="long" rot="R90"/>
<pad name="15" x="-6.35" y="5.08" drill="0.8" diameter="1.4" shape="long" rot="R90"/>
<pad name="14" x="-3.81" y="5.08" drill="0.8" diameter="1.4" shape="long" rot="R90"/>
<pad name="13" x="-1.27" y="5.08" drill="0.8" diameter="1.4" shape="long" rot="R90"/>
<pad name="12" x="1.27" y="5.08" drill="0.8" diameter="1.4" shape="long" rot="R90"/>
<pad name="11" x="3.81" y="5.08" drill="0.8" diameter="1.4" shape="long" rot="R90"/>
<pad name="10" x="6.35" y="5.08" drill="0.8" diameter="1.4" shape="long" rot="R90"/>
<pad name="9" x="8.89" y="5.08" drill="0.8" diameter="1.4" shape="long" rot="R90"/>
<text x="-10.16" y="6.858" size="1.778" layer="25">&gt;NAME</text>
<text x="-10.16" y="-3.302" size="1.778" layer="27">&gt;VALUE</text>
</package>
</packages>
<symbols>
<symbol name="PCF8574">
<wire x1="-10.16" y1="15.24" x2="10.16" y2="15.24" width="0.254" layer="94"/>
<wire x1="10.16" y1="15.24" x2="10.16" y2="-12.7" width="0.254" layer="94"/>
<wire x1="10.16" y1="-12.7" x2="-10.16" y2="-12.7" width="0.254" layer="94"/>
<wire x1="-10.16" y1="-12.7" x2="-10.16" y2="15.24" width="0.254" layer="94"/>
<pin name="A0" x="-12.7" y="12.7" length="short"/>
<pin name="A1" x="-12.7" y="10.16" length="short"/>
<pin name="A2" x="-12.7" y="7.62" length="short"/>
<pin name="P0" x="12.7" y="12.7" length="short" rot="R180"/>
<pin name="P1" x="12.7" y="10.16" length="short" rot="R180"/>
<pin name="P2" x="12.7" y="7.62" length="short" rot="R180"/>
<pin name="P3" x="12.7" y="5.08" length="short" rot="R180"/>
<pin name="P4" x="12.7" y="2.54" length="short" rot="R180"/>
<pin name="P5" x="12.7" y="0" length="short" rot="R180"/>
<pin name="P6" x="12.7" y="-2.54" length="short" rot="R180"/>
<pin name="P7" x="12.7" y="-5.08" length="short" rot="R180"/>
<pin name="SCL" x="-12.7" y="-2.54" length="short"/>
<pin name="SDA" x="-12.7" y="-5.08" length="short"/>
<pin name="INT" x="-12.7" y="-7.62" length="short"/>
<pin name="VDD" x="12.7" y="-10.16" length="short" direction="pwr" rot="R180"/>
<pin name="VSS" x="12.7" y="-7.62" length="short" direction="pwr" rot="R180"/>
<text x="-10.16" y="17.78" size="1.778" layer="95">&gt;NAME</text>
<text x="-10.16" y="-15.24" size="1.778" layer="96">&gt;VALUE</text>
</symbol>
</symbols>
<devicesets>
<deviceset name="PCF8574" prefix="U">
<description>I2C to 8-bit parallel expander</description>
<gates><gate name="G$1" symbol="PCF8574" x="0" y="0"/></gates>
<devices>
<device name="" package="DIL16">
<connects>
<connect gate="G$1" pin="A0" pad="1"/>
<connect gate="G$1" pin="A1" pad="2"/>
<connect gate="G$1" pin="A2" pad="3"/>
<connect gate="G$1" pin="P0" pad="4"/>
<connect gate="G$1" pin="P1" pad="5"/>
<connect gate="G$1" pin="P2" pad="6"/>
<connect gate="G$1" pin="P3" pad="7"/>
<connect gate="G$1" pin="VSS" pad="8"/>
<connect gate="G$1" pin="P4" pad="9"/>
<connect gate="G$1" pin="P5" pad="10"/>
<connect gate="G$1" pin="P6" pad="11"/>
<connect gate="G$1" pin="P7" pad="12"/>
<connect gate="G$1" pin="INT" pad="13"/>
<connect gate="G$1" pin="SCL" pad="14"/>
<connect gate="G$1" pin="SDA" pad="15"/>
<connect gate="G$1" pin="VDD" pad="16"/>
</connects>
<technologies><technology name=""/></technologies>
</device>
</devices>
</deviceset>
</library>
<!-- ========== LIBRARY: 24C08 EEPROM ========== -->
<library name="eeprom">
<packages>
<package name="DIL08">
<description>DIP-8</description>
<wire x1="-5.08" y1="-1.27" x2="-5.08" y2="3.81" width="0.15" layer="21"/>
<wire x1="-5.08" y1="3.81" x2="5.08" y2="3.81" width="0.15" layer="21"/>
<wire x1="5.08" y1="3.81" x2="5.08" y2="-1.27" width="0.15" layer="21"/>
<wire x1="5.08" y1="-1.27" x2="-5.08" y2="-1.27" width="0.15" layer="21"/>
<pad name="1" x="-3.81" y="0" drill="0.8" diameter="1.4" shape="long" rot="R90"/>
<pad name="2" x="-1.27" y="0" drill="0.8" diameter="1.4" shape="long" rot="R90"/>
<pad name="3" x="1.27" y="0" drill="0.8" diameter="1.4" shape="long" rot="R90"/>
<pad name="4" x="3.81" y="0" drill="0.8" diameter="1.4" shape="long" rot="R90"/>
<pad name="5" x="3.81" y="2.54" drill="0.8" diameter="1.4" shape="long" rot="R90"/>
<pad name="6" x="1.27" y="2.54" drill="0.8" diameter="1.4" shape="long" rot="R90"/>
<pad name="7" x="-1.27" y="2.54" drill="0.8" diameter="1.4" shape="long" rot="R90"/>
<pad name="8" x="-3.81" y="2.54" drill="0.8" diameter="1.4" shape="long" rot="R90"/>
<text x="-5.08" y="4.318" size="1.778" layer="25">&gt;NAME</text>
<text x="-5.08" y="-3.302" size="1.778" layer="27">&gt;VALUE</text>
</package>
</packages>
<symbols>
<symbol name="24C08">
<wire x1="-7.62" y1="7.62" x2="7.62" y2="7.62" width="0.254" layer="94"/>
<wire x1="7.62" y1="7.62" x2="7.62" y2="-7.62" width="0.254" layer="94"/>
<wire x1="7.62" y1="-7.62" x2="-7.62" y2="-7.62" width="0.254" layer="94"/>
<wire x1="-7.62" y1="-7.62" x2="-7.62" y2="7.62" width="0.254" layer="94"/>
<pin name="A0" x="-10.16" y="5.08" length="short"/>
<pin name="A1" x="-10.16" y="2.54" length="short"/>
<pin name="A2" x="-10.16" y="0" length="short"/>
<pin name="VSS" x="-10.16" y="-5.08" length="short" direction="pwr"/>
<pin name="SDA" x="10.16" y="5.08" length="short" rot="R180"/>
<pin name="SCL" x="10.16" y="2.54" length="short" rot="R180"/>
<pin name="WP" x="10.16" y="-2.54" length="short" rot="R180"/>
<pin name="VDD" x="10.16" y="-5.08" length="short" direction="pwr" rot="R180"/>
<text x="-7.62" y="10.16" size="1.778" layer="95">&gt;NAME</text>
<text x="-7.62" y="-10.16" size="1.778" layer="96">&gt;VALUE</text>
</symbol>
</symbols>
<devicesets>
<deviceset name="24C08" prefix="U">
<description>24C08 8Kbit I2C EEPROM</description>
<gates><gate name="G$1" symbol="24C08" x="0" y="0"/></gates>
<devices>
<device name="" package="DIL08">
<connects>
<connect gate="G$1" pin="A0" pad="1"/>
<connect gate="G$1" pin="A1" pad="2"/>
<connect gate="G$1" pin="A2" pad="3"/>
<connect gate="G$1" pin="VSS" pad="4"/>
<connect gate="G$1" pin="SDA" pad="5"/>
<connect gate="G$1" pin="SCL" pad="6"/>
<connect gate="G$1" pin="WP" pad="7"/>
<connect gate="G$1" pin="VDD" pad="8"/>
</connects>
<technologies><technology name=""/></technologies>
</device>
</devices>
</deviceset>
</library>
<!-- ========== LIBRARY: SIM800L ========== -->
<library name="sim800l">
<packages>
<package name="SIM800L_MODULE">
<description>SIM800L Module footprint (breakout board)</description>
<wire x1="-12" y1="-10" x2="-12" y2="10" width="0.15" layer="21"/>
<wire x1="-12" y1="10" x2="12" y2="10" width="0.15" layer="21"/>
<wire x1="12" y1="10" x2="12" y2="-10" width="0.15" layer="21"/>
<wire x1="12" y1="-10" x2="-12" y2="-10" width="0.15" layer="21"/>
<pad name="VCC" x="-10" y="8" drill="0.8" diameter="1.4"/>
<pad name="GND" x="-10" y="5.5" drill="0.8" diameter="1.4"/>
<pad name="TXD" x="-10" y="3" drill="0.8" diameter="1.4"/>
<pad name="RXD" x="-10" y="0.5" drill="0.8" diameter="1.4"/>
<pad name="RST" x="-10" y="-2" drill="0.8" diameter="1.4"/>
<pad name="PWRKEY" x="-10" y="-4.5" drill="0.8" diameter="1.4"/>
<pad name="STATUS" x="-10" y="-7" drill="0.8" diameter="1.4"/>
<pad name="DTR" x="10" y="8" drill="0.8" diameter="1.4"/>
<pad name="NETLIGHT" x="10" y="5.5" drill="0.8" diameter="1.4"/>
<pad name="SIM_DATA" x="10" y="3" drill="0.8" diameter="1.4"/>
<pad name="SIM_CLK" x="10" y="0.5" drill="0.8" diameter="1.4"/>
<pad name="SIM_RST" x="10" y="-2" drill="0.8" diameter="1.4"/>
<pad name="GND2" x="10" y="-4.5" drill="0.8" diameter="1.4"/>
<pad name="VCC2" x="10" y="-7" drill="0.8" diameter="1.4"/>
<text x="-11" y="10" size="1.27" layer="25">&gt;NAME</text>
<text x="-11" y="-11" size="1.27" layer="27">&gt;VALUE</text>
</package>
</packages>
<symbols>
<symbol name="SIM800L">
<wire x1="-12.7" y1="17.78" x2="12.7" y2="17.78" width="0.254" layer="94"/>
<wire x1="12.7" y1="17.78" x2="12.7" y2="-17.78" width="0.254" layer="94"/>
<wire x1="12.7" y1="-17.78" x2="-12.7" y2="-17.78" width="0.254" layer="94"/>
<wire x1="-12.7" y1="-17.78" x2="-12.7" y2="17.78" width="0.254" layer="94"/>
<pin name="VCC" x="-15.24" y="15.24" length="short" direction="pwr"/>
<pin name="GND" x="-15.24" y="12.7" length="short" direction="pwr"/>
<pin name="TXD" x="-15.24" y="7.62" length="short"/>
<pin name="RXD" x="-15.24" y="5.08" length="short"/>
<pin name="PWRKEY" x="-15.24" y="0" length="short"/>
<pin name="STATUS" x="-15.24" y="-2.54" length="short"/>
<pin name="DTR" x="15.24" y="15.24" length="short" rot="R180"/>
<pin name="RST" x="-15.24" y="-5.08" length="short"/>
<pin name="NETLIGHT" x="15.24" y="10.16" length="short" rot="R180"/>
<pin name="GND2" x="-15.24" y="10.16" length="short" direction="pwr"/>
<text x="-12.7" y="20.32" size="1.778" layer="95">&gt;NAME</text>
<text x="-12.7" y="-20.32" size="1.778" layer="96">&gt;VALUE</text>
</symbol>
</symbols>
<devicesets>
<deviceset name="SIM800L" prefix="U">
<description>SIM800L GSM/GPRS Module</description>
<gates><gate name="G$1" symbol="SIM800L" x="0" y="0"/></gates>
<devices>
<device name="" package="SIM800L_MODULE">
<connects>
<connect gate="G$1" pin="VCC" pad="VCC"/>
<connect gate="G$1" pin="GND" pad="GND"/>
<connect gate="G$1" pin="TXD" pad="TXD"/>
<connect gate="G$1" pin="RXD" pad="RXD"/>
<connect gate="G$1" pin="PWRKEY" pad="PWRKEY"/>
<connect gate="G$1" pin="STATUS" pad="STATUS"/>
<connect gate="G$1" pin="DTR" pad="DTR"/>
<connect gate="G$1" pin="RST" pad="RST"/>
<connect gate="G$1" pin="NETLIGHT" pad="NETLIGHT"/>
<connect gate="G$1" pin="GND2" pad="GND2"/>
</connects>
<technologies><technology name=""/></technologies>
</device>
</devices>
</deviceset>
</library>
<!-- ========== LIBRARY: LED ========== -->
<library name="led">
<packages>
<package name="LED0805">
<description>0805 LED</description>
<wire x1="-0.41" y1="0.635" x2="0.41" y2="0.635" width="0.1524" layer="51"/>
<wire x1="-0.41" y1="-0.635" x2="0.41" y2="-0.635" width="0.1524" layer="51"/>
<smd name="A" x="-0.95" y="0" dx="1.3" dy="1.5" layer="1"/>
<smd name="K" x="0.95" y="0" dx="1.3" dy="1.5" layer="1"/>
<text x="-0.635" y="1.27" size="1.27" layer="25">&gt;NAME</text>
<text x="-0.635" y="-2.54" size="1.27" layer="27">&gt;VALUE</text>
</package>
</packages>
<symbols>
<symbol name="LED">
<wire x1="1.27" y1="0" x2="0" y2="-2.54" width="0.254" layer="94"/>
<wire x1="0" y1="-2.54" x2="-1.27" y2="0" width="0.254" layer="94"/>
<wire x1="1.27" y1="-2.54" x2="0" y2="-2.54" width="0.254" layer="94"/>
<wire x1="0" y1="-2.54" x2="-1.27" y2="-2.54" width="0.254" layer="94"/>
<wire x1="1.27" y1="0" x2="0" y2="0" width="0.254" layer="94"/>
<wire x1="0" y1="0" x2="-1.27" y2="0" width="0.254" layer="94"/>
<wire x1="0" y1="-2.54" x2="0" y2="-5.08" width="0.1524" layer="94"/>
<wire x1="-1.27" y1="-1.27" x2="-2.54" y2="-2.54" width="0.1524" layer="94"/>
<wire x1="-1.27" y1="-3.81" x2="-2.54" y2="-5.08" width="0.1524" layer="94"/>
<pin name="A" x="0" y="2.54" visible="off" length="short" direction="pas" rot="R270"/>
<pin name="K" x="0" y="-5.08" visible="off" length="short" direction="pas" rot="R90"/>
<text x="2.54" y="-1.27" size="1.778" layer="95">&gt;NAME</text>
<text x="2.54" y="-3.81" size="1.778" layer="96">&gt;VALUE</text>
</symbol>
</symbols>
<devicesets>
<deviceset name="LED" prefix="D" uservalue="yes">
<gates><gate name="G$1" symbol="LED" x="0" y="0"/></gates>
<devices>
<device name="0805" package="LED0805">
<connects>
<connect gate="G$1" pin="A" pad="A"/>
<connect gate="G$1" pin="K" pad="K"/>
</connects>
<technologies><technology name=""/></technologies>
</device>
</devices>
</deviceset>
</library>
<!-- ========== LIBRARY: Power symbols ========== -->
<library name="supply">
<packages>
</packages>
<symbols>
<symbol name="+3V3">
<wire x1="1.27" y1="-1.905" x2="0" y2="3.81" width="0.254" layer="94"/>
<wire x1="0" y1="3.81" x2="-1.27" y2="-1.905" width="0.254" layer="94"/>
<wire x1="1.27" y1="-1.905" x2="-1.27" y2="-1.905" width="0.254" layer="94"/>
<text x="-2.54" y="5.08" size="1.778" layer="96">&gt;VALUE</text>
<pin name="+3V3" x="0" y="0" visible="off" length="short" direction="sup" rot="R90"/>
</symbol>
<symbol name="+5V">
<wire x1="1.27" y1="-1.905" x2="0" y2="3.81" width="0.254" layer="94"/>
<wire x1="0" y1="3.81" x2="-1.27" y2="-1.905" width="0.254" layer="94"/>
<wire x1="1.27" y1="-1.905" x2="-1.27" y2="-1.905" width="0.254" layer="94"/>
<text x="-2.54" y="5.08" size="1.778" layer="96">&gt;VALUE</text>
<pin name="+5V" x="0" y="0" visible="off" length="short" direction="sup" rot="R90"/>
</symbol>
<symbol name="GND">
<wire x1="-1.905" y1="0" x2="1.905" y2="0" width="0.254" layer="94"/>
<wire x1="1.905" y1="0" x2="0" y2="-1.905" width="0.254" layer="94"/>
<wire x1="0" y1="-1.905" x2="-1.905" y2="0" width="0.254" layer="94"/>
<text x="-2.54" y="-3.81" size="1.778" layer="96">&gt;VALUE</text>
<pin name="GND" x="0" y="2.54" visible="off" length="short" direction="sup" rot="R270"/>
</symbol>
<symbol name="+12V">
<wire x1="1.27" y1="-1.905" x2="0" y2="3.81" width="0.254" layer="94"/>
<wire x1="0" y1="3.81" x2="-1.27" y2="-1.905" width="0.254" layer="94"/>
<wire x1="1.27" y1="-1.905" x2="-1.27" y2="-1.905" width="0.254" layer="94"/>
<text x="-3.81" y="5.08" size="1.778" layer="96">&gt;VALUE</text>
<pin name="+12V" x="0" y="0" visible="off" length="short" direction="sup" rot="R90"/>
</symbol>
<symbol name="+4V">
<wire x1="1.27" y1="-1.905" x2="0" y2="3.81" width="0.254" layer="94"/>
<wire x1="0" y1="3.81" x2="-1.27" y2="-1.905" width="0.254" layer="94"/>
<wire x1="1.27" y1="-1.905" x2="-1.27" y2="-1.905" width="0.254" layer="94"/>
<text x="-3.81" y="5.08" size="1.778" layer="96">&gt;VALUE</text>
<pin name="+4V" x="0" y="0" visible="off" length="short" direction="sup" rot="R90"/>
</symbol>
</symbols>
<devicesets>
<deviceset name="+3V3" prefix="+3V3">
<gates><gate name="G$1" symbol="+3V3" x="0" y="0"/></gates>
<devices><device name=""><technologies><technology name=""/></technologies></device></devices>
</deviceset>
<deviceset name="+5V" prefix="+5V">
<gates><gate name="G$1" symbol="+5V" x="0" y="0"/></gates>
<devices><device name=""><technologies><technology name=""/></technologies></device></devices>
</deviceset>
<deviceset name="GND" prefix="GND">
<gates><gate name="G$1" symbol="GND" x="0" y="0"/></gates>
<devices><device name=""><technologies><technology name=""/></technologies></device></devices>
</deviceset>
<deviceset name="+12V" prefix="+12V">
<gates><gate name="G$1" symbol="+12V" x="0" y="0"/></gates>
<devices><device name=""><technologies><technology name=""/></technologies></device></devices>
</deviceset>
<deviceset name="+4V" prefix="+4V">
<gates><gate name="G$1" symbol="+4V" x="0" y="0"/></gates>
<devices><device name=""><technologies><technology name=""/></technologies></device></devices>
</deviceset>
</devicesets>
</library>
<!-- ========== LIBRARY: iButton socket ========== -->
<library name="ibutton">
<packages>
<package name="IBUTTON_SOCKET">
<description>iButton touch socket (DS1990A)</description>
<circle x="0" y="0" radius="8" width="0.15" layer="21"/>
<pad name="DATA" x="0" y="0" drill="1" diameter="2"/>
<pad name="GND" x="-5" y="0" drill="1" diameter="2"/>
<text x="-6" y="9" size="1.27" layer="25">&gt;NAME</text>
<text x="-6" y="-10" size="1.27" layer="27">&gt;VALUE</text>
</package>
</packages>
<symbols>
<symbol name="IBUTTON">
<wire x1="-5.08" y1="2.54" x2="5.08" y2="2.54" width="0.254" layer="94"/>
<wire x1="5.08" y1="2.54" x2="5.08" y2="-2.54" width="0.254" layer="94"/>
<wire x1="5.08" y1="-2.54" x2="-5.08" y2="-2.54" width="0.254" layer="94"/>
<wire x1="-5.08" y1="-2.54" x2="-5.08" y2="2.54" width="0.254" layer="94"/>
<pin name="DATA" x="-7.62" y="0" length="short"/>
<pin name="GND" x="7.62" y="0" length="short" rot="R180"/>
<text x="-5.08" y="5.08" size="1.778" layer="95">&gt;NAME</text>
<text x="-5.08" y="-5.08" size="1.778" layer="96">&gt;VALUE</text>
</symbol>
</symbols>
<devicesets>
<deviceset name="IBUTTON" prefix="X">
<description>DS1990A iButton contact socket</description>
<gates><gate name="G$1" symbol="IBUTTON" x="0" y="0"/></gates>
<devices>
<device name="" package="IBUTTON_SOCKET">
<connects>
<connect gate="G$1" pin="DATA" pad="DATA"/>
<connect gate="G$1" pin="GND" pad="GND"/>
</connects>
<technologies><technology name=""/></technologies>
</device>
</devices>
</deviceset>
</library>
</libraries>
<!-- ================================================================ -->
<!-- PARTS LIST                                                       -->
<!-- ================================================================ -->
<parts>
<!-- MCU -->
<part name="U1" library="stm32f103" deviceset="STM32F103C8T6" value="STM32F103C8T6"/>
<!-- Decoupling: 100nF per VDD pin (4 pins) -->
<part name="C_VDD1" library="rcl" deviceset="C" value="100nF"/>
<part name="C_VDD2" library="rcl" deviceset="C" value="100nF"/>
<part name="C_VDD3" library="rcl" deviceset="C" value="100nF"/>
<part name="C_VDD4" library="rcl" deviceset="C" value="100nF"/>
<!-- Bulk decoupling -->
<part name="C_BULK" library="rcl" deviceset="C" value="4.7uF"/>
<!-- VDDA LC filter -->
<part name="L_VDDA" library="rcl" deviceset="L" value="10uH"/>
<part name="C_VDDA1" library="rcl" deviceset="C" value="100nF"/>
<part name="C_VDDA2" library="rcl" deviceset="C" value="1uF"/>
<!-- NRST pull-up -->
<part name="R_NRST" library="rcl" deviceset="R" value="10k"/>
<!-- NRST cap -->
<part name="C_NRST" library="rcl" deviceset="C" value="100nF"/>
<!-- BOOT0 pull-down -->
<part name="R_BOOT0" library="rcl" deviceset="R" value="10k"/>
<!-- 8MHz Crystal -->
<part name="Y1" library="crystal" deviceset="8MHZ" value="8MHz"/>
<!-- Crystal load caps -->
<part name="C_OSC1" library="rcl" deviceset="C" value="22pF"/>
<part name="C_OSC2" library="rcl" deviceset="C" value="22pF"/>
<!-- SWD connector -->
<part name="J_SWD" library="con-jst-xh" deviceset="JST-XH-04" value="SWD"/>
<!-- SIM800L -->
<part name="U2" library="sim800l" deviceset="SIM800L" value="SIM800L"/>
<!-- SIM800L bypass 1000uF -->
<part name="C_SIM" library="rcl" deviceset="C" value="1000uF"/>
<!-- GSM NPN PWRKEY -->
<part name="Q_PWR" library="transistor" deviceset="BC847" value="BC847"/>
<part name="R_PWRKEY_B" library="rcl" deviceset="R" value="10k"/>
<part name="R_PWRKEY_C" library="rcl" deviceset="R" value="10k"/>
<!-- GSM STATUS voltage divider -->
<part name="R_STS1" library="rcl" deviceset="R" value="1k"/>
<part name="R_STS2" library="rcl" deviceset="R" value="2k"/>
<!-- GSM UART connector -->
<part name="J_GSM" library="con-jst-xh" deviceset="JST-XH-05" value="GSM"/>
<!-- PCF8574 I2C LCD backpack -->
<part name="U3" library="pcf8574" deviceset="PCF8574" value="PCF8574"/>
<!-- 24C08 EEPROM -->
<part name="U4" library="eeprom" deviceset="24C08" value="24C08"/>
<!-- I2C pull-ups -->
<part name="R_SCL" library="rcl" deviceset="R" value="4.7k"/>
<part name="R_SDA" library="rcl" deviceset="R" value="4.7k"/>
<!-- LCD connector (to HD44780 via PCF8574) -->
<part name="J_LCD" library="con-jst-xh" deviceset="JST-XH-04" value="LCD"/>
<!-- NRI G-13.6000 coin acceptor connector -->
<part name="J_NRI" library="con-jst-xh" deviceset="JST-XH-10" value="NRI_G13"/>
<!-- Coin channel NPN transistors (6x) -->
<part name="Q1" library="transistor" deviceset="BC847" value="BC847"/>
<part name="Q2" library="transistor" deviceset="BC847" value="BC847"/>
<part name="Q3" library="transistor" deviceset="BC847" value="BC847"/>
<part name="Q4" library="transistor" deviceset="BC847" value="BC847"/>
<part name="Q5" library="transistor" deviceset="BC847" value="BC847"/>
<part name="Q6" library="transistor" deviceset="BC847" value="BC847"/>
<!-- Coin NPN base resistors (6x) -->
<part name="R_B1" library="rcl" deviceset="R" value="10k"/>
<part name="R_B2" library="rcl" deviceset="R" value="10k"/>
<part name="R_B3" library="rcl" deviceset="R" value="10k"/>
<part name="R_B4" library="rcl" deviceset="R" value="10k"/>
<part name="R_B5" library="rcl" deviceset="R" value="10k"/>
<part name="R_B6" library="rcl" deviceset="R" value="10k"/>
<!-- Coin NPN collector pull-ups (6x) -->
<part name="R_C1" library="rcl" deviceset="R" value="10k"/>
<part name="R_C2" library="rcl" deviceset="R" value="10k"/>
<part name="R_C3" library="rcl" deviceset="R" value="10k"/>
<part name="R_C4" library="rcl" deviceset="R" value="10k"/>
<part name="R_C5" library="rcl" deviceset="R" value="10k"/>
<part name="R_C6" library="rcl" deviceset="R" value="10k"/>
<!-- Coin BLOCK direct (PB14) -->
<part name="R_BLOCK" library="rcl" deviceset="R" value="100"/>
<!-- Hopper A NPN -->
<part name="Q_HOPA" library="transistor" deviceset="BC847" value="BC847"/>
<part name="R_HOPA_B" library="rcl" deviceset="R" value="10k"/>
<part name="R_HOPA_C" library="rcl" deviceset="R" value="10k"/>
<!-- Hopper connector -->
<part name="J_HOP" library="con-jst-xh" deviceset="JST-XH-05" value="HOPPER"/>
<!-- 12V power connector -->
<part name="J_12V" library="con-jst-xh" deviceset="JST-XH-02" value="PWR_12V"/>
<!-- Buttons: PREV/NEXT/OK/CANCEL -->
<part name="R_BTN1" library="rcl" deviceset="R" value="10k"/>
<part name="R_BTN2" library="rcl" deviceset="R" value="10k"/>
<part name="R_BTN3" library="rcl" deviceset="R" value="10k"/>
<part name="R_BTN4" library="rcl" deviceset="R" value="10k"/>
<part name="J_BTN" library="con-jst-xh" deviceset="JST-XH-04" value="BUTTONS"/>
<!-- Door reed switches -->
<part name="R_DOOR1" library="rcl" deviceset="R" value="10k"/>
<part name="R_DOOR2" library="rcl" deviceset="R" value="10k"/>
<part name="J_DOOR1" library="con-jst-xh" deviceset="JST-XH-02" value="DOOR1"/>
<part name="J_DOOR2" library="con-jst-xh" deviceset="JST-XH-02" value="DOOR2"/>
<!-- iButton -->
<part name="R_1W" library="rcl" deviceset="R" value="4.7k"/>
<part name="X_IBTN" library="ibutton" deviceset="IBUTTON" value="DS1990A"/>
<!-- LEDs -->
<part name="D_LEDG" library="led" deviceset="LED" value="GREEN"/>
<part name="D_LEDR" library="led" deviceset="LED" value="RED"/>
<part name="R_LEDG" library="rcl" deviceset="R" value="1k"/>
<part name="R_LEDR" library="rcl" deviceset="R" value="1k"/>
<!-- Power Fail -->
<part name="J_PWRFAIL" library="con-jst-xh" deviceset="JST-XH-02" value="PWR_FAIL"/>
<!-- 3.3V power connector -->
<part name="J_3V3" library="con-jst-xh" deviceset="JST-XH-02" value="PWR_3V3"/>
<!-- 5V power connector -->
<part name="J_5V" library="con-jst-xh" deviceset="JST-XH-02" value="PWR_5V"/>
<!-- 4V SIM800L LDO connector -->
<part name="J_4V" library="con-jst-xh" deviceset="JST-XH-02" value="PWR_4V"/>
</parts>
<!-- ================================================================ -->
<!-- SHEETS                                                           -->
<!-- ================================================================ -->
<sheets>
<!-- ============================================================== -->
<!-- SHEET 1: MCU + Power + Decoupling + Crystal + SWD              -->
<!-- ============================================================== -->
<sheet>
<plain>
<text x="2.54" y="101.6" size="3.81" layer="97">Бахилизатор v2.0</text>
<text x="2.54" y="96.52" size="2.54" layer="97">Sheet 1/5: MCU + Питание + Декаплинг + Кристалл + SWD</text>
<wire x1="0" y1="0" x2="266.7" y2="0" width="0.254" layer="97"/>
<wire x1="266.7" y1="0" x2="266.7" y2="182.88" width="0.254" layer="97"/>
<wire x1="266.7" y1="182.88" x2="0" y2="182.88" width="0.254" layer="97"/>
<wire x1="0" y1="182.88" x2="0" y2="0" width="0.254" layer="97"/>
</plain>
<instances>
<!-- MCU at center-left -->
<instance part="U1" gate="G$1" x="76.2" y="91.44"/>
<!-- Decoupling caps near VDD pins -->
<instance part="C_VDD1" gate="G$1" x="25.4" y="132.08" rot="R90"/>
<instance part="C_VDD2" gate="G$1" x="25.4" y="121.92" rot="R90"/>
<instance part="C_VDD3" gate="G$1" x="25.4" y="111.76" rot="R90"/>
<instance part="C_VDD4" gate="G$1" x="25.4" y="101.6" rot="R90"/>
<instance part="C_BULK" gate="G$1" x="15.24" y="121.92" rot="R90"/>
<!-- VDDA LC filter -->
<instance part="L_VDDA" gate="G$1" x="35.56" y="144.78"/>
<instance part="C_VDDA1" gate="G$1" x="38.1" y="139.7" rot="R90"/>
<instance part="C_VDDA2" gate="G$1" x="43.18" y="139.7" rot="R90"/>
<!-- NRST pull-up -->
<instance part="R_NRST" gate="G$1" x="25.4" y="88.9" rot="R90"/>
<instance part="C_NRST" gate="G$1" x="30.48" y="88.9" rot="R90"/>
<!-- BOOT0 pull-down -->
<instance part="R_BOOT0" gate="G$1" x="25.4" y="60.96" rot="R90"/>
<!-- 8MHz Crystal -->
<instance part="Y1" gate="G$1" x="25.4" y="48.26"/>
<instance part="C_OSC1" gate="G$1" x="15.24" y="43.18" rot="R90"/>
<instance part="C_OSC2" gate="G$1" x="35.56" y="43.18" rot="R90"/>
<!-- SWD connector -->
<instance part="J_SWD" gate="G$1" x="139.7" y="68.58"/>
<!-- Power connectors -->
<instance part="J_3V3" gate="G$1" x="139.7" y="144.78"/>
<instance part="J_5V" gate="G$1" x="139.7" y="132.08"/>
<instance part="J_12V" gate="G$1" x="139.7" y="119.38"/>
<!-- Power supply symbols -->
<instance part="+3V3" gate="G$1" x="15.24" y="152.4"/>
<instance part="GND" gate="G$1" x="15.24" y="40.64"/>
</instances>
<nets>
<net name="+3V3" class="0">
<segment>
<wire x1="15.24" y1="152.4" x2="15.24" y2="144.78" width="0.1524" layer="91"/>
<pinref part="+3V3" gate="G$1" pin="+3V3"/>
<wire x1="15.24" y1="144.78" x2="30.48" y2="144.78" width="0.1524" layer="91"/>
<pinref part="L_VDDA" gate="G$1" pin="1"/>
<wire x1="15.24" y1="144.78" x2="15.24" y2="132.08" width="0.1524" layer="91"/>
<pinref part="R_NRST" gate="G$1" pin="2"/>
<wire x1="15.24" y1="132.08" x2="15.24" y2="93.98" width="0.1524" layer="91"/>
<wire x1="43.18" y1="127" x2="43.18" y2="132.08" width="0.1524" layer="91"/>
<pinref part="U1" gate="G$1" pin="VDD"/>
<wire x1="43.18" y1="127" x2="43.18" y2="127" width="0.1524" layer="91"/>
<junction x="15.24" y="144.78"/>
</segment>
<segment>
<pinref part="J_3V3" gate="G$1" pin="1"/>
<wire x1="134.62" y1="147.32" x2="129.54" y2="147.32" width="0.1524" layer="91"/>
</segment>
</net>
<net name="GND" class="0">
<segment>
<pinref part="GND" gate="G$1" pin="GND"/>
<wire x1="15.24" y1="43.18" x2="15.24" y2="38.1" width="0.1524" layer="91"/>
<pinref part="C_OSC1" gate="G$1" pin="2"/>
<wire x1="15.24" y1="38.1" x2="35.56" y2="38.1" width="0.1524" layer="91"/>
<pinref part="C_OSC2" gate="G$1" pin="2"/>
<wire x1="35.56" y1="40.64" x2="35.56" y2="38.1" width="0.1524" layer="91"/>
<junction x="15.24" y="38.1"/>
<pinref part="R_BOOT0" gate="G$1" pin="1"/>
<wire x1="35.56" y1="38.1" x2="15.24" y2="38.1" width="0.1524" layer="91"/>
<wire x1="15.24" y1="38.1" x2="25.4" y2="38.1" width="0.1524" layer="91"/>
<wire x1="25.4" y1="38.1" x2="25.4" y2="55.88" width="0.1524" layer="91"/>
</segment>
<segment>
<pinref part="U1" gate="G$1" pin="VSS"/>
<wire x1="43.18" y1="119.38" x2="38.1" y2="119.38" width="0.1524" layer="91"/>
<wire x1="38.1" y1="119.38" x2="38.1" y2="109.22" width="0.1524" layer="91"/>
<pinref part="U1" gate="G$1" pin="VSSA"/>
<wire x1="43.18" y1="116.84" x2="38.1" y2="116.84" width="0.1524" layer="91"/>
<wire x1="38.1" y1="109.22" x2="38.1" y2="99.06" width="0.1524" layer="91"/>
<junction x="38.1" y="109.22"/>
</segment>
<segment>
<pinref part="J_3V3" gate="G$1" pin="2"/>
<wire x1="134.62" y1="144.78" x2="129.54" y2="144.78" width="0.1524" layer="91"/>
</segment>
<segment>
<pinref part="J_SWD" gate="G$1" pin="2"/>
<wire x1="134.62" y1="73.66" x2="129.54" y2="73.66" width="0.1524" layer="91"/>
</segment>
<segment>
<pinref part="J_12V" gate="G$1" pin="2"/>
<wire x1="134.62" y1="119.38" x2="129.54" y2="119.38" width="0.1524" layer="91"/>
</segment>
</net>
<net name="VDDA_FILT" class="0">
<segment>
<pinref part="L_VDDA" gate="G$1" pin="2"/>
<pinref part="C_VDDA1" gate="G$1" pin="1"/>
<wire x1="40.64" y1="144.78" x2="38.1" y2="144.78" width="0.1524" layer="91"/>
<wire x1="38.1" y1="144.78" x2="38.1" y2="144.78" width="0.1524" layer="91"/>
<pinref part="U1" gate="G$1" pin="VDDA"/>
<wire x1="43.18" y1="124.46" x2="40.64" y2="124.46" width="0.1524" layer="91"/>
<wire x1="40.64" y1="124.46" x2="40.64" y2="144.78" width="0.1524" layer="91"/>
</segment>
</net>
<net name="NRST" class="0">
<segment>
<pinref part="U1" gate="G$1" pin="NRST"/>
<pinref part="R_NRST" gate="G$1" pin="1"/>
<pinref part="C_NRST" gate="G$1" pin="1"/>
<wire x1="43.18" y1="114.3" x2="25.4" y2="114.3" width="0.1524" layer="91"/>
<wire x1="25.4" y1="114.3" x2="25.4" y2="93.98" width="0.1524" layer="91"/>
</segment>
</net>
<net name="BOOT0" class="0">
<segment>
<pinref part="U1" gate="G$1" pin="BOOT0"/>
<pinref part="R_BOOT0" gate="G$1" pin="2"/>
<wire x1="43.18" y1="66.04" x2="25.4" y2="66.04" width="0.1524" layer="91"/>
</segment>
</net>
<net name="OSC_IN" class="0">
<segment>
<pinref part="U1" gate="G$1" pin="OSC_IN"/>
<pinref part="Y1" gate="G$1" pin="1"/>
<pinref part="C_OSC1" gate="G$1" pin="1"/>
<wire x1="43.18" y1="63.5" x2="20.32" y2="63.5" width="0.1524" layer="91"/>
<wire x1="20.32" y1="63.5" x2="20.32" y2="48.26" width="0.1524" layer="91"/>
</segment>
</net>
<net name="OSC_OUT" class="0">
<segment>
<pinref part="U1" gate="G$1" pin="OSC_OUT"/>
<pinref part="Y1" gate="G$1" pin="2"/>
<pinref part="C_OSC2" gate="G$1" pin="1"/>
<wire x1="43.18" y1="60.96" x2="30.48" y2="60.96" width="0.1524" layer="91"/>
<wire x1="30.48" y1="60.96" x2="30.48" y2="48.26" width="0.1524" layer="91"/>
</segment>
</net>
<net name="SWDIO" class="0">
<segment>
<pinref part="U1" gate="G$1" pin="PA13/SWDIO"/>
<wire x1="43.18" y1="76.2" x2="38.1" y2="76.2" width="0.1524" layer="91"/>
<label x="38.1" y="76.2" size="1.27" layer="95" rot="R180" xref="yes"/>
</segment>
<segment>
<pinref part="J_SWD" gate="G$1" pin="3"/>
<wire x1="134.62" y1="71.12" x2="129.54" y2="71.12" width="0.1524" layer="91"/>
<label x="129.54" y="71.12" size="1.27" layer="95" rot="R180" xref="yes"/>
</segment>
</net>
<net name="SWCLK" class="0">
<segment>
<pinref part="U1" gate="G$1" pin="PA14/SWCLK"/>
<wire x1="43.18" y1="73.66" x2="38.1" y2="73.66" width="0.1524" layer="91"/>
<label x="38.1" y="73.66" size="1.27" layer="95" rot="R180" xref="yes"/>
</segment>
<segment>
<pinref part="J_SWD" gate="G$1" pin="4"/>
<wire x1="134.62" y1="68.58" x2="129.54" y2="68.58" width="0.1524" layer="91"/>
<label x="129.54" y="68.58" size="1.27" layer="95" rot="R180" xref="yes"/>
</segment>
</net>
<net name="PA0" class="0">
<segment>
<pinref part="U1" gate="G$1" pin="PA0"/>
<wire x1="43.18" y1="109.22" x2="38.1" y2="109.22" width="0.1524" layer="91"/>
<label x="38.1" y="109.22" size="1.27" layer="95" rot="R180" xref="yes"/>
</segment>
</net>
<net name="PA1" class="0">
<segment>
<pinref part="U1" gate="G$1" pin="PA1"/>
<wire x1="43.18" y1="106.68" x2="38.1" y2="106.68" width="0.1524" layer="91"/>
<label x="38.1" y="106.68" size="1.27" layer="95" rot="R180" xref="yes"/>
</segment>
</net>
<net name="PA2" class="0">
<segment>
<pinref part="U1" gate="G$1" pin="PA2"/>
<wire x1="43.18" y1="104.14" x2="38.1" y2="104.14" width="0.1524" layer="91"/>
<label x="38.1" y="104.14" size="1.27" layer="95" rot="R180" xref="yes"/>
</segment>
</net>
<net name="PA3" class="0">
<segment>
<pinref part="U1" gate="G$1" pin="PA3"/>
<label x="38.1" y="101.6" size="1.27" layer="95" rot="R180" xref="yes"/>
</segment>
</net>
<net name="PA4" class="0">
<segment>
<pinref part="U1" gate="G$1" pin="PA4"/>
<label x="38.1" y="99.06" size="1.27" layer="95" rot="R180" xref="yes"/>
</segment>
</net>
<net name="PA5" class="0">
<segment>
<pinref part="U1" gate="G$1" pin="PA5"/>
<label x="38.1" y="96.52" size="1.27" layer="95" rot="R180" xref="yes"/>
</segment>
</net>
<net name="PA6" class="0">
<segment>
<pinref part="U1" gate="G$1" pin="PA6"/>
<label x="38.1" y="93.98" size="1.27" layer="95" rot="R180" xref="yes"/>
</segment>
</net>
<net name="PA7" class="0">
<segment>
<pinref part="U1" gate="G$1" pin="PA7"/>
<label x="38.1" y="91.44" size="1.27" layer="95" rot="R180" xref="yes"/>
</segment>
</net>
<net name="PA8" class="0">
<segment>
<pinref part="U1" gate="G$1" pin="PA8"/>
<label x="38.1" y="88.9" size="1.27" layer="95" rot="R180" xref="yes"/>
</segment>
</net>
<net name="PA9_TX" class="0">
<segment>
<pinref part="U1" gate="G$1" pin="PA9/USART1_TX"/>
<label x="38.1" y="86.36" size="1.27" layer="95" rot="R180" xref="yes"/>
</segment>
</net>
<net name="PA10_RX" class="0">
<segment>
<pinref part="U1" gate="G$1" pin="PA10/USART1_RX"/>
<label x="38.1" y="83.82" size="1.27" layer="95" rot="R180" xref="yes"/>
</segment>
</net>
<net name="PA11" class="0">
<segment>
<pinref part="U1" gate="G$1" pin="PA11"/>
<label x="38.1" y="81.28" size="1.27" layer="95" rot="R180" xref="yes"/>
</segment>
</net>
<net name="PB0" class="0">
<segment>
<pinref part="U1" gate="G$1" pin="PB0"/>
<label x="109.22" y="109.22" size="1.27" layer="95" xref="yes"/>
</segment>
</net>
<net name="PB1" class="0">
<segment>
<pinref part="U1" gate="G$1" pin="PB1"/>
<label x="109.22" y="106.68" size="1.27" layer="95" xref="yes"/>
</segment>
</net>
<net name="I2C_SCL" class="0">
<segment>
<pinref part="U1" gate="G$1" pin="PB6/I2C1_SCL"/>
<label x="109.22" y="104.14" size="1.27" layer="95" xref="yes"/>
</segment>
</net>
<net name="I2C_SDA" class="0">
<segment>
<pinref part="U1" gate="G$1" pin="PB7/I2C1_SDA"/>
<label x="109.22" y="101.6" size="1.27" layer="95" xref="yes"/>
</segment>
</net>
<net name="PB8" class="0">
<segment>
<pinref part="U1" gate="G$1" pin="PB8"/>
<label x="109.22" y="99.06" size="1.27" layer="95" xref="yes"/>
</segment>
</net>
<net name="PB9" class="0">
<segment>
<pinref part="U1" gate="G$1" pin="PB9"/>
<label x="109.22" y="96.52" size="1.27" layer="95" xref="yes"/>
</segment>
</net>
<net name="PB10" class="0">
<segment>
<pinref part="U1" gate="G$1" pin="PB10"/>
<label x="109.22" y="93.98" size="1.27" layer="95" xref="yes"/>
</segment>
</net>
<net name="PB11" class="0">
<segment>
<pinref part="U1" gate="G$1" pin="PB11"/>
<label x="109.22" y="91.44" size="1.27" layer="95" xref="yes"/>
</segment>
</net>
<net name="PB12" class="0">
<segment>
<pinref part="U1" gate="G$1" pin="PB12"/>
<label x="109.22" y="88.9" size="1.27" layer="95" xref="yes"/>
</segment>
</net>
<net name="PB13" class="0">
<segment>
<pinref part="U1" gate="G$1" pin="PB13"/>
<label x="109.22" y="86.36" size="1.27" layer="95" xref="yes"/>
</segment>
</net>
<net name="PB14" class="0">
<segment>
<pinref part="U1" gate="G$1" pin="PB14"/>
<label x="109.22" y="83.82" size="1.27" layer="95" xref="yes"/>
</segment>
</net>
<net name="PB15" class="0">
<segment>
<pinref part="U1" gate="G$1" pin="PB15"/>
<label x="109.22" y="81.28" size="1.27" layer="95" xref="yes"/>
</segment>
</net>
<net name="PC13" class="0">
<segment>
<pinref part="U1" gate="G$1" pin="PC13"/>
<label x="109.22" y="76.2" size="1.27" layer="95" xref="yes"/>
</segment>
</net>
<net name="PC14" class="0">
<segment>
<pinref part="U1" gate="G$1" pin="PC14"/>
<label x="109.22" y="73.66" size="1.27" layer="95" xref="yes"/>
</segment>
</net>
<net name="PC15" class="0">
<segment>
<pinref part="U1" gate="G$1" pin="PC15"/>
<label x="109.22" y="71.12" size="1.27" layer="95" xref="yes"/>
</segment>
</net>
<net name="+5V" class="0">
<segment>
<pinref part="J_5V" gate="G$1" pin="1"/>
<wire x1="134.62" y1="134.62" x2="129.54" y2="134.62" width="0.1524" layer="91"/>
</segment>
</net>
<net name="+12V" class="0">
<segment>
<pinref part="J_12V" gate="G$1" pin="1"/>
<wire x1="134.62" y1="121.92" x2="129.54" y2="121.92" width="0.1524" layer="91"/>
</segment>
</net>
<net name="VBAT" class="0">
<segment>
<pinref part="U1" gate="G$1" pin="VBAT"/>
<wire x1="43.18" y1="121.92" x2="38.1" y2="121.92" width="0.1524" layer="91"/>
<label x="38.1" y="121.92" size="1.27" layer="95" rot="R180" xref="yes"/>
</segment>
</net>
</nets>
</sheet>
<!-- ============================================================== -->
<!-- SHEET 2: GSM SIM800L                                           -->
<!-- ============================================================== -->
<sheet>
<plain>
<text x="2.54" y="101.6" size="2.54" layer="97">Sheet 2/5: GSM SIM800L</text>
<text x="2.54" y="96.52" size="1.778" layer="97">USART1: PA9(TX)→SIM800L RXD, PA10(RX)←SIM800L TXD</text>
<text x="2.54" y="91.44" size="1.778" layer="97">PA0=PWRKEY(NPN BC847), PA1=STATUS(делитель 1k/2k), PA2=DTR</text>
<wire x1="0" y1="0" x2="266.7" y2="0" width="0.254" layer="97"/>
<wire x1="266.7" y1="0" x2="266.7" y2="182.88" width="0.254" layer="97"/>
<wire x1="266.7" y1="182.88" x2="0" y2="182.88" width="0.254" layer="97"/>
<wire x1="0" y1="182.88" x2="0" y2="0" width="0.254" layer="97"/>
</plain>
<instances>
<instance part="U2" gate="G$1" x="119.38" y="86.36"/>
<!-- PWRKEY NPN -->
<instance part="Q_PWR" gate="G$1" x="66.04" y="68.58"/>
<instance part="R_PWRKEY_B" gate="G$1" x="50.8" y="68.58"/>
<instance part="R_PWRKEY_C" gate="G$1" x="66.04" y="86.36" rot="R90"/>
<!-- STATUS voltage divider -->
<instance part="R_STS1" gate="G$1" x="76.2" y="55.88" rot="R90"/>
<instance part="R_STS2" gate="G$1" x="76.2" y="43.18" rot="R90"/>
<!-- SIM800L bypass -->
<instance part="C_SIM" gate="G$1" x="137.16" y="60.96" rot="R90"/>
<!-- GSM connector -->
<instance part="J_GSM" gate="G$1" x="160.02" y="76.2"/>
<!-- 4V power connector -->
<instance part="J_4V" gate="G$1" x="160.02" y="109.22"/>
<!-- Power symbols -->
<instance part="+4V" gate="G$1" x="152.4" y="119.38"/>
<instance part="GND" gate="G$1" x="66.04" y="30.48"/>
<instance part="+3V3" gate="G$1" x="40.64" y="93.98"/>
</instances>
<nets>
<net name="PA0" class="0">
<segment>
<pinref part="R_PWRKEY_B" gate="G$1" pin="1"/>
<wire x1="45.72" y1="68.58" x2="40.64" y2="68.58" width="0.1524" layer="91"/>
<label x="40.64" y="68.58" size="1.27" layer="95" rot="R180" xref="yes"/>
</segment>
</net>
<net name="PA1" class="0">
<segment>
<pinref part="R_STS1" gate="G$1" pin="2"/>
<wire x1="76.2" y1="60.96" x2="71.12" y2="60.96" width="0.1524" layer="91"/>
<label x="71.12" y="60.96" size="1.27" layer="95" rot="R180" xref="yes"/>
</segment>
</net>
<net name="PA2" class="0">
<segment>
<pinref part="U2" gate="G$1" pin="DTR"/>
<wire x1="134.62" y1="101.6" x2="142.24" y2="101.6" width="0.1524" layer="91"/>
<label x="142.24" y="101.6" size="1.27" layer="95" xref="yes"/>
</segment>
</net>
<net name="PA9_TX" class="0">
<segment>
<pinref part="U2" gate="G$1" pin="RXD"/>
<wire x1="104.14" y1="91.44" x2="96.52" y2="91.44" width="0.1524" layer="91"/>
<label x="96.52" y="91.44" size="1.27" layer="95" rot="R180" xref="yes"/>
</segment>
</net>
<net name="PA10_RX" class="0">
<segment>
<pinref part="U2" gate="G$1" pin="TXD"/>
<wire x1="104.14" y1="93.98" x2="96.52" y2="93.98" width="0.1524" layer="91"/>
<label x="96.52" y="93.98" size="1.27" layer="95" rot="R180" xref="yes"/>
</segment>
</net>
<net name="GSM_STATUS_MID" class="0">
<segment>
<pinref part="R_STS1" gate="G$1" pin="1"/>
<pinref part="R_STS2" gate="G$1" pin="2"/>
<pinref part="U2" gate="G$1" pin="STATUS"/>
<wire x1="76.2" y1="50.8" x2="76.2" y2="48.26" width="0.1524" layer="91"/>
<wire x1="104.14" y1="83.82" x2="76.2" y2="83.82" width="0.1524" layer="91"/>
<wire x1="76.2" y1="83.82" x2="76.2" y2="50.8" width="0.1524" layer="91"/>
<junction x="76.2" y="50.8"/>
<label x="86.36" y="83.82" size="1.27" layer="95"/>
</segment>
</net>
<net name="GSM_PWRKEY_N" class="0">
<segment>
<pinref part="Q_PWR" gate="G$1" pin="C"/>
<pinref part="R_PWRKEY_C" gate="G$1" pin="1"/>
<pinref part="U2" gate="G$1" pin="PWRKEY"/>
<wire x1="66.04" y1="76.2" x2="66.04" y2="81.28" width="0.1524" layer="91"/>
<wire x1="104.14" y1="86.36" x2="66.04" y2="86.36" width="0.1524" layer="91"/>
<wire x1="66.04" y1="81.28" x2="66.04" y2="86.36" width="0.1524" layer="91"/>
</segment>
</net>
<net name="+3V3" class="0">
<segment>
<pinref part="R_PWRKEY_C" gate="G$1" pin="2"/>
<pinref part="+3V3" gate="G$1" pin="+3V3"/>
<wire x1="66.04" y1="91.44" x2="40.64" y2="91.44" width="0.1524" layer="91"/>
<wire x1="40.64" y1="91.44" x2="40.64" y2="93.98" width="0.1524" layer="91"/>
</segment>
</net>
<net name="GND" class="0">
<segment>
<pinref part="Q_PWR" gate="G$1" pin="E"/>
<wire x1="66.04" y1="66.04" x2="66.04" y2="30.48" width="0.1524" layer="91"/>
<pinref part="GND" gate="G$1" pin="GND"/>
<wire x1="66.04" y1="30.48" x2="66.04" y2="33.02" width="0.1524" layer="91"/>
<pinref part="R_STS2" gate="G$1" pin="1"/>
<wire x1="76.2" y1="38.1" x2="76.2" y2="30.48" width="0.1524" layer="91"/>
<wire x1="76.2" y1="30.48" x2="66.04" y2="30.48" width="0.1524" layer="91"/>
<junction x="66.04" y="30.48"/>
<pinref part="U2" gate="G$1" pin="GND"/>
<wire x1="104.14" y1="99.06" x2="93.98" y2="99.06" width="0.1524" layer="91"/>
<wire x1="93.98" y1="99.06" x2="93.98" y2="30.48" width="0.1524" layer="91"/>
<wire x1="93.98" y1="30.48" x2="76.2" y2="30.48" width="0.1524" layer="91"/>
<junction x="76.2" y="30.48"/>
<pinref part="U2" gate="G$1" pin="GND2"/>
<wire x1="104.14" y1="96.52" x2="93.98" y2="96.52" width="0.1524" layer="91"/>
<junction x="93.98" y="30.48"/>
<pinref part="J_4V" gate="G$1" pin="2"/>
<wire x1="154.94" y1="109.22" x2="152.4" y2="109.22" width="0.1524" layer="91"/>
<wire x1="152.4" y1="109.22" x2="152.4" y2="30.48" width="0.1524" layer="91"/>
<wire x1="152.4" y1="30.48" x2="93.98" y2="30.48" width="0.1524" layer="91"/>
</segment>
</net>
<net name="PWRKEY_BASE" class="0">
<segment>
<pinref part="R_PWRKEY_B" gate="G$1" pin="2"/>
<pinref part="Q_PWR" gate="G$1" pin="B"/>
<wire x1="55.88" y1="68.58" x2="63.5" y2="68.58" width="0.1524" layer="91"/>
</segment>
</net>
<net name="+4V" class="0">
<segment>
<pinref part="+4V" gate="G$1" pin="+4V"/>
<pinref part="U2" gate="G$1" pin="VCC"/>
<wire x1="152.4" y1="119.38" x2="152.4" y2="101.6" width="0.1524" layer="91"/>
<wire x1="152.4" y1="101.6" x2="104.14" y2="101.6" width="0.1524" layer="91"/>
<pinref part="C_SIM" gate="G$1" pin="1"/>
<wire x1="137.16" y1="66.04" x2="137.16" y2="101.6" width="0.1524" layer="91"/>
<wire x1="137.16" y1="101.6" x2="152.4" y2="101.6" width="0.1524" layer="91"/>
<junction x="152.4" y="101.6"/>
<pinref part="J_4V" gate="G$1" pin="1"/>
<wire x1="154.94" y1="111.76" x2="152.4" y2="111.76" width="0.1524" layer="91"/>
<wire x1="152.4" y1="111.76" x2="152.4" y2="101.6" width="0.1524" layer="91"/>
</segment>
</net>
</nets>
</sheet>
<!-- ============================================================== -->
<!-- SHEET 3: I2C Bus - PCF8574 + 24C08                            -->
<!-- ============================================================== -->
<sheet>
<plain>
<text x="2.54" y="101.6" size="2.54" layer="97">Sheet 3/5: I2C — PCF8574 (LCD) + 24C08 EEPROM</text>
<text x="2.54" y="96.52" size="1.778" layer="97">I2C1: PB6(SCL), PB7(SDA) — 100kHz</text>
<text x="2.54" y="91.44" size="1.778" layer="97">PCF8574 addr 0x27 → HD44780, 24C08 addr 0x50</text>
<text x="2.54" y="86.36" size="1.778" layer="97">Pull-up 4.7kΩ на SDA/SCL к +3.3V</text>
<wire x1="0" y1="0" x2="266.7" y2="0" width="0.254" layer="97"/>
<wire x1="266.7" y1="0" x2="266.7" y2="182.88" width="0.254" layer="97"/>
<wire x1="266.7" y1="182.88" x2="0" y2="182.88" width="0.254" layer="97"/>
<wire x1="0" y1="182.88" x2="0" y2="0" width="0.254" layer="97"/>
</plain>
<instances>
<instance part="U3" gate="G$1" x="76.2" y="86.36"/>
<instance part="U4" gate="G$1" x="76.2" y="33.02"/>
<instance part="R_SCL" gate="G$1" x="38.1" y="78.74" rot="R90"/>
<instance part="R_SDA" gate="G$1" x="30.48" y="78.74" rot="R90"/>
<instance part="J_LCD" gate="G$1" x="139.7" y="91.44"/>
<instance part="+3V3" gate="G$1" x="30.48" y="109.22"/>
<instance part="+5V" gate="G$1" x="139.7" y="109.22"/>
<instance part="GND" gate="G$1" x="55.88" y="15.24"/>
</instances>
<nets>
<net name="I2C_SCL" class="0">
<segment>
<pinref part="U3" gate="G$1" pin="SCL"/>
<pinref part="R_SCL" gate="G$1" pin="1"/>
<wire x1="63.5" y1="83.82" x2="38.1" y2="83.82" width="0.1524" layer="91"/>
<wire x1="38.1" y1="83.82" x2="38.1" y2="73.66" width="0.1524" layer="91"/>
<label x="48.26" y="83.82" size="1.27" layer="95"/>
</segment>
<segment>
<pinref part="U4" gate="G$1" pin="SCL"/>
<wire x1="86.36" y1="35.56" x2="93.98" y2="35.56" width="0.1524" layer="91"/>
<label x="93.98" y="35.56" size="1.27" layer="95" xref="yes"/>
</segment>
</net>
<net name="I2C_SDA" class="0">
<segment>
<pinref part="U3" gate="G$1" pin="SDA"/>
<pinref part="R_SDA" gate="G$1" pin="1"/>
<wire x1="63.5" y1="81.28" x2="30.48" y2="81.28" width="0.1524" layer="91"/>
<wire x1="30.48" y1="81.28" x2="30.48" y2="73.66" width="0.1524" layer="91"/>
<label x="48.26" y="81.28" size="1.27" layer="95"/>
</segment>
<segment>
<pinref part="U4" gate="G$1" pin="SDA"/>
<wire x1="86.36" y1="38.1" x2="93.98" y2="38.1" width="0.1524" layer="91"/>
<label x="93.98" y="38.1" size="1.27" layer="95" xref="yes"/>
</segment>
</net>
<net name="+3V3" class="0">
<segment>
<pinref part="R_SCL" gate="G$1" pin="2"/>
<pinref part="R_SDA" gate="G$1" pin="2"/>
<pinref part="+3V3" gate="G$1" pin="+3V3"/>
<wire x1="38.1" y1="83.82" x2="30.48" y2="83.82" width="0.1524" layer="91"/>
<wire x1="30.48" y1="83.82" x2="30.48" y2="109.22" width="0.1524" layer="91"/>
<junction x="30.48" y="83.82"/>
<wire x1="38.1" y1="83.82" x2="38.1" y2="88.9" width="0.1524" layer="91"/>
<wire x1="38.1" y1="88.9" x2="30.48" y2="88.9" width="0.1524" layer="91"/>
<junction x="30.48" y="88.9"/>
</segment>
<segment>
<pinref part="U4" gate="G$1" pin="VDD"/>
<wire x1="86.36" y1="27.94" x2="93.98" y2="27.94" width="0.1524" layer="91"/>
<label x="93.98" y="27.94" size="1.27" layer="95" xref="yes"/>
</segment>
</net>
<net name="GND" class="0">
<segment>
<pinref part="GND" gate="G$1" pin="GND"/>
<pinref part="U3" gate="G$1" pin="VSS"/>
<wire x1="88.9" y1="78.74" x2="55.88" y2="78.74" width="0.1524" layer="91"/>
<wire x1="55.88" y1="78.74" x2="55.88" y2="17.78" width="0.1524" layer="91"/>
<pinref part="U3" gate="G$1" pin="A0"/>
<wire x1="63.5" y1="99.06" x2="55.88" y2="99.06" width="0.1524" layer="91"/>
<wire x1="55.88" y1="99.06" x2="55.88" y2="78.74" width="0.1524" layer="91"/>
<junction x="55.88" y="78.74"/>
<pinref part="U3" gate="G$1" pin="A1"/>
<wire x1="63.5" y1="96.52" x2="55.88" y2="96.52" width="0.1524" layer="91"/>
<junction x="55.88" y="96.52"/>
<pinref part="U3" gate="G$1" pin="A2"/>
<wire x1="63.5" y1="93.98" x2="55.88" y2="93.98" width="0.1524" layer="91"/>
<junction x="55.88" y="93.98"/>
<pinref part="U4" gate="G$1" pin="VSS"/>
<wire x1="66.04" y1="27.94" x2="55.88" y2="27.94" width="0.1524" layer="91"/>
<wire x1="55.88" y1="27.94" x2="55.88" y2="17.78" width="0.1524" layer="91"/>
<junction x="55.88" y="17.78"/>
<pinref part="U4" gate="G$1" pin="A0"/>
<wire x1="66.04" y1="38.1" x2="55.88" y2="38.1" width="0.1524" layer="91"/>
<wire x1="55.88" y1="38.1" x2="55.88" y2="27.94" width="0.1524" layer="91"/>
<junction x="55.88" y="27.94"/>
<pinref part="U4" gate="G$1" pin="A1"/>
<wire x1="66.04" y1="35.56" x2="55.88" y2="35.56" width="0.1524" layer="91"/>
<junction x="55.88" y="35.56"/>
<pinref part="U4" gate="G$1" pin="A2"/>
<wire x1="66.04" y1="33.02" x2="55.88" y2="33.02" width="0.1524" layer="91"/>
<junction x="55.88" y="33.02"/>
<pinref part="U4" gate="G$1" pin="WP"/>
<wire x1="86.36" y1="30.48" x2="93.98" y2="30.48" width="0.1524" layer="91"/>
<wire x1="93.98" y1="30.48" x2="93.98" y2="17.78" width="0.1524" layer="91"/>
<wire x1="93.98" y1="17.78" x2="55.88" y2="17.78" width="0.1524" layer="91"/>
<junction x="55.88" y="17.78"/>
</segment>
</net>
<net name="+5V" class="0">
<segment>
<pinref part="U3" gate="G$1" pin="VDD"/>
<pinref part="+5V" gate="G$1" pin="+5V"/>
<wire x1="88.9" y1="76.2" x2="99.06" y2="76.2" width="0.1524" layer="91"/>
<wire x1="99.06" y1="76.2" x2="99.06" y2="109.22" width="0.1524" layer="91"/>
<wire x1="99.06" y1="109.22" x2="139.7" y2="109.22" width="0.1524" layer="91"/>
</segment>
</net>
<net name="LCD_P0" class="0">
<segment>
<pinref part="U3" gate="G$1" pin="P0"/>
<wire x1="88.9" y1="99.06" x2="99.06" y2="99.06" width="0.1524" layer="91"/>
<label x="99.06" y="99.06" size="1.27" layer="95" xref="yes"/>
</segment>
</net>
<net name="LCD_P1" class="0">
<segment>
<pinref part="U3" gate="G$1" pin="P1"/>
<wire x1="88.9" y1="96.52" x2="99.06" y2="96.52" width="0.1524" layer="91"/>
<label x="99.06" y="96.52" size="1.27" layer="95" xref="yes"/>
</segment>
</net>
<net name="LCD_P2" class="0">
<segment>
<pinref part="U3" gate="G$1" pin="P2"/>
<wire x1="88.9" y1="93.98" x2="99.06" y2="93.98" width="0.1524" layer="91"/>
<label x="99.06" y="93.98" size="1.27" layer="95" xref="yes"/>
</segment>
</net>
<net name="LCD_P3" class="0">
<segment>
<pinref part="U3" gate="G$1" pin="P3"/>
<wire x1="88.9" y1="91.44" x2="99.06" y2="91.44" width="0.1524" layer="91"/>
<label x="99.06" y="91.44" size="1.27" layer="95" xref="yes"/>
</segment>
</net>
</nets>
</sheet>
<!-- ============================================================== -->
<!-- SHEET 4: NRI G-13 Coin Acceptor + Hoppers                      -->
<!-- ============================================================== -->
<sheet>
<plain>
<text x="2.54" y="101.6" size="2.54" layer="97">Sheet 4/5: NRI G-13.6000 + Хопперы</text>
<text x="2.54" y="96.52" size="1.778" layer="97">Coin CH1-6: PB8-PB13 (NPN BC847, инверсия active-low→HIGH)</text>
<text x="2.54" y="91.44" size="1.778" layer="97">Coin BLOCK: PB14 (прямое, active HIGH)</text>
<text x="2.54" y="86.36" size="1.778" layer="97">Hopper A Enable: PB15 (NPN BC847→мотор)</text>
<wire x1="0" y1="0" x2="266.7" y2="0" width="0.254" layer="97"/>
<wire x1="266.7" y1="0" x2="266.7" y2="182.88" width="0.254" layer="97"/>
<wire x1="266.7" y1="182.88" x2="0" y2="182.88" width="0.254" layer="97"/>
<wire x1="0" y1="182.88" x2="0" y2="0" width="0.254" layer="97"/>
</plain>
<instances>
<!-- NRI connector -->
<instance part="J_NRI" gate="G$1" x="160.02" y="60.96"/>
<!-- 6x NPN for coin channels -->
<instance part="Q1" gate="G$1" x="50.8" y="119.38"/>
<instance part="Q2" gate="G$1" x="50.8" y="99.06"/>
<instance part="Q3" gate="G$1" x="50.8" y="78.74"/>
<instance part="Q4" gate="G$1" x="50.8" y="58.42"/>
<instance part="Q5" gate="G$1" x="50.8" y="38.1"/>
<instance part="Q6" gate="G$1" x="50.8" y="17.78"/>
<!-- Base resistors -->
<instance part="R_B1" gate="G$1" x="35.56" y="119.38"/>
<instance part="R_B2" gate="G$1" x="35.56" y="99.06"/>
<instance part="R_B3" gate="G$1" x="35.56" y="78.74"/>
<instance part="R_B4" gate="G$1" x="35.56" y="58.42"/>
<instance part="R_B5" gate="G$1" x="35.56" y="38.1"/>
<instance part="R_B6" gate="G$1" x="35.56" y="17.78"/>
<!-- Collector pull-ups -->
<instance part="R_C1" gate="G$1" x="50.8" y="132.08" rot="R90"/>
<instance part="R_C2" gate="G$1" x="50.8" y="111.76" rot="R90"/>
<instance part="R_C3" gate="G$1" x="50.8" y="91.44" rot="R90"/>
<instance part="R_C4" gate="G$1" x="50.8" y="71.12" rot="R90"/>
<instance part="R_C5" gate="G$1" x="50.8" y="50.8" rot="R90"/>
<instance part="R_C6" gate="G$1" x="50.8" y="30.48" rot="R90"/>
<!-- Coin BLOCK series resistor -->
<instance part="R_BLOCK" gate="G$1" x="86.36" y="55.88"/>
<!-- Hopper A NPN -->
<instance part="Q_HOPA" gate="G$1" x="139.7" y="30.48"/>
<instance part="R_HOPA_B" gate="G$1" x="124.46" y="30.48"/>
<instance part="R_HOPA_C" gate="G$1" x="139.7" y="43.18" rot="R90"/>
<!-- Hopper connector -->
<instance part="J_HOP" gate="G$1" x="160.02" y="17.78"/>
<!-- Power -->
<instance part="+3V3" gate="G$1" x="25.4" y="142.24"/>
<instance part="+12V" gate="G$1" x="170.18" y="93.98"/>
<instance part="GND" gate="G$1" x="50.8" y="5.08"/>
</instances>
<nets>
<net name="PB8" class="0">
<segment>
<pinref part="R_B1" gate="G$1" pin="1"/>
<wire x1="30.48" y1="119.38" x2="25.4" y2="119.38" width="0.1524" layer="91"/>
<label x="25.4" y="119.38" size="1.27" layer="95" rot="R180" xref="yes"/>
</segment>
</net>
<net name="PB9" class="0">
<segment>
<pinref part="R_B2" gate="G$1" pin="1"/>
<wire x1="30.48" y1="99.06" x2="25.4" y2="99.06" width="0.1524" layer="91"/>
<label x="25.4" y="99.06" size="1.27" layer="95" rot="R180" xref="yes"/>
</segment>
</net>
<net name="PB10" class="0">
<segment>
<pinref part="R_B3" gate="G$1" pin="1"/>
<wire x1="30.48" y1="78.74" x2="25.4" y2="78.74" width="0.1524" layer="91"/>
<label x="25.4" y="78.74" size="1.27" layer="95" rot="R180" xref="yes"/>
</segment>
</net>
<net name="PB11" class="0">
<segment>
<pinref part="R_B4" gate="G$1" pin="1"/>
<wire x1="30.48" y1="58.42" x2="25.4" y2="58.42" width="0.1524" layer="91"/>
<label x="25.4" y="58.42" size="1.27" layer="95" rot="R180" xref="yes"/>
</segment>
</net>
<net name="PB12" class="0">
<segment>
<pinref part="R_B5" gate="G$1" pin="1"/>
<wire x1="30.48" y1="38.1" x2="25.4" y2="38.1" width="0.1524" layer="91"/>
<label x="25.4" y="38.1" size="1.27" layer="95" rot="R180" xref="yes"/>
</segment>
</net>
<net name="PB13" class="0">
<segment>
<pinref part="R_B6" gate="G$1" pin="1"/>
<wire x1="30.48" y1="17.78" x2="25.4" y2="17.78" width="0.1524" layer="91"/>
<label x="25.4" y="17.78" size="1.27" layer="95" rot="R180" xref="yes"/>
</segment>
</net>
<net name="PB14" class="0">
<segment>
<pinref part="R_BLOCK" gate="G$1" pin="1"/>
<wire x1="81.28" y1="55.88" x2="76.2" y2="55.88" width="0.1524" layer="91"/>
<label x="76.2" y="55.88" size="1.27" layer="95" rot="R180" xref="yes"/>
</segment>
</net>
<net name="PB15" class="0">
<segment>
<pinref part="R_HOPA_B" gate="G$1" pin="1"/>
<wire x1="119.38" y1="30.48" x2="114.3" y2="30.48" width="0.1524" layer="91"/>
<label x="114.3" y="30.48" size="1.27" layer="95" rot="R180" xref="yes"/>
</segment>
</net>
<net name="+3V3" class="0">
<segment>
<pinref part="+3V3" gate="G$1" pin="+3V3"/>
<pinref part="R_C1" gate="G$1" pin="2"/>
<wire x1="25.4" y1="142.24" x2="25.4" y2="137.16" width="0.1524" layer="91"/>
<wire x1="25.4" y1="137.16" x2="50.8" y2="137.16" width="0.1524" layer="91"/>
<pinref part="R_C2" gate="G$1" pin="2"/>
<wire x1="50.8" y1="116.84" x2="25.4" y2="116.84" width="0.1524" layer="91"/>
<wire x1="25.4" y1="116.84" x2="25.4" y2="137.16" width="0.1524" layer="91"/>
<junction x="25.4" y="137.16"/>
<pinref part="R_C3" gate="G$1" pin="2"/>
<wire x1="50.8" y1="96.52" x2="25.4" y2="96.52" width="0.1524" layer="91"/>
<wire x1="25.4" y1="96.52" x2="25.4" y2="116.84" width="0.1524" layer="91"/>
<junction x="25.4" y="116.84"/>
<pinref part="R_C4" gate="G$1" pin="2"/>
<wire x1="50.8" y1="76.2" x2="25.4" y2="76.2" width="0.1524" layer="91"/>
<wire x1="25.4" y1="76.2" x2="25.4" y2="96.52" width="0.1524" layer="91"/>
<junction x="25.4" y="96.52"/>
<pinref part="R_C5" gate="G$1" pin="2"/>
<wire x1="50.8" y1="55.88" x2="25.4" y2="55.88" width="0.1524" layer="91"/>
<wire x1="25.4" y1="55.88" x2="25.4" y2="76.2" width="0.1524" layer="91"/>
<junction x="25.4" y="76.2"/>
<pinref part="R_C6" gate="G$1" pin="2"/>
<wire x1="50.8" y1="35.56" x2="25.4" y2="35.56" width="0.1524" layer="91"/>
<wire x1="25.4" y1="35.56" x2="25.4" y2="55.88" width="0.1524" layer="91"/>
<junction x="25.4" y="55.88"/>
<pinref part="R_HOPA_C" gate="G$1" pin="2"/>
<wire x1="139.7" y1="48.26" x2="139.7" y2="50.8" width="0.1524" layer="91"/>
<wire x1="139.7" y1="50.8" x2="25.4" y2="50.8" width="0.1524" layer="91"/>
<wire x1="25.4" y1="50.8" x2="25.4" y2="35.56" width="0.1524" layer="91"/>
<junction x="25.4" y="35.56"/>
</segment>
</net>
<net name="GND" class="0">
<segment>
<pinref part="GND" gate="G$1" pin="GND"/>
<pinref part="Q1" gate="G$1" pin="E"/>
<wire x1="50.8" y1="116.84" x2="43.18" y2="116.84" width="0.1524" layer="91"/>
<wire x1="43.18" y1="116.84" x2="43.18" y2="7.62" width="0.1524" layer="91"/>
<pinref part="Q2" gate="G$1" pin="E"/>
<wire x1="50.8" y1="96.52" x2="43.18" y2="96.52" width="0.1524" layer="91"/>
<junction x="43.18" y="96.52"/>
<pinref part="Q3" gate="G$1" pin="E"/>
<wire x1="50.8" y1="76.2" x2="43.18" y2="76.2" width="0.1524" layer="91"/>
<junction x="43.18" y="76.2"/>
<pinref part="Q4" gate="G$1" pin="E"/>
<wire x1="50.8" y1="55.88" x2="43.18" y2="55.88" width="0.1524" layer="91"/>
<junction x="43.18" y="55.88"/>
<pinref part="Q5" gate="G$1" pin="E"/>
<wire x1="50.8" y1="35.56" x2="43.18" y2="35.56" width="0.1524" layer="91"/>
<junction x="43.18" y="35.56"/>
<pinref part="Q6" gate="G$1" pin="E"/>
<wire x1="50.8" y1="15.24" x2="43.18" y2="15.24" width="0.1524" layer="91"/>
<junction x="43.18" y="15.24"/>
<wire x1="43.18" y1="7.62" x2="50.8" y2="7.62" width="0.1524" layer="91"/>
<wire x1="50.8" y1="7.62" x2="50.8" y2="5.08" width="0.1524" layer="91"/>
<junction x="43.18" y="7.62"/>
<pinref part="Q_HOPA" gate="G$1" pin="E"/>
<wire x1="139.7" y1="27.94" x2="139.7" y2="7.62" width="0.1524" layer="91"/>
<wire x1="139.7" y1="7.62" x2="50.8" y2="7.62" width="0.1524" layer="91"/>
<junction x="50.8" y="7.62"/>
</segment>
</net>
<net name="COIN_CH1_C" class="0">
<segment>
<pinref part="Q1" gate="G$1" pin="C"/>
<pinref part="R_C1" gate="G$1" pin="1"/>
<wire x1="50.8" y1="127" x2="50.8" y2="124.46" width="0.1524" layer="91"/>
<label x="58.42" y="127" size="1.27" layer="95" xref="yes"/>
</segment>
</net>
<net name="COIN_CH2_C" class="0">
<segment>
<pinref part="Q2" gate="G$1" pin="C"/>
<pinref part="R_C2" gate="G$1" pin="1"/>
<wire x1="50.8" y1="106.68" x2="50.8" y2="104.14" width="0.1524" layer="91"/>
<label x="58.42" y="106.68" size="1.27" layer="95" xref="yes"/>
</segment>
</net>
<net name="COIN_CH3_C" class="0">
<segment>
<pinref part="Q3" gate="G$1" pin="C"/>
<pinref part="R_C3" gate="G$1" pin="1"/>
<wire x1="50.8" y1="86.36" x2="50.8" y2="83.82" width="0.1524" layer="91"/>
<label x="58.42" y="86.36" size="1.27" layer="95" xref="yes"/>
</segment>
</net>
<net name="COIN_CH4_C" class="0">
<segment>
<pinref part="Q4" gate="G$1" pin="C"/>
<pinref part="R_C4" gate="G$1" pin="1"/>
<wire x1="50.8" y1="66.04" x2="50.8" y2="63.5" width="0.1524" layer="91"/>
<label x="58.42" y="66.04" size="1.27" layer="95" xref="yes"/>
</segment>
</net>
<net name="COIN_CH5_C" class="0">
<segment>
<pinref part="Q5" gate="G$1" pin="C"/>
<pinref part="R_C5" gate="G$1" pin="1"/>
<wire x1="50.8" y1="45.72" x2="50.8" y2="43.18" width="0.1524" layer="91"/>
<label x="58.42" y="45.72" size="1.27" layer="95" xref="yes"/>
</segment>
</net>
<net name="COIN_CH6_C" class="0">
<segment>
<pinref part="Q6" gate="G$1" pin="C"/>
<pinref part="R_C6" gate="G$1" pin="1"/>
<wire x1="50.8" y1="25.4" x2="50.8" y2="22.86" width="0.1524" layer="91"/>
<label x="58.42" y="25.4" size="1.27" layer="95" xref="yes"/>
</segment>
</net>
<net name="COIN_BLOCK_OUT" class="0">
<segment>
<pinref part="R_BLOCK" gate="G$1" pin="2"/>
<wire x1="91.44" y1="55.88" x2="96.52" y2="55.88" width="0.1524" layer="91"/>
<label x="96.52" y="55.88" size="1.27" layer="95" xref="yes"/>
</segment>
</net>
<net name="HOPPA_C" class="0">
<segment>
<pinref part="Q_HOPA" gate="G$1" pin="C"/>
<pinref part="R_HOPA_C" gate="G$1" pin="1"/>
<wire x1="139.7" y1="38.1" x2="139.7" y2="35.56" width="0.1524" layer="91"/>
<label x="147.32" y="38.1" size="1.27" layer="95" xref="yes"/>
</segment>
</net>
<net name="+12V" class="0">
<segment>
<pinref part="+12V" gate="G$1" pin="+12V"/>
<pinref part="J_NRI" gate="G$1" pin="2"/>
<wire x1="170.18" y1="93.98" x2="170.18" y2="81.28" width="0.1524" layer="91"/>
<wire x1="170.18" y1="81.28" x2="154.94" y2="81.28" width="0.1524" layer="91"/>
</segment>
</net>
</nets>
</sheet>
<!-- ============================================================== -->
<!-- SHEET 5: Buttons + Doors + iButton + LED                       -->
<!-- ============================================================== -->
<sheet>
<plain>
<text x="2.54" y="101.6" size="2.54" layer="97">Sheet 5/5: Кнопки + Двери + iButton + LED</text>
<text x="2.54" y="96.52" size="1.778" layer="97">PREV=PA3, NEXT=PA4, OK=PA5, CANCEL=PA6 — pull-up 10kΩ к GND</text>
<text x="2.54" y="91.44" size="1.778" layer="97">Door1=PA7, Door2=PA8 — геркон, pull-up 10kΩ</text>
<text x="2.54" y="86.36" size="1.778" layer="97">iButton: PA11 (1-Wire, 4.7kΩ pull-up к +3.3V)</text>
<text x="2.54" y="81.28" size="1.778" layer="97">LED_GREEN=PB0(1kΩ), LED_RED=PC15(1kΩ), PowerFail=PB1</text>
<wire x1="0" y1="0" x2="266.7" y2="0" width="0.254" layer="97"/>
<wire x1="266.7" y1="0" x2="266.7" y2="182.88" width="0.254" layer="97"/>
<wire x1="266.7" y1="182.88" x2="0" y2="182.88" width="0.254" layer="97"/>
<wire x1="0" y1="182.88" x2="0" y2="0" width="0.254" layer="97"/>
</plain>
<instances>
<!-- Button pull-ups -->
<instance part="R_BTN1" gate="G$1" x="33.02" y="91.44" rot="R90"/>
<instance part="R_BTN2" gate="G$1" x="43.18" y="91.44" rot="R90"/>
<instance part="R_BTN3" gate="G$1" x="53.34" y="91.44" rot="R90"/>
<instance part="R_BTN4" gate="G$1" x="63.5" y="91.44" rot="R90"/>
<!-- Button connector -->
<instance part="J_BTN" gate="G$1" x="86.36" y="78.74"/>
<!-- Door pull-ups -->
<instance part="R_DOOR1" gate="G$1" x="33.02" y="53.34" rot="R90"/>
<instance part="R_DOOR2" gate="G$1" x="43.18" y="53.34" rot="R90"/>
<!-- Door connectors -->
<instance part="J_DOOR1" gate="G$1" x="86.36" y="55.88"/>
<instance part="J_DOOR2" gate="G$1" x="86.36" y="45.72"/>
<!-- iButton -->
<instance part="R_1W" gate="G$1" x="33.02" y="27.94" rot="R90"/>
<instance part="X_IBTN" gate="G$1" x="86.36" y="27.94"/>
<!-- LEDs -->
<instance part="D_LEDG" gate="G$1" x="154.94" y="86.36"/>
<instance part="D_LEDR" gate="G$1" x="154.94" y="68.58"/>
<instance part="R_LEDG" gate="G$1" x="154.94" y="96.52" rot="R90"/>
<instance part="R_LEDR" gate="G$1" x="154.94" y="78.74" rot="R90"/>
<!-- Power Fail connector -->
<instance part="J_PWRFAIL" gate="G$1" x="86.36" y="35.56"/>
<!-- Power -->
<instance part="+3V3" gate="G$1" x="22.86" y="109.22"/>
<instance part="GND" gate="G$1" x="22.86" y="12.7"/>
</instances>
<nets>
<net name="PA3" class="0">
<segment>
<pinref part="R_BTN1" gate="G$1" pin="1"/>
<wire x1="33.02" y1="86.36" x2="33.02" y2="83.82" width="0.1524" layer="91"/>
<label x="33.02" y="83.82" size="1.27" layer="95" rot="R270" xref="yes"/>
</segment>
</net>
<net name="PA4" class="0">
<segment>
<pinref part="R_BTN2" gate="G$1" pin="1"/>
<wire x1="43.18" y1="86.36" x2="43.18" y2="83.82" width="0.1524" layer="91"/>
<label x="43.18" y="83.82" size="1.27" layer="95" rot="R270" xref="yes"/>
</segment>
</net>
<net name="PA5" class="0">
<segment>
<pinref part="R_BTN3" gate="G$1" pin="1"/>
<wire x1="53.34" y1="86.36" x2="53.34" y2="83.82" width="0.1524" layer="91"/>
<label x="53.34" y="83.82" size="1.27" layer="95" rot="R270" xref="yes"/>
</segment>
</net>
<net name="PA6" class="0">
<segment>
<pinref part="R_BTN4" gate="G$1" pin="1"/>
<wire x1="63.5" y1="86.36" x2="63.5" y2="83.82" width="0.1524" layer="91"/>
<label x="63.5" y="83.82" size="1.27" layer="95" rot="R270" xref="yes"/>
</segment>
</net>
<net name="PA7" class="0">
<segment>
<pinref part="R_DOOR1" gate="G$1" pin="1"/>
<wire x1="33.02" y1="48.26" x2="33.02" y2="45.72" width="0.1524" layer="91"/>
<label x="33.02" y="45.72" size="1.27" layer="95" rot="R270" xref="yes"/>
</segment>
</net>
<net name="PA8" class="0">
<segment>
<pinref part="R_DOOR2" gate="G$1" pin="1"/>
<wire x1="43.18" y1="48.26" x2="43.18" y2="45.72" width="0.1524" layer="91"/>
<label x="43.18" y="45.72" size="1.27" layer="95" rot="R270" xref="yes"/>
</segment>
</net>
<net name="PA11" class="0">
<segment>
<pinref part="R_1W" gate="G$1" pin="1"/>
<wire x1="33.02" y1="22.86" x2="33.02" y2="20.32" width="0.1524" layer="91"/>
<label x="33.02" y="20.32" size="1.27" layer="95" rot="R270" xref="yes"/>
</segment>
</net>
<net name="+3V3" class="0">
<segment>
<pinref part="+3V3" gate="G$1" pin="+3V3"/>
<pinref part="R_BTN1" gate="G$1" pin="2"/>
<wire x1="22.86" y1="109.22" x2="22.86" y2="96.52" width="0.1524" layer="91"/>
<wire x1="22.86" y1="96.52" x2="33.02" y2="96.52" width="0.1524" layer="91"/>
<pinref part="R_BTN2" gate="G$1" pin="2"/>
<wire x1="43.18" y1="96.52" x2="33.02" y2="96.52" width="0.1524" layer="91"/>
<junction x="33.02" y="96.52"/>
<pinref part="R_BTN3" gate="G$1" pin="2"/>
<wire x1="53.34" y1="96.52" x2="43.18" y2="96.52" width="0.1524" layer="91"/>
<junction x="43.18" y="96.52"/>
<pinref part="R_BTN4" gate="G$1" pin="2"/>
<wire x1="63.5" y1="96.52" x2="53.34" y2="96.52" width="0.1524" layer="91"/>
<junction x="53.34" y="96.52"/>
<pinref part="R_DOOR1" gate="G$1" pin="2"/>
<wire x1="33.02" y1="58.42" x2="22.86" y2="58.42" width="0.1524" layer="91"/>
<wire x1="22.86" y1="58.42" x2="22.86" y2="96.52" width="0.1524" layer="91"/>
<junction x="22.86" y="96.52"/>
<pinref part="R_DOOR2" gate="G$1" pin="2"/>
<wire x1="43.18" y1="58.42" x2="33.02" y2="58.42" width="0.1524" layer="91"/>
<junction x="33.02" y="58.42"/>
<pinref part="R_1W" gate="G$1" pin="2"/>
<wire x1="33.02" y1="33.02" x2="22.86" y2="33.02" width="0.1524" layer="91"/>
<wire x1="22.86" y1="33.02" x2="22.86" y2="58.42" width="0.1524" layer="91"/>
<junction x="22.86" y="58.42"/>
</segment>
</net>
<net name="GND" class="0">
<segment>
<pinref part="GND" gate="G$1" pin="GND"/>
<pinref part="J_DOOR1" gate="G$1" pin="2"/>
<wire x1="81.28" y1="58.42" x2="73.66" y2="58.42" width="0.1524" layer="91"/>
<wire x1="73.66" y1="58.42" x2="73.66" y2="15.24" width="0.1524" layer="91"/>
<wire x1="73.66" y1="15.24" x2="22.86" y2="15.24" width="0.1524" layer="91"/>
<pinref part="J_DOOR2" gate="G$1" pin="2"/>
<wire x1="81.28" y1="48.26" x2="73.66" y2="48.26" width="0.1524" layer="91"/>
<wire x1="73.66" y1="48.26" x2="73.66" y2="15.24" width="0.1524" layer="91"/>
<junction x="73.66" y="15.24"/>
<pinref part="X_IBTN" gate="G$1" pin="GND"/>
<wire x1="93.98" y1="27.94" x2="99.06" y2="27.94" width="0.1524" layer="91"/>
<wire x1="99.06" y1="27.94" x2="99.06" y2="15.24" width="0.1524" layer="91"/>
<wire x1="99.06" y1="15.24" x2="73.66" y2="15.24" width="0.1524" layer="91"/>
<junction x="73.66" y="15.24"/>
<pinref part="D_LEDG" gate="G$1" pin="K"/>
<wire x1="154.94" y1="81.28" x2="154.94" y2="15.24" width="0.1524" layer="91"/>
<wire x1="154.94" y1="15.24" x2="99.06" y2="15.24" width="0.1524" layer="91"/>
<junction x="99.06" y="15.24"/>
<pinref part="D_LEDR" gate="G$1" pin="K"/>
<wire x1="154.94" y1="63.5" x2="154.94" y2="15.24" width="0.1524" layer="91"/>
<junction x="154.94" y="15.24"/>
<pinref part="J_PWRFAIL" gate="G$1" pin="2"/>
<wire x1="81.28" y1="38.1" x2="73.66" y2="38.1" width="0.1524" layer="91"/>
<wire x1="73.66" y1="38.1" x2="73.66" y2="15.24" width="0.1524" layer="91"/>
<junction x="73.66" y="15.24"/>
</segment>
</net>
<net name="PB0" class="0">
<segment>
<pinref part="R_LEDG" gate="G$1" pin="1"/>
<wire x1="154.94" y1="91.44" x2="154.94" y2="93.98" width="0.1524" layer="91"/>
<label x="154.94" y="93.98" size="1.27" layer="95" rot="R90" xref="yes"/>
</segment>
</net>
<net name="PC15" class="0">
<segment>
<pinref part="R_LEDR" gate="G$1" pin="1"/>
<wire x1="154.94" y1="73.66" x2="154.94" y2="76.2" width="0.1524" layer="91"/>
<label x="154.94" y="76.2" size="1.27" layer="95" rot="R90" xref="yes"/>
</segment>
</net>
<net name="PB1" class="0">
<segment>
<pinref part="J_PWRFAIL" gate="G$1" pin="1"/>
<wire x1="81.28" y1="40.64" x2="73.66" y2="40.64" width="0.1524" layer="91"/>
<label x="73.66" y="40.64" size="1.27" layer="95" rot="R180" xref="yes"/>
</segment>
</net>
</nets>
</sheet>
</sheets>
</schematic>
</drawing>
</eagle>