#[doc = "Register `ATT` reader"]
pub type R = crate::R<ATT_SPEC>;
#[doc = "Register `ATT` writer"]
pub type W = crate::W<ATT_SPEC>;
#[doc = "Field `UR` reader - User read enable: '0': Disabled (user, read accesses are NOT allowed). '1': Enabled (user, read accesses are allowed)."]
pub type UR_R = crate::BitReader;
#[doc = "Field `UR` writer - User read enable: '0': Disabled (user, read accesses are NOT allowed). '1': Enabled (user, read accesses are allowed)."]
pub type UR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UW` reader - User write enable: '0': Disabled (user, write accesses are NOT allowed). '1': Enabled (user, write accesses are allowed)."]
pub type UW_R = crate::BitReader;
#[doc = "Field `UW` writer - User write enable: '0': Disabled (user, write accesses are NOT allowed). '1': Enabled (user, write accesses are allowed)."]
pub type UW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UX` reader - User execute enable: '0': Disabled (user, execute accesses are NOT allowed). '1': Enabled (user, execute accesses are allowed)."]
pub type UX_R = crate::BitReader;
#[doc = "Field `UX` writer - User execute enable: '0': Disabled (user, execute accesses are NOT allowed). '1': Enabled (user, execute accesses are allowed)."]
pub type UX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR` reader - Privileged read enable: '0': Disabled (privileged, read accesses are NOT allowed). '1': Enabled (privileged, read accesses are allowed)."]
pub type PR_R = crate::BitReader;
#[doc = "Field `PR` writer - Privileged read enable: '0': Disabled (privileged, read accesses are NOT allowed). '1': Enabled (privileged, read accesses are allowed)."]
pub type PR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PW` reader - Privileged write enable: '0': Disabled (privileged, write accesses are NOT allowed). '1': Enabled (privileged, write accesses are allowed)."]
pub type PW_R = crate::BitReader;
#[doc = "Field `PW` writer - Privileged write enable: '0': Disabled (privileged, write accesses are NOT allowed). '1': Enabled (privileged, write accesses are allowed)."]
pub type PW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PX` reader - Privileged execute enable: '0': Disabled (privileged, execute accesses are NOT allowed). '1': Enabled (privileged, execute accesses are allowed)."]
pub type PX_R = crate::BitReader;
#[doc = "Field `PX` writer - Privileged execute enable: '0': Disabled (privileged, execute accesses are NOT allowed). '1': Enabled (privileged, execute accesses are allowed)."]
pub type PX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NS` reader - Non-secure: '0': Secure (secure accesses allowed, non-secure access NOT allowed). '1': Non-secure (both secure and non-secure accesses allowed)."]
pub type NS_R = crate::BitReader;
#[doc = "Field `NS` writer - Non-secure: '0': Secure (secure accesses allowed, non-secure access NOT allowed). '1': Non-secure (both secure and non-secure accesses allowed)."]
pub type NS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGION_SIZE` reader - This field specifies the region size: '0'-'6': Undefined. '7': 256 B region '8': 512 B region '9': 1 KB region '10': 2 KB region '11': 4 KB region '12': 8 KB region '13': 16 KB region '14': 32 KB region '15': 64 KB region '16': 128 KB region '17': 256 KB region '18': 512 KB region '19': 1 MB region '20': 2 MB region '21': 4 MB region '22': 8 MB region '23': 16 MB region '24': 32 MB region '25': 64 MB region '26': 128 MB region '27': 256 MB region '28': 512 MB region '39': 1 GB region '30': 2 GB region '31': 4 GB region"]
pub type REGION_SIZE_R = crate::FieldReader;
#[doc = "Field `REGION_SIZE` writer - This field specifies the region size: '0'-'6': Undefined. '7': 256 B region '8': 512 B region '9': 1 KB region '10': 2 KB region '11': 4 KB region '12': 8 KB region '13': 16 KB region '14': 32 KB region '15': 64 KB region '16': 128 KB region '17': 256 KB region '18': 512 KB region '19': 1 MB region '20': 2 MB region '21': 4 MB region '22': 8 MB region '23': 16 MB region '24': 32 MB region '25': 64 MB region '26': 128 MB region '27': 256 MB region '28': 512 MB region '39': 1 GB region '30': 2 GB region '31': 4 GB region"]
pub type REGION_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ENABLED` reader - Region enable: '0': Disabled. A disabled region will never result in a match on the bus transfer address. '1': Enabled. Note: a disabled address region performs logic gating to reduce dynamic power consumption."]
pub type ENABLED_R = crate::BitReader;
#[doc = "Field `ENABLED` writer - Region enable: '0': Disabled. A disabled region will never result in a match on the bus transfer address. '1': Enabled. Note: a disabled address region performs logic gating to reduce dynamic power consumption."]
pub type ENABLED_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - User read enable: '0': Disabled (user, read accesses are NOT allowed). '1': Enabled (user, read accesses are allowed)."]
    #[inline(always)]
    pub fn ur(&self) -> UR_R {
        UR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - User write enable: '0': Disabled (user, write accesses are NOT allowed). '1': Enabled (user, write accesses are allowed)."]
    #[inline(always)]
    pub fn uw(&self) -> UW_R {
        UW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - User execute enable: '0': Disabled (user, execute accesses are NOT allowed). '1': Enabled (user, execute accesses are allowed)."]
    #[inline(always)]
    pub fn ux(&self) -> UX_R {
        UX_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Privileged read enable: '0': Disabled (privileged, read accesses are NOT allowed). '1': Enabled (privileged, read accesses are allowed)."]
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Privileged write enable: '0': Disabled (privileged, write accesses are NOT allowed). '1': Enabled (privileged, write accesses are allowed)."]
    #[inline(always)]
    pub fn pw(&self) -> PW_R {
        PW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Privileged execute enable: '0': Disabled (privileged, execute accesses are NOT allowed). '1': Enabled (privileged, execute accesses are allowed)."]
    #[inline(always)]
    pub fn px(&self) -> PX_R {
        PX_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Non-secure: '0': Secure (secure accesses allowed, non-secure access NOT allowed). '1': Non-secure (both secure and non-secure accesses allowed)."]
    #[inline(always)]
    pub fn ns(&self) -> NS_R {
        NS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 24:28 - This field specifies the region size: '0'-'6': Undefined. '7': 256 B region '8': 512 B region '9': 1 KB region '10': 2 KB region '11': 4 KB region '12': 8 KB region '13': 16 KB region '14': 32 KB region '15': 64 KB region '16': 128 KB region '17': 256 KB region '18': 512 KB region '19': 1 MB region '20': 2 MB region '21': 4 MB region '22': 8 MB region '23': 16 MB region '24': 32 MB region '25': 64 MB region '26': 128 MB region '27': 256 MB region '28': 512 MB region '39': 1 GB region '30': 2 GB region '31': 4 GB region"]
    #[inline(always)]
    pub fn region_size(&self) -> REGION_SIZE_R {
        REGION_SIZE_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Region enable: '0': Disabled. A disabled region will never result in a match on the bus transfer address. '1': Enabled. Note: a disabled address region performs logic gating to reduce dynamic power consumption."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - User read enable: '0': Disabled (user, read accesses are NOT allowed). '1': Enabled (user, read accesses are allowed)."]
    #[inline(always)]
    #[must_use]
    pub fn ur(&mut self) -> UR_W<ATT_SPEC> {
        UR_W::new(self, 0)
    }
    #[doc = "Bit 1 - User write enable: '0': Disabled (user, write accesses are NOT allowed). '1': Enabled (user, write accesses are allowed)."]
    #[inline(always)]
    #[must_use]
    pub fn uw(&mut self) -> UW_W<ATT_SPEC> {
        UW_W::new(self, 1)
    }
    #[doc = "Bit 2 - User execute enable: '0': Disabled (user, execute accesses are NOT allowed). '1': Enabled (user, execute accesses are allowed)."]
    #[inline(always)]
    #[must_use]
    pub fn ux(&mut self) -> UX_W<ATT_SPEC> {
        UX_W::new(self, 2)
    }
    #[doc = "Bit 3 - Privileged read enable: '0': Disabled (privileged, read accesses are NOT allowed). '1': Enabled (privileged, read accesses are allowed)."]
    #[inline(always)]
    #[must_use]
    pub fn pr(&mut self) -> PR_W<ATT_SPEC> {
        PR_W::new(self, 3)
    }
    #[doc = "Bit 4 - Privileged write enable: '0': Disabled (privileged, write accesses are NOT allowed). '1': Enabled (privileged, write accesses are allowed)."]
    #[inline(always)]
    #[must_use]
    pub fn pw(&mut self) -> PW_W<ATT_SPEC> {
        PW_W::new(self, 4)
    }
    #[doc = "Bit 5 - Privileged execute enable: '0': Disabled (privileged, execute accesses are NOT allowed). '1': Enabled (privileged, execute accesses are allowed)."]
    #[inline(always)]
    #[must_use]
    pub fn px(&mut self) -> PX_W<ATT_SPEC> {
        PX_W::new(self, 5)
    }
    #[doc = "Bit 6 - Non-secure: '0': Secure (secure accesses allowed, non-secure access NOT allowed). '1': Non-secure (both secure and non-secure accesses allowed)."]
    #[inline(always)]
    #[must_use]
    pub fn ns(&mut self) -> NS_W<ATT_SPEC> {
        NS_W::new(self, 6)
    }
    #[doc = "Bits 24:28 - This field specifies the region size: '0'-'6': Undefined. '7': 256 B region '8': 512 B region '9': 1 KB region '10': 2 KB region '11': 4 KB region '12': 8 KB region '13': 16 KB region '14': 32 KB region '15': 64 KB region '16': 128 KB region '17': 256 KB region '18': 512 KB region '19': 1 MB region '20': 2 MB region '21': 4 MB region '22': 8 MB region '23': 16 MB region '24': 32 MB region '25': 64 MB region '26': 128 MB region '27': 256 MB region '28': 512 MB region '39': 1 GB region '30': 2 GB region '31': 4 GB region"]
    #[inline(always)]
    #[must_use]
    pub fn region_size(&mut self) -> REGION_SIZE_W<ATT_SPEC> {
        REGION_SIZE_W::new(self, 24)
    }
    #[doc = "Bit 31 - Region enable: '0': Disabled. A disabled region will never result in a match on the bus transfer address. '1': Enabled. Note: a disabled address region performs logic gating to reduce dynamic power consumption."]
    #[inline(always)]
    #[must_use]
    pub fn enabled(&mut self) -> ENABLED_W<ATT_SPEC> {
        ENABLED_W::new(self, 31)
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
#[doc = "MPU region attrributes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`att::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`att::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ATT_SPEC;
impl crate::RegisterSpec for ATT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`att::R`](R) reader structure"]
impl crate::Readable for ATT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`att::W`](W) writer structure"]
impl crate::Writable for ATT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ATT to value 0"]
impl crate::Resettable for ATT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
