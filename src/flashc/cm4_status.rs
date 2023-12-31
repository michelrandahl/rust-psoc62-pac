#[doc = "Register `CM4_STATUS` reader"]
pub type R = crate::R<CM4_STATUS_SPEC>;
#[doc = "Register `CM4_STATUS` writer"]
pub type W = crate::W<CM4_STATUS_SPEC>;
#[doc = "Field `MAIN_INTERNAL_ERR` reader - See CM0_STATUS.MAIN_INTERNAL_ERROR."]
pub type MAIN_INTERNAL_ERR_R = crate::BitReader;
#[doc = "Field `MAIN_INTERNAL_ERR` writer - See CM0_STATUS.MAIN_INTERNAL_ERROR."]
pub type MAIN_INTERNAL_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WORK_INTERNAL_ERR` reader - See CM0_STATUS.MAIN_INTERNAL_ERROR."]
pub type WORK_INTERNAL_ERR_R = crate::BitReader;
#[doc = "Field `WORK_INTERNAL_ERR` writer - See CM0_STATUS.MAIN_INTERNAL_ERROR."]
pub type WORK_INTERNAL_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - See CM0_STATUS.MAIN_INTERNAL_ERROR."]
    #[inline(always)]
    pub fn main_internal_err(&self) -> MAIN_INTERNAL_ERR_R {
        MAIN_INTERNAL_ERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - See CM0_STATUS.MAIN_INTERNAL_ERROR."]
    #[inline(always)]
    pub fn work_internal_err(&self) -> WORK_INTERNAL_ERR_R {
        WORK_INTERNAL_ERR_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - See CM0_STATUS.MAIN_INTERNAL_ERROR."]
    #[inline(always)]
    #[must_use]
    pub fn main_internal_err(&mut self) -> MAIN_INTERNAL_ERR_W<CM4_STATUS_SPEC> {
        MAIN_INTERNAL_ERR_W::new(self, 0)
    }
    #[doc = "Bit 1 - See CM0_STATUS.MAIN_INTERNAL_ERROR."]
    #[inline(always)]
    #[must_use]
    pub fn work_internal_err(&mut self) -> WORK_INTERNAL_ERR_W<CM4_STATUS_SPEC> {
        WORK_INTERNAL_ERR_W::new(self, 1)
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
#[doc = "CM4 interface status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm4_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cm4_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CM4_STATUS_SPEC;
impl crate::RegisterSpec for CM4_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cm4_status::R`](R) reader structure"]
impl crate::Readable for CM4_STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cm4_status::W`](W) writer structure"]
impl crate::Writable for CM4_STATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CM4_STATUS to value 0"]
impl crate::Resettable for CM4_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
