#[doc = "Register `BUFF_CTL` reader"]
pub type R = crate::R<BUFF_CTL_SPEC>;
#[doc = "Register `BUFF_CTL` writer"]
pub type W = crate::W<BUFF_CTL_SPEC>;
#[doc = "Field `WRITE_BUFF` reader - Specifies if write transfer can be buffered in the bus infrastructure bridges: '0': Write transfers are not buffered, independent of the transfer's bufferable attribute. '1': Write transfers can be buffered, if the transfer's bufferable attribute indicates that the transfer is a bufferable/posted write."]
pub type WRITE_BUFF_R = crate::BitReader;
#[doc = "Field `WRITE_BUFF` writer - Specifies if write transfer can be buffered in the bus infrastructure bridges: '0': Write transfers are not buffered, independent of the transfer's bufferable attribute. '1': Write transfers can be buffered, if the transfer's bufferable attribute indicates that the transfer is a bufferable/posted write."]
pub type WRITE_BUFF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Specifies if write transfer can be buffered in the bus infrastructure bridges: '0': Write transfers are not buffered, independent of the transfer's bufferable attribute. '1': Write transfers can be buffered, if the transfer's bufferable attribute indicates that the transfer is a bufferable/posted write."]
    #[inline(always)]
    pub fn write_buff(&self) -> WRITE_BUFF_R {
        WRITE_BUFF_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Specifies if write transfer can be buffered in the bus infrastructure bridges: '0': Write transfers are not buffered, independent of the transfer's bufferable attribute. '1': Write transfers can be buffered, if the transfer's bufferable attribute indicates that the transfer is a bufferable/posted write."]
    #[inline(always)]
    #[must_use]
    pub fn write_buff(&mut self) -> WRITE_BUFF_W<BUFF_CTL_SPEC> {
        WRITE_BUFF_W::new(self, 0)
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
#[doc = "Buffer control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buff_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buff_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BUFF_CTL_SPEC;
impl crate::RegisterSpec for BUFF_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buff_ctl::R`](R) reader structure"]
impl crate::Readable for BUFF_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`buff_ctl::W`](W) writer structure"]
impl crate::Writable for BUFF_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BUFF_CTL to value 0x01"]
impl crate::Resettable for BUFF_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
