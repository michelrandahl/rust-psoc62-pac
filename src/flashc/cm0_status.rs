#[doc = "Register `CM0_STATUS` reader"]
pub type R = crate::R<CM0_STATUS_SPEC>;
#[doc = "Register `CM0_STATUS` writer"]
pub type W = crate::W<CM0_STATUS_SPEC>;
#[doc = "Field `MAIN_INTERNAL_ERR` reader - Specifies/registers the occurrence of a FLASH macro main interface internal error (typically the result of a read access while a program erase operation is ongoing) as a result of a CM0+ access. SW clears this field to '0'. HW sets this field to '1' on a FLASH macro main interface internal error. Typically, SW reads this field after a code section to detect the occurrence of an error. Note: this field is independent of FLASH_CTL.MAIN_ERR_SILENT."]
pub type MAIN_INTERNAL_ERR_R = crate::BitReader;
#[doc = "Field `MAIN_INTERNAL_ERR` writer - Specifies/registers the occurrence of a FLASH macro main interface internal error (typically the result of a read access while a program erase operation is ongoing) as a result of a CM0+ access. SW clears this field to '0'. HW sets this field to '1' on a FLASH macro main interface internal error. Typically, SW reads this field after a code section to detect the occurrence of an error. Note: this field is independent of FLASH_CTL.MAIN_ERR_SILENT."]
pub type MAIN_INTERNAL_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WORK_INTERNAL_ERR` reader - See CM0_STATUS.MAIN_INTERNAL_ERROR."]
pub type WORK_INTERNAL_ERR_R = crate::BitReader;
#[doc = "Field `WORK_INTERNAL_ERR` writer - See CM0_STATUS.MAIN_INTERNAL_ERROR."]
pub type WORK_INTERNAL_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Specifies/registers the occurrence of a FLASH macro main interface internal error (typically the result of a read access while a program erase operation is ongoing) as a result of a CM0+ access. SW clears this field to '0'. HW sets this field to '1' on a FLASH macro main interface internal error. Typically, SW reads this field after a code section to detect the occurrence of an error. Note: this field is independent of FLASH_CTL.MAIN_ERR_SILENT."]
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
    #[doc = "Bit 0 - Specifies/registers the occurrence of a FLASH macro main interface internal error (typically the result of a read access while a program erase operation is ongoing) as a result of a CM0+ access. SW clears this field to '0'. HW sets this field to '1' on a FLASH macro main interface internal error. Typically, SW reads this field after a code section to detect the occurrence of an error. Note: this field is independent of FLASH_CTL.MAIN_ERR_SILENT."]
    #[inline(always)]
    #[must_use]
    pub fn main_internal_err(&mut self) -> MAIN_INTERNAL_ERR_W<CM0_STATUS_SPEC> {
        MAIN_INTERNAL_ERR_W::new(self, 0)
    }
    #[doc = "Bit 1 - See CM0_STATUS.MAIN_INTERNAL_ERROR."]
    #[inline(always)]
    #[must_use]
    pub fn work_internal_err(&mut self) -> WORK_INTERNAL_ERR_W<CM0_STATUS_SPEC> {
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
#[doc = "CM0+ interface status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm0_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cm0_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CM0_STATUS_SPEC;
impl crate::RegisterSpec for CM0_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cm0_status::R`](R) reader structure"]
impl crate::Readable for CM0_STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cm0_status::W`](W) writer structure"]
impl crate::Writable for CM0_STATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CM0_STATUS to value 0"]
impl crate::Resettable for CM0_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
