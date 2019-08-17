#[doc = "Reader of register CALIB"]
pub type R = crate::R<u32, super::CALIB>;
#[doc = "Reader of field `TENMS`"]
pub type TENMS_R = crate::R<u32, u32>;
#[doc = "Possible values of the field `SKEW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SKEW_A {
    #[doc = "10ms calibration value is exact"]
    VALUE_0,
    #[doc = "10ms calibration value is inexact, because of the clock frequency"]
    VALUE_1,
}
impl crate::ToBits<bool> for SKEW_A {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SKEW_A::VALUE_0 => false,
            SKEW_A::VALUE_1 => true,
        }
    }
}
#[doc = "Reader of field `SKEW`"]
pub type SKEW_R = crate::R<bool, SKEW_A>;
impl SKEW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SKEW_A {
        match self.bits {
            false => SKEW_A::VALUE_0,
            true => SKEW_A::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == SKEW_A::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == SKEW_A::VALUE_1
    }
}
#[doc = "Possible values of the field `NOREF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NOREF_A {
    #[doc = "The reference clock is provided"]
    VALUE_0,
    #[doc = "The reference clock is not provided"]
    VALUE_1,
}
impl crate::ToBits<bool> for NOREF_A {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            NOREF_A::VALUE_0 => false,
            NOREF_A::VALUE_1 => true,
        }
    }
}
#[doc = "Reader of field `NOREF`"]
pub type NOREF_R = crate::R<bool, NOREF_A>;
impl NOREF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NOREF_A {
        match self.bits {
            false => NOREF_A::VALUE_0,
            true => NOREF_A::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == NOREF_A::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == NOREF_A::VALUE_1
    }
}
impl R {
    #[doc = "Bits 0:23 - Reload value to use for 10ms timing"]
    #[inline(always)]
    pub fn tenms(&self) -> TENMS_R {
        TENMS_R::new((self.bits & 0x00ff_ffff) as u32)
    }
    #[doc = "Bit 30 - TENMS is rounded from non-integer ratio"]
    #[inline(always)]
    pub fn skew(&self) -> SKEW_R {
        SKEW_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - No Separate Reference Clock"]
    #[inline(always)]
    pub fn noref(&self) -> NOREF_R {
        NOREF_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
