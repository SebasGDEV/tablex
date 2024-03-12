use crate::{utils::Drivers, DbInstance};
use tauri::State;

pub async fn establish_connection(
    state: &State<'_, DbInstance>,
    conn_string: String,
    driver: Drivers,
) -> Result<(), String> {
    //println!("MSSQL Connection");

    let pool = deadpool_tiberius::Manager::from_ado_string(&conn_string)
        .unwrap()
        .create_pool()
        .map_err(|_| "Couldn't establish connection to db".to_string())?;

    *state.mssql_pool.lock().await = Some(pool);
    *state.driver.lock().await = Some(driver);

    #[cfg(not(debug_assertions))]
    {
        use tauri::api::process::Command;

        let (_, child) = Command::new_sidecar("meta-x")
            .expect("failed to create `meta-x` binary command")
            .args(["pg", "--url", conn_string.as_str()])
            .spawn()
            .expect("failed to spawn sidecar");
        *state.metax_command_child.lock().await = Some(child);
    }
    core::result::Result::Ok(())
}
