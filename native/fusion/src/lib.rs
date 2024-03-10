use rustler::{Env, Term};

mod dataframe;
mod datatypes;
mod error;
// mod expression;

use dataframe::io::*;
use dataframe::*;

pub use datatypes::{
   FzDataFrame,  FzDataFrameRef, FzExpr, FzExprRef
};

pub use error::FusionError;
// use expression::*;

fn on_load(env: Env, _info: Term) -> bool {
    rustler::resource!(FzDataFrameRef, env);
    rustler::resource!(FzExprRef, env);
    true
}

rustler::init!("Elixir.DataFusion.Native", [
    df_from_csv,
    df_from_parquet,
    df_names,
    df_n_rows,
], load = on_load);
