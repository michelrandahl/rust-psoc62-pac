#[doc = "Register `TOUT_CTRL_R` reader"]
pub type R = crate::R<TOUT_CTRL_R_SPEC>;
#[doc = "Register `TOUT_CTRL_R` writer"]
pub type W = crate::W<TOUT_CTRL_R_SPEC>;
#[doc = "Field `TOUT_CNT` reader - N/A"]
pub type TOUT_CNT_R = crate::FieldReader;
#[doc = "Field `TOUT_CNT` writer - N/A"]
pub type TOUT_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - N/A"]
    #[inline(always)]
    pub fn tout_cnt(&self) -> TOUT_CNT_R {
        TOUT_CNT_R::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn tout_cnt(&mut self) -> TOUT_CNT_W<TOUT_CTRL_R_SPEC> {
        TOUT_CNT_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Timeout Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tout_ctrl_r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tout_ctrl_r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TOUT_CTRL_R_SPEC;
impl crate::RegisterSpec for TOUT_CTRL_R_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tout_ctrl_r::R`](R) reader structure"]
impl crate::Readable for TOUT_CTRL_R_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tout_ctrl_r::W`](W) writer structure"]
impl crate::Writable for TOUT_CTRL_R_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TOUT_CTRL_R to value 0"]
impl crate::Resettable for TOUT_CTRL_R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
