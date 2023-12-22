#[doc = "Register `START_CTRL` reader"]
pub type R = crate::R<START_CTRL_SPEC>;
#[doc = "Register `START_CTRL` writer"]
pub type W = crate::W<START_CTRL_SPEC>;
#[doc = "Field `FW_TRIGGER` reader - When firmware writes a 1 here it will trigger the next scan of enabled channels, hardware clears this bit when the scan started with this trigger is completed. If scanning continuously the trigger is ignored and hardware clears this bit after the next scan is done. This bit is also cleared when the SAR is disabled."]
pub type FW_TRIGGER_R = crate::BitReader;
#[doc = "Field `FW_TRIGGER` writer - When firmware writes a 1 here it will trigger the next scan of enabled channels, hardware clears this bit when the scan started with this trigger is completed. If scanning continuously the trigger is ignored and hardware clears this bit after the next scan is done. This bit is also cleared when the SAR is disabled."]
pub type FW_TRIGGER_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - When firmware writes a 1 here it will trigger the next scan of enabled channels, hardware clears this bit when the scan started with this trigger is completed. If scanning continuously the trigger is ignored and hardware clears this bit after the next scan is done. This bit is also cleared when the SAR is disabled."]
    #[inline(always)]
    pub fn fw_trigger(&self) -> FW_TRIGGER_R {
        FW_TRIGGER_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When firmware writes a 1 here it will trigger the next scan of enabled channels, hardware clears this bit when the scan started with this trigger is completed. If scanning continuously the trigger is ignored and hardware clears this bit after the next scan is done. This bit is also cleared when the SAR is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn fw_trigger(&mut self) -> FW_TRIGGER_W<START_CTRL_SPEC> {
        FW_TRIGGER_W::new(self, 0)
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
#[doc = "Start control register (firmware trigger).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`start_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`start_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct START_CTRL_SPEC;
impl crate::RegisterSpec for START_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`start_ctrl::R`](R) reader structure"]
impl crate::Readable for START_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`start_ctrl::W`](W) writer structure"]
impl crate::Writable for START_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets START_CTRL to value 0"]
impl crate::Resettable for START_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
