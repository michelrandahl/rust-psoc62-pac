#[doc = "Register `SLOW_CA_CMD` reader"]
pub type R = crate::R<SLOW_CA_CMD_SPEC>;
#[doc = "Register `SLOW_CA_CMD` writer"]
pub type W = crate::W<SLOW_CA_CMD_SPEC>;
#[doc = "Field `INV` reader - Cache and prefetch buffer invalidation. SW writes a '1' to clear the cache and prefetch buffer. The cache's LRU structure is also reset to its default state. Note, A write access will invalidate the prefetch buffer automatically in hardware. A write access should invalidate both fast and slow caches, by firmware. Note, firmware should invalidate the cache and prefetch buffer only when STATUS.BUSY is '0'."]
pub type INV_R = crate::BitReader;
#[doc = "Field `INV` writer - Cache and prefetch buffer invalidation. SW writes a '1' to clear the cache and prefetch buffer. The cache's LRU structure is also reset to its default state. Note, A write access will invalidate the prefetch buffer automatically in hardware. A write access should invalidate both fast and slow caches, by firmware. Note, firmware should invalidate the cache and prefetch buffer only when STATUS.BUSY is '0'."]
pub type INV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Cache and prefetch buffer invalidation. SW writes a '1' to clear the cache and prefetch buffer. The cache's LRU structure is also reset to its default state. Note, A write access will invalidate the prefetch buffer automatically in hardware. A write access should invalidate both fast and slow caches, by firmware. Note, firmware should invalidate the cache and prefetch buffer only when STATUS.BUSY is '0'."]
    #[inline(always)]
    pub fn inv(&self) -> INV_R {
        INV_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cache and prefetch buffer invalidation. SW writes a '1' to clear the cache and prefetch buffer. The cache's LRU structure is also reset to its default state. Note, A write access will invalidate the prefetch buffer automatically in hardware. A write access should invalidate both fast and slow caches, by firmware. Note, firmware should invalidate the cache and prefetch buffer only when STATUS.BUSY is '0'."]
    #[inline(always)]
    #[must_use]
    pub fn inv(&mut self) -> INV_W<SLOW_CA_CMD_SPEC> {
        INV_W::new(self, 0)
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
#[doc = "Slow cache command\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slow_ca_cmd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slow_ca_cmd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLOW_CA_CMD_SPEC;
impl crate::RegisterSpec for SLOW_CA_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slow_ca_cmd::R`](R) reader structure"]
impl crate::Readable for SLOW_CA_CMD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slow_ca_cmd::W`](W) writer structure"]
impl crate::Writable for SLOW_CA_CMD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLOW_CA_CMD to value 0"]
impl crate::Resettable for SLOW_CA_CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
