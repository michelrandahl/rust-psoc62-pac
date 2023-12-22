#[doc = "Register `SIE_EP1_CR0` reader"]
pub type R = crate::R<SIE_EP1_CR0_SPEC>;
#[doc = "Register `SIE_EP1_CR0` writer"]
pub type W = crate::W<SIE_EP1_CR0_SPEC>;
#[doc = "Field `MODE` reader - The mode controls how the USB SIE responds to traffic and how the USB SIE changes the mode of that endpoint as a result of host packets to the endpoint."]
pub type MODE_R = crate::FieldReader<MODE_A>;
#[doc = "The mode controls how the USB SIE responds to traffic and how the USB SIE changes the mode of that endpoint as a result of host packets to the endpoint.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Ignore all USB traffic to this endpoint"]
    DISABLE = 0,
    #[doc = "1: SETUP: Accept IN: NAK OUT: NAK"]
    NAK_INOUT = 1,
    #[doc = "2: SETUP: Accept IN: STALL OUT: ACK 0B tokens, NAK others"]
    STATUS_OUT_ONLY = 2,
    #[doc = "3: SETUP: Accept IN: STALL OUT: STALL"]
    STALL_INOUT = 3,
    #[doc = "5: SETUP: Ignore IN: Ignore OUT: Accept Isochronous OUT token"]
    ISO_OUT = 5,
    #[doc = "6: SETUP: Accept IN: Respond with 0B data OUT: Stall"]
    STATUS_IN_ONLY = 6,
    #[doc = "7: SETUP: Ignore IN: Accept Isochronous IN token OUT: Ignore"]
    ISO_IN = 7,
    #[doc = "8: SETUP: Ignore IN: Ignore OUT: NAK"]
    NAK_OUT = 8,
    #[doc = "9: SETUP: Ignore IN: Ignore OUT: Accept data and ACK if STALL=0, STALL otherwise. Change to MODE=8 after one succesfull OUT token."]
    ACK_OUT = 9,
    #[doc = "11: SETUP: Accept IN: Respond with 0B data OUT: Accept data"]
    ACK_OUT_STATUS_IN = 11,
    #[doc = "12: SETUP: Ignore IN: NAK OUT: Ignore"]
    NAK_IN = 12,
    #[doc = "13: SETUP: Ignore IN: Respond to IN with data if STALL=0, STALL otherwise OUT: Ignore"]
    ACK_IN = 13,
    #[doc = "15: SETUP: Accept IN: Respond to IN with data OUT: ACK 0B tokens, NAK others"]
    ACK_IN_STATUS_OUT = 15,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE_A {
    type Ux = u8;
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MODE_A> {
        match self.bits {
            0 => Some(MODE_A::DISABLE),
            1 => Some(MODE_A::NAK_INOUT),
            2 => Some(MODE_A::STATUS_OUT_ONLY),
            3 => Some(MODE_A::STALL_INOUT),
            5 => Some(MODE_A::ISO_OUT),
            6 => Some(MODE_A::STATUS_IN_ONLY),
            7 => Some(MODE_A::ISO_IN),
            8 => Some(MODE_A::NAK_OUT),
            9 => Some(MODE_A::ACK_OUT),
            11 => Some(MODE_A::ACK_OUT_STATUS_IN),
            12 => Some(MODE_A::NAK_IN),
            13 => Some(MODE_A::ACK_IN),
            15 => Some(MODE_A::ACK_IN_STATUS_OUT),
            _ => None,
        }
    }
    #[doc = "Ignore all USB traffic to this endpoint"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MODE_A::DISABLE
    }
    #[doc = "SETUP: Accept IN: NAK OUT: NAK"]
    #[inline(always)]
    pub fn is_nak_inout(&self) -> bool {
        *self == MODE_A::NAK_INOUT
    }
    #[doc = "SETUP: Accept IN: STALL OUT: ACK 0B tokens, NAK others"]
    #[inline(always)]
    pub fn is_status_out_only(&self) -> bool {
        *self == MODE_A::STATUS_OUT_ONLY
    }
    #[doc = "SETUP: Accept IN: STALL OUT: STALL"]
    #[inline(always)]
    pub fn is_stall_inout(&self) -> bool {
        *self == MODE_A::STALL_INOUT
    }
    #[doc = "SETUP: Ignore IN: Ignore OUT: Accept Isochronous OUT token"]
    #[inline(always)]
    pub fn is_iso_out(&self) -> bool {
        *self == MODE_A::ISO_OUT
    }
    #[doc = "SETUP: Accept IN: Respond with 0B data OUT: Stall"]
    #[inline(always)]
    pub fn is_status_in_only(&self) -> bool {
        *self == MODE_A::STATUS_IN_ONLY
    }
    #[doc = "SETUP: Ignore IN: Accept Isochronous IN token OUT: Ignore"]
    #[inline(always)]
    pub fn is_iso_in(&self) -> bool {
        *self == MODE_A::ISO_IN
    }
    #[doc = "SETUP: Ignore IN: Ignore OUT: NAK"]
    #[inline(always)]
    pub fn is_nak_out(&self) -> bool {
        *self == MODE_A::NAK_OUT
    }
    #[doc = "SETUP: Ignore IN: Ignore OUT: Accept data and ACK if STALL=0, STALL otherwise. Change to MODE=8 after one succesfull OUT token."]
    #[inline(always)]
    pub fn is_ack_out(&self) -> bool {
        *self == MODE_A::ACK_OUT
    }
    #[doc = "SETUP: Accept IN: Respond with 0B data OUT: Accept data"]
    #[inline(always)]
    pub fn is_ack_out_status_in(&self) -> bool {
        *self == MODE_A::ACK_OUT_STATUS_IN
    }
    #[doc = "SETUP: Ignore IN: NAK OUT: Ignore"]
    #[inline(always)]
    pub fn is_nak_in(&self) -> bool {
        *self == MODE_A::NAK_IN
    }
    #[doc = "SETUP: Ignore IN: Respond to IN with data if STALL=0, STALL otherwise OUT: Ignore"]
    #[inline(always)]
    pub fn is_ack_in(&self) -> bool {
        *self == MODE_A::ACK_IN
    }
    #[doc = "SETUP: Accept IN: Respond to IN with data OUT: ACK 0B tokens, NAK others"]
    #[inline(always)]
    pub fn is_ack_in_status_out(&self) -> bool {
        *self == MODE_A::ACK_IN_STATUS_OUT
    }
}
#[doc = "Field `MODE` writer - The mode controls how the USB SIE responds to traffic and how the USB SIE changes the mode of that endpoint as a result of host packets to the endpoint."]
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 4, MODE_A>;
impl<'a, REG> MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Ignore all USB traffic to this endpoint"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::DISABLE)
    }
    #[doc = "SETUP: Accept IN: NAK OUT: NAK"]
    #[inline(always)]
    pub fn nak_inout(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::NAK_INOUT)
    }
    #[doc = "SETUP: Accept IN: STALL OUT: ACK 0B tokens, NAK others"]
    #[inline(always)]
    pub fn status_out_only(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::STATUS_OUT_ONLY)
    }
    #[doc = "SETUP: Accept IN: STALL OUT: STALL"]
    #[inline(always)]
    pub fn stall_inout(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::STALL_INOUT)
    }
    #[doc = "SETUP: Ignore IN: Ignore OUT: Accept Isochronous OUT token"]
    #[inline(always)]
    pub fn iso_out(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::ISO_OUT)
    }
    #[doc = "SETUP: Accept IN: Respond with 0B data OUT: Stall"]
    #[inline(always)]
    pub fn status_in_only(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::STATUS_IN_ONLY)
    }
    #[doc = "SETUP: Ignore IN: Accept Isochronous IN token OUT: Ignore"]
    #[inline(always)]
    pub fn iso_in(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::ISO_IN)
    }
    #[doc = "SETUP: Ignore IN: Ignore OUT: NAK"]
    #[inline(always)]
    pub fn nak_out(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::NAK_OUT)
    }
    #[doc = "SETUP: Ignore IN: Ignore OUT: Accept data and ACK if STALL=0, STALL otherwise. Change to MODE=8 after one succesfull OUT token."]
    #[inline(always)]
    pub fn ack_out(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::ACK_OUT)
    }
    #[doc = "SETUP: Accept IN: Respond with 0B data OUT: Accept data"]
    #[inline(always)]
    pub fn ack_out_status_in(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::ACK_OUT_STATUS_IN)
    }
    #[doc = "SETUP: Ignore IN: NAK OUT: Ignore"]
    #[inline(always)]
    pub fn nak_in(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::NAK_IN)
    }
    #[doc = "SETUP: Ignore IN: Respond to IN with data if STALL=0, STALL otherwise OUT: Ignore"]
    #[inline(always)]
    pub fn ack_in(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::ACK_IN)
    }
    #[doc = "SETUP: Accept IN: Respond to IN with data OUT: ACK 0B tokens, NAK others"]
    #[inline(always)]
    pub fn ack_in_status_out(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::ACK_IN_STATUS_OUT)
    }
}
#[doc = "Field `ACKED_TXN` reader - The ACK'd transaction bit is set whenever the SIE engages in a transaction to the register's endpoint that completes with an ACK packet. This bit is cleared by any writes to the register."]
pub type ACKED_TXN_R = crate::BitReader<ACKED_TXN_A>;
#[doc = "The ACK'd transaction bit is set whenever the SIE engages in a transaction to the register's endpoint that completes with an ACK packet. This bit is cleared by any writes to the register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACKED_TXN_A {
    #[doc = "0: No ACK'd transactions since bit was last cleared."]
    ACKED_NO = 0,
    #[doc = "1: Indicates a transaction ended with an ACK."]
    ACKED_YES = 1,
}
impl From<ACKED_TXN_A> for bool {
    #[inline(always)]
    fn from(variant: ACKED_TXN_A) -> Self {
        variant as u8 != 0
    }
}
impl ACKED_TXN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ACKED_TXN_A {
        match self.bits {
            false => ACKED_TXN_A::ACKED_NO,
            true => ACKED_TXN_A::ACKED_YES,
        }
    }
    #[doc = "No ACK'd transactions since bit was last cleared."]
    #[inline(always)]
    pub fn is_acked_no(&self) -> bool {
        *self == ACKED_TXN_A::ACKED_NO
    }
    #[doc = "Indicates a transaction ended with an ACK."]
    #[inline(always)]
    pub fn is_acked_yes(&self) -> bool {
        *self == ACKED_TXN_A::ACKED_YES
    }
}
#[doc = "Field `ACKED_TXN` writer - The ACK'd transaction bit is set whenever the SIE engages in a transaction to the register's endpoint that completes with an ACK packet. This bit is cleared by any writes to the register."]
pub type ACKED_TXN_W<'a, REG> = crate::BitWriter<'a, REG, ACKED_TXN_A>;
impl<'a, REG> ACKED_TXN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No ACK'd transactions since bit was last cleared."]
    #[inline(always)]
    pub fn acked_no(self) -> &'a mut crate::W<REG> {
        self.variant(ACKED_TXN_A::ACKED_NO)
    }
    #[doc = "Indicates a transaction ended with an ACK."]
    #[inline(always)]
    pub fn acked_yes(self) -> &'a mut crate::W<REG> {
        self.variant(ACKED_TXN_A::ACKED_YES)
    }
}
#[doc = "Field `NAK_INT_EN` reader - When set this bit causes an endpoint interrupt to be generated even when a transfer completes with a NAK."]
pub type NAK_INT_EN_R = crate::BitReader;
#[doc = "Field `NAK_INT_EN` writer - When set this bit causes an endpoint interrupt to be generated even when a transfer completes with a NAK."]
pub type NAK_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR_IN_TXN` reader - The Error in transaction bit is set whenever an error is detected. For an IN transaction, this indicates a no response from HOST scenario. For an OUT transaction, this represents an RxErr (PID error/ CRC error/ bit-stuff error scenario). This bit is cleared by any writes to the register."]
pub type ERR_IN_TXN_R = crate::BitReader;
#[doc = "Field `ERR_IN_TXN` writer - The Error in transaction bit is set whenever an error is detected. For an IN transaction, this indicates a no response from HOST scenario. For an OUT transaction, this represents an RxErr (PID error/ CRC error/ bit-stuff error scenario). This bit is cleared by any writes to the register."]
pub type ERR_IN_TXN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALL` reader - When this bit is set the SIE stalls an OUT packet if the Mode bits are set to ACK-OUT. The SIE stalls an IN packet if the mode bits are set to ACK-IN. This bit must be clear for all other modes."]
pub type STALL_R = crate::BitReader;
#[doc = "Field `STALL` writer - When this bit is set the SIE stalls an OUT packet if the Mode bits are set to ACK-OUT. The SIE stalls an IN packet if the mode bits are set to ACK-IN. This bit must be clear for all other modes."]
pub type STALL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - The mode controls how the USB SIE responds to traffic and how the USB SIE changes the mode of that endpoint as a result of host packets to the endpoint."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - The ACK'd transaction bit is set whenever the SIE engages in a transaction to the register's endpoint that completes with an ACK packet. This bit is cleared by any writes to the register."]
    #[inline(always)]
    pub fn acked_txn(&self) -> ACKED_TXN_R {
        ACKED_TXN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When set this bit causes an endpoint interrupt to be generated even when a transfer completes with a NAK."]
    #[inline(always)]
    pub fn nak_int_en(&self) -> NAK_INT_EN_R {
        NAK_INT_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The Error in transaction bit is set whenever an error is detected. For an IN transaction, this indicates a no response from HOST scenario. For an OUT transaction, this represents an RxErr (PID error/ CRC error/ bit-stuff error scenario). This bit is cleared by any writes to the register."]
    #[inline(always)]
    pub fn err_in_txn(&self) -> ERR_IN_TXN_R {
        ERR_IN_TXN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - When this bit is set the SIE stalls an OUT packet if the Mode bits are set to ACK-OUT. The SIE stalls an IN packet if the mode bits are set to ACK-IN. This bit must be clear for all other modes."]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - The mode controls how the USB SIE responds to traffic and how the USB SIE changes the mode of that endpoint as a result of host packets to the endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<SIE_EP1_CR0_SPEC> {
        MODE_W::new(self, 0)
    }
    #[doc = "Bit 4 - The ACK'd transaction bit is set whenever the SIE engages in a transaction to the register's endpoint that completes with an ACK packet. This bit is cleared by any writes to the register."]
    #[inline(always)]
    #[must_use]
    pub fn acked_txn(&mut self) -> ACKED_TXN_W<SIE_EP1_CR0_SPEC> {
        ACKED_TXN_W::new(self, 4)
    }
    #[doc = "Bit 5 - When set this bit causes an endpoint interrupt to be generated even when a transfer completes with a NAK."]
    #[inline(always)]
    #[must_use]
    pub fn nak_int_en(&mut self) -> NAK_INT_EN_W<SIE_EP1_CR0_SPEC> {
        NAK_INT_EN_W::new(self, 5)
    }
    #[doc = "Bit 6 - The Error in transaction bit is set whenever an error is detected. For an IN transaction, this indicates a no response from HOST scenario. For an OUT transaction, this represents an RxErr (PID error/ CRC error/ bit-stuff error scenario). This bit is cleared by any writes to the register."]
    #[inline(always)]
    #[must_use]
    pub fn err_in_txn(&mut self) -> ERR_IN_TXN_W<SIE_EP1_CR0_SPEC> {
        ERR_IN_TXN_W::new(self, 6)
    }
    #[doc = "Bit 7 - When this bit is set the SIE stalls an OUT packet if the Mode bits are set to ACK-OUT. The SIE stalls an IN packet if the mode bits are set to ACK-IN. This bit must be clear for all other modes."]
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> STALL_W<SIE_EP1_CR0_SPEC> {
        STALL_W::new(self, 7)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Non-control endpoint's control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sie_ep1_cr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sie_ep1_cr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SIE_EP1_CR0_SPEC;
impl crate::RegisterSpec for SIE_EP1_CR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sie_ep1_cr0::R`](R) reader structure"]
impl crate::Readable for SIE_EP1_CR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sie_ep1_cr0::W`](W) writer structure"]
impl crate::Writable for SIE_EP1_CR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SIE_EP1_CR0 to value 0"]
impl crate::Resettable for SIE_EP1_CR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
