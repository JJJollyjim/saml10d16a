#[doc = "Reader of register COMPCTRL[%s]"]
pub type R = crate::R<u32, super::COMPCTRL>;
#[doc = "Writer for register COMPCTRL[%s]"]
pub type W = crate::W<u32, super::COMPCTRL>;
#[doc = "Register COMPCTRL[%s] `reset()`'s with value 0"]
impl crate::ResetValue for super::COMPCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `SINGLE`"]
pub type SINGLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SINGLE`"]
pub struct SINGLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SINGLE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Possible values of the field `INTSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTSEL_A {
    #[doc = "Interrupt on comparator output toggle"]
    TOGGLE,
    #[doc = "Interrupt on comparator output rising"]
    RISING,
    #[doc = "Interrupt on comparator output falling"]
    FALLING,
    #[doc = "Interrupt on end of comparison (single-shot mode only)"]
    EOC,
}
impl crate::ToBits<u8> for INTSEL_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            INTSEL_A::TOGGLE => 0,
            INTSEL_A::RISING => 1,
            INTSEL_A::FALLING => 2,
            INTSEL_A::EOC => 3,
        }
    }
}
#[doc = "Reader of field `INTSEL`"]
pub type INTSEL_R = crate::R<u8, INTSEL_A>;
impl INTSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTSEL_A {
        match self.bits {
            0 => INTSEL_A::TOGGLE,
            1 => INTSEL_A::RISING,
            2 => INTSEL_A::FALLING,
            3 => INTSEL_A::EOC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == INTSEL_A::TOGGLE
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == INTSEL_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == INTSEL_A::FALLING
    }
    #[doc = "Checks if the value of the field is `EOC`"]
    #[inline(always)]
    pub fn is_eoc(&self) -> bool {
        *self == INTSEL_A::EOC
    }
}
#[doc = "Write proxy for field `INTSEL`"]
pub struct INTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> INTSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTSEL_A) -> &'a mut W {
        use crate::ToBits;
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Interrupt on comparator output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(INTSEL_A::TOGGLE)
    }
    #[doc = "Interrupt on comparator output rising"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(INTSEL_A::RISING)
    }
    #[doc = "Interrupt on comparator output falling"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(INTSEL_A::FALLING)
    }
    #[doc = "Interrupt on end of comparison (single-shot mode only)"]
    #[inline(always)]
    pub fn eoc(self) -> &'a mut W {
        self.variant(INTSEL_A::EOC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "Reader of field `RUNSTDBY`"]
pub type RUNSTDBY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RUNSTDBY`"]
pub struct RUNSTDBY_W<'a> {
    w: &'a mut W,
}
impl<'a> RUNSTDBY_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Possible values of the field `MUXNEG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MUXNEG_A {
    #[doc = "I/O pin 0"]
    PIN0,
    #[doc = "I/O pin 1"]
    PIN1,
    #[doc = "I/O pin 2"]
    PIN2,
    #[doc = "I/O pin 3"]
    PIN3,
    #[doc = "Ground"]
    GND,
    #[doc = "VDD scaler"]
    VSCALE,
    #[doc = "Internal bandgap voltage"]
    BANDGAP,
    #[doc = "DAC output (on AC0), OPAMP output (on AC1)"]
    DAC_OR_OPAMP,
}
impl crate::ToBits<u8> for MUXNEG_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            MUXNEG_A::PIN0 => 0,
            MUXNEG_A::PIN1 => 1,
            MUXNEG_A::PIN2 => 2,
            MUXNEG_A::PIN3 => 3,
            MUXNEG_A::GND => 4,
            MUXNEG_A::VSCALE => 5,
            MUXNEG_A::BANDGAP => 6,
            MUXNEG_A::DAC_OR_OPAMP => 7,
        }
    }
}
#[doc = "Reader of field `MUXNEG`"]
pub type MUXNEG_R = crate::R<u8, MUXNEG_A>;
impl MUXNEG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MUXNEG_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MUXNEG_A::PIN0),
            1 => Val(MUXNEG_A::PIN1),
            2 => Val(MUXNEG_A::PIN2),
            3 => Val(MUXNEG_A::PIN3),
            4 => Val(MUXNEG_A::GND),
            5 => Val(MUXNEG_A::VSCALE),
            6 => Val(MUXNEG_A::BANDGAP),
            7 => Val(MUXNEG_A::DAC_OR_OPAMP),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PIN0`"]
    #[inline(always)]
    pub fn is_pin0(&self) -> bool {
        *self == MUXNEG_A::PIN0
    }
    #[doc = "Checks if the value of the field is `PIN1`"]
    #[inline(always)]
    pub fn is_pin1(&self) -> bool {
        *self == MUXNEG_A::PIN1
    }
    #[doc = "Checks if the value of the field is `PIN2`"]
    #[inline(always)]
    pub fn is_pin2(&self) -> bool {
        *self == MUXNEG_A::PIN2
    }
    #[doc = "Checks if the value of the field is `PIN3`"]
    #[inline(always)]
    pub fn is_pin3(&self) -> bool {
        *self == MUXNEG_A::PIN3
    }
    #[doc = "Checks if the value of the field is `GND`"]
    #[inline(always)]
    pub fn is_gnd(&self) -> bool {
        *self == MUXNEG_A::GND
    }
    #[doc = "Checks if the value of the field is `VSCALE`"]
    #[inline(always)]
    pub fn is_vscale(&self) -> bool {
        *self == MUXNEG_A::VSCALE
    }
    #[doc = "Checks if the value of the field is `BANDGAP`"]
    #[inline(always)]
    pub fn is_bandgap(&self) -> bool {
        *self == MUXNEG_A::BANDGAP
    }
    #[doc = "Checks if the value of the field is `DAC_OR_OPAMP`"]
    #[inline(always)]
    pub fn is_opamp(&self) -> bool {
        *self == MUXNEG_A::DAC_OR_OPAMP
    }
}
#[doc = "Write proxy for field `MUXNEG`"]
pub struct MUXNEG_W<'a> {
    w: &'a mut W,
}
impl<'a> MUXNEG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MUXNEG_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "I/O pin 0"]
    #[inline(always)]
    pub fn pin0(self) -> &'a mut W {
        self.variant(MUXNEG_A::PIN0)
    }
    #[doc = "I/O pin 1"]
    #[inline(always)]
    pub fn pin1(self) -> &'a mut W {
        self.variant(MUXNEG_A::PIN1)
    }
    #[doc = "I/O pin 2"]
    #[inline(always)]
    pub fn pin2(self) -> &'a mut W {
        self.variant(MUXNEG_A::PIN2)
    }
    #[doc = "I/O pin 3"]
    #[inline(always)]
    pub fn pin3(self) -> &'a mut W {
        self.variant(MUXNEG_A::PIN3)
    }
    #[doc = "Ground"]
    #[inline(always)]
    pub fn gnd(self) -> &'a mut W {
        self.variant(MUXNEG_A::GND)
    }
    #[doc = "VDD scaler"]
    #[inline(always)]
    pub fn vscale(self) -> &'a mut W {
        self.variant(MUXNEG_A::VSCALE)
    }
    #[doc = "Internal bandgap voltage"]
    #[inline(always)]
    pub fn bandgap(self) -> &'a mut W {
        self.variant(MUXNEG_A::BANDGAP)
    }
    #[doc = "DAC output (on AC0) / OPAMP output (on AC1)"]
    #[inline(always)]
    pub fn dac_or_opamp(self) -> &'a mut W {
        self.variant(MUXNEG_A::DAC_OR_OPAMP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `MUXPOS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MUXPOS_A {
    #[doc = "I/O pin 0"]
    PIN0,
    #[doc = "I/O pin 1"]
    PIN1,
    #[doc = "I/O pin 2"]
    PIN2,
    #[doc = "I/O pin 3"]
    PIN3,
    #[doc = "VDD Scaler"]
    VSCALE,
}
impl crate::ToBits<u8> for MUXPOS_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            MUXPOS_A::PIN0 => 0,
            MUXPOS_A::PIN1 => 1,
            MUXPOS_A::PIN2 => 2,
            MUXPOS_A::PIN3 => 3,
            MUXPOS_A::VSCALE => 4,
        }
    }
}
#[doc = "Reader of field `MUXPOS`"]
pub type MUXPOS_R = crate::R<u8, MUXPOS_A>;
impl MUXPOS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MUXPOS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MUXPOS_A::PIN0),
            1 => Val(MUXPOS_A::PIN1),
            2 => Val(MUXPOS_A::PIN2),
            3 => Val(MUXPOS_A::PIN3),
            4 => Val(MUXPOS_A::VSCALE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PIN0`"]
    #[inline(always)]
    pub fn is_pin0(&self) -> bool {
        *self == MUXPOS_A::PIN0
    }
    #[doc = "Checks if the value of the field is `PIN1`"]
    #[inline(always)]
    pub fn is_pin1(&self) -> bool {
        *self == MUXPOS_A::PIN1
    }
    #[doc = "Checks if the value of the field is `PIN2`"]
    #[inline(always)]
    pub fn is_pin2(&self) -> bool {
        *self == MUXPOS_A::PIN2
    }
    #[doc = "Checks if the value of the field is `PIN3`"]
    #[inline(always)]
    pub fn is_pin3(&self) -> bool {
        *self == MUXPOS_A::PIN3
    }
    #[doc = "Checks if the value of the field is `VSCALE`"]
    #[inline(always)]
    pub fn is_vscale(&self) -> bool {
        *self == MUXPOS_A::VSCALE
    }
}
#[doc = "Write proxy for field `MUXPOS`"]
pub struct MUXPOS_W<'a> {
    w: &'a mut W,
}
impl<'a> MUXPOS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MUXPOS_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "I/O pin 0"]
    #[inline(always)]
    pub fn pin0(self) -> &'a mut W {
        self.variant(MUXPOS_A::PIN0)
    }
    #[doc = "I/O pin 1"]
    #[inline(always)]
    pub fn pin1(self) -> &'a mut W {
        self.variant(MUXPOS_A::PIN1)
    }
    #[doc = "I/O pin 2"]
    #[inline(always)]
    pub fn pin2(self) -> &'a mut W {
        self.variant(MUXPOS_A::PIN2)
    }
    #[doc = "I/O pin 3"]
    #[inline(always)]
    pub fn pin3(self) -> &'a mut W {
        self.variant(MUXPOS_A::PIN3)
    }
    #[doc = "VDD Scaler"]
    #[inline(always)]
    pub fn vscale(self) -> &'a mut W {
        self.variant(MUXPOS_A::VSCALE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Reader of field `SWAP`"]
pub type SWAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWAP`"]
pub struct SWAP_W<'a> {
    w: &'a mut W,
}
impl<'a> SWAP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Possible values of the field `SPEED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPEED_A {
    #[doc = "Low speed"]
    LOW,
    #[doc = "Medium low speed"]
    MEDLOW,
    #[doc = "Medium high speed"]
    MEDHIGH,
    #[doc = "High speed"]
    HIGH,
}
impl crate::ToBits<u8> for SPEED_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SPEED_A::LOW => 0,
            SPEED_A::MEDLOW => 1,
            SPEED_A::MEDHIGH => 2,
            SPEED_A::HIGH => 3,
        }
    }
}
#[doc = "Reader of field `SPEED`"]
pub type SPEED_R = crate::R<u8, SPEED_A>;
impl SPEED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPEED_A {
        match self.bits {
            0 => SPEED_A::LOW,
            1 => SPEED_A::MEDLOW,
            2 => SPEED_A::MEDHIGH,
            3 => SPEED_A::HIGH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == SPEED_A::LOW
    }
    #[doc = "Checks if the value of the field is `MEDLOW`"]
    #[inline(always)]
    pub fn is_medlow(&self) -> bool {
        *self == SPEED_A::MEDLOW
    }
    #[doc = "Checks if the value of the field is `MEDHIGH`"]
    #[inline(always)]
    pub fn is_medhigh(&self) -> bool {
        *self == SPEED_A::MEDHIGH
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SPEED_A::HIGH
    }
}
#[doc = "Write proxy for field `SPEED`"]
pub struct SPEED_W<'a> {
    w: &'a mut W,
}
impl<'a> SPEED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPEED_A) -> &'a mut W {
        use crate::ToBits;
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(SPEED_A::LOW)
    }
    #[doc = "Medium low speed"]
    #[inline(always)]
    pub fn medlow(self) -> &'a mut W {
        self.variant(SPEED_A::MEDLOW)
    }
    #[doc = "Medium high speed"]
    #[inline(always)]
    pub fn medhigh(self) -> &'a mut W {
        self.variant(SPEED_A::MEDHIGH)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(SPEED_A::HIGH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `HYSTEN`"]
pub type HYSTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HYSTEN`"]
pub struct HYSTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HYSTEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Possible values of the field `HYST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HYST_A {
    #[doc = "50mV"]
    HYST50,
    #[doc = "70mV"]
    HYST70,
    #[doc = "90mV"]
    HYST90,
    #[doc = "110mV"]
    HYST110,
}
impl crate::ToBits<u8> for HYST_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            HYST_A::HYST50 => 0,
            HYST_A::HYST70 => 1,
            HYST_A::HYST90 => 2,
            HYST_A::HYST110 => 3,
        }
    }
}
#[doc = "Reader of field `HYST`"]
pub type HYST_R = crate::R<u8, HYST_A>;
impl HYST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HYST_A {
        match self.bits {
            0 => HYST_A::HYST50,
            1 => HYST_A::HYST70,
            2 => HYST_A::HYST90,
            3 => HYST_A::HYST110,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HYST50`"]
    #[inline(always)]
    pub fn is_hyst50(&self) -> bool {
        *self == HYST_A::HYST50
    }
    #[doc = "Checks if the value of the field is `HYST70`"]
    #[inline(always)]
    pub fn is_hyst70(&self) -> bool {
        *self == HYST_A::HYST70
    }
    #[doc = "Checks if the value of the field is `HYST90`"]
    #[inline(always)]
    pub fn is_hyst90(&self) -> bool {
        *self == HYST_A::HYST90
    }
    #[doc = "Checks if the value of the field is `HYST110`"]
    #[inline(always)]
    pub fn is_hyst110(&self) -> bool {
        *self == HYST_A::HYST110
    }
}
#[doc = "Write proxy for field `HYST`"]
pub struct HYST_W<'a> {
    w: &'a mut W,
}
impl<'a> HYST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HYST_A) -> &'a mut W {
        use crate::ToBits;
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "50mV"]
    #[inline(always)]
    pub fn hyst50(self) -> &'a mut W {
        self.variant(HYST_A::HYST50)
    }
    #[doc = "70mV"]
    #[inline(always)]
    pub fn hyst70(self) -> &'a mut W {
        self.variant(HYST_A::HYST70)
    }
    #[doc = "90mV"]
    #[inline(always)]
    pub fn hyst90(self) -> &'a mut W {
        self.variant(HYST_A::HYST90)
    }
    #[doc = "110mV"]
    #[inline(always)]
    pub fn hyst110(self) -> &'a mut W {
        self.variant(HYST_A::HYST110)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Possible values of the field `FLEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEN_A {
    #[doc = "No filtering"]
    OFF,
    #[doc = "3-bit majority function (2 of 3)"]
    MAJ3,
    #[doc = "5-bit majority function (3 of 5)"]
    MAJ5,
}
impl crate::ToBits<u8> for FLEN_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            FLEN_A::OFF => 0,
            FLEN_A::MAJ3 => 1,
            FLEN_A::MAJ5 => 2,
        }
    }
}
#[doc = "Reader of field `FLEN`"]
pub type FLEN_R = crate::R<u8, FLEN_A>;
impl FLEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FLEN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FLEN_A::OFF),
            1 => Val(FLEN_A::MAJ3),
            2 => Val(FLEN_A::MAJ5),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == FLEN_A::OFF
    }
    #[doc = "Checks if the value of the field is `MAJ3`"]
    #[inline(always)]
    pub fn is_maj3(&self) -> bool {
        *self == FLEN_A::MAJ3
    }
    #[doc = "Checks if the value of the field is `MAJ5`"]
    #[inline(always)]
    pub fn is_maj5(&self) -> bool {
        *self == FLEN_A::MAJ5
    }
}
#[doc = "Write proxy for field `FLEN`"]
pub struct FLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLEN_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No filtering"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(FLEN_A::OFF)
    }
    #[doc = "3-bit majority function (2 of 3)"]
    #[inline(always)]
    pub fn maj3(self) -> &'a mut W {
        self.variant(FLEN_A::MAJ3)
    }
    #[doc = "5-bit majority function (3 of 5)"]
    #[inline(always)]
    pub fn maj5(self) -> &'a mut W {
        self.variant(FLEN_A::MAJ5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Possible values of the field `OUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUT_A {
    #[doc = "The output of COMPn is not routed to the COMPn I/O port"]
    OFF,
    #[doc = "The asynchronous output of COMPn is routed to the COMPn I/O port"]
    ASYNC,
    #[doc = "The synchronous output (including filtering) of COMPn is routed to the COMPn I/O port"]
    SYNC,
}
impl crate::ToBits<u8> for OUT_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            OUT_A::OFF => 0,
            OUT_A::ASYNC => 1,
            OUT_A::SYNC => 2,
        }
    }
}
#[doc = "Reader of field `OUT`"]
pub type OUT_R = crate::R<u8, OUT_A>;
impl OUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OUT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(OUT_A::OFF),
            1 => Val(OUT_A::ASYNC),
            2 => Val(OUT_A::SYNC),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == OUT_A::OFF
    }
    #[doc = "Checks if the value of the field is `ASYNC`"]
    #[inline(always)]
    pub fn is_async(&self) -> bool {
        *self == OUT_A::ASYNC
    }
    #[doc = "Checks if the value of the field is `SYNC`"]
    #[inline(always)]
    pub fn is_sync(&self) -> bool {
        *self == OUT_A::SYNC
    }
}
#[doc = "Write proxy for field `OUT`"]
pub struct OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The output of COMPn is not routed to the COMPn I/O port"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(OUT_A::OFF)
    }
    #[doc = "The asynchronous output of COMPn is routed to the COMPn I/O port"]
    #[inline(always)]
    pub fn r#async(self) -> &'a mut W {
        self.variant(OUT_A::ASYNC)
    }
    #[doc = "The synchronous output (including filtering) of COMPn is routed to the COMPn I/O port"]
    #[inline(always)]
    pub fn sync(self) -> &'a mut W {
        self.variant(OUT_A::SYNC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Single-Shot Mode"]
    #[inline(always)]
    pub fn single(&self) -> SINGLE_R {
        SINGLE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - Interrupt Selection"]
    #[inline(always)]
    pub fn intsel(&self) -> INTSEL_R {
        INTSEL_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Negative Input Mux Selection"]
    #[inline(always)]
    pub fn muxneg(&self) -> MUXNEG_R {
        MUXNEG_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - Positive Input Mux Selection"]
    #[inline(always)]
    pub fn muxpos(&self) -> MUXPOS_R {
        MUXPOS_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 15 - Swap Inputs and Invert"]
    #[inline(always)]
    pub fn swap(&self) -> SWAP_R {
        SWAP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Speed Selection"]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 19 - Hysteresis Enable"]
    #[inline(always)]
    pub fn hysten(&self) -> HYSTEN_R {
        HYSTEN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 20:21 - Hysteresis Level"]
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 24:26 - Filter Length"]
    #[inline(always)]
    pub fn flen(&self) -> FLEN_R {
        FLEN_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 28:29 - Output"]
    #[inline(always)]
    pub fn out(&self) -> OUT_R {
        OUT_R::new(((self.bits >> 28) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bit 2 - Single-Shot Mode"]
    #[inline(always)]
    pub fn single(&mut self) -> SINGLE_W {
        SINGLE_W { w: self }
    }
    #[doc = "Bits 3:4 - Interrupt Selection"]
    #[inline(always)]
    pub fn intsel(&mut self) -> INTSEL_W {
        INTSEL_W { w: self }
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&mut self) -> RUNSTDBY_W {
        RUNSTDBY_W { w: self }
    }
    #[doc = "Bits 8:10 - Negative Input Mux Selection"]
    #[inline(always)]
    pub fn muxneg(&mut self) -> MUXNEG_W {
        MUXNEG_W { w: self }
    }
    #[doc = "Bits 12:14 - Positive Input Mux Selection"]
    #[inline(always)]
    pub fn muxpos(&mut self) -> MUXPOS_W {
        MUXPOS_W { w: self }
    }
    #[doc = "Bit 15 - Swap Inputs and Invert"]
    #[inline(always)]
    pub fn swap(&mut self) -> SWAP_W {
        SWAP_W { w: self }
    }
    #[doc = "Bits 16:17 - Speed Selection"]
    #[inline(always)]
    pub fn speed(&mut self) -> SPEED_W {
        SPEED_W { w: self }
    }
    #[doc = "Bit 19 - Hysteresis Enable"]
    #[inline(always)]
    pub fn hysten(&mut self) -> HYSTEN_W {
        HYSTEN_W { w: self }
    }
    #[doc = "Bits 20:21 - Hysteresis Level"]
    #[inline(always)]
    pub fn hyst(&mut self) -> HYST_W {
        HYST_W { w: self }
    }
    #[doc = "Bits 24:26 - Filter Length"]
    #[inline(always)]
    pub fn flen(&mut self) -> FLEN_W {
        FLEN_W { w: self }
    }
    #[doc = "Bits 28:29 - Output"]
    #[inline(always)]
    pub fn out(&mut self) -> OUT_W {
        OUT_W { w: self }
    }
}
