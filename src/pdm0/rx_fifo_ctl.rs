#[doc = "Register `RX_FIFO_CTL` reader"]
pub type R = crate::R<RX_FIFO_CTL_SPEC>;
#[doc = "Register `RX_FIFO_CTL` writer"]
pub type W = crate::W<RX_FIFO_CTL_SPEC>;
#[doc = "Field `TRIGGER_LEVEL` reader - Trigger level. When the RX FIFO has more entries than the number of this field, a receiver trigger event is generated. Note: software can configure up to 254 in Mono channel enabled (MODE_CTL.PCM_CH_SET = '1' or '2'), up to 253 in Stereo channel enabled (MODE_CTL.PCM_CH_SET = '3')."]
pub type TRIGGER_LEVEL_R = crate::FieldReader;
#[doc = "Field `TRIGGER_LEVEL` writer - Trigger level. When the RX FIFO has more entries than the number of this field, a receiver trigger event is generated. Note: software can configure up to 254 in Mono channel enabled (MODE_CTL.PCM_CH_SET = '1' or '2'), up to 253 in Stereo channel enabled (MODE_CTL.PCM_CH_SET = '3')."]
pub type TRIGGER_LEVEL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CLEAR` reader - When '1', the RX FIFO and RX_BUF are cleared/invalidated. Invalidation will last for as long as this field is '1'. If a quick clear/invalidation is required, the field should be set to '1' and be followed by a set to '0'. If a clear/invalidation is required for an extended time period, the field should be set to '1' during the complete time period."]
pub type CLEAR_R = crate::BitReader;
#[doc = "Field `CLEAR` writer - When '1', the RX FIFO and RX_BUF are cleared/invalidated. Invalidation will last for as long as this field is '1'. If a quick clear/invalidation is required, the field should be set to '1' and be followed by a set to '0'. If a clear/invalidation is required for an extended time period, the field should be set to '1' during the complete time period."]
pub type CLEAR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FREEZE` reader - When '1', hardware writes to the RX FIFO have no effect. Freeze will not advance the RX FIFO write pointer.This field is used only for debugging purposes."]
pub type FREEZE_R = crate::BitReader;
#[doc = "Field `FREEZE` writer - When '1', hardware writes to the RX FIFO have no effect. Freeze will not advance the RX FIFO write pointer.This field is used only for debugging purposes."]
pub type FREEZE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Trigger level. When the RX FIFO has more entries than the number of this field, a receiver trigger event is generated. Note: software can configure up to 254 in Mono channel enabled (MODE_CTL.PCM_CH_SET = '1' or '2'), up to 253 in Stereo channel enabled (MODE_CTL.PCM_CH_SET = '3')."]
    #[inline(always)]
    pub fn trigger_level(&self) -> TRIGGER_LEVEL_R {
        TRIGGER_LEVEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 16 - When '1', the RX FIFO and RX_BUF are cleared/invalidated. Invalidation will last for as long as this field is '1'. If a quick clear/invalidation is required, the field should be set to '1' and be followed by a set to '0'. If a clear/invalidation is required for an extended time period, the field should be set to '1' during the complete time period."]
    #[inline(always)]
    pub fn clear(&self) -> CLEAR_R {
        CLEAR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - When '1', hardware writes to the RX FIFO have no effect. Freeze will not advance the RX FIFO write pointer.This field is used only for debugging purposes."]
    #[inline(always)]
    pub fn freeze(&self) -> FREEZE_R {
        FREEZE_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Trigger level. When the RX FIFO has more entries than the number of this field, a receiver trigger event is generated. Note: software can configure up to 254 in Mono channel enabled (MODE_CTL.PCM_CH_SET = '1' or '2'), up to 253 in Stereo channel enabled (MODE_CTL.PCM_CH_SET = '3')."]
    #[inline(always)]
    #[must_use]
    pub fn trigger_level(&mut self) -> TRIGGER_LEVEL_W<RX_FIFO_CTL_SPEC> {
        TRIGGER_LEVEL_W::new(self, 0)
    }
    #[doc = "Bit 16 - When '1', the RX FIFO and RX_BUF are cleared/invalidated. Invalidation will last for as long as this field is '1'. If a quick clear/invalidation is required, the field should be set to '1' and be followed by a set to '0'. If a clear/invalidation is required for an extended time period, the field should be set to '1' during the complete time period."]
    #[inline(always)]
    #[must_use]
    pub fn clear(&mut self) -> CLEAR_W<RX_FIFO_CTL_SPEC> {
        CLEAR_W::new(self, 16)
    }
    #[doc = "Bit 17 - When '1', hardware writes to the RX FIFO have no effect. Freeze will not advance the RX FIFO write pointer.This field is used only for debugging purposes."]
    #[inline(always)]
    #[must_use]
    pub fn freeze(&mut self) -> FREEZE_W<RX_FIFO_CTL_SPEC> {
        FREEZE_W::new(self, 17)
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
#[doc = "RX FIFO control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_fifo_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_fifo_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_FIFO_CTL_SPEC;
impl crate::RegisterSpec for RX_FIFO_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_fifo_ctl::R`](R) reader structure"]
impl crate::Readable for RX_FIFO_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rx_fifo_ctl::W`](W) writer structure"]
impl crate::Writable for RX_FIFO_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RX_FIFO_CTL to value 0"]
impl crate::Resettable for RX_FIFO_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
