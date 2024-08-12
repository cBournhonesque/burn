use cubecl::cube;
use crate::{kernel::reduce::ProdDim};
use cubecl::prelude::{ABSOLUTE_POS, Cast, Numeric, Tensor, UInt};

use super::base::ReduceDimNaive;

#[cube]
impl<EI: Numeric, EO: Numeric> ReduceDimNaive<EI, EO> for ProdDim {
    type Accumulator = EI;

    fn initialize_naive() -> EI {
        EI::from_int(1)
    }

    fn inner_loop_naive(
        accumulator: &mut EI,
        current_value: EI,
        _i: UInt,
    ) {
        *accumulator *= current_value;
    }

    fn assign_naive(
        output: &mut Tensor<EO>,
        accumulator: EI,
        _shape_reduce_dim: UInt,
    ) {
        output[ABSOLUTE_POS] = EO::cast_from(accumulator);
    }
}
