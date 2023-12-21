# [doc = "Register `DMA_THRES16` reader"] pub type R = crate :: R < DMA_THRES16_SPEC > ; # [doc = "Register `DMA_THRES16` writer"] pub type W = crate :: W < DMA_THRES16_SPEC > ; # [doc = "Field `DMA_THS16` reader - DMA Threshold count"] pub type DMA_THS16_R = crate :: FieldReader < u16 > ; # [doc = "Field `DMA_THS16` writer - DMA Threshold count"] pub type DMA_THS16_W < 'a , REG > = crate :: FieldWriter < 'a , REG , 9 , u16 > ; impl R { # [doc = "Bits 0:8 - DMA Threshold count"] # [inline (always)] pub fn dma_ths16 (& self) -> DMA_THS16_R { DMA_THS16_R :: new ((self . bits & 0x01ff) as u16) } } impl W { # [doc = "Bits 0:8 - DMA Threshold count"] # [inline (always)] # [must_use] pub fn dma_ths16 (& mut self) -> DMA_THS16_W < DMA_THRES16_SPEC > { DMA_THS16_W :: new (self , 0) } # [doc = r" Writes raw bits to the register."] # [doc = r""] # [doc = r" # Safety"] # [doc = r""] # [doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . bits = bits ; self } } # [doc = "DMA Burst / Threshold Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_thres16::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_thres16::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct DMA_THRES16_SPEC ; impl crate :: RegisterSpec for DMA_THRES16_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [`dma_thres16::R`](R) reader structure"] impl crate :: Readable for DMA_THRES16_SPEC { } # [doc = "`write(|w| ..)` method takes [`dma_thres16::W`](W) writer structure"] impl crate :: Writable for DMA_THRES16_SPEC { const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets DMA_THRES16 to value 0"] impl crate :: Resettable for DMA_THRES16_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }