#[doc = "Register `RTC_RW` reader"]
pub type R = crate::R<RTC_RW_SPEC>;
#[doc = "Register `RTC_RW` writer"]
pub type W = crate::W<RTC_RW_SPEC>;
#[doc = "Field `READ` reader - Read bit When this bit is set the RTC registers will be copied to user registers and frozen so that a coherent RTC value can safely be read. The RTC will keep on running. Do not set the read bit if the RTC is still busy with a previous update (see RTC_BUSY bit) or if the Write bit is set. Do not set the Read bit at the same time that the Write bit is cleared."]
pub type READ_R = crate::BitReader;
#[doc = "Field `READ` writer - Read bit When this bit is set the RTC registers will be copied to user registers and frozen so that a coherent RTC value can safely be read. The RTC will keep on running. Do not set the read bit if the RTC is still busy with a previous update (see RTC_BUSY bit) or if the Write bit is set. Do not set the Read bit at the same time that the Write bit is cleared."]
pub type READ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE` reader - Write bit Only when this bit is set can the RTC registers be written to (otherwise writes are ignored). This bit cannot be set if the RTC is still busy with a previous update (see RTC_BUSY bit) or if the Read bit is set or getting set. The user writes to the RTC user registers, when the Write bit is cleared by the user then the user registers content is copied to the actual RTC registers. Only user RTC registers that were written to will get copied, others will not be affected. When the SECONDS field is updated then TICKS will also be reset (WDT is not affected). When the Write bit is cleared by a reset (brown out/DeepSleep) then the RTC update will be ignored/lost. Do not set the Write bit if the RTC if the RTC is still busy with a previous update (see RTC_BUSY). Do not set the Write bit at the same time that the Read bit is cleared."]
pub type WRITE_R = crate::BitReader;
#[doc = "Field `WRITE` writer - Write bit Only when this bit is set can the RTC registers be written to (otherwise writes are ignored). This bit cannot be set if the RTC is still busy with a previous update (see RTC_BUSY bit) or if the Read bit is set or getting set. The user writes to the RTC user registers, when the Write bit is cleared by the user then the user registers content is copied to the actual RTC registers. Only user RTC registers that were written to will get copied, others will not be affected. When the SECONDS field is updated then TICKS will also be reset (WDT is not affected). When the Write bit is cleared by a reset (brown out/DeepSleep) then the RTC update will be ignored/lost. Do not set the Write bit if the RTC if the RTC is still busy with a previous update (see RTC_BUSY). Do not set the Write bit at the same time that the Read bit is cleared."]
pub type WRITE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Read bit When this bit is set the RTC registers will be copied to user registers and frozen so that a coherent RTC value can safely be read. The RTC will keep on running. Do not set the read bit if the RTC is still busy with a previous update (see RTC_BUSY bit) or if the Write bit is set. Do not set the Read bit at the same time that the Write bit is cleared."]
    #[inline(always)]
    pub fn read(&self) -> READ_R {
        READ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write bit Only when this bit is set can the RTC registers be written to (otherwise writes are ignored). This bit cannot be set if the RTC is still busy with a previous update (see RTC_BUSY bit) or if the Read bit is set or getting set. The user writes to the RTC user registers, when the Write bit is cleared by the user then the user registers content is copied to the actual RTC registers. Only user RTC registers that were written to will get copied, others will not be affected. When the SECONDS field is updated then TICKS will also be reset (WDT is not affected). When the Write bit is cleared by a reset (brown out/DeepSleep) then the RTC update will be ignored/lost. Do not set the Write bit if the RTC if the RTC is still busy with a previous update (see RTC_BUSY). Do not set the Write bit at the same time that the Read bit is cleared."]
    #[inline(always)]
    pub fn write(&self) -> WRITE_R {
        WRITE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Read bit When this bit is set the RTC registers will be copied to user registers and frozen so that a coherent RTC value can safely be read. The RTC will keep on running. Do not set the read bit if the RTC is still busy with a previous update (see RTC_BUSY bit) or if the Write bit is set. Do not set the Read bit at the same time that the Write bit is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn read(&mut self) -> READ_W<RTC_RW_SPEC> {
        READ_W::new(self, 0)
    }
    #[doc = "Bit 1 - Write bit Only when this bit is set can the RTC registers be written to (otherwise writes are ignored). This bit cannot be set if the RTC is still busy with a previous update (see RTC_BUSY bit) or if the Read bit is set or getting set. The user writes to the RTC user registers, when the Write bit is cleared by the user then the user registers content is copied to the actual RTC registers. Only user RTC registers that were written to will get copied, others will not be affected. When the SECONDS field is updated then TICKS will also be reset (WDT is not affected). When the Write bit is cleared by a reset (brown out/DeepSleep) then the RTC update will be ignored/lost. Do not set the Write bit if the RTC if the RTC is still busy with a previous update (see RTC_BUSY). Do not set the Write bit at the same time that the Read bit is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn write(&mut self) -> WRITE_W<RTC_RW_SPEC> {
        WRITE_W::new(self, 1)
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
#[doc = "RTC Read Write register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_rw::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_rw::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTC_RW_SPEC;
impl crate::RegisterSpec for RTC_RW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_rw::R`](R) reader structure"]
impl crate::Readable for RTC_RW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtc_rw::W`](W) writer structure"]
impl crate::Writable for RTC_RW_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTC_RW to value 0"]
impl crate::Resettable for RTC_RW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
