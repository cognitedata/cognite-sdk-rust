use crate::{
    Create, DeleteWithIgnoreUnknownIds, FilterItems, Identity, Patch, Resource,
    RetrieveWithIgnoreUnknownIds, Update, WithBasePath,
};

use crate::extpipes::*;

pub type ExtPipes = Resource<ExtPipe>;

impl WithBasePath for ExtPipes {
    const BASE_PATH: &'static str = "extpipes";
}

impl Create<AddExtPipe, ExtPipe> for ExtPipes {}
impl DeleteWithIgnoreUnknownIds<Identity> for ExtPipes {}
impl Update<Patch<PatchExtPipe>, ExtPipe> for ExtPipes {}
impl RetrieveWithIgnoreUnknownIds<Identity, ExtPipe> for ExtPipes {}
impl FilterItems<ExtPipeFilter, ExtPipe> for ExtPipes {}

pub type ExtPipeRuns = Resource<ExtPipeRun>;

impl WithBasePath for ExtPipeRuns {
    const BASE_PATH: &'static str = "extpipes/runs";
}

impl Create<AddExtPipeRun, ExtPipeRun> for ExtPipeRuns {}
impl FilterItems<ExtPipeRunFilter, ExtPipeRun> for ExtPipeRuns {}
