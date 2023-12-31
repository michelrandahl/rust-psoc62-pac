#[doc = "Register `GP_IN_R` reader"]
pub type R = crate::R<GP_IN_R_SPEC>;
#[doc = "Field `GP_IN` reader - It reflects the value of gp_in ports. NOT USED - ALWAYS READS 0"]
pub type GP_IN_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - It reflects the value of gp_in ports. NOT USED - ALWAYS READS 0"]
    #[inline(always)]
    pub fn gp_in(&self) -> GP_IN_R {
        GP_IN_R::new((self.bits & 1) != 0)
    }
}
#[doc = "General Purpose Input register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gp_in_r::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GP_IN_R_SPEC;
impl crate::RegisterSpec for GP_IN_R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gp_in_r::R`](R) reader structure"]
impl crate::Readable for GP_IN_R_SPEC {}
#[doc = "`reset()` method sets GP_IN_R to value 0"]
impl crate::Resettable for GP_IN_R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
