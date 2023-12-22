#[doc = "Register `SYNC_CTL` reader"]
pub type R = crate::R<SYNC_CTL_SPEC>;
#[doc = "Register `SYNC_CTL` writer"]
pub type W = crate::W<SYNC_CTL_SPEC>;
#[doc = "Field `IO_SYNC_EN` reader - Synchronization of the IO pin input signals to 'clk_fabric', one bit for each IO pin: IO_SYNC_EN\\[i\\]
is for IO pin i. '0': No synchronization. '1': Synchronization."]
pub type IO_SYNC_EN_R = crate::FieldReader;
#[doc = "Field `IO_SYNC_EN` writer - Synchronization of the IO pin input signals to 'clk_fabric', one bit for each IO pin: IO_SYNC_EN\\[i\\]
is for IO pin i. '0': No synchronization. '1': Synchronization."]
pub type IO_SYNC_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CHIP_SYNC_EN` reader - Synchronization of the chip input signals to 'clk_fabric', one bit for each input: CHIP_SYNC_EN\\[i\\]
is for input i. '0': No synchronization. '1': Synchronization."]
pub type CHIP_SYNC_EN_R = crate::FieldReader;
#[doc = "Field `CHIP_SYNC_EN` writer - Synchronization of the chip input signals to 'clk_fabric', one bit for each input: CHIP_SYNC_EN\\[i\\]
is for input i. '0': No synchronization. '1': Synchronization."]
pub type CHIP_SYNC_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Synchronization of the IO pin input signals to 'clk_fabric', one bit for each IO pin: IO_SYNC_EN\\[i\\]
is for IO pin i. '0': No synchronization. '1': Synchronization."]
    #[inline(always)]
    pub fn io_sync_en(&self) -> IO_SYNC_EN_R {
        IO_SYNC_EN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Synchronization of the chip input signals to 'clk_fabric', one bit for each input: CHIP_SYNC_EN\\[i\\]
is for input i. '0': No synchronization. '1': Synchronization."]
    #[inline(always)]
    pub fn chip_sync_en(&self) -> CHIP_SYNC_EN_R {
        CHIP_SYNC_EN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Synchronization of the IO pin input signals to 'clk_fabric', one bit for each IO pin: IO_SYNC_EN\\[i\\]
is for IO pin i. '0': No synchronization. '1': Synchronization."]
    #[inline(always)]
    #[must_use]
    pub fn io_sync_en(&mut self) -> IO_SYNC_EN_W<SYNC_CTL_SPEC> {
        IO_SYNC_EN_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Synchronization of the chip input signals to 'clk_fabric', one bit for each input: CHIP_SYNC_EN\\[i\\]
is for input i. '0': No synchronization. '1': Synchronization."]
    #[inline(always)]
    #[must_use]
    pub fn chip_sync_en(&mut self) -> CHIP_SYNC_EN_W<SYNC_CTL_SPEC> {
        CHIP_SYNC_EN_W::new(self, 8)
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
#[doc = "Synchronization control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sync_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sync_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYNC_CTL_SPEC;
impl crate::RegisterSpec for SYNC_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sync_ctl::R`](R) reader structure"]
impl crate::Readable for SYNC_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sync_ctl::W`](W) writer structure"]
impl crate::Writable for SYNC_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYNC_CTL to value 0"]
impl crate::Resettable for SYNC_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
