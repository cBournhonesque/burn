use cubecl::cube;
use cubecl::frontend::{ABSOLUTE_POS, Tensor, UInt, F32, Float};
use cubecl::prelude::{Cast, Numeric};
use crate::kernel::reduce::Argmax;
use super::base::ReduceDimNaive;


#[cube]
impl<EI: Numeric, EO: Numeric> ReduceDimNaive<EI, EO> for Argmax {

    type Accumulator = (F32, UInt);

    fn initialize_naive() -> (F32, UInt) {
        // (F32::new(f32::NEG_INFINITY), UInt::new(0))
        let a = F32::new(0.0);
        let b = F32::new(1000000.0);
        (a-b, UInt::new(0))
    }

    fn inner_loop_naive(
        accumulator: &mut (F32, UInt),
        current_value: EI,
        i: UInt,
    ) {
        let (max, index) = accumulator;
        let val = F32::cast_from(current_value);
        if val > *max {
            *max = val;
            *index = i;
        }
    }

    fn assign_naive(
        output: &mut Tensor<EO>,
        accumulator: (F32, UInt),
        _shape_reduce_dim: UInt,
    ) {
        let (_, index) = accumulator;
        output[ABSOLUTE_POS] = EO::cast_from(index);
    }
}
