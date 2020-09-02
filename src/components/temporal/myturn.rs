use super::*;
use crate::components::prelude::*;
use crate::components::WorkOrder;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MyTurn {
    pub active: bool,
    pub shift: ScheduleTime,
    pub job: JobType,
    pub order: WorkOrder,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum JobType {
    None,
    FellTree {
        tree_id: usize,
        tree_pos: usize,
        tool_id: Option<usize>,
        step: LumberjackSteps,
    },
    ConstructBuilding {
        building_id: usize,
        building_pos: usize,
        step: BuildingSteps,
        components: Vec<(usize, usize, bool)>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LumberjackSteps {
    FindAxe,
    TravelToAxe { path: Vec<usize> },
    CollectAxe,
    FindTree,
    TravelToTree { path: Vec<usize> },
    ChopTree,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum BuildingSteps {
    FindComponent,
    TravelToComponent {
        path: Vec<usize>,
        component_id: usize,
    },
    CollectComponent {
        component_id: usize,
    },
    FindBuilding {
        component_id: usize,
    },
    TravelToTBuilding {
        path: Vec<usize>,
        component_id: usize,
    },
    Construct,
}
