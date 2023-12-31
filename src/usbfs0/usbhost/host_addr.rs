#[doc = "Register `HOST_ADDR` reader"]
pub type R = crate::R<HOST_ADDR_SPEC>;
#[doc = "Register `HOST_ADDR` writer"]
pub type W = crate::W<HOST_ADDR_SPEC>;
#[doc = "Field `ADDRESS` reader - These bits are used to specify a token address. Note: - This bit is not initialized even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type ADDRESS_R = crate::FieldReader;
#[doc = "Field `ADDRESS` writer - These bits are used to specify a token address. Note: - This bit is not initialized even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type ADDRESS_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - These bits are used to specify a token address. Note: - This bit is not initialized even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn address(&self) -> ADDRESS_R {
        ADDRESS_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - These bits are used to specify a token address. Note: - This bit is not initialized even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    #[must_use]
    pub fn address(&mut self) -> ADDRESS_W<HOST_ADDR_SPEC> {
        ADDRESS_W::new(self, 0)
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
#[doc = "Host Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOST_ADDR_SPEC;
impl crate::RegisterSpec for HOST_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_addr::R`](R) reader structure"]
impl crate::Readable for HOST_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`host_addr::W`](W) writer structure"]
impl crate::Writable for HOST_ADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HOST_ADDR to value 0"]
impl crate::Resettable for HOST_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
