#[doc = "Reader of register ICSR"]
pub type R = crate::R<u32, super::ICSR>;
#[doc = "Writer for register ICSR"]
pub type W = crate::W<u32, super::ICSR>;
#[doc = "Register ICSR `reset()`'s with value 0"]
impl crate::ResetValue for super::ICSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VECTACTIVE`"]
pub type VECTACTIVE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `VECTACTIVE`"]
pub struct VECTACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> VECTACTIVE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
#[doc = "Reader of field `VECTPENDING`"]
pub type VECTPENDING_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `VECTPENDING`"]
pub struct VECTPENDING_W<'a> {
    w: &'a mut W,
}
impl<'a> VECTPENDING_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 12)) | (((value as u32) & 0x01ff) << 12);
        self.w
    }
}
#[doc = "Reader of field `ISRPENDING`"]
pub type ISRPENDING_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISRPENDING`"]
pub struct ISRPENDING_W<'a> {
    w: &'a mut W,
}
impl<'a> ISRPENDING_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `ISRPREEMPT`"]
pub type ISRPREEMPT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISRPREEMPT`"]
pub struct ISRPREEMPT_W<'a> {
    w: &'a mut W,
}
impl<'a> ISRPREEMPT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Possible values of the field `PENDSTCLR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PENDSTCLR_A {
    #[doc = "No effect"]
    VALUE_0,
    #[doc = "Removes the pending state from the SysTick exception"]
    VALUE_1,
}
impl crate::ToBits<bool> for PENDSTCLR_A {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            PENDSTCLR_A::VALUE_0 => false,
            PENDSTCLR_A::VALUE_1 => true,
        }
    }
}
#[doc = "Reader of field `PENDSTCLR`"]
pub type PENDSTCLR_R = crate::R<bool, PENDSTCLR_A>;
impl PENDSTCLR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PENDSTCLR_A {
        match self.bits {
            false => PENDSTCLR_A::VALUE_0,
            true => PENDSTCLR_A::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == PENDSTCLR_A::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == PENDSTCLR_A::VALUE_1
    }
}
#[doc = "Write proxy for field `PENDSTCLR`"]
pub struct PENDSTCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> PENDSTCLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PENDSTCLR_A) -> &'a mut W {
        use crate::ToBits;
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut W {
        self.variant(PENDSTCLR_A::VALUE_0)
    }
    #[doc = "Removes the pending state from the SysTick exception"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut W {
        self.variant(PENDSTCLR_A::VALUE_1)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Possible values of the field `PENDSTSET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PENDSTSET_A {
    #[doc = "Write: no effect; read: SysTick exception is not pending"]
    VALUE_0,
    #[doc = "Write: changes SysTick exception state to pending; read: SysTick exception is pending"]
    VALUE_1,
}
impl crate::ToBits<bool> for PENDSTSET_A {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            PENDSTSET_A::VALUE_0 => false,
            PENDSTSET_A::VALUE_1 => true,
        }
    }
}
#[doc = "Reader of field `PENDSTSET`"]
pub type PENDSTSET_R = crate::R<bool, PENDSTSET_A>;
impl PENDSTSET_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PENDSTSET_A {
        match self.bits {
            false => PENDSTSET_A::VALUE_0,
            true => PENDSTSET_A::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == PENDSTSET_A::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == PENDSTSET_A::VALUE_1
    }
}
#[doc = "Write proxy for field `PENDSTSET`"]
pub struct PENDSTSET_W<'a> {
    w: &'a mut W,
}
impl<'a> PENDSTSET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PENDSTSET_A) -> &'a mut W {
        use crate::ToBits;
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Write: no effect; read: SysTick exception is not pending"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut W {
        self.variant(PENDSTSET_A::VALUE_0)
    }
    #[doc = "Write: changes SysTick exception state to pending; read: SysTick exception is pending"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut W {
        self.variant(PENDSTSET_A::VALUE_1)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Possible values of the field `PENDSVCLR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PENDSVCLR_A {
    #[doc = "No effect"]
    VALUE_0,
    #[doc = "Removes the pending state from the PendSV exception"]
    VALUE_1,
}
impl crate::ToBits<bool> for PENDSVCLR_A {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            PENDSVCLR_A::VALUE_0 => false,
            PENDSVCLR_A::VALUE_1 => true,
        }
    }
}
#[doc = "Reader of field `PENDSVCLR`"]
pub type PENDSVCLR_R = crate::R<bool, PENDSVCLR_A>;
impl PENDSVCLR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PENDSVCLR_A {
        match self.bits {
            false => PENDSVCLR_A::VALUE_0,
            true => PENDSVCLR_A::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == PENDSVCLR_A::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == PENDSVCLR_A::VALUE_1
    }
}
#[doc = "Write proxy for field `PENDSVCLR`"]
pub struct PENDSVCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> PENDSVCLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PENDSVCLR_A) -> &'a mut W {
        use crate::ToBits;
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut W {
        self.variant(PENDSVCLR_A::VALUE_0)
    }
    #[doc = "Removes the pending state from the PendSV exception"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut W {
        self.variant(PENDSVCLR_A::VALUE_1)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Possible values of the field `PENDSVSET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PENDSVSET_A {
    #[doc = "Write: no effect; read: PendSV exception is not pending"]
    VALUE_0,
    #[doc = "Write: changes PendSV exception state to pending; read: PendSV exception is pending"]
    VALUE_1,
}
impl crate::ToBits<bool> for PENDSVSET_A {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            PENDSVSET_A::VALUE_0 => false,
            PENDSVSET_A::VALUE_1 => true,
        }
    }
}
#[doc = "Reader of field `PENDSVSET`"]
pub type PENDSVSET_R = crate::R<bool, PENDSVSET_A>;
impl PENDSVSET_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PENDSVSET_A {
        match self.bits {
            false => PENDSVSET_A::VALUE_0,
            true => PENDSVSET_A::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == PENDSVSET_A::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == PENDSVSET_A::VALUE_1
    }
}
#[doc = "Write proxy for field `PENDSVSET`"]
pub struct PENDSVSET_W<'a> {
    w: &'a mut W,
}
impl<'a> PENDSVSET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PENDSVSET_A) -> &'a mut W {
        use crate::ToBits;
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Write: no effect; read: PendSV exception is not pending"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut W {
        self.variant(PENDSVSET_A::VALUE_0)
    }
    #[doc = "Write: changes PendSV exception state to pending; read: PendSV exception is pending"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut W {
        self.variant(PENDSVSET_A::VALUE_1)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Possible values of the field `NMIPENDSET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NMIPENDSET_A {
    #[doc = "Write: no effect; read: NMI exception is not pending"]
    VALUE_0,
    #[doc = "Write: changes NMI exception state to pending; read: NMI exception is pending"]
    VALUE_1,
}
impl crate::ToBits<bool> for NMIPENDSET_A {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            NMIPENDSET_A::VALUE_0 => false,
            NMIPENDSET_A::VALUE_1 => true,
        }
    }
}
#[doc = "Reader of field `NMIPENDSET`"]
pub type NMIPENDSET_R = crate::R<bool, NMIPENDSET_A>;
impl NMIPENDSET_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NMIPENDSET_A {
        match self.bits {
            false => NMIPENDSET_A::VALUE_0,
            true => NMIPENDSET_A::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == NMIPENDSET_A::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == NMIPENDSET_A::VALUE_1
    }
}
#[doc = "Write proxy for field `NMIPENDSET`"]
pub struct NMIPENDSET_W<'a> {
    w: &'a mut W,
}
impl<'a> NMIPENDSET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NMIPENDSET_A) -> &'a mut W {
        use crate::ToBits;
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Write: no effect; read: NMI exception is not pending"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut W {
        self.variant(NMIPENDSET_A::VALUE_0)
    }
    #[doc = "Write: changes NMI exception state to pending; read: NMI exception is pending"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut W {
        self.variant(NMIPENDSET_A::VALUE_1)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - Debug: Exception number of currently executing exception, or 0 if thread mode"]
    #[inline(always)]
    pub fn vectactive(&self) -> VECTACTIVE_R {
        VECTACTIVE_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 12:20 - Exception number of the highest priority pending enabled exception"]
    #[inline(always)]
    pub fn vectpending(&self) -> VECTPENDING_R {
        VECTPENDING_R::new(((self.bits >> 12) & 0x01ff) as u16)
    }
    #[doc = "Bit 22 - Debug: NVIC interrupt pending"]
    #[inline(always)]
    pub fn isrpending(&self) -> ISRPENDING_R {
        ISRPENDING_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Debug: Pending exception serviced on exit from debug halt"]
    #[inline(always)]
    pub fn isrpreempt(&self) -> ISRPREEMPT_R {
        ISRPREEMPT_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 25 - SysTick exception clear-pending bit"]
    #[inline(always)]
    pub fn pendstclr(&self) -> PENDSTCLR_R {
        PENDSTCLR_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - SysTick exception set-pending bit"]
    #[inline(always)]
    pub fn pendstset(&self) -> PENDSTSET_R {
        PENDSTSET_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - PendSV clear-pending bit"]
    #[inline(always)]
    pub fn pendsvclr(&self) -> PENDSVCLR_R {
        PENDSVCLR_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - PendSV set-pending bit"]
    #[inline(always)]
    pub fn pendsvset(&self) -> PENDSVSET_R {
        PENDSVSET_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 31 - NMI set-pending bit"]
    #[inline(always)]
    pub fn nmipendset(&self) -> NMIPENDSET_R {
        NMIPENDSET_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - Debug: Exception number of currently executing exception, or 0 if thread mode"]
    #[inline(always)]
    pub fn vectactive(&mut self) -> VECTACTIVE_W {
        VECTACTIVE_W { w: self }
    }
    #[doc = "Bits 12:20 - Exception number of the highest priority pending enabled exception"]
    #[inline(always)]
    pub fn vectpending(&mut self) -> VECTPENDING_W {
        VECTPENDING_W { w: self }
    }
    #[doc = "Bit 22 - Debug: NVIC interrupt pending"]
    #[inline(always)]
    pub fn isrpending(&mut self) -> ISRPENDING_W {
        ISRPENDING_W { w: self }
    }
    #[doc = "Bit 23 - Debug: Pending exception serviced on exit from debug halt"]
    #[inline(always)]
    pub fn isrpreempt(&mut self) -> ISRPREEMPT_W {
        ISRPREEMPT_W { w: self }
    }
    #[doc = "Bit 25 - SysTick exception clear-pending bit"]
    #[inline(always)]
    pub fn pendstclr(&mut self) -> PENDSTCLR_W {
        PENDSTCLR_W { w: self }
    }
    #[doc = "Bit 26 - SysTick exception set-pending bit"]
    #[inline(always)]
    pub fn pendstset(&mut self) -> PENDSTSET_W {
        PENDSTSET_W { w: self }
    }
    #[doc = "Bit 27 - PendSV clear-pending bit"]
    #[inline(always)]
    pub fn pendsvclr(&mut self) -> PENDSVCLR_W {
        PENDSVCLR_W { w: self }
    }
    #[doc = "Bit 28 - PendSV set-pending bit"]
    #[inline(always)]
    pub fn pendsvset(&mut self) -> PENDSVSET_W {
        PENDSVSET_W { w: self }
    }
    #[doc = "Bit 31 - NMI set-pending bit"]
    #[inline(always)]
    pub fn nmipendset(&mut self) -> NMIPENDSET_W {
        NMIPENDSET_W { w: self }
    }
}
