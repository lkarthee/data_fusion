use datafusion::prelude::*;
use tokio::runtime::Runtime;
use crate::{FzDataFrame, FusionError};

pub mod io;

#[rustler::nif]
pub fn df_names(df: FzDataFrame) ->  Result<Vec<String>, FusionError>
{
    // let rt = Runtime::new().unwrap();

    // let result =  rt.block_on().unwrap();
    let names = df.clone().schema().fields().into_iter().map(|f| f.name().to_string()).collect();
    Ok(names)
}

#[rustler::nif]
pub fn df_n_rows(df: FzDataFrame) -> Result<usize, FusionError> {
    let rt = Runtime::new().unwrap();
    let result =  rt.block_on(df.clone().count()).unwrap();
    Ok(result)
}