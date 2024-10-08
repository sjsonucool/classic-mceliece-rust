//! Syndrome computation

use crate::gf::{gf_add, gf_inv, gf_mul, Gf};
use crate::params::{SYS_N, SYS_T};
use crate::root::eval;

/// Given Goppa polynomial `f`, support `l`, and received word `r`
/// compute `out`, the syndrome of length 2t
pub(crate) fn synd(
    out: &mut [Gf; SYS_T * 2],
    f: &[Gf; SYS_T + 1],
    l: &[Gf; SYS_N],
    r: &[u8; SYS_N / 8],
) {
    out[0..2 * SYS_T].fill(0);

    for i in 0..SYS_N {
        let c: Gf = (r[i / 8] >> (i % 8)) as u16 & 1;
        let e: Gf = eval(f, l[i]);
        let mut e_inv: Gf = gf_inv(gf_mul(e, e));

        for itr_out in out.iter_mut() {
            *itr_out = gf_add(*itr_out, gf_mul(e_inv, c));
            e_inv = gf_mul(e_inv, l[i]);
        }
    }
}
