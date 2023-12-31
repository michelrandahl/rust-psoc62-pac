#[doc = "Register `CMD` reader"]
pub type R = crate::R<CMD_SPEC>;
#[doc = "Register `CMD` writer"]
pub type W = crate::W<CMD_SPEC>;
#[doc = "Field `TX_START` reader - Transmitter enable: '0': Disabled. '1': Enabled."]
pub type TX_START_R = crate::BitReader;
#[doc = "Field `TX_START` writer - Transmitter enable: '0': Disabled. '1': Enabled."]
pub type TX_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_PAUSE` reader - Pause enable: '0': Disabled (TX FIFO data is sent over I2S). '1': Enabled ('0' data is sent over I2S, instead of TX FIFO data)."]
pub type TX_PAUSE_R = crate::BitReader;
#[doc = "Field `TX_PAUSE` writer - Pause enable: '0': Disabled (TX FIFO data is sent over I2S). '1': Enabled ('0' data is sent over I2S, instead of TX FIFO data)."]
pub type TX_PAUSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_START` reader - Receiver enable: '0': Disabled. '1': Enabled."]
pub type RX_START_R = crate::BitReader;
#[doc = "Field `RX_START` writer - Receiver enable: '0': Disabled. '1': Enabled."]
pub type RX_START_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transmitter enable: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn tx_start(&self) -> TX_START_R {
        TX_START_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Pause enable: '0': Disabled (TX FIFO data is sent over I2S). '1': Enabled ('0' data is sent over I2S, instead of TX FIFO data)."]
    #[inline(always)]
    pub fn tx_pause(&self) -> TX_PAUSE_R {
        TX_PAUSE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Receiver enable: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn rx_start(&self) -> RX_START_R {
        RX_START_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmitter enable: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    #[must_use]
    pub fn tx_start(&mut self) -> TX_START_W<CMD_SPEC> {
        TX_START_W::new(self, 0)
    }
    #[doc = "Bit 8 - Pause enable: '0': Disabled (TX FIFO data is sent over I2S). '1': Enabled ('0' data is sent over I2S, instead of TX FIFO data)."]
    #[inline(always)]
    #[must_use]
    pub fn tx_pause(&mut self) -> TX_PAUSE_W<CMD_SPEC> {
        TX_PAUSE_W::new(self, 8)
    }
    #[doc = "Bit 16 - Receiver enable: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    #[must_use]
    pub fn rx_start(&mut self) -> RX_START_W<CMD_SPEC> {
        RX_START_W::new(self, 16)
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
#[doc = "Command\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd::R`](R) reader structure"]
impl crate::Readable for CMD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmd::W`](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
