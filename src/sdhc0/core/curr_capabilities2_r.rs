#[doc = "Register `CURR_CAPABILITIES2_R` reader"]
pub type R = crate::R<CURR_CAPABILITIES2_R_SPEC>;
#[doc = "Field `MAX_CUR_VDD2_18V` reader - Maximum Current for 1.8V VDD2 This bit specifies the Maximum Current for 1.8V VDD2 power supply for the UHS-II card. - 0: Get information through another method - 1: 4mA - 2: 8mA - 3: 13mA - ....... - 255: 1020mA"]
pub type MAX_CUR_VDD2_18V_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Maximum Current for 1.8V VDD2 This bit specifies the Maximum Current for 1.8V VDD2 power supply for the UHS-II card. - 0: Get information through another method - 1: 4mA - 2: 8mA - 3: 13mA - ....... - 255: 1020mA"]
    #[inline(always)]
    pub fn max_cur_vdd2_18v(&self) -> MAX_CUR_VDD2_18V_R {
        MAX_CUR_VDD2_18V_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Maximum Current Capabilities Register - 32 to 63\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`curr_capabilities2_r::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CURR_CAPABILITIES2_R_SPEC;
impl crate::RegisterSpec for CURR_CAPABILITIES2_R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`curr_capabilities2_r::R`](R) reader structure"]
impl crate::Readable for CURR_CAPABILITIES2_R_SPEC {}
#[doc = "`reset()` method sets CURR_CAPABILITIES2_R to value 0"]
impl crate::Resettable for CURR_CAPABILITIES2_R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
