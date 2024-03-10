// mod fz_dtypes;

use datafusion::prelude::*;

// use crate::atoms;
use crate::FusionError;

use rustler::{NifStruct, ResourceArc};

// pub use fz_dtypes::*;

use std::ops::Deref;

pub struct FzDataFrameRef(pub DataFrame);
pub struct FzExprRef(pub Expr);


#[derive(NifStruct)]
#[module = "DataFusion.DataFrame"]
pub struct FzDataFrame {
    pub resource: ResourceArc<FzDataFrameRef>,
}

#[derive(NifStruct)]
#[module = "DataFusion.Expression"]
pub struct FzExpr {
    pub resource: ResourceArc<FzExprRef>,
}

impl FzDataFrameRef {
    pub fn new(df: DataFrame) -> Self {
        Self(df)
    }
}

impl FzExprRef {
    pub fn new(expr: Expr) -> Self {
        Self(expr)
    }
}

impl FzDataFrame {
    pub fn new(df: DataFrame) -> Self {
        Self {
            resource: ResourceArc::new(FzDataFrameRef::new(df)),
        }
    }

    // Returns a clone of the DataFrame inside the ResourceArc container.
    pub fn clone_inner(&self) -> DataFrame {
        self.resource.0.clone()
    }
}

// Implement Deref so we can call `DataFrame` functions directly from a `FzDataFrame` struct.
impl Deref for FzDataFrame {
    type Target = DataFrame;

    fn deref(&self) -> &Self::Target {
        &self.resource.0
    }
}

impl FzExpr {
    pub fn new(expr: Expr) -> Self {
        Self {
            resource: ResourceArc::new(FzExprRef::new(expr)),
        }
    }

    // Returns a clone of the Expr inside the ResourceArc container.
    pub fn clone_inner(&self) -> Expr {
        self.resource.0.clone()
    }
}

// Implement Deref so we can call `Expr` functions directly from a `FzExpr` struct.
impl Deref for FzExpr {
    type Target = Expr;

    fn deref(&self) -> &Self::Target {
        &self.resource.0
    }
}