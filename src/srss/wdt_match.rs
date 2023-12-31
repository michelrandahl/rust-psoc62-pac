#[doc = "Register `WDT_MATCH` reader"]
pub type R = crate::R<WDT_MATCH_SPEC>;
#[doc = "Register `WDT_MATCH` writer"]
pub type W = crate::W<WDT_MATCH_SPEC>;
#[doc = "Field `MATCH` reader - Match value for Watchdog counter. Every time WDT_COUNTER reaches MATCH an interrupt is generated. Two unserviced interrupts will lead to a system reset (i.e. at the third match)."]
pub type MATCH_R = crate::FieldReader<u16>;
#[doc = "Field `MATCH` writer - Match value for Watchdog counter. Every time WDT_COUNTER reaches MATCH an interrupt is generated. Two unserviced interrupts will lead to a system reset (i.e. at the third match)."]
pub type MATCH_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `IGNORE_BITS` reader - The number of MSB bits of the watchdog timer that are NOT checked against MATCH. This value provides control over the time-to-reset of the watchdog (which happens after 3 successive matches). Up to 12 MSB can be ignored. Settings >12 behave like a setting of 12."]
pub type IGNORE_BITS_R = crate::FieldReader;
#[doc = "Field `IGNORE_BITS` writer - The number of MSB bits of the watchdog timer that are NOT checked against MATCH. This value provides control over the time-to-reset of the watchdog (which happens after 3 successive matches). Up to 12 MSB can be ignored. Settings >12 behave like a setting of 12."]
pub type IGNORE_BITS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:15 - Match value for Watchdog counter. Every time WDT_COUNTER reaches MATCH an interrupt is generated. Two unserviced interrupts will lead to a system reset (i.e. at the third match)."]
    #[inline(always)]
    pub fn match_(&self) -> MATCH_R {
        MATCH_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - The number of MSB bits of the watchdog timer that are NOT checked against MATCH. This value provides control over the time-to-reset of the watchdog (which happens after 3 successive matches). Up to 12 MSB can be ignored. Settings >12 behave like a setting of 12."]
    #[inline(always)]
    pub fn ignore_bits(&self) -> IGNORE_BITS_R {
        IGNORE_BITS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Match value for Watchdog counter. Every time WDT_COUNTER reaches MATCH an interrupt is generated. Two unserviced interrupts will lead to a system reset (i.e. at the third match)."]
    #[inline(always)]
    #[must_use]
    pub fn match_(&mut self) -> MATCH_W<WDT_MATCH_SPEC> {
        MATCH_W::new(self, 0)
    }
    #[doc = "Bits 16:19 - The number of MSB bits of the watchdog timer that are NOT checked against MATCH. This value provides control over the time-to-reset of the watchdog (which happens after 3 successive matches). Up to 12 MSB can be ignored. Settings >12 behave like a setting of 12."]
    #[inline(always)]
    #[must_use]
    pub fn ignore_bits(&mut self) -> IGNORE_BITS_W<WDT_MATCH_SPEC> {
        IGNORE_BITS_W::new(self, 16)
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
#[doc = "Watchdog Counter Match Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdt_match::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdt_match::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDT_MATCH_SPEC;
impl crate::RegisterSpec for WDT_MATCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdt_match::R`](R) reader structure"]
impl crate::Readable for WDT_MATCH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wdt_match::W`](W) writer structure"]
impl crate::Writable for WDT_MATCH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WDT_MATCH to value 0x1000"]
impl crate::Resettable for WDT_MATCH_SPEC {
    const RESET_VALUE: Self::Ux = 0x1000;
}
