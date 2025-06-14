# Rust API client for figma-api

This is the OpenAPI specification for the [Figma REST API](https://www.figma.com/developers/api).

Note: we are releasing the OpenAPI specification as a beta given the large surface area and complexity of the REST API. If you notice any inaccuracies with the specification, please [file an issue](https://github.com/figma/rest-api-spec/issues).


## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 0.31.0
- Package version: 0.31.0
- Generator version: 7.13.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `figma-api` and add the following to `Cargo.toml` under `[dependencies]`:

```
figma-api = { path = "./figma-api" }
```

## Documentation for API Endpoints

All URIs are relative to *https://api.figma.com*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*ComponentSetsApi* | [**get_component_set**](docs/ComponentSetsApi.md#get_component_set) | **GET** /v1/component_sets/{key} | Get component set
*ComponentSetsApi* | [**get_file_component_sets**](docs/ComponentSetsApi.md#get_file_component_sets) | **GET** /v1/files/{file_key}/component_sets | Get file component sets
*ComponentsApi* | [**get_component**](docs/ComponentsApi.md#get_component) | **GET** /v1/components/{key} | Get component
*ComponentsApi* | [**get_file_components**](docs/ComponentsApi.md#get_file_components) | **GET** /v1/files/{file_key}/components | Get file components
*FilesApi* | [**get_file**](docs/FilesApi.md#get_file) | **GET** /v1/files/{file_key} | Get file JSON
*FilesApi* | [**get_file_meta**](docs/FilesApi.md#get_file_meta) | **GET** /v1/files/{file_key}/meta | Get file metadata
*FilesApi* | [**get_file_nodes**](docs/FilesApi.md#get_file_nodes) | **GET** /v1/files/{file_key}/nodes | Get file JSON for specific nodes
*FilesApi* | [**get_image_fills**](docs/FilesApi.md#get_image_fills) | **GET** /v1/files/{file_key}/images | Get image fills
*FilesApi* | [**get_images**](docs/FilesApi.md#get_images) | **GET** /v1/images/{file_key} | Render images of file nodes
*StylesApi* | [**get_file_styles**](docs/StylesApi.md#get_file_styles) | **GET** /v1/files/{file_key}/styles | Get file styles
*StylesApi* | [**get_style**](docs/StylesApi.md#get_style) | **GET** /v1/styles/{key} | Get style


## Documentation For Models

 - [Action](docs/Action.md)
 - [ActionOneOf](docs/ActionOneOf.md)
 - [AfterTimeoutTrigger](docs/AfterTimeoutTrigger.md)
 - [ArcData](docs/ArcData.md)
 - [BackgroundBlurEffect](docs/BackgroundBlurEffect.md)
 - [BaseBlurEffect](docs/BaseBlurEffect.md)
 - [BaseBlurEffectBoundVariables](docs/BaseBlurEffectBoundVariables.md)
 - [BaseNoiseEffect](docs/BaseNoiseEffect.md)
 - [BasePaint](docs/BasePaint.md)
 - [BaseShadowEffect](docs/BaseShadowEffect.md)
 - [BaseShadowEffectBoundVariables](docs/BaseShadowEffectBoundVariables.md)
 - [BaseTypeStyle](docs/BaseTypeStyle.md)
 - [BlendMode](docs/BlendMode.md)
 - [BlurEffect](docs/BlurEffect.md)
 - [BooleanOperationNode](docs/BooleanOperationNode.md)
 - [CanvasNode](docs/CanvasNode.md)
 - [ColorStop](docs/ColorStop.md)
 - [ColorStopBoundVariables](docs/ColorStopBoundVariables.md)
 - [Comment](docs/Comment.md)
 - [CommentClientMeta](docs/CommentClientMeta.md)
 - [CommentFragment](docs/CommentFragment.md)
 - [Component](docs/Component.md)
 - [ComponentNode](docs/ComponentNode.md)
 - [ComponentPropertiesTrait](docs/ComponentPropertiesTrait.md)
 - [ComponentProperty](docs/ComponentProperty.md)
 - [ComponentPropertyBoundVariables](docs/ComponentPropertyBoundVariables.md)
 - [ComponentPropertyDefinition](docs/ComponentPropertyDefinition.md)
 - [ComponentPropertyDefinitionDefaultValue](docs/ComponentPropertyDefinitionDefaultValue.md)
 - [ComponentPropertyType](docs/ComponentPropertyType.md)
 - [ComponentPropertyValue](docs/ComponentPropertyValue.md)
 - [ComponentSet](docs/ComponentSet.md)
 - [ComponentSetNode](docs/ComponentSetNode.md)
 - [ConditionalAction](docs/ConditionalAction.md)
 - [ConditionalBlock](docs/ConditionalBlock.md)
 - [ConnectorEndpoint](docs/ConnectorEndpoint.md)
 - [ConnectorEndpointOneOf](docs/ConnectorEndpointOneOf.md)
 - [ConnectorEndpointOneOf1](docs/ConnectorEndpointOneOf1.md)
 - [ConnectorLineType](docs/ConnectorLineType.md)
 - [ConnectorNode](docs/ConnectorNode.md)
 - [ConnectorTextBackground](docs/ConnectorTextBackground.md)
 - [Constraint](docs/Constraint.md)
 - [CornerRadiusShapeTraits](docs/CornerRadiusShapeTraits.md)
 - [CornerTrait](docs/CornerTrait.md)
 - [DefaultShapeTraits](docs/DefaultShapeTraits.md)
 - [DirectionalTransition](docs/DirectionalTransition.md)
 - [DocumentNode](docs/DocumentNode.md)
 - [DocumentationLink](docs/DocumentationLink.md)
 - [DropShadowEffect](docs/DropShadowEffect.md)
 - [DuotoneNoiseEffect](docs/DuotoneNoiseEffect.md)
 - [Easing](docs/Easing.md)
 - [EasingEasingFunctionCubicBezier](docs/EasingEasingFunctionCubicBezier.md)
 - [EasingEasingFunctionSpring](docs/EasingEasingFunctionSpring.md)
 - [EasingType](docs/EasingType.md)
 - [Effect](docs/Effect.md)
 - [EllipseNode](docs/EllipseNode.md)
 - [EmbedNode](docs/EmbedNode.md)
 - [ErrorResponsePayloadWithErrMessage](docs/ErrorResponsePayloadWithErrMessage.md)
 - [ErrorResponsePayloadWithErrorBoolean](docs/ErrorResponsePayloadWithErrorBoolean.md)
 - [ExportSetting](docs/ExportSetting.md)
 - [Expression](docs/Expression.md)
 - [ExpressionFunction](docs/ExpressionFunction.md)
 - [FlowStartingPoint](docs/FlowStartingPoint.md)
 - [FrameInfo](docs/FrameInfo.md)
 - [FrameInfoContainingComponentSet](docs/FrameInfoContainingComponentSet.md)
 - [FrameInfoContainingStateGroup](docs/FrameInfoContainingStateGroup.md)
 - [FrameNode](docs/FrameNode.md)
 - [FrameOffset](docs/FrameOffset.md)
 - [FrameOffsetRegion](docs/FrameOffsetRegion.md)
 - [FrameTraits](docs/FrameTraits.md)
 - [GradientPaint](docs/GradientPaint.md)
 - [GroupNode](docs/GroupNode.md)
 - [HasBlendModeAndOpacityTrait](docs/HasBlendModeAndOpacityTrait.md)
 - [HasChildrenTrait](docs/HasChildrenTrait.md)
 - [HasEffectsTrait](docs/HasEffectsTrait.md)
 - [HasExportSettingsTrait](docs/HasExportSettingsTrait.md)
 - [HasFramePropertiesTrait](docs/HasFramePropertiesTrait.md)
 - [HasGeometryTrait](docs/HasGeometryTrait.md)
 - [HasLayoutTrait](docs/HasLayoutTrait.md)
 - [HasMaskTrait](docs/HasMaskTrait.md)
 - [HasTextSublayerTrait](docs/HasTextSublayerTrait.md)
 - [Hyperlink](docs/Hyperlink.md)
 - [ImageFilters](docs/ImageFilters.md)
 - [ImagePaint](docs/ImagePaint.md)
 - [IndividualStrokesTrait](docs/IndividualStrokesTrait.md)
 - [InlineObject](docs/InlineObject.md)
 - [InlineObject1](docs/InlineObject1.md)
 - [InlineObject10](docs/InlineObject10.md)
 - [InlineObject10Meta](docs/InlineObject10Meta.md)
 - [InlineObject11](docs/InlineObject11.md)
 - [InlineObject11Meta](docs/InlineObject11Meta.md)
 - [InlineObject12](docs/InlineObject12.md)
 - [InlineObject13](docs/InlineObject13.md)
 - [InlineObject13Meta](docs/InlineObject13Meta.md)
 - [InlineObject14](docs/InlineObject14.md)
 - [InlineObject14Meta](docs/InlineObject14Meta.md)
 - [InlineObject15](docs/InlineObject15.md)
 - [InlineObject15Meta](docs/InlineObject15Meta.md)
 - [InlineObject16](docs/InlineObject16.md)
 - [InlineObject17](docs/InlineObject17.md)
 - [InlineObject18](docs/InlineObject18.md)
 - [InlineObject19](docs/InlineObject19.md)
 - [InlineObject1NodesValue](docs/InlineObject1NodesValue.md)
 - [InlineObject2](docs/InlineObject2.md)
 - [InlineObject20](docs/InlineObject20.md)
 - [InlineObject21](docs/InlineObject21.md)
 - [InlineObject22](docs/InlineObject22.md)
 - [InlineObject23](docs/InlineObject23.md)
 - [InlineObject24](docs/InlineObject24.md)
 - [InlineObject25](docs/InlineObject25.md)
 - [InlineObject26](docs/InlineObject26.md)
 - [InlineObject3](docs/InlineObject3.md)
 - [InlineObject3Meta](docs/InlineObject3Meta.md)
 - [InlineObject4](docs/InlineObject4.md)
 - [InlineObject5](docs/InlineObject5.md)
 - [InlineObject6](docs/InlineObject6.md)
 - [InlineObject6Meta](docs/InlineObject6Meta.md)
 - [InlineObject7](docs/InlineObject7.md)
 - [InlineObject8](docs/InlineObject8.md)
 - [InlineObject8Meta](docs/InlineObject8Meta.md)
 - [InlineObject9](docs/InlineObject9.md)
 - [InlineObjectBranchesInner](docs/InlineObjectBranchesInner.md)
 - [InnerShadowEffect](docs/InnerShadowEffect.md)
 - [InstanceNode](docs/InstanceNode.md)
 - [InstanceSwapPreferredValue](docs/InstanceSwapPreferredValue.md)
 - [Interaction](docs/Interaction.md)
 - [IsLayerTrait](docs/IsLayerTrait.md)
 - [IsLayerTraitBoundVariables](docs/IsLayerTraitBoundVariables.md)
 - [IsLayerTraitBoundVariablesIndividualStrokeWeights](docs/IsLayerTraitBoundVariablesIndividualStrokeWeights.md)
 - [IsLayerTraitBoundVariablesRectangleCornerRadii](docs/IsLayerTraitBoundVariablesRectangleCornerRadii.md)
 - [IsLayerTraitBoundVariablesSize](docs/IsLayerTraitBoundVariablesSize.md)
 - [LayerBlurEffect](docs/LayerBlurEffect.md)
 - [LayoutConstraint](docs/LayoutConstraint.md)
 - [LayoutGrid](docs/LayoutGrid.md)
 - [LayoutGridBoundVariables](docs/LayoutGridBoundVariables.md)
 - [LibraryItemData](docs/LibraryItemData.md)
 - [LineNode](docs/LineNode.md)
 - [LinkUnfurlNode](docs/LinkUnfurlNode.md)
 - [LocalVariable](docs/LocalVariable.md)
 - [LocalVariableCollection](docs/LocalVariableCollection.md)
 - [LocalVariableCollectionModesInner](docs/LocalVariableCollectionModesInner.md)
 - [LocalVariableValuesByModeValue](docs/LocalVariableValuesByModeValue.md)
 - [Measurement](docs/Measurement.md)
 - [MeasurementOffset](docs/MeasurementOffset.md)
 - [MeasurementOffsetInner](docs/MeasurementOffsetInner.md)
 - [MeasurementOffsetOuter](docs/MeasurementOffsetOuter.md)
 - [MeasurementStartEnd](docs/MeasurementStartEnd.md)
 - [MinimalFillsTrait](docs/MinimalFillsTrait.md)
 - [MinimalStrokesTrait](docs/MinimalStrokesTrait.md)
 - [MonotoneNoiseEffect](docs/MonotoneNoiseEffect.md)
 - [MultitoneNoiseEffect](docs/MultitoneNoiseEffect.md)
 - [Navigation](docs/Navigation.md)
 - [Node](docs/Node.md)
 - [NodeAction](docs/NodeAction.md)
 - [NoiseEffect](docs/NoiseEffect.md)
 - [NormalBlurEffect](docs/NormalBlurEffect.md)
 - [OnKeyDownTrigger](docs/OnKeyDownTrigger.md)
 - [OnMediaHitTrigger](docs/OnMediaHitTrigger.md)
 - [OpenUrlAction](docs/OpenUrlAction.md)
 - [Overrides](docs/Overrides.md)
 - [Paint](docs/Paint.md)
 - [PaintOverride](docs/PaintOverride.md)
 - [Path](docs/Path.md)
 - [PatternPaint](docs/PatternPaint.md)
 - [ProgressiveBlurEffect](docs/ProgressiveBlurEffect.md)
 - [PrototypeDevice](docs/PrototypeDevice.md)
 - [PublishedComponent](docs/PublishedComponent.md)
 - [PublishedComponentSet](docs/PublishedComponentSet.md)
 - [PublishedStyle](docs/PublishedStyle.md)
 - [PublishedVariable](docs/PublishedVariable.md)
 - [PublishedVariableCollection](docs/PublishedVariableCollection.md)
 - [Reaction](docs/Reaction.md)
 - [Rectangle](docs/Rectangle.md)
 - [RectangleNode](docs/RectangleNode.md)
 - [RectangularShapeTraits](docs/RectangularShapeTraits.md)
 - [Region](docs/Region.md)
 - [RegularPolygonNode](docs/RegularPolygonNode.md)
 - [ResponseCursor](docs/ResponseCursor.md)
 - [ResponsePagination](docs/ResponsePagination.md)
 - [Rgb](docs/Rgb.md)
 - [Rgba](docs/Rgba.md)
 - [SectionNode](docs/SectionNode.md)
 - [SetVariableAction](docs/SetVariableAction.md)
 - [SetVariableModeAction](docs/SetVariableModeAction.md)
 - [ShapeType](docs/ShapeType.md)
 - [ShapeWithTextNode](docs/ShapeWithTextNode.md)
 - [SimpleTransition](docs/SimpleTransition.md)
 - [Size](docs/Size.md)
 - [SliceNode](docs/SliceNode.md)
 - [SolidPaint](docs/SolidPaint.md)
 - [SolidPaintAllOfBoundVariables](docs/SolidPaintAllOfBoundVariables.md)
 - [StarNode](docs/StarNode.md)
 - [StickyNode](docs/StickyNode.md)
 - [StrokeWeights](docs/StrokeWeights.md)
 - [Style](docs/Style.md)
 - [StyleType](docs/StyleType.md)
 - [SubcanvasNode](docs/SubcanvasNode.md)
 - [TableCellNode](docs/TableCellNode.md)
 - [TableNode](docs/TableNode.md)
 - [TextNode](docs/TextNode.md)
 - [TextPathNode](docs/TextPathNode.md)
 - [TextPathPropertiesTrait](docs/TextPathPropertiesTrait.md)
 - [TextPathTypeStyle](docs/TextPathTypeStyle.md)
 - [TextPathTypeStyleAllOfBoundVariables](docs/TextPathTypeStyleAllOfBoundVariables.md)
 - [TextureEffect](docs/TextureEffect.md)
 - [TransformGroupNode](docs/TransformGroupNode.md)
 - [Transition](docs/Transition.md)
 - [TransitionSourceTrait](docs/TransitionSourceTrait.md)
 - [Trigger](docs/Trigger.md)
 - [TriggerOneOf](docs/TriggerOneOf.md)
 - [TriggerOneOf1](docs/TriggerOneOf1.md)
 - [TriggerOneOf2](docs/TriggerOneOf2.md)
 - [TypePropertiesTrait](docs/TypePropertiesTrait.md)
 - [TypeStyle](docs/TypeStyle.md)
 - [TypeStyleAllOfBoundVariables](docs/TypeStyleAllOfBoundVariables.md)
 - [UpdateMediaRuntimeAction](docs/UpdateMediaRuntimeAction.md)
 - [UpdateMediaRuntimeActionOneOf](docs/UpdateMediaRuntimeActionOneOf.md)
 - [UpdateMediaRuntimeActionOneOf1](docs/UpdateMediaRuntimeActionOneOf1.md)
 - [UpdateMediaRuntimeActionOneOf2](docs/UpdateMediaRuntimeActionOneOf2.md)
 - [User](docs/User.md)
 - [VariableAlias](docs/VariableAlias.md)
 - [VariableChange](docs/VariableChange.md)
 - [VariableCodeSyntax](docs/VariableCodeSyntax.md)
 - [VariableCollectionChange](docs/VariableCollectionChange.md)
 - [VariableCollectionCreate](docs/VariableCollectionCreate.md)
 - [VariableCollectionDelete](docs/VariableCollectionDelete.md)
 - [VariableCollectionUpdate](docs/VariableCollectionUpdate.md)
 - [VariableCreate](docs/VariableCreate.md)
 - [VariableData](docs/VariableData.md)
 - [VariableDataType](docs/VariableDataType.md)
 - [VariableDataValue](docs/VariableDataValue.md)
 - [VariableDelete](docs/VariableDelete.md)
 - [VariableModeChange](docs/VariableModeChange.md)
 - [VariableModeCreate](docs/VariableModeCreate.md)
 - [VariableModeDelete](docs/VariableModeDelete.md)
 - [VariableModeUpdate](docs/VariableModeUpdate.md)
 - [VariableModeValue](docs/VariableModeValue.md)
 - [VariableResolvedDataType](docs/VariableResolvedDataType.md)
 - [VariableScope](docs/VariableScope.md)
 - [VariableUpdate](docs/VariableUpdate.md)
 - [VariableValue](docs/VariableValue.md)
 - [Vector](docs/Vector.md)
 - [VectorNode](docs/VectorNode.md)
 - [Version](docs/Version.md)
 - [WashiTapeNode](docs/WashiTapeNode.md)
 - [WidgetNode](docs/WidgetNode.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author

support@figma.com

