#[doc = "Register `SIE_EP5_CNT0` reader"]
pub type R = crate::R<SIE_EP5_CNT0_SPEC>;
#[doc = "Register `SIE_EP5_CNT0` writer"]
pub type W = crate::W<SIE_EP5_CNT0_SPEC>;
#[doc = "Field `DATA_COUNT_MSB` reader - These bits are the 3 MSb bits of an 11-bit counter. The LSb are the Data Count\\[7:0\\]
bits of the CNT1 register. Refer to the CNT1 register for more information."]
pub type DATA_COUNT_MSB_R = crate::FieldReader;
#[doc = "Field `DATA_COUNT_MSB` writer - These bits are the 3 MSb bits of an 11-bit counter. The LSb are the Data Count\\[7:0\\]
bits of the CNT1 register. Refer to the CNT1 register for more information."]
pub type DATA_COUNT_MSB_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DATA_VALID` reader - This bit is used for OUT transactions only and is read only. It is cleared to '0' if CRC bit stuffing errors or PID errors occur. This bit does not update for some endpoint mode settings."]
pub type DATA_VALID_R = crate::BitReader<DATA_VALID_A>;
#[doc = "This bit is used for OUT transactions only and is read only. It is cleared to '0' if CRC bit stuffing errors or PID errors occur. This bit does not update for some endpoint mode settings.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DATA_VALID_A {
    #[doc = "0: No ACK'd transactions since bit was last cleared."]
    DATA_ERROR = 0,
    #[doc = "1: Indicates a transaction ended with an ACK."]
    DATA_VALID = 1,
}
impl From<DATA_VALID_A> for bool {
    #[inline(always)]
    fn from(variant: DATA_VALID_A) -> Self {
        variant as u8 != 0
    }
}
impl DATA_VALID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DATA_VALID_A {
        match self.bits {
            false => DATA_VALID_A::DATA_ERROR,
            true => DATA_VALID_A::DATA_VALID,
        }
    }
    #[doc = "No ACK'd transactions since bit was last cleared."]
    #[inline(always)]
    pub fn is_data_error(&self) -> bool {
        *self == DATA_VALID_A::DATA_ERROR
    }
    #[doc = "Indicates a transaction ended with an ACK."]
    #[inline(always)]
    pub fn is_data_valid(&self) -> bool {
        *self == DATA_VALID_A::DATA_VALID
    }
}
#[doc = "Field `DATA_VALID` writer - This bit is used for OUT transactions only and is read only. It is cleared to '0' if CRC bit stuffing errors or PID errors occur. This bit does not update for some endpoint mode settings."]
pub type DATA_VALID_W<'a, REG> = crate::BitWriter<'a, REG, DATA_VALID_A>;
impl<'a, REG> DATA_VALID_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No ACK'd transactions since bit was last cleared."]
    #[inline(always)]
    pub fn data_error(self) -> &'a mut crate::W<REG> {
        self.variant(DATA_VALID_A::DATA_ERROR)
    }
    #[doc = "Indicates a transaction ended with an ACK."]
    #[inline(always)]
    pub fn data_valid(self) -> &'a mut crate::W<REG> {
        self.variant(DATA_VALID_A::DATA_VALID)
    }
}
#[doc = "Field `DATA_TOGGLE` reader - This bit selects the DATA packet's toggle state. For IN transactions firmware must set this bit to the expected state. For OUT transactions the hardware sets this bit to the state of the received Data Toggle bit."]
pub type DATA_TOGGLE_R = crate::BitReader;
#[doc = "Field `DATA_TOGGLE` writer - This bit selects the DATA packet's toggle state. For IN transactions firmware must set this bit to the expected state. For OUT transactions the hardware sets this bit to the state of the received Data Toggle bit."]
pub type DATA_TOGGLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - These bits are the 3 MSb bits of an 11-bit counter. The LSb are the Data Count\\[7:0\\]
bits of the CNT1 register. Refer to the CNT1 register for more information."]
    #[inline(always)]
    pub fn data_count_msb(&self) -> DATA_COUNT_MSB_R {
        DATA_COUNT_MSB_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 6 - This bit is used for OUT transactions only and is read only. It is cleared to '0' if CRC bit stuffing errors or PID errors occur. This bit does not update for some endpoint mode settings."]
    #[inline(always)]
    pub fn data_valid(&self) -> DATA_VALID_R {
        DATA_VALID_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This bit selects the DATA packet's toggle state. For IN transactions firmware must set this bit to the expected state. For OUT transactions the hardware sets this bit to the state of the received Data Toggle bit."]
    #[inline(always)]
    pub fn data_toggle(&self) -> DATA_TOGGLE_R {
        DATA_TOGGLE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - These bits are the 3 MSb bits of an 11-bit counter. The LSb are the Data Count\\[7:0\\]
bits of the CNT1 register. Refer to the CNT1 register for more information."]
    #[inline(always)]
    #[must_use]
    pub fn data_count_msb(&mut self) -> DATA_COUNT_MSB_W<SIE_EP5_CNT0_SPEC> {
        DATA_COUNT_MSB_W::new(self, 0)
    }
    #[doc = "Bit 6 - This bit is used for OUT transactions only and is read only. It is cleared to '0' if CRC bit stuffing errors or PID errors occur. This bit does not update for some endpoint mode settings."]
    #[inline(always)]
    #[must_use]
    pub fn data_valid(&mut self) -> DATA_VALID_W<SIE_EP5_CNT0_SPEC> {
        DATA_VALID_W::new(self, 6)
    }
    #[doc = "Bit 7 - This bit selects the DATA packet's toggle state. For IN transactions firmware must set this bit to the expected state. For OUT transactions the hardware sets this bit to the state of the received Data Toggle bit."]
    #[inline(always)]
    #[must_use]
    pub fn data_toggle(&mut self) -> DATA_TOGGLE_W<SIE_EP5_CNT0_SPEC> {
        DATA_TOGGLE_W::new(self, 7)
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
#[doc = "Non-control endpoint count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sie_ep5_cnt0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sie_ep5_cnt0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SIE_EP5_CNT0_SPEC;
impl crate::RegisterSpec for SIE_EP5_CNT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sie_ep5_cnt0::R`](R) reader structure"]
impl crate::Readable for SIE_EP5_CNT0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sie_ep5_cnt0::W`](W) writer structure"]
impl crate::Writable for SIE_EP5_CNT0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SIE_EP5_CNT0 to value 0"]
impl crate::Resettable for SIE_EP5_CNT0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
