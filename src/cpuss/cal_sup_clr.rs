#[doc = "Register `CAL_SUP_CLR` reader"]
pub type R = crate::R<CAL_SUP_CLR_SPEC>;
#[doc = "Register `CAL_SUP_CLR` writer"]
pub type W = crate::W<CAL_SUP_CLR_SPEC>;
#[doc = "Field `DATA` reader - Read side effect: when read all bits are cleared, write 1 to clear a specific bit Note: no exception for the debug host, it also causes the read side effect"]
pub type DATA_R = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - Read side effect: when read all bits are cleared, write 1 to clear a specific bit Note: no exception for the debug host, it also causes the read side effect"]
pub type DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Read side effect: when read all bits are cleared, write 1 to clear a specific bit Note: no exception for the debug host, it also causes the read side effect"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Read side effect: when read all bits are cleared, write 1 to clear a specific bit Note: no exception for the debug host, it also causes the read side effect"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<CAL_SUP_CLR_SPEC> {
        DATA_W::new(self, 0)
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
#[doc = "Calibration support clear and reset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cal_sup_clr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cal_sup_clr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAL_SUP_CLR_SPEC;
impl crate::RegisterSpec for CAL_SUP_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cal_sup_clr::R`](R) reader structure"]
impl crate::Readable for CAL_SUP_CLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cal_sup_clr::W`](W) writer structure"]
impl crate::Writable for CAL_SUP_CLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAL_SUP_CLR to value 0"]
impl crate::Resettable for CAL_SUP_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
