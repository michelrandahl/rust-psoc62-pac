#[doc = "Register `FLASH_CMD` reader"]
pub type R = crate::R<FLASH_CMD_SPEC>;
#[doc = "Register `FLASH_CMD` writer"]
pub type W = crate::W<FLASH_CMD_SPEC>;
#[doc = "Field `INV` reader - Invalidation of ALL caches (for CM0+ and CM4) and ALL buffers. SW writes a '1' to clear the caches. HW sets this field to '0' when the operation is completed. The operation takes a maximum of three clock cycles on the slowest of the clk_slow and clk_fast clocks. The caches' LRU structures are also reset to their default state."]
pub type INV_R = crate::BitReader;
#[doc = "Field `INV` writer - Invalidation of ALL caches (for CM0+ and CM4) and ALL buffers. SW writes a '1' to clear the caches. HW sets this field to '0' when the operation is completed. The operation takes a maximum of three clock cycles on the slowest of the clk_slow and clk_fast clocks. The caches' LRU structures are also reset to their default state."]
pub type INV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFF_INV` reader - Invalidation of ALL buffers (does not invalidate the caches). SW writes a '1' to clear the buffers. HW sets this field to '0' when the operation is completed. The operation takes a maximum of three clock cycles on the slowest of the clk_slow and clk_fast clocks. Note: the caches only capture FLASH macro main array data. Therefore, invalidating just the buffers (BUFF_INV) does not invalidate captures main array data in the caches."]
pub type BUFF_INV_R = crate::BitReader;
#[doc = "Field `BUFF_INV` writer - Invalidation of ALL buffers (does not invalidate the caches). SW writes a '1' to clear the buffers. HW sets this field to '0' when the operation is completed. The operation takes a maximum of three clock cycles on the slowest of the clk_slow and clk_fast clocks. Note: the caches only capture FLASH macro main array data. Therefore, invalidating just the buffers (BUFF_INV) does not invalidate captures main array data in the caches."]
pub type BUFF_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Invalidation of ALL caches (for CM0+ and CM4) and ALL buffers. SW writes a '1' to clear the caches. HW sets this field to '0' when the operation is completed. The operation takes a maximum of three clock cycles on the slowest of the clk_slow and clk_fast clocks. The caches' LRU structures are also reset to their default state."]
    #[inline(always)]
    pub fn inv(&self) -> INV_R {
        INV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Invalidation of ALL buffers (does not invalidate the caches). SW writes a '1' to clear the buffers. HW sets this field to '0' when the operation is completed. The operation takes a maximum of three clock cycles on the slowest of the clk_slow and clk_fast clocks. Note: the caches only capture FLASH macro main array data. Therefore, invalidating just the buffers (BUFF_INV) does not invalidate captures main array data in the caches."]
    #[inline(always)]
    pub fn buff_inv(&self) -> BUFF_INV_R {
        BUFF_INV_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Invalidation of ALL caches (for CM0+ and CM4) and ALL buffers. SW writes a '1' to clear the caches. HW sets this field to '0' when the operation is completed. The operation takes a maximum of three clock cycles on the slowest of the clk_slow and clk_fast clocks. The caches' LRU structures are also reset to their default state."]
    #[inline(always)]
    #[must_use]
    pub fn inv(&mut self) -> INV_W<FLASH_CMD_SPEC> {
        INV_W::new(self, 0)
    }
    #[doc = "Bit 1 - Invalidation of ALL buffers (does not invalidate the caches). SW writes a '1' to clear the buffers. HW sets this field to '0' when the operation is completed. The operation takes a maximum of three clock cycles on the slowest of the clk_slow and clk_fast clocks. Note: the caches only capture FLASH macro main array data. Therefore, invalidating just the buffers (BUFF_INV) does not invalidate captures main array data in the caches."]
    #[inline(always)]
    #[must_use]
    pub fn buff_inv(&mut self) -> BUFF_INV_W<FLASH_CMD_SPEC> {
        BUFF_INV_W::new(self, 1)
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
#[doc = "Command\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_cmd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_cmd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLASH_CMD_SPEC;
impl crate::RegisterSpec for FLASH_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_cmd::R`](R) reader structure"]
impl crate::Readable for FLASH_CMD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`flash_cmd::W`](W) writer structure"]
impl crate::Writable for FLASH_CMD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLASH_CMD to value 0"]
impl crate::Resettable for FLASH_CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
