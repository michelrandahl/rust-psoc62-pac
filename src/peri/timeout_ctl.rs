#[doc = "Register `TIMEOUT_CTL` reader"]
pub type R = crate::R<TIMEOUT_CTL_SPEC>;
#[doc = "Register `TIMEOUT_CTL` writer"]
pub type W = crate::W<TIMEOUT_CTL_SPEC>;
#[doc = "Field `TIMEOUT` reader - This field specifies a number of clock cycles (clk_slow). If an AHB-Lite bus transfer takes more than the specified number of cycles (timeout detection), the bus transfer is terminated with an AHB-Lite bus error and a fault is generated (and possibly recorded in the fault report structure(s)). '0x0000'-'0xfffe': Number of clock cycles. '0xffff': This value is the default/reset value and specifies that no timeout detection is performed: a bus transfer will never be terminated and a fault will never be generated."]
pub type TIMEOUT_R = crate::FieldReader<u16>;
#[doc = "Field `TIMEOUT` writer - This field specifies a number of clock cycles (clk_slow). If an AHB-Lite bus transfer takes more than the specified number of cycles (timeout detection), the bus transfer is terminated with an AHB-Lite bus error and a fault is generated (and possibly recorded in the fault report structure(s)). '0x0000'-'0xfffe': Number of clock cycles. '0xffff': This value is the default/reset value and specifies that no timeout detection is performed: a bus transfer will never be terminated and a fault will never be generated."]
pub type TIMEOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - This field specifies a number of clock cycles (clk_slow). If an AHB-Lite bus transfer takes more than the specified number of cycles (timeout detection), the bus transfer is terminated with an AHB-Lite bus error and a fault is generated (and possibly recorded in the fault report structure(s)). '0x0000'-'0xfffe': Number of clock cycles. '0xffff': This value is the default/reset value and specifies that no timeout detection is performed: a bus transfer will never be terminated and a fault will never be generated."]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This field specifies a number of clock cycles (clk_slow). If an AHB-Lite bus transfer takes more than the specified number of cycles (timeout detection), the bus transfer is terminated with an AHB-Lite bus error and a fault is generated (and possibly recorded in the fault report structure(s)). '0x0000'-'0xfffe': Number of clock cycles. '0xffff': This value is the default/reset value and specifies that no timeout detection is performed: a bus transfer will never be terminated and a fault will never be generated."]
    #[inline(always)]
    #[must_use]
    pub fn timeout(&mut self) -> TIMEOUT_W<TIMEOUT_CTL_SPEC> {
        TIMEOUT_W::new(self, 0)
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
#[doc = "Timeout control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timeout_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timeout_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMEOUT_CTL_SPEC;
impl crate::RegisterSpec for TIMEOUT_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timeout_ctl::R`](R) reader structure"]
impl crate::Readable for TIMEOUT_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timeout_ctl::W`](W) writer structure"]
impl crate::Writable for TIMEOUT_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMEOUT_CTL to value 0xffff"]
impl crate::Resettable for TIMEOUT_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
