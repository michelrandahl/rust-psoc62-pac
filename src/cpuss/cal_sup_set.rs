#[doc = "Register `CAL_SUP_SET` reader"]
pub type R = crate::R<CAL_SUP_SET_SPEC>;
#[doc = "Register `CAL_SUP_SET` writer"]
pub type W = crate::W<CAL_SUP_SET_SPEC>;
#[doc = "Field `DATA` reader - Read without side effect, write 1 to set"]
pub type DATA_R = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - Read without side effect, write 1 to set"]
pub type DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Read without side effect, write 1 to set"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Read without side effect, write 1 to set"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<CAL_SUP_SET_SPEC> {
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
#[doc = "Calibration support set and read\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cal_sup_set::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cal_sup_set::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAL_SUP_SET_SPEC;
impl crate::RegisterSpec for CAL_SUP_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cal_sup_set::R`](R) reader structure"]
impl crate::Readable for CAL_SUP_SET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cal_sup_set::W`](W) writer structure"]
impl crate::Writable for CAL_SUP_SET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAL_SUP_SET to value 0"]
impl crate::Resettable for CAL_SUP_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
