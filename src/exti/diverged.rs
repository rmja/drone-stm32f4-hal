use drone_cortexm::reg::prelude::*;
use drone_stm32_map::periph::exti::{
    ExtiFtsrFt, ExtiMap, ExtiPeriph, ExtiPrPif, ExtiRtsrRt, ExtiSwierSwi, SyscfgExticrExti,
};

#[allow(dead_code)]
pub(crate) struct ExtiDiverged<
    Exti: ExtiMap + SyscfgExticrExti + ExtiRtsrRt + ExtiFtsrFt + ExtiSwierSwi + ExtiPrPif,
> {
    pub(crate) syscfg_exticr_exti: Exti::SSyscfgExticrExti,
    pub(crate) exti_imr_im: Exti::SExtiImrIm,
    pub(crate) exti_emr_em: Exti::SExtiEmrEm,
    pub(crate) exti_rtsr_rt: Exti::SExtiRtsrRt,
    pub(crate) exti_ftsr_ft: Exti::SExtiFtsrFt,
    pub(crate) exti_swier_swi: Exti::SExtiSwierSwi,
    pub(crate) exti_pr_pif: Exti::CExtiPrPif,
}

impl<Exti: ExtiMap + SyscfgExticrExti + ExtiRtsrRt + ExtiFtsrFt + ExtiSwierSwi + ExtiPrPif>
    From<ExtiPeriph<Exti>> for ExtiDiverged<Exti>
{
    fn from(periph: ExtiPeriph<Exti>) -> Self {
        let ExtiPeriph {
            syscfg_exticr_exti,
            exti_imr_im,
            exti_emr_em,
            exti_rtsr_rt,
            exti_ftsr_ft,
            exti_swier_swi,
            exti_pr_pif,
        } = periph;
        Self {
            syscfg_exticr_exti,
            exti_imr_im,
            exti_emr_em,
            exti_rtsr_rt,
            exti_ftsr_ft,
            exti_swier_swi,
            exti_pr_pif: exti_pr_pif.into_copy(),
        }
    }
}
