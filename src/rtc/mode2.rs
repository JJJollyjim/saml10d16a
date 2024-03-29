#[doc = "MODE2 Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrla](ctrla) module"]
pub type CTRLA = crate::Reg<u16, _CTRLA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRLA;
#[doc = "`read()` method returns [ctrla::R](ctrla::R) reader structure"]
impl crate::Readable for CTRLA {}
#[doc = "`write(|w| ..)` method takes [ctrla::W](ctrla::W) writer structure"]
impl crate::Writable for CTRLA {}
#[doc = "MODE2 Control A"]
pub mod ctrla;
#[doc = "MODE2 Control B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrlb](ctrlb) module"]
pub type CTRLB = crate::Reg<u16, _CTRLB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRLB;
#[doc = "`read()` method returns [ctrlb::R](ctrlb::R) reader structure"]
impl crate::Readable for CTRLB {}
#[doc = "`write(|w| ..)` method takes [ctrlb::W](ctrlb::W) writer structure"]
impl crate::Writable for CTRLB {}
#[doc = "MODE2 Control B"]
pub mod ctrlb;
#[doc = "MODE2 Event Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [evctrl](evctrl) module"]
pub type EVCTRL = crate::Reg<u32, _EVCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVCTRL;
#[doc = "`read()` method returns [evctrl::R](evctrl::R) reader structure"]
impl crate::Readable for EVCTRL {}
#[doc = "`write(|w| ..)` method takes [evctrl::W](evctrl::W) writer structure"]
impl crate::Writable for EVCTRL {}
#[doc = "MODE2 Event Control"]
pub mod evctrl;
#[doc = "MODE2 Interrupt Enable Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intenclr](intenclr) module"]
pub type INTENCLR = crate::Reg<u16, _INTENCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENCLR;
#[doc = "`read()` method returns [intenclr::R](intenclr::R) reader structure"]
impl crate::Readable for INTENCLR {}
#[doc = "`write(|w| ..)` method takes [intenclr::W](intenclr::W) writer structure"]
impl crate::Writable for INTENCLR {}
#[doc = "MODE2 Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "MODE2 Interrupt Enable Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intenset](intenset) module"]
pub type INTENSET = crate::Reg<u16, _INTENSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENSET;
#[doc = "`read()` method returns [intenset::R](intenset::R) reader structure"]
impl crate::Readable for INTENSET {}
#[doc = "`write(|w| ..)` method takes [intenset::W](intenset::W) writer structure"]
impl crate::Writable for INTENSET {}
#[doc = "MODE2 Interrupt Enable Set"]
pub mod intenset;
#[doc = "MODE2 Interrupt Flag Status and Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intflag](intflag) module"]
pub type INTFLAG = crate::Reg<u16, _INTFLAG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTFLAG;
#[doc = "`read()` method returns [intflag::R](intflag::R) reader structure"]
impl crate::Readable for INTFLAG {}
#[doc = "`write(|w| ..)` method takes [intflag::W](intflag::W) writer structure"]
impl crate::Writable for INTFLAG {}
#[doc = "MODE2 Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "Debug Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dbgctrl](dbgctrl) module"]
pub type DBGCTRL = crate::Reg<u8, _DBGCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DBGCTRL;
#[doc = "`read()` method returns [dbgctrl::R](dbgctrl::R) reader structure"]
impl crate::Readable for DBGCTRL {}
#[doc = "`write(|w| ..)` method takes [dbgctrl::W](dbgctrl::W) writer structure"]
impl crate::Writable for DBGCTRL {}
#[doc = "Debug Control"]
pub mod dbgctrl;
#[doc = "MODE2 Synchronization Busy Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [syncbusy](syncbusy) module"]
pub type SYNCBUSY = crate::Reg<u32, _SYNCBUSY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYNCBUSY;
#[doc = "`read()` method returns [syncbusy::R](syncbusy::R) reader structure"]
impl crate::Readable for SYNCBUSY {}
#[doc = "MODE2 Synchronization Busy Status"]
pub mod syncbusy;
#[doc = "Frequency Correction\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [freqcorr](freqcorr) module"]
pub type FREQCORR = crate::Reg<u8, _FREQCORR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FREQCORR;
#[doc = "`read()` method returns [freqcorr::R](freqcorr::R) reader structure"]
impl crate::Readable for FREQCORR {}
#[doc = "`write(|w| ..)` method takes [freqcorr::W](freqcorr::W) writer structure"]
impl crate::Writable for FREQCORR {}
#[doc = "Frequency Correction"]
pub mod freqcorr;
#[doc = "MODE2 Clock Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clock](clock) module"]
pub type CLOCK = crate::Reg<u32, _CLOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLOCK;
#[doc = "`read()` method returns [clock::R](clock::R) reader structure"]
impl crate::Readable for CLOCK {}
#[doc = "`write(|w| ..)` method takes [clock::W](clock::W) writer structure"]
impl crate::Writable for CLOCK {}
#[doc = "MODE2 Clock Value"]
pub mod clock;
#[doc = "General Purpose\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gp](gp) module"]
pub type GP = crate::Reg<u32, _GP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GP;
#[doc = "`read()` method returns [gp::R](gp::R) reader structure"]
impl crate::Readable for GP {}
#[doc = "`write(|w| ..)` method takes [gp::W](gp::W) writer structure"]
impl crate::Writable for GP {}
#[doc = "General Purpose"]
pub mod gp;
#[doc = "Tamper Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tampctrl](tampctrl) module"]
pub type TAMPCTRL = crate::Reg<u32, _TAMPCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMPCTRL;
#[doc = "`read()` method returns [tampctrl::R](tampctrl::R) reader structure"]
impl crate::Readable for TAMPCTRL {}
#[doc = "`write(|w| ..)` method takes [tampctrl::W](tampctrl::W) writer structure"]
impl crate::Writable for TAMPCTRL {}
#[doc = "Tamper Control"]
pub mod tampctrl;
#[doc = "MODE2 Timestamp\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timestamp](timestamp) module"]
pub type TIMESTAMP = crate::Reg<u32, _TIMESTAMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMESTAMP;
#[doc = "`read()` method returns [timestamp::R](timestamp::R) reader structure"]
impl crate::Readable for TIMESTAMP {}
#[doc = "MODE2 Timestamp"]
pub mod timestamp;
#[doc = "Tamper ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tampid](tampid) module"]
pub type TAMPID = crate::Reg<u32, _TAMPID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMPID;
#[doc = "`read()` method returns [tampid::R](tampid::R) reader structure"]
impl crate::Readable for TAMPID {}
#[doc = "`write(|w| ..)` method takes [tampid::W](tampid::W) writer structure"]
impl crate::Writable for TAMPID {}
#[doc = "Tamper ID"]
pub mod tampid;
#[doc = "Tamper Control B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tampctrlb](tampctrlb) module"]
pub type TAMPCTRLB = crate::Reg<u32, _TAMPCTRLB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMPCTRLB;
#[doc = "`read()` method returns [tampctrlb::R](tampctrlb::R) reader structure"]
impl crate::Readable for TAMPCTRLB {}
#[doc = "`write(|w| ..)` method takes [tampctrlb::W](tampctrlb::W) writer structure"]
impl crate::Writable for TAMPCTRLB {}
#[doc = "Tamper Control B"]
pub mod tampctrlb;
#[doc = r"Register block"]
#[repr(C)]
pub struct MODE2_ALARM {
    #[doc = "0x00 - MODE2_ALARM Alarm n Value"]
    pub alarm: self::mode2_alarm::ALARM,
    #[doc = "0x04 - MODE2_ALARM Alarm n Mask"]
    pub mask: self::mode2_alarm::MASK,
}
#[doc = r"Register block"]
#[doc = "N/A"]
pub mod mode2_alarm;
