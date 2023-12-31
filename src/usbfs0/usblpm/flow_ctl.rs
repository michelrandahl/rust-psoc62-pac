#[doc = "Register `FLOW_CTL` reader"]
pub type R = crate::R<FLOW_CTL_SPEC>;
#[doc = "Register `FLOW_CTL` writer"]
pub type W = crate::W<FLOW_CTL_SPEC>;
#[doc = "Field `EP1_ERR_RESP` reader - End Point 1 error response 0: do nothing (backward compatibility mode) 1: if this is an IN EP and an underflow occurs then cause a CRC error, if this is an OUT EP and an overflow occurs then send a NAK"]
pub type EP1_ERR_RESP_R = crate::BitReader;
#[doc = "Field `EP1_ERR_RESP` writer - End Point 1 error response 0: do nothing (backward compatibility mode) 1: if this is an IN EP and an underflow occurs then cause a CRC error, if this is an OUT EP and an overflow occurs then send a NAK"]
pub type EP1_ERR_RESP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP2_ERR_RESP` reader - End Point 2 error response"]
pub type EP2_ERR_RESP_R = crate::BitReader;
#[doc = "Field `EP2_ERR_RESP` writer - End Point 2 error response"]
pub type EP2_ERR_RESP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP3_ERR_RESP` reader - End Point 3 error response"]
pub type EP3_ERR_RESP_R = crate::BitReader;
#[doc = "Field `EP3_ERR_RESP` writer - End Point 3 error response"]
pub type EP3_ERR_RESP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP4_ERR_RESP` reader - End Point 4 error response"]
pub type EP4_ERR_RESP_R = crate::BitReader;
#[doc = "Field `EP4_ERR_RESP` writer - End Point 4 error response"]
pub type EP4_ERR_RESP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP5_ERR_RESP` reader - End Point 5 error response"]
pub type EP5_ERR_RESP_R = crate::BitReader;
#[doc = "Field `EP5_ERR_RESP` writer - End Point 5 error response"]
pub type EP5_ERR_RESP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP6_ERR_RESP` reader - End Point 6 error response"]
pub type EP6_ERR_RESP_R = crate::BitReader;
#[doc = "Field `EP6_ERR_RESP` writer - End Point 6 error response"]
pub type EP6_ERR_RESP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP7_ERR_RESP` reader - End Point 7 error response"]
pub type EP7_ERR_RESP_R = crate::BitReader;
#[doc = "Field `EP7_ERR_RESP` writer - End Point 7 error response"]
pub type EP7_ERR_RESP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP8_ERR_RESP` reader - End Point 8 error response"]
pub type EP8_ERR_RESP_R = crate::BitReader;
#[doc = "Field `EP8_ERR_RESP` writer - End Point 8 error response"]
pub type EP8_ERR_RESP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - End Point 1 error response 0: do nothing (backward compatibility mode) 1: if this is an IN EP and an underflow occurs then cause a CRC error, if this is an OUT EP and an overflow occurs then send a NAK"]
    #[inline(always)]
    pub fn ep1_err_resp(&self) -> EP1_ERR_RESP_R {
        EP1_ERR_RESP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - End Point 2 error response"]
    #[inline(always)]
    pub fn ep2_err_resp(&self) -> EP2_ERR_RESP_R {
        EP2_ERR_RESP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - End Point 3 error response"]
    #[inline(always)]
    pub fn ep3_err_resp(&self) -> EP3_ERR_RESP_R {
        EP3_ERR_RESP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - End Point 4 error response"]
    #[inline(always)]
    pub fn ep4_err_resp(&self) -> EP4_ERR_RESP_R {
        EP4_ERR_RESP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - End Point 5 error response"]
    #[inline(always)]
    pub fn ep5_err_resp(&self) -> EP5_ERR_RESP_R {
        EP5_ERR_RESP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - End Point 6 error response"]
    #[inline(always)]
    pub fn ep6_err_resp(&self) -> EP6_ERR_RESP_R {
        EP6_ERR_RESP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - End Point 7 error response"]
    #[inline(always)]
    pub fn ep7_err_resp(&self) -> EP7_ERR_RESP_R {
        EP7_ERR_RESP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - End Point 8 error response"]
    #[inline(always)]
    pub fn ep8_err_resp(&self) -> EP8_ERR_RESP_R {
        EP8_ERR_RESP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - End Point 1 error response 0: do nothing (backward compatibility mode) 1: if this is an IN EP and an underflow occurs then cause a CRC error, if this is an OUT EP and an overflow occurs then send a NAK"]
    #[inline(always)]
    #[must_use]
    pub fn ep1_err_resp(&mut self) -> EP1_ERR_RESP_W<FLOW_CTL_SPEC> {
        EP1_ERR_RESP_W::new(self, 0)
    }
    #[doc = "Bit 1 - End Point 2 error response"]
    #[inline(always)]
    #[must_use]
    pub fn ep2_err_resp(&mut self) -> EP2_ERR_RESP_W<FLOW_CTL_SPEC> {
        EP2_ERR_RESP_W::new(self, 1)
    }
    #[doc = "Bit 2 - End Point 3 error response"]
    #[inline(always)]
    #[must_use]
    pub fn ep3_err_resp(&mut self) -> EP3_ERR_RESP_W<FLOW_CTL_SPEC> {
        EP3_ERR_RESP_W::new(self, 2)
    }
    #[doc = "Bit 3 - End Point 4 error response"]
    #[inline(always)]
    #[must_use]
    pub fn ep4_err_resp(&mut self) -> EP4_ERR_RESP_W<FLOW_CTL_SPEC> {
        EP4_ERR_RESP_W::new(self, 3)
    }
    #[doc = "Bit 4 - End Point 5 error response"]
    #[inline(always)]
    #[must_use]
    pub fn ep5_err_resp(&mut self) -> EP5_ERR_RESP_W<FLOW_CTL_SPEC> {
        EP5_ERR_RESP_W::new(self, 4)
    }
    #[doc = "Bit 5 - End Point 6 error response"]
    #[inline(always)]
    #[must_use]
    pub fn ep6_err_resp(&mut self) -> EP6_ERR_RESP_W<FLOW_CTL_SPEC> {
        EP6_ERR_RESP_W::new(self, 5)
    }
    #[doc = "Bit 6 - End Point 7 error response"]
    #[inline(always)]
    #[must_use]
    pub fn ep7_err_resp(&mut self) -> EP7_ERR_RESP_W<FLOW_CTL_SPEC> {
        EP7_ERR_RESP_W::new(self, 6)
    }
    #[doc = "Bit 7 - End Point 8 error response"]
    #[inline(always)]
    #[must_use]
    pub fn ep8_err_resp(&mut self) -> EP8_ERR_RESP_W<FLOW_CTL_SPEC> {
        EP8_ERR_RESP_W::new(self, 7)
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
#[doc = "Flow Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flow_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flow_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLOW_CTL_SPEC;
impl crate::RegisterSpec for FLOW_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flow_ctl::R`](R) reader structure"]
impl crate::Readable for FLOW_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`flow_ctl::W`](W) writer structure"]
impl crate::Writable for FLOW_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLOW_CTL to value 0"]
impl crate::Resettable for FLOW_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
