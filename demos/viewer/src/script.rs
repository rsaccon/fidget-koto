use anyhow::Result;
use crossbeam_channel::{Receiver, Sender};
use log::debug;

use fidget_koto::{Engine, ScriptContext};

/// Receives scripts and executes them with Fidget
pub(crate) fn koto_script_thread(
    rx: Receiver<String>,
    tx: Sender<Result<ScriptContext, String>>,
) -> Result<()> {
    let mut engine = Engine::default();
    loop {
        let script = rx.recv()?;
        debug!("koto script thread received script");
        let r = engine.run(&script).map_err(|e| e.to_string());
        debug!("koto script thread is sending result to render thread");
        tx.send(r)?;
    }
}
