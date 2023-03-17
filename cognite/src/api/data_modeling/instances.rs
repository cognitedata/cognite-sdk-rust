use crate::dto::data_modeling::instances::{NodeOrEdge, SlimNodeOrEdge};
use crate::models::{InstancesFilter, NodeAndEdgeCreateCollection, NodeOrEdgeSpecification};
use crate::{
    DeleteWithResponse, Filter, FilterWithRequest, ItemsWithCursor, Retrieve, UpsertCollection,
};
use crate::{Resource, WithBasePath};

pub struct Instance {}
pub type Instances = Resource<Instance>;

impl WithBasePath for Instances {
    const BASE_PATH: &'static str = "models/instances";
}

impl FilterWithRequest<Filter<InstancesFilter>, ItemsWithCursor<NodeOrEdge>> for Instances {}
impl Retrieve<NodeOrEdgeSpecification, NodeOrEdge> for Instances {}
impl UpsertCollection<NodeAndEdgeCreateCollection, SlimNodeOrEdge> for Instances {}
impl DeleteWithResponse<NodeOrEdgeSpecification, NodeOrEdgeSpecification> for Instances {}
