#[doc = "Register `ATT1` reader"]
pub type R = crate::R<ATT1_SPEC>;
#[doc = "Register `ATT1` writer"]
pub type W = crate::W<ATT1_SPEC>;
#[doc = "Field `UR` reader - User read enable: '0': Disabled (user, read accesses are NOT allowed). '1': Enabled (user, read accesses are allowed). Note that this register is constant '1'; i.e. user read accesses are ALWAYS allowed."]
pub type UR_R = crate::BitReader;
#[doc = "Field `UW` reader - User write enable: '0': Disabled (user, write accesses are NOT allowed). '1': Enabled (user, write accesses are allowed)."]
pub type UW_R = crate::BitReader;
#[doc = "Field `UW` writer - User write enable: '0': Disabled (user, write accesses are NOT allowed). '1': Enabled (user, write accesses are allowed)."]
pub type UW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UX` reader - User execute enable: '0': Disabled (user, execute accesses are NOT allowed). '1': Enabled (user, execute accesses are allowed). Note that this register is constant '0'; i.e. user execute accesses are NEVER allowed."]
pub type UX_R = crate::BitReader;
#[doc = "Field `PR` reader - Privileged read enable: '0': Disabled (privileged, read accesses are NOT allowed). '1': Enabled (privileged, read accesses are allowed). Note that this register is constant '1'; i.e. privileged read accesses are ALWAYS allowed."]
pub type PR_R = crate::BitReader;
#[doc = "Field `PW` reader - Privileged write enable: '0': Disabled (privileged, write accesses are NOT allowed). '1': Enabled (privileged, write accesses are allowed)."]
pub type PW_R = crate::BitReader;
#[doc = "Field `PW` writer - Privileged write enable: '0': Disabled (privileged, write accesses are NOT allowed). '1': Enabled (privileged, write accesses are allowed)."]
pub type PW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PX` reader - Privileged execute enable: '0': Disabled (privileged, execute accesses are NOT allowed). '1': Enabled (privileged, execute accesses are allowed). Note that this register is constant '0'; i.e. privileged execute accesses are NEVER allowed."]
pub type PX_R = crate::BitReader;
#[doc = "Field `NS` reader - Non-secure: '0': Secure (secure accesses allowed, non-secure access NOT allowed). '1': Non-secure (both secure and non-secure accesses allowed)."]
pub type NS_R = crate::BitReader;
#[doc = "Field `NS` writer - Non-secure: '0': Secure (secure accesses allowed, non-secure access NOT allowed). '1': Non-secure (both secure and non-secure accesses allowed)."]
pub type NS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC_MASK_0` reader - This field specifies protection context identifier based access control for protection context '0'."]
pub type PC_MASK_0_R = crate::BitReader;
#[doc = "Field `PC_MASK_15_TO_1` reader - This field specifies protection context identifier based access control. Bit i: protection context i+1 enable. If '0', protection context i+1 access is disabled; i.e. not allowed. If '1', protection context i+1 access is enabled; i.e. allowed."]
pub type PC_MASK_15_TO_1_R = crate::FieldReader<u16>;
#[doc = "Field `PC_MASK_15_TO_1` writer - This field specifies protection context identifier based access control. Bit i: protection context i+1 enable. If '0', protection context i+1 access is disabled; i.e. not allowed. If '1', protection context i+1 access is enabled; i.e. allowed."]
pub type PC_MASK_15_TO_1_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `REGION_SIZE` reader - This field specifies the region size: '7': 256 B region (8 32 B subregions) Note: this field is read-only."]
pub type REGION_SIZE_R = crate::FieldReader;
#[doc = "Field `PC_MATCH` reader - This field specifies if the PC field participates in the 'matching' process or the 'access evaluation' process: '0': PC field participates in 'access evaluation'. '1': PC field participates in 'matching'. 'Matching' process. For each protection structure, the process identifies if a transfer address is contained within the address range. This identifies the 'matching' regions. 'Access evaluation' process. For each protection structure, the process evaluates the bus transfer access attributes against the access control attributes. Note that it is possible to define different access control for multiple protection contexts by using multiple protection structures with the same address region and PC_MATCH set to '1'."]
pub type PC_MATCH_R = crate::BitReader;
#[doc = "Field `PC_MATCH` writer - This field specifies if the PC field participates in the 'matching' process or the 'access evaluation' process: '0': PC field participates in 'access evaluation'. '1': PC field participates in 'matching'. 'Matching' process. For each protection structure, the process identifies if a transfer address is contained within the address range. This identifies the 'matching' regions. 'Access evaluation' process. For each protection structure, the process evaluates the bus transfer access attributes against the access control attributes. Note that it is possible to define different access control for multiple protection contexts by using multiple protection structures with the same address region and PC_MATCH set to '1'."]
pub type PC_MATCH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLED` reader - Region enable: '0': Disabled. A disabled region will never result in a match on the bus transfer address. '1': Enabled."]
pub type ENABLED_R = crate::BitReader;
#[doc = "Field `ENABLED` writer - Region enable: '0': Disabled. A disabled region will never result in a match on the bus transfer address. '1': Enabled."]
pub type ENABLED_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - User read enable: '0': Disabled (user, read accesses are NOT allowed). '1': Enabled (user, read accesses are allowed). Note that this register is constant '1'; i.e. user read accesses are ALWAYS allowed."]
    #[inline(always)]
    pub fn ur(&self) -> UR_R {
        UR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - User write enable: '0': Disabled (user, write accesses are NOT allowed). '1': Enabled (user, write accesses are allowed)."]
    #[inline(always)]
    pub fn uw(&self) -> UW_R {
        UW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - User execute enable: '0': Disabled (user, execute accesses are NOT allowed). '1': Enabled (user, execute accesses are allowed). Note that this register is constant '0'; i.e. user execute accesses are NEVER allowed."]
    #[inline(always)]
    pub fn ux(&self) -> UX_R {
        UX_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Privileged read enable: '0': Disabled (privileged, read accesses are NOT allowed). '1': Enabled (privileged, read accesses are allowed). Note that this register is constant '1'; i.e. privileged read accesses are ALWAYS allowed."]
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Privileged write enable: '0': Disabled (privileged, write accesses are NOT allowed). '1': Enabled (privileged, write accesses are allowed)."]
    #[inline(always)]
    pub fn pw(&self) -> PW_R {
        PW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Privileged execute enable: '0': Disabled (privileged, execute accesses are NOT allowed). '1': Enabled (privileged, execute accesses are allowed). Note that this register is constant '0'; i.e. privileged execute accesses are NEVER allowed."]
    #[inline(always)]
    pub fn px(&self) -> PX_R {
        PX_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Non-secure: '0': Secure (secure accesses allowed, non-secure access NOT allowed). '1': Non-secure (both secure and non-secure accesses allowed)."]
    #[inline(always)]
    pub fn ns(&self) -> NS_R {
        NS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - This field specifies protection context identifier based access control for protection context '0'."]
    #[inline(always)]
    pub fn pc_mask_0(&self) -> PC_MASK_0_R {
        PC_MASK_0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:23 - This field specifies protection context identifier based access control. Bit i: protection context i+1 enable. If '0', protection context i+1 access is disabled; i.e. not allowed. If '1', protection context i+1 access is enabled; i.e. allowed."]
    #[inline(always)]
    pub fn pc_mask_15_to_1(&self) -> PC_MASK_15_TO_1_R {
        PC_MASK_15_TO_1_R::new(((self.bits >> 9) & 0x7fff) as u16)
    }
    #[doc = "Bits 24:28 - This field specifies the region size: '7': 256 B region (8 32 B subregions) Note: this field is read-only."]
    #[inline(always)]
    pub fn region_size(&self) -> REGION_SIZE_R {
        REGION_SIZE_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - This field specifies if the PC field participates in the 'matching' process or the 'access evaluation' process: '0': PC field participates in 'access evaluation'. '1': PC field participates in 'matching'. 'Matching' process. For each protection structure, the process identifies if a transfer address is contained within the address range. This identifies the 'matching' regions. 'Access evaluation' process. For each protection structure, the process evaluates the bus transfer access attributes against the access control attributes. Note that it is possible to define different access control for multiple protection contexts by using multiple protection structures with the same address region and PC_MATCH set to '1'."]
    #[inline(always)]
    pub fn pc_match(&self) -> PC_MATCH_R {
        PC_MATCH_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Region enable: '0': Disabled. A disabled region will never result in a match on the bus transfer address. '1': Enabled."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - User write enable: '0': Disabled (user, write accesses are NOT allowed). '1': Enabled (user, write accesses are allowed)."]
    #[inline(always)]
    #[must_use]
    pub fn uw(&mut self) -> UW_W<ATT1_SPEC> {
        UW_W::new(self, 1)
    }
    #[doc = "Bit 4 - Privileged write enable: '0': Disabled (privileged, write accesses are NOT allowed). '1': Enabled (privileged, write accesses are allowed)."]
    #[inline(always)]
    #[must_use]
    pub fn pw(&mut self) -> PW_W<ATT1_SPEC> {
        PW_W::new(self, 4)
    }
    #[doc = "Bit 6 - Non-secure: '0': Secure (secure accesses allowed, non-secure access NOT allowed). '1': Non-secure (both secure and non-secure accesses allowed)."]
    #[inline(always)]
    #[must_use]
    pub fn ns(&mut self) -> NS_W<ATT1_SPEC> {
        NS_W::new(self, 6)
    }
    #[doc = "Bits 9:23 - This field specifies protection context identifier based access control. Bit i: protection context i+1 enable. If '0', protection context i+1 access is disabled; i.e. not allowed. If '1', protection context i+1 access is enabled; i.e. allowed."]
    #[inline(always)]
    #[must_use]
    pub fn pc_mask_15_to_1(&mut self) -> PC_MASK_15_TO_1_W<ATT1_SPEC> {
        PC_MASK_15_TO_1_W::new(self, 9)
    }
    #[doc = "Bit 30 - This field specifies if the PC field participates in the 'matching' process or the 'access evaluation' process: '0': PC field participates in 'access evaluation'. '1': PC field participates in 'matching'. 'Matching' process. For each protection structure, the process identifies if a transfer address is contained within the address range. This identifies the 'matching' regions. 'Access evaluation' process. For each protection structure, the process evaluates the bus transfer access attributes against the access control attributes. Note that it is possible to define different access control for multiple protection contexts by using multiple protection structures with the same address region and PC_MATCH set to '1'."]
    #[inline(always)]
    #[must_use]
    pub fn pc_match(&mut self) -> PC_MATCH_W<ATT1_SPEC> {
        PC_MATCH_W::new(self, 30)
    }
    #[doc = "Bit 31 - Region enable: '0': Disabled. A disabled region will never result in a match on the bus transfer address. '1': Enabled."]
    #[inline(always)]
    #[must_use]
    pub fn enabled(&mut self) -> ENABLED_W<ATT1_SPEC> {
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
#[doc = "SMPU region attributes 1 (master structure)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`att1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`att1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ATT1_SPEC;
impl crate::RegisterSpec for ATT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`att1::R`](R) reader structure"]
impl crate::Readable for ATT1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`att1::W`](W) writer structure"]
impl crate::Writable for ATT1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ATT1 to value 0x0700_0109"]
impl crate::Resettable for ATT1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0700_0109;
}
