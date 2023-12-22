#[doc = "Register `ARB_CFG` reader"]
pub type R = crate::R<ARB_CFG_SPEC>;
#[doc = "Register `ARB_CFG` writer"]
pub type W = crate::W<ARB_CFG_SPEC>;
#[doc = "Field `AUTO_MEM` reader - Enables Auto Memory Configuration. Manual memory configuration by default."]
pub type AUTO_MEM_R = crate::BitReader;
#[doc = "Field `AUTO_MEM` writer - Enables Auto Memory Configuration. Manual memory configuration by default."]
pub type AUTO_MEM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_CFG` reader - DMA Access Configuration."]
pub type DMA_CFG_R = crate::FieldReader<DMA_CFG_A>;
#[doc = "DMA Access Configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMA_CFG_A {
    #[doc = "0: No DMA"]
    DMA_NONE = 0,
    #[doc = "1: Manual DMA"]
    DMA_MANUAL = 1,
    #[doc = "2: Auto DMA"]
    DMA_AUTO = 2,
}
impl From<DMA_CFG_A> for u8 {
    #[inline(always)]
    fn from(variant: DMA_CFG_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DMA_CFG_A {
    type Ux = u8;
}
impl DMA_CFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DMA_CFG_A> {
        match self.bits {
            0 => Some(DMA_CFG_A::DMA_NONE),
            1 => Some(DMA_CFG_A::DMA_MANUAL),
            2 => Some(DMA_CFG_A::DMA_AUTO),
            _ => None,
        }
    }
    #[doc = "No DMA"]
    #[inline(always)]
    pub fn is_dma_none(&self) -> bool {
        *self == DMA_CFG_A::DMA_NONE
    }
    #[doc = "Manual DMA"]
    #[inline(always)]
    pub fn is_dma_manual(&self) -> bool {
        *self == DMA_CFG_A::DMA_MANUAL
    }
    #[doc = "Auto DMA"]
    #[inline(always)]
    pub fn is_dma_auto(&self) -> bool {
        *self == DMA_CFG_A::DMA_AUTO
    }
}
#[doc = "Field `DMA_CFG` writer - DMA Access Configuration."]
pub type DMA_CFG_W<'a, REG> = crate::FieldWriter<'a, REG, 2, DMA_CFG_A>;
impl<'a, REG> DMA_CFG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No DMA"]
    #[inline(always)]
    pub fn dma_none(self) -> &'a mut crate::W<REG> {
        self.variant(DMA_CFG_A::DMA_NONE)
    }
    #[doc = "Manual DMA"]
    #[inline(always)]
    pub fn dma_manual(self) -> &'a mut crate::W<REG> {
        self.variant(DMA_CFG_A::DMA_MANUAL)
    }
    #[doc = "Auto DMA"]
    #[inline(always)]
    pub fn dma_auto(self) -> &'a mut crate::W<REG> {
        self.variant(DMA_CFG_A::DMA_AUTO)
    }
}
#[doc = "Field `CFG_CMP` reader - Register Configuration Complete Indication. Posedge is detected on this bit. Hence a 0 to 1 transition is required."]
pub type CFG_CMP_R = crate::BitReader;
#[doc = "Field `CFG_CMP` writer - Register Configuration Complete Indication. Posedge is detected on this bit. Hence a 0 to 1 transition is required."]
pub type CFG_CMP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - Enables Auto Memory Configuration. Manual memory configuration by default."]
    #[inline(always)]
    pub fn auto_mem(&self) -> AUTO_MEM_R {
        AUTO_MEM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - DMA Access Configuration."]
    #[inline(always)]
    pub fn dma_cfg(&self) -> DMA_CFG_R {
        DMA_CFG_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Register Configuration Complete Indication. Posedge is detected on this bit. Hence a 0 to 1 transition is required."]
    #[inline(always)]
    pub fn cfg_cmp(&self) -> CFG_CMP_R {
        CFG_CMP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Enables Auto Memory Configuration. Manual memory configuration by default."]
    #[inline(always)]
    #[must_use]
    pub fn auto_mem(&mut self) -> AUTO_MEM_W<ARB_CFG_SPEC> {
        AUTO_MEM_W::new(self, 4)
    }
    #[doc = "Bits 5:6 - DMA Access Configuration."]
    #[inline(always)]
    #[must_use]
    pub fn dma_cfg(&mut self) -> DMA_CFG_W<ARB_CFG_SPEC> {
        DMA_CFG_W::new(self, 5)
    }
    #[doc = "Bit 7 - Register Configuration Complete Indication. Posedge is detected on this bit. Hence a 0 to 1 transition is required."]
    #[inline(always)]
    #[must_use]
    pub fn cfg_cmp(&mut self) -> CFG_CMP_W<ARB_CFG_SPEC> {
        CFG_CMP_W::new(self, 7)
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
#[doc = "Arbiter Configuration Register *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ARB_CFG_SPEC;
impl crate::RegisterSpec for ARB_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`arb_cfg::R`](R) reader structure"]
impl crate::Readable for ARB_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`arb_cfg::W`](W) writer structure"]
impl crate::Writable for ARB_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ARB_CFG to value 0"]
impl crate::Resettable for ARB_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
