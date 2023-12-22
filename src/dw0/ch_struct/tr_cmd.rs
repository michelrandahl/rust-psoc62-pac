#[doc = "Register `TR_CMD` reader"]
pub type R = crate::R<TR_CMD_SPEC>;
#[doc = "Register `TR_CMD` writer"]
pub type W = crate::W<TR_CMD_SPEC>;
#[doc = "Field `ACTIVATE` reader - Software trigger. When written with '1', a trigger is generated which sets 'trigger pending' (only if the channel is enabled). A read always returns a 0."]
pub type ACTIVATE_R = crate::BitReader;
#[doc = "Field `ACTIVATE` writer - Software trigger. When written with '1', a trigger is generated which sets 'trigger pending' (only if the channel is enabled). A read always returns a 0."]
pub type ACTIVATE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Software trigger. When written with '1', a trigger is generated which sets 'trigger pending' (only if the channel is enabled). A read always returns a 0."]
    #[inline(always)]
    pub fn activate(&self) -> ACTIVATE_R {
        ACTIVATE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software trigger. When written with '1', a trigger is generated which sets 'trigger pending' (only if the channel is enabled). A read always returns a 0."]
    #[inline(always)]
    #[must_use]
    pub fn activate(&mut self) -> ACTIVATE_W<TR_CMD_SPEC> {
        ACTIVATE_W::new(self, 0)
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
#[doc = "Channel software trigger\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tr_cmd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tr_cmd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TR_CMD_SPEC;
impl crate::RegisterSpec for TR_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tr_cmd::R`](R) reader structure"]
impl crate::Readable for TR_CMD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tr_cmd::W`](W) writer structure"]
impl crate::Writable for TR_CMD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TR_CMD to value 0"]
impl crate::Resettable for TR_CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
