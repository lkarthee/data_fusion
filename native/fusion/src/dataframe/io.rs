use datafusion::prelude::*;
use tokio::runtime::Runtime;
use crate::{FzDataFrame, FusionError};

#[rustler::nif(schedule = "DirtyIo")]
pub fn df_from_csv(
    filename: &str) -> Result<FzDataFrame, FusionError> 
{
  let rt = Runtime::new().unwrap();
  let ctx = SessionContext::new();
  let csv = ctx.read_csv(filename, CsvReadOptions::new());
  let df =  rt.block_on(csv).unwrap();
  Ok(FzDataFrame::new(df))
}

#[rustler::nif(schedule = "DirtyIo")]
pub fn df_from_parquet(
    filename: &str) -> Result<FzDataFrame, FusionError> 
{
  let rt = Runtime::new().unwrap();
  let ctx = SessionContext::new();
  let csv = ctx.read_parquet(filename, ParquetReadOptions::default());
  let df =  rt.block_on(csv).unwrap();
  Ok(FzDataFrame::new(df))
}