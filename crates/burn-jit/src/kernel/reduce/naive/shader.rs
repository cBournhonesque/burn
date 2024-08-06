use cubecl::prelude::*;
use crate::{element::JitElement, tensor::JitTensor, JitRuntime};

use super::base::ReduceDimNaive;

#[cube(launch)]
pub(crate) fn naive_reduce_dim_compute_shader<RD: ReduceDimNaive<EI, EO>, EI: Numeric, EO: Numeric>(
    input: &Tensor<EI>,
    output: &mut Tensor<EO>,
    dim: UInt,
) {
    if ABSOLUTE_POS >= output.len() {
        return;
    }

    let mut offset_input = UInt::new(0);

    for i in range(0, input.rank(), Comptime::new(false)) {
        let mut offset_local = ABSOLUTE_POS / output.stride(i);
        offset_local = offset_local % output.shape(i);
        if i != dim {
            offset_input += offset_local * input.stride(i);
        }
    }

    let mut accumulator = RD::initialize_naive();

    for i in range(0, input.shape(dim), Comptime::new(false)) {
        let index = i * input.stride(dim) + offset_input;
        RD::inner_loop_naive(
            &mut accumulator,
            input[index],
            dim,
        );
    }

    RD::assign_naive(output, accumulator, input.shape(dim));
}


/// Executes the naive kernel for reduce dim
pub fn reduce_dim_naive<
    RD: ReduceDimNaive<EI::Primitive, EO::Primitive>,
    R: JitRuntime,
    EI: JitElement,
    EO: JitElement,
    const D: usize,
>(
    input: JitTensor<R, EI, D>,
    output: JitTensor<R, EO, D>,
    dim: usize,
) -> JitTensor<R, EO, D> {
    let cube_dim = CubeDim::default();
    let cube_count = CubeCount::Static(1, 1, 1);

    naive_reduce_dim_compute_shader::launch::<RD, EI::Primitive, EO::Primitive, R>(
        &input.client,
        cube_count,
        cube_dim,
        TensorArg::new(&input.handle, &input.strides, &input.shape.dims),
        TensorArg::new(&output.handle, &output.strides, &output.shape.dims),
        ScalarArg::new(dim as u32),
    );

    output
}