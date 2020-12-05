//! Rust representation of the VMAP format, deserializable from a [File]
use crate::{
    dmx::{Color, File, Qangle, Quaternion, Vector2, Vector3, Vector4},
    serde::{from_file, BufferWrapper, StringWrapper},
};
use serde::{
    de::{value::Error, IntoDeserializer},
    Deserialize, Serialize,
};
use std::{
    collections::HashMap,
    fmt::Debug,
    os::raw::{c_float, c_int},
};

pub fn read_vmap<'de, B, S: Debug>(file: &'de File<B, S>) -> Result<CMapRootElement<B, S>, Error>
where
    BufferWrapper<'de, B>: IntoDeserializer<'de>,
    StringWrapper<'de, S>: IntoDeserializer<'de>,
    CMapRootElement<B, S>: Deserialize<'de>,
{
    from_file(file)
}

pub type Element<B, S> = Box<ElementType<B, S>>;

#[derive(Debug, Serialize, Deserialize)]
pub enum PolygonMeshStreamData {
    IntArray(Vec<c_int>),
    FloatArray(Vec<c_float>),
    Vector2Array(Vec<Vector2>),
    Vector3Array(Vec<Vector3>),
    Vector4Array(Vec<Vector4>),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PropValue<B, S> {
    String(S),
    DmElement(DmElement<B, S>),
}

pub type EditGameClassProps<B, S> = HashMap<String, PropValue<B, S>>;

#[derive(Debug, Serialize, Deserialize)]
pub enum ElementType<B, S> {
    CDmExtraVertexData(CDmExtraVertexData<B, S>),
    CDmExtraVertexStream(CDmExtraVertexStream<S>),
    CDmeDrawCallSnapshot(CDmeDrawCallSnapshot<S>),
    CDmeNavData(CDmeNavData),
    CDmeNodeInstanceData(CDmeNodeInstanceData),
    CDmePolygonMesh(CDmePolygonMesh<B, S>),
    CDmePolygonMeshDataArray(CDmePolygonMeshDataArray<B, S>),
    CDmePolygonMeshDataStream(CDmePolygonMeshDataStream<B, S>),
    CDmePolygonMeshSubdivisionData(CDmePolygonMeshSubdivisionData<B, S>),
    CDmePolygonMeshSubdivisiondataBinding(CDmePolygonMeshSubdivisiondataBinding),
    CDmeReferencedMeshSnapshot(CDmeReferencedMeshSnapshot<B, S>),
    CDmeTileMesh(CDmeTileMesh<S>),
    CFaceSelectionSetDataElement(CFaceSelectionSetDataElement<B, S>),
    CMapCable(CMapCable<B, S>),
    CMapCordon(CMapCordon<B, S>),
    CMapEntity(CMapEntity<B, S>),
    CMapGroup(CMapGroup<B, S>),
    CMapGroupProxy(CMapGroupProxy<B, S>),
    CMapInstance(CMapInstance<B, S>),
    CMapMesh(CMapMesh<B, S>),
    CMapNavData(CMapNavData<B, S>),
    CMapPath(CMapPath<B, S>),
    CMapPathNode(CMapPathNode<B, S>),
    CMapPhysicsPin(CMapPhysicsPin<B, S>),
    CMapPrefab(CMapPrefab<B, S>),
    CMapProxyInstance(CMapProxyInstance<B, S>),
    CMapRootElement(CMapRootElement<B, S>),
    CMapSelectionSet(CMapSelectionSet<B, S>),
    CMapStaticOverlay(CMapStaticOverlay<B, S>),
    CMapTile(CMapTile<B, S>),
    CMapTileMesh(CMapTileMesh<B, S>),
    CMapTileSet(CMapTileSet<B, S>),
    CMapVariableChoice(CMapVariableChoice<S>),
    CMapVariableChoiceGroup(CMapVariableChoiceGroup<B, S>),
    CMapVariableSet(CMapVariableSet<B, S>),
    CMapWorld(CMapWorld<B, S>),
    CObjectSelectionSetDataElement(CObjectSelectionSetDataElement<B, S>),
    CStoredCamera(CStoredCamera),
    CStoredCameras(CStoredCameras<B, S>),
    CTileSetMaterialSet(CTileSetMaterialSet<B, S>),
    CTileSetProperty(CTileSetProperty<S>),
    CTrajectoryPath(CTrajectoryPath<B, S>),
    CTrajectoryPathNode(CTrajectoryPathNode<B, S>),
    CVisibilityMgr(CVisibilityMgr<B, S>),
    DmElement(DmElement<B, S>),
    DmeConnectionData(DmeConnectionData<S>),
    DmePlugList(DmePlugList<S>),
    DmeVertexData(DmeVertexData<S>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CDmExtraVertexData<B, S> {
    #[serde(rename = "m_ExtraStreams")]
    pub extra_streams: Vec<Element<B, S>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CDmExtraVertexStream<S> {
    #[serde(rename = "m_nDrawCallIndex")]
    pub n_draw_call_index: c_int,
    #[serde(rename = "m_nMeshIndex")]
    pub n_mesh_index: c_int,
    #[serde(rename = "m_pVertexData")]
    pub p_vertex_data: DmeVertexData<S>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CDmeDrawCallSnapshot<S> {
    #[serde(rename = "m_Material")]
    pub material: S,
    #[serde(rename = "m_Normals")]
    pub normals: Vec<Vector3>,
    #[serde(rename = "m_Positions")]
    pub positions: Vec<Vector3>,
    #[serde(rename = "m_Texcoords")]
    pub texcoords: Vec<Vector2>,
    #[serde(rename = "m_nHash")]
    pub n_hash: c_int,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CDmeNavData {
    #[serde(rename = "settingsAgentHeight_0")]
    pub settings_agent_height_0: c_float,
    #[serde(rename = "settingsAgentHeight_1")]
    pub settings_agent_height_1: c_float,
    #[serde(rename = "settingsAgentHeight_2")]
    pub settings_agent_height_2: c_float,
    #[serde(rename = "settingsAgentMaxClimb_0")]
    pub settings_agent_max_climb_0: c_float,
    #[serde(rename = "settingsAgentMaxClimb_1")]
    pub settings_agent_max_climb_1: c_float,
    #[serde(rename = "settingsAgentMaxClimb_2")]
    pub settings_agent_max_climb_2: c_float,
    #[serde(rename = "settingsAgentMaxJumpDownDist_0")]
    pub settings_agent_max_jump_down_dist_0: c_float,
    #[serde(rename = "settingsAgentMaxJumpDownDist_1")]
    pub settings_agent_max_jump_down_dist_1: c_float,
    #[serde(rename = "settingsAgentMaxJumpDownDist_2")]
    pub settings_agent_max_jump_down_dist_2: c_float,
    #[serde(rename = "settingsAgentMaxJumpHorizDistBase_0")]
    pub settings_agent_max_jump_horiz_dist_base_0: c_float,
    #[serde(rename = "settingsAgentMaxJumpHorizDistBase_1")]
    pub settings_agent_max_jump_horiz_dist_base_1: c_float,
    #[serde(rename = "settingsAgentMaxJumpHorizDistBase_2")]
    pub settings_agent_max_jump_horiz_dist_base_2: c_float,
    #[serde(rename = "settingsAgentMaxJumpUpDist_0")]
    pub settings_agent_max_jump_up_dist_0: c_float,
    #[serde(rename = "settingsAgentMaxJumpUpDist_1")]
    pub settings_agent_max_jump_up_dist_1: c_float,
    #[serde(rename = "settingsAgentMaxJumpUpDist_2")]
    pub settings_agent_max_jump_up_dist_2: c_float,
    #[serde(rename = "settingsAgentMaxSlope_0")]
    pub settings_agent_max_slope_0: c_int,
    #[serde(rename = "settingsAgentMaxSlope_1")]
    pub settings_agent_max_slope_1: c_int,
    #[serde(rename = "settingsAgentMaxSlope_2")]
    pub settings_agent_max_slope_2: c_int,
    #[serde(rename = "settingsAgentNumHulls")]
    pub settings_agent_num_hulls: c_int,
    #[serde(rename = "settingsAgentRadius_0")]
    pub settings_agent_radius_0: c_float,
    #[serde(rename = "settingsAgentRadius_1")]
    pub settings_agent_radius_1: c_float,
    #[serde(rename = "settingsAgentRadius_2")]
    pub settings_agent_radius_2: c_float,
    #[serde(rename = "settingsCellHeight")]
    pub settings_cell_height: c_float,
    #[serde(rename = "settingsCellSize")]
    pub settings_cell_size: c_float,
    #[serde(rename = "settingsDetailSampleDist")]
    pub settings_detail_sample_dist: c_float,
    #[serde(rename = "settingsDetailSampleMaxError")]
    pub settings_detail_sample_max_error: c_float,
    #[serde(rename = "settingsEdgeMaxError")]
    pub settings_edge_max_error: c_float,
    #[serde(rename = "settingsEdgeMaxLen")]
    pub settings_edge_max_len: c_int,
    #[serde(rename = "settingsRegionMergeSize")]
    pub settings_region_merge_size: c_int,
    #[serde(rename = "settingsRegionMinSize")]
    pub settings_region_min_size: c_int,
    #[serde(rename = "settingsSmallAreaOnEdgeRemovalSize")]
    pub settings_small_area_on_edge_removal_size: c_float,
    #[serde(rename = "settingsTileSize")]
    pub settings_tile_size: c_float,
    #[serde(rename = "settingsUseProjectDefaults")]
    pub settings_use_project_defaults: bool,
    #[serde(rename = "settingsVertsPerPoly")]
    pub settings_verts_per_poly: c_int,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CDmeNodeInstanceData {
    #[serde(rename = "vertexLightingData")]
    pub vertex_lighting_data: Vec<Color>,
    #[serde(rename = "vertexLightingPositions")]
    pub vertex_lighting_positions: Vec<Vector3>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CDmePolygonMesh<B, S> {
    #[serde(rename = "edgeData")]
    pub edge_data: CDmePolygonMeshDataArray<B, S>,
    #[serde(rename = "edgeDataIndices")]
    pub edge_data_indices: Vec<c_int>,
    #[serde(rename = "edgeFaceIndices")]
    pub edge_face_indices: Vec<c_int>,
    #[serde(rename = "edgeNextIndices")]
    pub edge_next_indices: Vec<c_int>,
    #[serde(rename = "edgeOppositeIndices")]
    pub edge_opposite_indices: Vec<c_int>,
    #[serde(rename = "edgeVertexDataIndices")]
    pub edge_vertex_data_indices: Vec<c_int>,
    #[serde(rename = "edgeVertexIndices")]
    pub edge_vertex_indices: Vec<c_int>,
    #[serde(rename = "faceData")]
    pub face_data: CDmePolygonMeshDataArray<B, S>,
    #[serde(rename = "faceDataIndices")]
    pub face_data_indices: Vec<c_int>,
    #[serde(rename = "faceEdgeIndices")]
    pub face_edge_indices: Vec<c_int>,
    #[serde(rename = "faceVertexData")]
    pub face_vertex_data: CDmePolygonMeshDataArray<B, S>,
    pub materials: Vec<S>,
    #[serde(rename = "subdivisionData")]
    pub subdivision_data: CDmePolygonMeshSubdivisionData<B, S>,
    #[serde(rename = "vertexData")]
    pub vertex_data: CDmePolygonMeshDataArray<B, S>,
    #[serde(rename = "vertexDataIndices")]
    pub vertex_data_indices: Vec<c_int>,
    #[serde(rename = "vertexEdgeIndices")]
    pub vertex_edge_indices: Vec<c_int>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CDmePolygonMeshDataArray<B, S> {
    pub size: c_int,
    pub streams: Vec<Element<B, S>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CDmePolygonMeshDataStream<B, S> {
    pub data: PolygonMeshStreamData,
    #[serde(rename = "dataStateFlags")]
    pub data_state_flags: c_int,
    #[serde(rename = "semanticIndex")]
    pub semantic_index: c_int,
    #[serde(rename = "semanticName")]
    pub semantic_name: S,
    #[serde(rename = "standardAttributeName")]
    pub standard_attribute_name: S,
    #[serde(rename = "subdivisionBinding")]
    pub subdivision_binding: Option<Element<B, S>>,
    #[serde(rename = "vertexBufferLocation")]
    pub vertex_buffer_location: c_int,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CDmePolygonMeshSubdivisionData<B, S> {
    pub streams: Vec<Element<B, S>>,
    #[serde(rename = "subdivisionLevels")]
    pub subdivision_levels: Vec<c_int>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CDmePolygonMeshSubdivisiondataBinding {
    #[serde(rename = "streamSourceType")]
    pub stream_source_type: c_int,
    #[serde(rename = "targetDataType")]
    pub target_data_type: c_int,
    #[serde(rename = "targetStreamIndex")]
    pub target_stream_index: c_int,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CDmeReferencedMeshSnapshot<B, S> {
    #[serde(rename = "m_DrawCalls")]
    pub draw_calls: Vec<Element<B, S>>,
    #[serde(rename = "m_MeshResourceName")]
    pub mesh_resource_name: S,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CDmeTileMesh<S> {
    #[serde(rename = "faceIds")]
    pub face_ids: Vec<c_int>,
    #[serde(rename = "materialSetAssignments")]
    pub material_set_assignments: Vec<c_int>,
    #[serde(rename = "propertyNameIndices")]
    pub property_name_indices: Vec<c_int>,
    #[serde(rename = "propertyStrings")]
    pub property_strings: Vec<S>,
    #[serde(rename = "propertyValueIndices")]
    pub property_value_indices: Vec<c_int>,
    #[serde(rename = "tileConfiguration")]
    pub tile_configuration: Vec<c_int>,
    #[serde(rename = "tileSetAssignments")]
    pub tile_set_assignments: Vec<c_int>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CFaceSelectionSetDataElement<B, S> {
    pub faces: Vec<c_int>,
    pub meshes: Vec<Element<B, S>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CMapCable<B, S> {
    pub angles: Qangle,
    pub children: Vec<Element<B, S>>,
    #[serde(rename = "collisionEnabled")]
    pub collision_enabled: bool,
    #[serde(rename = "connectionsData")]
    pub connections_data: Vec<Element<B, S>>,
    #[serde(rename = "editorOnly")]
    pub editor_only: bool,
    pub entity_properties: EditGameClassProps<B, S>,
    pub force_hidden: bool,
    #[serde(rename = "hitNormal")]
    pub hit_normal: Vector3,
    #[serde(rename = "interpolationType")]
    pub interpolation_type: c_int,
    #[serde(rename = "isProceduralEntity")]
    pub is_procedural_entity: bool,
    #[serde(rename = "lightingOriginName")]
    pub lighting_origin_name: S,
    #[serde(rename = "materialName")]
    pub material_name: S,
    #[serde(rename = "nodeID")]
    pub node_id: c_int,
    #[serde(rename = "numSides")]
    pub num_sides: c_int,
    pub origin: Vector3,
    #[serde(rename = "physicsSimplificationError")]
    pub physics_simplification_error: c_float,
    pub radius: c_float,
    #[serde(rename = "referenceID")]
    pub reference_id: u64,
    #[serde(rename = "relayPlugData")]
    pub relay_plug_data: DmePlugList<S>,
    pub scales: Vector3,
    #[serde(rename = "tessellationSpacing")]
    pub tessellation_spacing: c_float,
    #[serde(rename = "textureOffsetAlongPath")]
    pub texture_offset_along_path: c_float,
    #[serde(rename = "textureOffsetCircumference")]
    pub texture_offset_circumference: c_float,
    #[serde(rename = "textureOrientation")]
    pub texture_orientation: c_int,
    #[serde(rename = "textureRepeatsCircumference")]
    pub texture_repeats_circumference: c_float,
    #[serde(rename = "textureScale")]
    pub texture_scale: c_float,
    #[serde(rename = "tintColor")]
    pub tint_color: Color,
    #[serde(rename = "transformLocked")]
    pub transform_locked: bool,
    #[serde(rename = "variableNames")]
    pub variable_names: Vec<S>,
    #[serde(rename = "variableTargetKeys")]
    pub variable_target_keys: Vec<S>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CMapCordon<B, S> {
    pub angles: Qangle,
    pub children: Vec<Element<B, S>>,
    #[serde(rename = "editorOnly")]
    pub editor_only: bool,
    pub force_hidden: bool,
    #[serde(rename = "nodeID")]
    pub node_id: c_int,
    pub origin: Vector3,
    #[serde(rename = "referenceID")]
    pub reference_id: u64,
    pub scales: Vector3,
    #[serde(rename = "transformLocked")]
    pub transform_locked: bool,
    #[serde(rename = "variableNames")]
    pub variable_names: Vec<S>,
    #[serde(rename = "variableTargetKeys")]
    pub variable_target_keys: Vec<S>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CMapEntity<B, S> {
    pub angles: Qangle,
    #[serde(rename = "boneNames")]
    pub bone_names: Option<Vec<S>>,
    #[serde(rename = "bonePositions")]
    pub bone_positions: Option<Vec<Vector3>>,
    #[serde(rename = "boneRotations")]
    pub bone_rotations: Option<Vec<Quaternion>>,
    pub children: Vec<Element<B, S>>,
    #[serde(rename = "connectionsData")]
    pub connections_data: Vec<Element<B, S>>,
    #[serde(rename = "editorOnly")]
    pub editor_only: bool,
    pub entity_properties: EditGameClassProps<B, S>,
    pub extra_vertex_data: Option<CDmExtraVertexData<B, S>>,
    pub force_hidden: bool,
    #[serde(rename = "hitNormal")]
    pub hit_normal: Vector3,
    #[serde(rename = "isProceduralEntity")]
    pub is_procedural_entity: bool,
    #[serde(rename = "nodeID")]
    pub node_id: c_int,
    pub origin: Vector3,
    #[serde(rename = "referenceID")]
    pub reference_id: u64,
    #[serde(rename = "relayPlugData")]
    pub relay_plug_data: DmePlugList<S>,
    pub scales: Vector3,
    #[serde(rename = "transformLocked")]
    pub transform_locked: Option<bool>,
    #[serde(rename = "variableNames")]
    pub variable_names: Vec<S>,
    #[serde(rename = "variableTargetKeys")]
    pub variable_target_keys: Vec<S>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CMapGroup<B, S> {
    pub angles: Qangle,
    pub children: Vec<Element<B, S>>,
    #[serde(rename = "editorOnly")]
    pub editor_only: bool,
    pub force_hidden: bool,
    #[serde(rename = "nodeID")]
    pub node_id: c_int,
    pub origin: Vector3,
    #[serde(rename = "referenceID")]
    pub reference_id: u64,
    pub scales: Vector3,
    #[serde(rename = "transformLocked")]
    pub transform_locked: bool,
    #[serde(rename = "variableNames")]
    pub variable_names: Vec<S>,
    #[serde(rename = "variableTargetKeys")]
    pub variable_target_keys: Vec<S>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CMapGroupProxy<B, S> {
    #[serde(rename = "alwaysOrientUp")]
    pub always_orient_up: bool,
    pub angles: Qangle,
    pub children: Vec<Element<B, S>>,
    pub conditional: bool,
    #[serde(rename = "contentsDeform")]
    pub contents_deform: bool,
    #[serde(rename = "editorOnly")]
    pub editor_only: bool,
    pub force_hidden: bool,
    #[serde(rename = "nodeID")]
    pub node_id: c_int,
    pub origin: Vector3,
    pub probability: c_float,
    #[serde(rename = "proxyName")]
    pub proxy_name: S,
    #[serde(rename = "referenceID")]
    pub reference_id: u64,
    pub removable: bool,
    #[serde(rename = "rootPreviewInstance")]
    pub root_preview_instance: Option<Element<B, S>>,
    pub scales: Vector3,
    #[serde(rename = "showPreview")]
    pub show_preview: bool,
    #[serde(rename = "targetGroupReferenceID")]
    pub target_group_reference_id: u64,
    #[serde(rename = "tile requirements_cornerPropertyValues0")]
    pub tile_requirements_corner_property_values0: Vec<S>,
    #[serde(rename = "tile requirements_cornerPropertyValues1")]
    pub tile_requirements_corner_property_values1: Vec<S>,
    #[serde(rename = "tile requirements_cornerPropertyValues2")]
    pub tile_requirements_corner_property_values2: Vec<S>,
    #[serde(rename = "tile requirements_cornerPropertyValues3")]
    pub tile_requirements_corner_property_values3: Vec<S>,
    #[serde(rename = "tile requirements_edgePropertyValues0")]
    pub tile_requirements_edge_property_values0: Vec<S>,
    #[serde(rename = "tile requirements_edgePropertyValues1")]
    pub tile_requirements_edge_property_values1: Vec<S>,
    #[serde(rename = "tile requirements_edgePropertyValues2")]
    pub tile_requirements_edge_property_values2: Vec<S>,
    #[serde(rename = "tile requirements_edgePropertyValues3")]
    pub tile_requirements_edge_property_values3: Vec<S>,
    #[serde(rename = "tile requirements_propertyNames")]
    pub tile_requirements_property_names: Vec<S>,
    #[serde(rename = "tile requirements_propertyValues")]
    pub tile_requirements_property_values: Vec<S>,
    #[serde(rename = "transformLocked")]
    pub transform_locked: bool,
    #[serde(rename = "variableNames")]
    pub variable_names: Vec<S>,
    #[serde(rename = "variableTargetKeys")]
    pub variable_target_keys: Vec<S>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CMapInstance<B, S> {
    pub angles: Qangle,
    pub children: Vec<Element<B, S>>,
    #[serde(rename = "connectionsData")]
    pub connections_data: Vec<Element<B, S>>,
    #[serde(rename = "editorOnly")]
    pub editor_only: bool,
    pub force_hidden: bool,
    #[serde(rename = "nodeID")]
    pub node_id: c_int,
    pub origin: Vector3,
    #[serde(rename = "referenceID")]
    pub reference_id: u64,
    #[serde(rename = "relayPlugData")]
    pub relay_plug_data: DmePlugList<S>,
    pub scales: Vector3,
    pub target: CMapGroup<B, S>,
    #[serde(rename = "transformLocked")]
    pub transform_locked: bool,
    #[serde(rename = "variableNames")]
    pub variable_names: Vec<S>,
    #[serde(rename = "variableTargetKeys")]
    pub variable_target_keys: Vec<S>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CMapMesh<B, S> {
    pub angles: Qangle,
    pub bakelighting: Option<bool>,
    pub bakelightoutput: Option<c_int>,
    pub children: Vec<Element<B, S>>,
    #[serde(rename = "cubeMapName")]
    pub cube_map_name: S,
    #[serde(rename = "disableHeightDisplacement")]
    pub disable_height_displacement: Option<bool>,
    #[serde(rename = "disableShadows")]
    pub disable_shadows: Option<bool>,
    #[serde(rename = "editorOnly")]
    pub editor_only: bool,
    pub excludefromimpostors: Option<bool>,
    pub fademaxdist: c_float,
    pub fademindist: c_float,
    pub force_hidden: bool,
    #[serde(rename = "lightGroup")]
    pub light_group: S,
    #[serde(rename = "meshData")]
    pub mesh_data: CDmePolygonMesh<B, S>,
    #[serde(rename = "nodeID")]
    pub node_id: c_int,
    pub origin: Vector3,
    #[serde(rename = "physicsGroup")]
    pub physics_group: S,
    #[serde(rename = "physicsInteractsAs")]
    pub physics_interacts_as: S,
    #[serde(rename = "physicsInteractsExclude")]
    pub physics_interacts_exclude: Option<S>,
    #[serde(rename = "physicsInteractsWith")]
    pub physics_interacts_with: S,
    #[serde(rename = "physicsSimplificationError")]
    pub physics_simplification_error: c_float,
    #[serde(rename = "physicsSimplificationOverride")]
    pub physics_simplification_override: Option<bool>,
    #[serde(rename = "physicsType")]
    pub physics_type: S,
    pub precomputelightprobes: bool,
    #[serde(rename = "referenceID")]
    pub reference_id: u64,
    #[serde(rename = "renderAmt")]
    pub render_amt: c_int,
    #[serde(rename = "renderToCubemaps")]
    pub render_to_cubemaps: bool,
    pub renderwithdynamic: Option<bool>,
    pub scales: Vector3,
    #[serde(rename = "smoothingAngle")]
    pub smoothing_angle: c_float,
    #[serde(rename = "tintColor")]
    pub tint_color: Color,
    #[serde(rename = "transformLocked")]
    pub transform_locked: Option<bool>,
    #[serde(rename = "useAsOccluder")]
    pub use_as_occluder: bool,
    #[serde(rename = "variableNames")]
    pub variable_names: Vec<S>,
    #[serde(rename = "variableTargetKeys")]
    pub variable_target_keys: Vec<S>,
    pub visexclude: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CMapNavData<B, S> {
    pub angles: Qangle,
    pub children: Vec<Element<B, S>>,
    #[serde(rename = "editorOnly")]
    pub editor_only: bool,
    pub force_hidden: bool,
    #[serde(rename = "navData")]
    pub nav_data: CDmeNavData,
    #[serde(rename = "nodeID")]
    pub node_id: c_int,
    pub origin: Vector3,
    #[serde(rename = "referenceID")]
    pub reference_id: u64,
    pub scales: Vector3,
    #[serde(rename = "transformLocked")]
    pub transform_locked: bool,
    #[serde(rename = "variableNames")]
    pub variable_names: Vec<S>,
    #[serde(rename = "variableTargetKeys")]
    pub variable_target_keys: Vec<S>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CMapPath<B, S> {
    pub angles: Qangle,
    pub children: Vec<Element<B, S>>,
    #[serde(rename = "connectionsData")]
    pub connections_data: Vec<Element<B, S>>,
    #[serde(rename = "editorOnly")]
    pub editor_only: bool,
    pub entity_properties: EditGameClassProps<B, S>,
    pub force_hidden: bool,
    #[serde(rename = "hitNormal")]
    pub hit_normal: Vector3,
    #[serde(rename = "interpolationType")]
    pub interpolation_type: c_int,
    #[serde(rename = "isProceduralEntity")]
    pub is_procedural_entity: bool,
    #[serde(rename = "nodeID")]
    pub node_id: c_int,
    pub origin: Vector3,
    #[serde(rename = "referenceID")]
    pub reference_id: u64,
    #[serde(rename = "relayPlugData")]
    pub relay_plug_data: DmePlugList<S>,
    pub scales: Vector3,
    #[serde(rename = "transformLocked")]
    pub transform_locked: bool,
    #[serde(rename = "variableNames")]
    pub variable_names: Vec<S>,
    #[serde(rename = "variableTargetKeys")]
    pub variable_target_keys: Vec<S>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CMapPathNode<B, S> {
    pub angles: Qangle,
    pub children: Vec<Element<B, S>>,
    #[serde(rename = "connectionsData")]
    pub connections_data: Vec<Element<B, S>>,
    #[serde(rename = "editorOnly")]
    pub editor_only: bool,
    pub entity_properties: EditGameClassProps<B, S>,
    pub force_hidden: bool,
    #[serde(rename = "hitNormal")]
    pub hit_normal: Vector3,
    #[serde(rename = "inTangent")]
    pub in_tangent: Vector3,
    #[serde(rename = "inTangentType")]
    pub in_tangent_type: c_int,
    #[serde(rename = "isProceduralEntity")]
    pub is_procedural_entity: bool,
    #[serde(rename = "nodeID")]
    pub node_id: c_int,
    pub origin: Vector3,
    #[serde(rename = "outTangent")]
    pub out_tangent: Vector3,
    #[serde(rename = "outTangentType")]
    pub out_tangent_type: c_int,
    #[serde(rename = "pathNodeName")]
    pub path_node_name: S,
    #[serde(rename = "pinEnabled")]
    pub pin_enabled: bool,
    #[serde(rename = "radiusScale")]
    pub radius_scale: c_float,
    #[serde(rename = "referenceID")]
    pub reference_id: u64,
    #[serde(rename = "relayPlugData")]
    pub relay_plug_data: DmePlugList<S>,
    pub scales: Vector3,
    #[serde(rename = "tintColor")]
    pub tint_color: Color,
    #[serde(rename = "transformLocked")]
    pub transform_locked: bool,
    #[serde(rename = "variableNames")]
    pub variable_names: Vec<S>,
    #[serde(rename = "variableTargetKeys")]
    pub variable_target_keys: Vec<S>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CMapPhysicsPin<B, S> {
    pub angles: Qangle,
    #[serde(rename = "bConstrainRotation")]
    pub b_constrain_rotation: Option<bool>,
    pub children: Vec<Element<B, S>>,
    #[serde(rename = "constrainRotation")]
    pub constrain_rotation: bool,
    #[serde(rename = "editorOnly")]
    pub editor_only: bool,
    pub force_hidden: bool,
    #[serde(rename = "nodeID")]
    pub node_id: c_int,
    pub origin: Vector3,
    #[serde(rename = "referenceID")]
    pub reference_id: u64,
    #[serde(rename = "rigidConstraint")]
    pub rigid_constraint: bool,
    pub scales: Vector3,
    #[serde(rename = "targetBodyName")]
    pub target_body_name: S,
    #[serde(rename = "targetOffset")]
    pub target_offset: Vector3,
    #[serde(rename = "transformLocked")]
    pub transform_locked: bool,
    #[serde(rename = "variableNames")]
    pub variable_names: Vec<S>,
    #[serde(rename = "variableTargetKeys")]
    pub variable_target_keys: Vec<S>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CMapPrefab<B, S> {
    pub angles: Qangle,
    pub children: Vec<Element<B, S>>,
    #[serde(rename = "connectionsData")]
    pub connections_data: Vec<Element<B, S>>,
    #[serde(rename = "displayColor")]
    pub display_color: Color,
    #[serde(rename = "editorOnly")]
    pub editor_only: bool,
    #[serde(rename = "fixupEntityNames")]
    pub fixup_entity_names: bool,
    pub force_hidden: bool,
    #[serde(rename = "loadAtRuntime")]
    pub load_at_runtime: bool,
    #[serde(rename = "loadIfNested")]
    pub load_if_nested: bool,
    #[serde(rename = "nodeID")]
    pub node_id: c_int,
    pub origin: Vector3,
    #[serde(rename = "referenceID")]
    pub reference_id: u64,
    #[serde(rename = "relayPlugData")]
    pub relay_plug_data: DmePlugList<S>,
    pub scales: Vector3,
    pub target: Option<Element<B, S>>,
    #[serde(rename = "targetMapPath")]
    pub target_map_path: S,
    #[serde(rename = "targetName")]
    pub target_name: S,
    #[serde(rename = "transformLocked")]
    pub transform_locked: bool,
    #[serde(rename = "variableNames")]
    pub variable_names: Vec<S>,
    #[serde(rename = "variableOverrideNames")]
    pub variable_override_names: Vec<S>,
    #[serde(rename = "variableOverrideValues")]
    pub variable_override_values: Vec<S>,
    #[serde(rename = "variableTargetKeys")]
    pub variable_target_keys: Vec<S>,
    pub visexclude: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CMapProxyInstance<B, S> {
    pub angles: Qangle,
    pub children: Vec<Element<B, S>>,
    #[serde(rename = "connectionsData")]
    pub connections_data: Vec<Element<B, S>>,
    #[serde(rename = "editorOnly")]
    pub editor_only: bool,
    pub force_hidden: bool,
    #[serde(rename = "nodeID")]
    pub node_id: c_int,
    pub origin: Vector3,
    #[serde(rename = "referenceID")]
    pub reference_id: u64,
    #[serde(rename = "relayPlugData")]
    pub relay_plug_data: DmePlugList<S>,
    pub scales: Vector3,
    #[serde(rename = "sourceProxy")]
    pub source_proxy: CMapGroupProxy<B, S>,
    pub target: Element<B, S>,
    #[serde(rename = "transformLocked")]
    pub transform_locked: bool,
    #[serde(rename = "variableNames")]
    pub variable_names: Vec<S>,
    #[serde(rename = "variableTargetKeys")]
    pub variable_target_keys: Vec<S>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CMapRootElement<B, S> {
    #[serde(rename = "3dcameras")]
    pub _3dcameras: Option<Element<B, S>>,
    pub defaultcamera: CStoredCamera,
    pub editorbuild: c_int,
    pub editorversion: c_int,
    pub gridspacing: c_float,
    pub isprefab: bool,
    #[serde(rename = "itemFile")]
    pub item_file: S,
    #[serde(rename = "m_ReferencedMeshSnapshots")]
    pub referenced_mesh_snapshots: Vec<Element<B, S>>,
    #[serde(rename = "m_bCordonsVisible")]
    pub b_cordons_visible: bool,
    #[serde(rename = "m_bIsCordoning")]
    pub b_is_cordoning: bool,
    #[serde(rename = "mapVariables")]
    pub map_variables: CMapVariableSet<B, S>,
    pub mapversion: Option<c_int>,
    #[serde(rename = "nodeInstanceData")]
    pub node_instance_data: Vec<Element<B, S>>,
    #[serde(rename = "rootSelectionSet")]
    pub root_selection_set: CMapSelectionSet<B, S>,
    pub show3dgrid: bool,
    pub showgrid: bool,
    pub snaprotationangle: c_int,
    pub snaptogrid: Option<bool>,
    pub visbility: CVisibilityMgr<B, S>,
    pub world: CMapWorld<B, S>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CMapSelectionSet<B, S> {
    pub children: Vec<Element<B, S>>,
    #[serde(rename = "selectionSetData")]
    pub selection_set_data: Option<Element<B, S>>,
    #[serde(rename = "selectionSetName")]
    pub selection_set_name: S,
    #[serde(rename = "setType")]
    pub set_type: Option<c_int>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CMapStaticOverlay<B, S> {
    pub angles: Qangle,
    pub bakelighting: bool,
    pub bakelightoutput: Option<c_int>,
    pub children: Vec<Element<B, S>>,
    #[serde(rename = "cubeMapName")]
    pub cube_map_name: S,
    #[serde(rename = "disableHeightDisplacement")]
    pub disable_height_displacement: bool,
    #[serde(rename = "disableShadows")]
    pub disable_shadows: bool,
    #[serde(rename = "disabledInLowQuality")]
    pub disabled_in_low_quality: bool,
    #[serde(rename = "editorOnly")]
    pub editor_only: bool,
    pub excludefromimpostors: Option<bool>,
    pub fademaxdist: c_float,
    pub fademindist: c_float,
    pub force_hidden: bool,
    #[serde(rename = "lightGroup")]
    pub light_group: S,
    #[serde(rename = "meshData")]
    pub mesh_data: CDmePolygonMesh<B, S>,
    #[serde(rename = "nodeID")]
    pub node_id: c_int,
    pub origin: Vector3,
    #[serde(rename = "physicsGroup")]
    pub physics_group: S,
    #[serde(rename = "physicsInteractsAs")]
    pub physics_interacts_as: S,
    #[serde(rename = "physicsInteractsExclude")]
    pub physics_interacts_exclude: S,
    #[serde(rename = "physicsInteractsWith")]
    pub physics_interacts_with: S,
    #[serde(rename = "physicsSimplificationError")]
    pub physics_simplification_error: c_float,
    #[serde(rename = "physicsSimplificationOverride")]
    pub physics_simplification_override: bool,
    #[serde(rename = "physicsType")]
    pub physics_type: S,
    pub precomputelightprobes: bool,
    #[serde(rename = "projectOnBackFaces")]
    pub project_on_back_faces: bool,
    #[serde(rename = "projectionFar")]
    pub projection_far: c_float,
    #[serde(rename = "projectionMode")]
    pub projection_mode: c_int,
    #[serde(rename = "projectionTargets")]
    pub projection_targets: Vec<c_int>,
    #[serde(rename = "referenceID")]
    pub reference_id: u64,
    #[serde(rename = "renderAmt")]
    pub render_amt: c_int,
    #[serde(rename = "renderOrder")]
    pub render_order: c_int,
    #[serde(rename = "renderToCubemaps")]
    pub render_to_cubemaps: bool,
    pub renderwithdynamic: bool,
    pub scales: Vector3,
    #[serde(rename = "smoothingAngle")]
    pub smoothing_angle: c_float,
    #[serde(rename = "tintColor")]
    pub tint_color: Color,
    #[serde(rename = "transformLocked")]
    pub transform_locked: bool,
    #[serde(rename = "useAsOccluder")]
    pub use_as_occluder: bool,
    #[serde(rename = "variableNames")]
    pub variable_names: Vec<S>,
    #[serde(rename = "variableTargetKeys")]
    pub variable_target_keys: Vec<S>,
    pub visexclude: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CMapTile<B, S> {
    #[serde(rename = "alignToAxis")]
    pub align_to_axis: c_int,
    pub angles: Qangle,
    #[serde(rename = "baseSize")]
    pub base_size: Vector2,
    pub children: Vec<Element<B, S>>,
    #[serde(rename = "cornerPropertyValues0")]
    pub corner_property_values0: Vec<S>,
    #[serde(rename = "cornerPropertyValues1")]
    pub corner_property_values1: Vec<S>,
    #[serde(rename = "cornerPropertyValues2")]
    pub corner_property_values2: Vec<S>,
    #[serde(rename = "cornerPropertyValues3")]
    pub corner_property_values3: Vec<S>,
    #[serde(rename = "displayBaseFace")]
    pub display_base_face: bool,
    #[serde(rename = "edgePropertyValues0")]
    pub edge_property_values0: Vec<S>,
    #[serde(rename = "edgePropertyValues1")]
    pub edge_property_values1: Vec<S>,
    #[serde(rename = "edgePropertyValues2")]
    pub edge_property_values2: Vec<S>,
    #[serde(rename = "edgePropertyValues3")]
    pub edge_property_values3: Vec<S>,
    #[serde(rename = "editorOnly")]
    pub editor_only: bool,
    pub force_hidden: bool,
    #[serde(rename = "maxScale")]
    pub max_scale: c_float,
    #[serde(rename = "minScale")]
    pub min_scale: c_float,
    #[serde(rename = "nodeID")]
    pub node_id: c_int,
    pub origin: Vector3,
    #[serde(rename = "pickBySize")]
    pub pick_by_size: bool,
    pub probability: c_float,
    #[serde(rename = "propertyNames")]
    pub property_names: Vec<S>,
    #[serde(rename = "propertyValues")]
    pub property_values: Vec<S>,
    #[serde(rename = "referenceID")]
    pub reference_id: u64,
    #[serde(rename = "rotationSnapping")]
    pub rotation_snapping: c_int,
    pub scales: Vector3,
    #[serde(rename = "transformLocked")]
    pub transform_locked: bool,
    #[serde(rename = "variableNames")]
    pub variable_names: Vec<S>,
    #[serde(rename = "variableTargetKeys")]
    pub variable_target_keys: Vec<S>,
    #[serde(rename = "variationId")]
    pub variation_id: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CMapTileMesh<B, S> {
    pub angles: Qangle,
    pub bakelighting: bool,
    pub bakelightoutput: Option<c_int>,
    pub children: Vec<Element<B, S>>,
    #[serde(rename = "cubeMapName")]
    pub cube_map_name: S,
    #[serde(rename = "disableHeightDisplacement")]
    pub disable_height_displacement: bool,
    #[serde(rename = "disableShadows")]
    pub disable_shadows: bool,
    #[serde(rename = "editorOnly")]
    pub editor_only: bool,
    pub excludefromimpostors: Option<bool>,
    pub fademaxdist: c_float,
    pub fademindist: c_float,
    pub force_hidden: bool,
    #[serde(rename = "lightGroup")]
    pub light_group: S,
    #[serde(rename = "meshData")]
    pub mesh_data: CDmePolygonMesh<B, S>,
    #[serde(rename = "nodeID")]
    pub node_id: c_int,
    pub origin: Vector3,
    #[serde(rename = "physicsGroup")]
    pub physics_group: S,
    #[serde(rename = "physicsInteractsAs")]
    pub physics_interacts_as: S,
    #[serde(rename = "physicsInteractsExclude")]
    pub physics_interacts_exclude: S,
    #[serde(rename = "physicsInteractsWith")]
    pub physics_interacts_with: S,
    #[serde(rename = "physicsSimplificationError")]
    pub physics_simplification_error: c_float,
    #[serde(rename = "physicsSimplificationOverride")]
    pub physics_simplification_override: bool,
    #[serde(rename = "physicsType")]
    pub physics_type: S,
    pub precomputelightprobes: bool,
    #[serde(rename = "referenceID")]
    pub reference_id: u64,
    #[serde(rename = "renderAmt")]
    pub render_amt: c_int,
    #[serde(rename = "renderToCubemaps")]
    pub render_to_cubemaps: bool,
    pub renderwithdynamic: bool,
    pub scales: Vector3,
    #[serde(rename = "smoothingAngle")]
    pub smoothing_angle: c_float,
    #[serde(rename = "tileHeightScale")]
    pub tile_height_scale: c_float,
    #[serde(rename = "tileMeshData")]
    pub tile_mesh_data: CDmeTileMesh<S>,
    #[serde(rename = "tileSetMapNames")]
    pub tile_set_map_names: Vec<S>,
    #[serde(rename = "tintColor")]
    pub tint_color: Color,
    #[serde(rename = "transformLocked")]
    pub transform_locked: bool,
    #[serde(rename = "useAsOccluder")]
    pub use_as_occluder: bool,
    #[serde(rename = "variableNames")]
    pub variable_names: Vec<S>,
    #[serde(rename = "variableTargetKeys")]
    pub variable_target_keys: Vec<S>,
    pub visexclude: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CMapTileSet<B, S> {
    pub angles: Qangle,
    pub children: Vec<Element<B, S>>,
    #[serde(rename = "convexityAngle")]
    pub convexity_angle: c_float,
    #[serde(rename = "defaultTileSize")]
    pub default_tile_size: Vector2,
    #[serde(rename = "editorOnly")]
    pub editor_only: bool,
    pub force_hidden: bool,
    #[serde(rename = "materialSets")]
    pub material_sets: Vec<Element<B, S>>,
    #[serde(rename = "nodeID")]
    pub node_id: c_int,
    pub origin: Vector3,
    pub properties: Vec<Element<B, S>>,
    #[serde(rename = "rampRuleSet")]
    pub ramp_rule_set: c_int,
    #[serde(rename = "referenceID")]
    pub reference_id: u64,
    pub scales: Vector3,
    #[serde(rename = "transformLocked")]
    pub transform_locked: bool,
    #[serde(rename = "variableNames")]
    pub variable_names: Vec<S>,
    #[serde(rename = "variableTargetKeys")]
    pub variable_target_keys: Vec<S>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CMapVariableChoice<S> {
    #[serde(rename = "m_ChoiceValues")]
    pub choice_values: Vec<S>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CMapVariableChoiceGroup<B, S> {
    #[serde(rename = "m_ActiveValue")]
    pub active_value: S,
    #[serde(rename = "m_ChoiceVariables")]
    pub choice_variables: Vec<S>,
    #[serde(rename = "m_Choices")]
    pub choices: Vec<Element<B, S>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CMapVariableSet<B, S> {
    #[serde(rename = "m_ChoiceGroups")]
    pub choice_groups: Vec<Element<B, S>>,
    #[serde(rename = "variableEditorOverrides")]
    pub variable_editor_overrides: Option<Vec<S>>,
    #[serde(rename = "variableNames")]
    pub variable_names: Vec<S>,
    #[serde(rename = "variableSubTypes")]
    pub variable_sub_types: Option<Vec<c_int>>,
    #[serde(rename = "variableTypeNames")]
    pub variable_type_names: Vec<S>,
    #[serde(rename = "variableTypeParameters")]
    pub variable_type_parameters: Option<Vec<S>>,
    #[serde(rename = "variableTypes")]
    pub variable_types: Option<Vec<c_int>>,
    #[serde(rename = "variableValues")]
    pub variable_values: Vec<S>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CMapWorld<B, S> {
    pub angles: Qangle,
    pub children: Vec<Element<B, S>>,
    #[serde(rename = "connectionsData")]
    pub connections_data: Vec<Element<B, S>>,
    #[serde(rename = "editorOnly")]
    pub editor_only: bool,
    pub entity_properties: EditGameClassProps<B, S>,
    #[serde(rename = "fixupEntityNames")]
    pub fixup_entity_names: bool,
    pub force_hidden: bool,
    #[serde(rename = "mapUsageType")]
    pub map_usage_type: S,
    #[serde(rename = "nextDecalID")]
    pub next_decal_id: c_int,
    #[serde(rename = "nodeID")]
    pub node_id: c_int,
    pub origin: Vector3,
    #[serde(rename = "referenceID")]
    pub reference_id: u64,
    #[serde(rename = "relayPlugData")]
    pub relay_plug_data: DmePlugList<S>,
    pub scales: Vector3,
    #[serde(rename = "transformLocked")]
    pub transform_locked: Option<bool>,
    #[serde(rename = "variableNames")]
    pub variable_names: Vec<S>,
    #[serde(rename = "variableTargetKeys")]
    pub variable_target_keys: Vec<S>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CObjectSelectionSetDataElement<B, S> {
    #[serde(rename = "selectedObjects")]
    pub selected_objects: Vec<Element<B, S>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CStoredCamera {
    pub lookat: Vector3,
    pub position: Vector3,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CStoredCameras<B, S> {
    pub activecamera: c_int,
    pub cameras: Vec<Element<B, S>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CTileSetMaterialSet<B, S> {
    #[serde(rename = "grassParams")]
    pub grass_params: Option<Element<B, S>>,
    #[serde(rename = "materialNames")]
    pub material_names: Vec<S>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CTileSetProperty<S> {
    #[serde(rename = "componentTypeFlags")]
    pub component_type_flags: c_int,
    #[serde(rename = "defaultValue")]
    pub default_value: c_int,
    #[serde(rename = "helperColor")]
    pub helper_color: Color,
    #[serde(rename = "helperName")]
    pub helper_name: S,
    #[serde(rename = "helperOffset")]
    pub helper_offset: c_float,
    pub procedural: bool,
    #[serde(rename = "targetTypeFlags")]
    pub target_type_flags: c_int,
    pub values: Vec<S>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CTrajectoryPath<B, S> {
    pub angles: Qangle,
    pub children: Vec<Element<B, S>>,
    #[serde(rename = "connectionsData")]
    pub connections_data: Vec<Element<B, S>>,
    #[serde(rename = "editorOnly")]
    pub editor_only: bool,
    pub entity_properties: EditGameClassProps<B, S>,
    #[serde(rename = "finalTime")]
    pub final_time: c_float,
    pub force_hidden: bool,
    pub gravity: Vector3,
    #[serde(rename = "hitNormal")]
    pub hit_normal: Vector3,
    #[serde(rename = "initialMass")]
    pub initial_mass: c_int,
    #[serde(rename = "interpolationType")]
    pub interpolation_type: c_int,
    #[serde(rename = "isProceduralEntity")]
    pub is_procedural_entity: bool,
    #[serde(rename = "maxThrust")]
    pub max_thrust: c_float,
    #[serde(rename = "minThrust")]
    pub min_thrust: c_float,
    #[serde(rename = "modelPath")]
    pub model_path: S,
    #[serde(rename = "nodeID")]
    pub node_id: c_int,
    #[serde(rename = "numTimeSteps")]
    pub num_time_steps: c_int,
    pub origin: Vector3,
    #[serde(rename = "referenceID")]
    pub reference_id: u64,
    #[serde(rename = "relayPlugData")]
    pub relay_plug_data: DmePlugList<S>,
    pub scales: Vector3,
    #[serde(rename = "specificImpulse")]
    pub specific_impulse: c_int,
    #[serde(rename = "transformLocked")]
    pub transform_locked: bool,
    #[serde(rename = "variableNames")]
    pub variable_names: Vec<S>,
    #[serde(rename = "variableTargetKeys")]
    pub variable_target_keys: Vec<S>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CTrajectoryPathNode<B, S> {
    pub angles: Qangle,
    #[serde(rename = "autoTime")]
    pub auto_time: bool,
    pub children: Vec<Element<B, S>>,
    #[serde(rename = "connectionsData")]
    pub connections_data: Vec<Element<B, S>>,
    #[serde(rename = "constrainVelocity")]
    pub constrain_velocity: bool,
    #[serde(rename = "editorOnly")]
    pub editor_only: bool,
    pub entity_properties: EditGameClassProps<B, S>,
    pub force_hidden: bool,
    #[serde(rename = "hitNormal")]
    pub hit_normal: Vector3,
    #[serde(rename = "inTangent")]
    pub in_tangent: Vector3,
    #[serde(rename = "inTangentType")]
    pub in_tangent_type: c_int,
    #[serde(rename = "isProceduralEntity")]
    pub is_procedural_entity: bool,
    #[serde(rename = "nodeID")]
    pub node_id: c_int,
    pub origin: Vector3,
    #[serde(rename = "outTangent")]
    pub out_tangent: Vector3,
    #[serde(rename = "outTangentType")]
    pub out_tangent_type: c_int,
    #[serde(rename = "pathNodeName")]
    pub path_node_name: S,
    #[serde(rename = "pinEnabled")]
    pub pin_enabled: bool,
    #[serde(rename = "posTolerance")]
    pub pos_tolerance: c_float,
    #[serde(rename = "radiusScale")]
    pub radius_scale: c_float,
    #[serde(rename = "referenceID")]
    pub reference_id: u64,
    #[serde(rename = "relayPlugData")]
    pub relay_plug_data: DmePlugList<S>,
    pub scales: Vector3,
    pub time: c_float,
    #[serde(rename = "tintColor")]
    pub tint_color: Color,
    #[serde(rename = "transformLocked")]
    pub transform_locked: bool,
    #[serde(rename = "variableNames")]
    pub variable_names: Vec<S>,
    #[serde(rename = "variableTargetKeys")]
    pub variable_target_keys: Vec<S>,
    #[serde(rename = "velTolerance")]
    pub vel_tolerance: c_float,
    pub velocity: Vector3,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CVisibilityMgr<B, S> {
    pub angles: Qangle,
    pub children: Vec<Element<B, S>>,
    #[serde(rename = "editorOnly")]
    pub editor_only: bool,
    pub force_hidden: bool,
    #[serde(rename = "hiddenFlags")]
    pub hidden_flags: Vec<c_int>,
    #[serde(rename = "nodeID")]
    pub node_id: c_int,
    pub nodes: Vec<Element<B, S>>,
    pub origin: Vector3,
    #[serde(rename = "referenceID")]
    pub reference_id: u64,
    pub scales: Vector3,
    #[serde(rename = "transformLocked")]
    pub transform_locked: Option<bool>,
    #[serde(rename = "variableNames")]
    pub variable_names: Vec<S>,
    #[serde(rename = "variableTargetKeys")]
    pub variable_target_keys: Vec<S>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DmElement<B, S> {
    #[serde(rename = "CanPhysPull")]
    pub can_phys_pull: Option<c_int>,
    #[serde(rename = "DoNotDrop")]
    pub do_not_drop: Option<c_int>,
    #[serde(rename = "ForceDropOnTeleport")]
    pub force_drop_on_teleport: Option<c_int>,
    #[serde(rename = "IgnoreConstraintOnPickup")]
    pub ignore_constraint_on_pickup: Option<c_int>,
    #[serde(rename = "LootableItem")]
    pub lootable_item: Option<c_int>,
    pub apply_carry_interactions_to_constraints: Option<c_int>,
    pub asset_preview_thumbnail: Option<B>,
    pub asset_preview_thumbnail_format: Option<S>,
    pub map_asset_references: Option<Vec<S>>,
    pub occlusion_scale: Option<c_float>,
    pub pitch: Option<c_float>,
    pub prevent_hand_to_hand_pickup: Option<c_int>,
    pub priority_energy_target: Option<c_int>,
    pub random_soundevent_01_timer_max: Option<c_float>,
    pub rigid_hold: Option<c_int>,
    pub volume_atten: Option<c_float>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DmeConnectionData<S> {
    pub delay: c_float,
    #[serde(rename = "inputName")]
    pub input_name: S,
    #[serde(rename = "outputName")]
    pub output_name: S,
    #[serde(rename = "overrideParam")]
    pub override_param: S,
    #[serde(rename = "targetName")]
    pub target_name: S,
    #[serde(rename = "targetType")]
    pub target_type: c_int,
    #[serde(rename = "timesToFire")]
    pub times_to_fire: c_int,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DmePlugList<S> {
    #[serde(rename = "dataTypes")]
    pub data_types: Vec<c_int>,
    pub descriptions: Vec<S>,
    pub names: Vec<S>,
    #[serde(rename = "plugTypes")]
    pub plug_types: Vec<c_int>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DmeVertexData<S> {
    #[serde(rename = "PerVertexLighting")]
    pub per_vertex_lighting: Option<Vec<Vector4>>,
    #[serde(rename = "PerVertexLightingIndices")]
    pub per_vertex_lighting_indices: Option<Vec<c_int>>,
    #[serde(rename = "VertexPaintBlendParams")]
    pub vertex_paint_blend_params: Option<Vec<Vector4>>,
    #[serde(rename = "VertexPaintBlendParamsIndices")]
    pub vertex_paint_blend_params_indices: Option<Vec<c_int>>,
    #[serde(rename = "flipVCoordinates")]
    pub flip_vcoordinates: bool,
    #[serde(rename = "jointCount")]
    pub joint_count: c_int,
    #[serde(rename = "vertexFormat")]
    pub vertex_format: Vec<S>,
}
