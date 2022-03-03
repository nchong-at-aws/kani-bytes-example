use bytes::BytesMut;

#[cfg(kani)]
mod verification {
    use super::*;

    // cargo kani --function bounded_reserve --unwind 5
    // TIMEOUT
    #[kani::proof]
    fn bounded_reserve() {
        // this is what we get from kani::any_slice()
        let arr: [u8; 4] = kani::any();
        let len: usize = kani::any();
        kani::assume(len <= 4);

        // additional assumption to remove 0 case
        kani::assume(0 < len);

        // now setup and call function under analysis
        let mut buf = BytesMut::from(&arr[..len]);
        let additional = 1;
        buf.reserve(additional);
    }

    // cargo kani --function bounded_reserve_case_split --unwind 5
    // PASS
    #[kani::proof]
    fn bounded_reserve_case_split() {
        let arr: [u8; 4] = kani::any();
        let mut buf = match kani::any() {
            1 => BytesMut::from(&arr[..1]),
            2 => BytesMut::from(&arr[..2]),
            3 => BytesMut::from(&arr[..3]),
            _ => BytesMut::from(&arr[..4]),
        };
        let additional = 1;
        buf.reserve(additional);
    }

    // cargo kani --function bounded_reserve_case_split_hoist --unwind 5
    // TIMEOUT
    #[kani::proof]
    fn bounded_reserve_case_split_hoist() {
        let arr: [u8; 4] = kani::any();
        let mut buf = BytesMut::from(match kani::any() {
            1 => &arr[..1],
            2 => &arr[..2],
            3 => &arr[..3],
            _ => &arr[..4],
        });
        let additional = 1;
        buf.reserve(additional);
    }
}
