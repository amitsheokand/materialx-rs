//
// Copyright Contributors to the MaterialX Project
// SPDX-License-Identifier: Apache-2.0
//

use crate::materialx::core::export;
use crate::materialx::core::definition;

//
// use super::InterfaceElement;
// use std::sync::Arc;
//
// pub type NodePredicate = std::boxed::Box<dyn Fn(NodePtr) -> bool>;
//
// pub struct Node {
//     pub parent: ElementPtr,
//     pub name: String,
//     static CATEGORY: str,
// }
//
// impl Node {
//     pub fn new(parent: ElementPtr, name: String) -> Self {
//         Self {
//             parent,
//             name,
//             CATEGORY: "node"
//         }
//     }
// }
//
// pub struct GraphElement {
//     pub parent: ElementPtr,
//     pub category: String,
//     pub name: String,
// }
//
// impl GraphElement {
//     pub fn new(parent: ElementPtr, category: String, name: String) -> Self {
//         Self {
//             parent,
//             category,
//             name,
//         }
//     }
// }
//
// pub struct NodeGraph {
//     pub parent: ElementPtr,
//     pub name: String,
//     static CATEGORY: str,
// }
//
// impl NodeGraph {
//     pub fn new(parent: ElementPtr, name: String) -> Self {
//         Self {
//             parent,
//             name,
//             CATEGORY: "nodegraph"
//         }
//     }
// }
//
// pub struct Backdrop {
//     pub parent: ElementPtr,
//     pub name: String,
//     static CATEGORY: str,
//     const CONTAINS_ATTRIBUTE: str,
//     const WIDTH_ATTRIBUTE: str,
//     const HEIGHT_ATTRIBUTE: str,
// }
//
// impl Backdrop {
//     pub fn new(parent: ElementPtr, name: String) -> Self {
//         Self {
//             parent,
//             name,
//             CATEGORY: "backdrop",
//             CONTAINS_ATTRIBUTE: "contains",
//             WIDTH_ATTRIBUTE: "width",
//             HEIGHT_ATTRIBUTE: "height"
//         }
//     }
// }
//
// pub type NodePtr = Arc<Node>;
// pub type ConstNodePtr = Arc<Node>;
//
// pub type GraphElementPtr = Arc<GraphElement>;
// pub type ConstGraphElementPtr = Arc<GraphElement>;
//
// pub type NodeGraphPtr = Arc<NodeGraph>;
// pub type ConstNodeGraphPtr = Arc<NodeGraph>;
//
// pub type BackdropPtr = Arc<Backdrop>;
// pub type ConstBackdropPtr = Arc<Backdrop>;