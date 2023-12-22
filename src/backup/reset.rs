#[doc = "Register `RESET` reader"]
pub type R = crate::R<RESET_SPEC>;
#[doc = "Register `RESET` writer"]
pub type W = crate::W<RESET_SPEC>;
#[doc = "Field `RESET` reader - Writing 1 to this register resets the backup logic. Hardware clears it when the reset is complete. After setting this register, firmware should confirm it reads as 0 before attempting to write other backup registers."]
pub type RESET_R = crate::BitReader;
#[doc = "Field `RESET` writer - Writing 1 to this register resets the backup logic. Hardware clears it when the reset is complete. After setting this register, firmware should confirm it reads as 0 before attempting to write other backup registers."]
pub type RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - Writing 1 to this register resets the backup logic. Hardware clears it when the reset is complete. After setting this register, firmware should confirm it reads as 0 before attempting to write other backup registers."]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Writing 1 to this register resets the backup logic. Hardware clears it when the reset is complete. After setting this register, firmware should confirm it reads as 0 before attempting to write other backup registers."]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> RESET_W<RESET_SPEC> {
        RESET_W::new(self, 31)
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
#[doc = "Backup reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESET_SPEC;
impl crate::RegisterSpec for RESET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reset::R`](R) reader structure"]
impl crate::Readable for RESET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`reset::W`](W) writer structure"]
impl crate::Writable for RESET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RESET to value 0"]
impl crate::Resettable for RESET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
