// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use pyo3::prelude::*;
use topi_proc::{main_, main_test_, topi_node_python::data::*};

#[pymodule]
pub fn pytopi_proc(_py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m, "_main")]
    fn main_py(_py: Python) {
        main_()
    }

    #[pyfn(m, "_main_test")]
    fn main_test_py(_py: Python) {
        main_test_()
    }

    m.add_class::<_Messenger>()?;
    m.add_class::<_SharedArrayMut_bool>()?;
    m.add_class::<_SharedArrayMut_u8>()?;
    m.add_class::<_SharedArrayMut_u16>()?;
    m.add_class::<_SharedArrayMut_u32>()?;
    m.add_class::<_SharedArrayMut_u64>()?;
    m.add_class::<_SharedArrayMut_i8>()?;
    m.add_class::<_SharedArrayMut_i16>()?;
    m.add_class::<_SharedArrayMut_i32>()?;
    m.add_class::<_SharedArrayMut_i64>()?;
    m.add_class::<_SharedArrayMut_f32>()?;
    m.add_class::<_SharedArrayMut_f64>()?;

    Ok(())
}
