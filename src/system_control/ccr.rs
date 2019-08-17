#[doc = "Reader of register CCR"]
pub type R = crate::R<u32, super::CCR>;
#[doc = "Possible values of the field `UNALIGN_TRP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNALIGN_TRP_A {
    #[doc = "Do not trap unaligned halfword and word accesses"]
    VALUE_0,
    #[doc = "Trap unaligned halfword and word accesses"]
    VALUE_1,
}
impl crate::ToBits<bool> for UNALIGN_TRP_A {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            UNALIGN_TRP_A::VALUE_0 => false,
            UNALIGN_TRP_A::VALUE_1 => true,
        }
    }
}
#[doc = "Reader of field `UNALIGN_TRP`"]
pub type UNALIGN_TRP_R = crate::R<bool, UNALIGN_TRP_A>;
impl UNALIGN_TRP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UNALIGN_TRP_A {
        match self.bits {
            false => UNALIGN_TRP_A::VALUE_0,
            true => UNALIGN_TRP_A::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == UNALIGN_TRP_A::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == UNALIGN_TRP_A::VALUE_1
    }
}
#[doc = "Possible values of the field `STKALIGN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STKALIGN_A {
    #[doc = "4-byte aligned"]
    VALUE_0,
    #[doc = "8-byte aligned"]
    VALUE_1,
}
impl crate::ToBits<bool> for STKALIGN_A {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            STKALIGN_A::VALUE_0 => false,
            STKALIGN_A::VALUE_1 => true,
        }
    }
}
#[doc = "Reader of field `STKALIGN`"]
pub type STKALIGN_R = crate::R<bool, STKALIGN_A>;
impl STKALIGN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STKALIGN_A {
        match self.bits {
            false => STKALIGN_A::VALUE_0,
            true => STKALIGN_A::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == STKALIGN_A::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == STKALIGN_A::VALUE_1
    }
}
impl R {
    #[doc = "Bit 3 - Unaligned accesses generates a Hard Fault"]
    #[inline(always)]
    pub fn unalign_trp(&self) -> UNALIGN_TRP_R {
        UNALIGN_TRP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Stack 8-byte aligned on exception entry"]
    #[inline(always)]
    pub fn stkalign(&self) -> STKALIGN_R {
        STKALIGN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
