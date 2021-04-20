// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use pyo3::prelude::*;
use std::default::Default;
use topi::{topi_base::data::c_api, Config, TopiNetworkWithMessenger, TopiNodeConfig};
use topi_node_python::data::*;

#[pyclass]
struct _TopiNetworkWithMessenger {
    inner: TopiNetworkWithMessenger<c_api::Messenger>,
}

#[pymethods]
impl _TopiNetworkWithMessenger {
    #[new]
    fn from_srcs(
        messenger_id: &str,
        node_configs: Vec<(i64, &str, &str)>,
        global_config: Option<&str>,
    ) -> PyResult<Self> {
        let global = if let Some(global) = global_config {
            serde_json::from_str(global)
                .map_err(|err| pyo3::exceptions::PyException::new_err(format!("{:?}", err)))?
        } else {
            topi::config_parse::GlobalConfig::default()
        };
        let node_config = node_configs.into_iter().map(|(proc_id, node_id, src)| {
            let cfg = Config::Python {
                source: String::from(src),
                must_be_main_thread: true,
            };
            Ok(TopiNodeConfig::from_config((proc_id, node_id), cfg))
        });
        let inner = TopiNetworkWithMessenger::from_node_config(global, node_config, messenger_id)
            .map_err(topi_err_to_py_err)?;
        Ok(Self { inner })
    }

    #[allow(clippy::unnecessary_wraps)]
    fn messenger(&mut self) -> PyResult<_Messenger> {
        Ok(_Messenger::from_messenger(
            self.inner.messenger() as *const _
        ))
    }
}

fn topi_err_to_py_err(err: topi::Error) -> PyErr {
    pyo3::exceptions::PyException::new_err(format!("{:?}", err))
}

#[pymodule]
fn pytopi(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<_TopiNetworkWithMessenger>()?;
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
