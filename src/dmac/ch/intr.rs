#[doc = "Register `INTR` reader"]
pub type R = crate::R<INTR_SPEC>;
#[doc = "Register `INTR` writer"]
pub type W = crate::W<INTR_SPEC>;
#[doc = "Field `COMPLETION` reader - Activated (set to '1') on completion of data transfer(s) as specified by the descriptor's CH_DESCR_CTL.INTR_TYPE."]
pub type COMPLETION_R = crate::BitReader;
#[doc = "Field `COMPLETION` writer - Activated (set to '1') on completion of data transfer(s) as specified by the descriptor's CH_DESCR_CTL.INTR_TYPE."]
pub type COMPLETION_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRC_BUS_ERROR` reader - Activated (set to '1') on a bus error for a load from the source."]
pub type SRC_BUS_ERROR_R = crate::BitReader;
#[doc = "Field `SRC_BUS_ERROR` writer - Activated (set to '1') on a bus error for a load from the source."]
pub type SRC_BUS_ERROR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DST_BUS_ERROR` reader - Activated (set to '1') on a bus error for a store to the destination."]
pub type DST_BUS_ERROR_R = crate::BitReader;
#[doc = "Field `DST_BUS_ERROR` writer - Activated (set to '1') on a bus error for a store to the destination."]
pub type DST_BUS_ERROR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRC_MISAL` reader - Activated (set to '1') on a misalignment of the source address."]
pub type SRC_MISAL_R = crate::BitReader;
#[doc = "Field `SRC_MISAL` writer - Activated (set to '1') on a misalignment of the source address."]
pub type SRC_MISAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DST_MISAL` reader - Activated (set to '1') on a misalignment of the destination address."]
pub type DST_MISAL_R = crate::BitReader;
#[doc = "Field `DST_MISAL` writer - Activated (set to '1') on a misalignment of the destination address."]
pub type DST_MISAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CURR_PTR_NULL` reader - Activated (set to '1') when the channel is enabled (CH_CTL.ENABLED is '1') and CH_CURR_PTR is '0'."]
pub type CURR_PTR_NULL_R = crate::BitReader;
#[doc = "Field `CURR_PTR_NULL` writer - Activated (set to '1') when the channel is enabled (CH_CTL.ENABLED is '1') and CH_CURR_PTR is '0'."]
pub type CURR_PTR_NULL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACTIVE_CH_DISABLED` reader - Activated (set to '1') if the channel is disabled by SW (accidentally/incorrectly) when the data transfer engine is busy."]
pub type ACTIVE_CH_DISABLED_R = crate::BitReader;
#[doc = "Field `ACTIVE_CH_DISABLED` writer - Activated (set to '1') if the channel is disabled by SW (accidentally/incorrectly) when the data transfer engine is busy."]
pub type ACTIVE_CH_DISABLED_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DESCR_BUS_ERROR` reader - Activated (set to '1') on a bus error for a load of the descriptor."]
pub type DESCR_BUS_ERROR_R = crate::BitReader;
#[doc = "Field `DESCR_BUS_ERROR` writer - Activated (set to '1') on a bus error for a load of the descriptor."]
pub type DESCR_BUS_ERROR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Activated (set to '1') on completion of data transfer(s) as specified by the descriptor's CH_DESCR_CTL.INTR_TYPE."]
    #[inline(always)]
    pub fn completion(&self) -> COMPLETION_R {
        COMPLETION_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Activated (set to '1') on a bus error for a load from the source."]
    #[inline(always)]
    pub fn src_bus_error(&self) -> SRC_BUS_ERROR_R {
        SRC_BUS_ERROR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Activated (set to '1') on a bus error for a store to the destination."]
    #[inline(always)]
    pub fn dst_bus_error(&self) -> DST_BUS_ERROR_R {
        DST_BUS_ERROR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Activated (set to '1') on a misalignment of the source address."]
    #[inline(always)]
    pub fn src_misal(&self) -> SRC_MISAL_R {
        SRC_MISAL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Activated (set to '1') on a misalignment of the destination address."]
    #[inline(always)]
    pub fn dst_misal(&self) -> DST_MISAL_R {
        DST_MISAL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Activated (set to '1') when the channel is enabled (CH_CTL.ENABLED is '1') and CH_CURR_PTR is '0'."]
    #[inline(always)]
    pub fn curr_ptr_null(&self) -> CURR_PTR_NULL_R {
        CURR_PTR_NULL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Activated (set to '1') if the channel is disabled by SW (accidentally/incorrectly) when the data transfer engine is busy."]
    #[inline(always)]
    pub fn active_ch_disabled(&self) -> ACTIVE_CH_DISABLED_R {
        ACTIVE_CH_DISABLED_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Activated (set to '1') on a bus error for a load of the descriptor."]
    #[inline(always)]
    pub fn descr_bus_error(&self) -> DESCR_BUS_ERROR_R {
        DESCR_BUS_ERROR_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Activated (set to '1') on completion of data transfer(s) as specified by the descriptor's CH_DESCR_CTL.INTR_TYPE."]
    #[inline(always)]
    #[must_use]
    pub fn completion(&mut self) -> COMPLETION_W<INTR_SPEC> {
        COMPLETION_W::new(self, 0)
    }
    #[doc = "Bit 1 - Activated (set to '1') on a bus error for a load from the source."]
    #[inline(always)]
    #[must_use]
    pub fn src_bus_error(&mut self) -> SRC_BUS_ERROR_W<INTR_SPEC> {
        SRC_BUS_ERROR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Activated (set to '1') on a bus error for a store to the destination."]
    #[inline(always)]
    #[must_use]
    pub fn dst_bus_error(&mut self) -> DST_BUS_ERROR_W<INTR_SPEC> {
        DST_BUS_ERROR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Activated (set to '1') on a misalignment of the source address."]
    #[inline(always)]
    #[must_use]
    pub fn src_misal(&mut self) -> SRC_MISAL_W<INTR_SPEC> {
        SRC_MISAL_W::new(self, 3)
    }
    #[doc = "Bit 4 - Activated (set to '1') on a misalignment of the destination address."]
    #[inline(always)]
    #[must_use]
    pub fn dst_misal(&mut self) -> DST_MISAL_W<INTR_SPEC> {
        DST_MISAL_W::new(self, 4)
    }
    #[doc = "Bit 5 - Activated (set to '1') when the channel is enabled (CH_CTL.ENABLED is '1') and CH_CURR_PTR is '0'."]
    #[inline(always)]
    #[must_use]
    pub fn curr_ptr_null(&mut self) -> CURR_PTR_NULL_W<INTR_SPEC> {
        CURR_PTR_NULL_W::new(self, 5)
    }
    #[doc = "Bit 6 - Activated (set to '1') if the channel is disabled by SW (accidentally/incorrectly) when the data transfer engine is busy."]
    #[inline(always)]
    #[must_use]
    pub fn active_ch_disabled(&mut self) -> ACTIVE_CH_DISABLED_W<INTR_SPEC> {
        ACTIVE_CH_DISABLED_W::new(self, 6)
    }
    #[doc = "Bit 7 - Activated (set to '1') on a bus error for a load of the descriptor."]
    #[inline(always)]
    #[must_use]
    pub fn descr_bus_error(&mut self) -> DESCR_BUS_ERROR_W<INTR_SPEC> {
        DESCR_BUS_ERROR_W::new(self, 7)
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
#[doc = "Interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_SPEC;
impl crate::RegisterSpec for INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr::R`](R) reader structure"]
impl crate::Readable for INTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intr::W`](W) writer structure"]
impl crate::Writable for INTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR to value 0"]
impl crate::Resettable for INTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
