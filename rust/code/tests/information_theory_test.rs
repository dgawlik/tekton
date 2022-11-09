
struct Histogram<const F:usize>{
    bins: [u32; F],
    milestones: [u128; F]
}

impl<const F:usize> Histogram<F> {

    // pub fn new(max: BigUint) -> Histogram<F> {

    // }
}