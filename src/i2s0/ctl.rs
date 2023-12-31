#[doc = "Register `CTL` reader"]
pub type R = crate::R<CTL_SPEC>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CTL_SPEC>;
#[doc = "Field `TX_ENABLED` reader - Enables the I2S TX component: '0': Disabled. '1': Enabled."]
pub type TX_ENABLED_R = crate::BitReader;
#[doc = "Field `TX_ENABLED` writer - Enables the I2S TX component: '0': Disabled. '1': Enabled."]
pub type TX_ENABLED_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_ENABLED` reader - Enables the I2S RX component: '0': Disabled. '1': Enabled."]
pub type RX_ENABLED_R = crate::BitReader;
#[doc = "Field `RX_ENABLED` writer - Enables the I2S RX component: '0': Disabled. '1': Enabled."]
pub type RX_ENABLED_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 30 - Enables the I2S TX component: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn tx_enabled(&self) -> TX_ENABLED_R {
        TX_ENABLED_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enables the I2S RX component: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn rx_enabled(&self) -> RX_ENABLED_R {
        RX_ENABLED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - Enables the I2S TX component: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    #[must_use]
    pub fn tx_enabled(&mut self) -> TX_ENABLED_W<CTL_SPEC> {
        TX_ENABLED_W::new(self, 30)
    }
    #[doc = "Bit 31 - Enables the I2S RX component: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    #[must_use]
    pub fn rx_enabled(&mut self) -> RX_ENABLED_W<CTL_SPEC> {
        RX_ENABLED_W::new(self, 31)
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
#[doc = "Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
