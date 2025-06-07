# ImagePaint

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**visible** | Option<**bool**> | Is the paint enabled? | [optional][default to true]
**opacity** | Option<**f64**> | Overall opacity of paint (colors within the paint can also have opacity values which would blend with this) | [optional][default to 1]
**blend_mode** | [**models::BlendMode**](BlendMode.md) | How this node blends with nodes behind it in the scene | 
**r#type** | **String** | The string literal \"IMAGE\" representing the paint's type. Always check the `type` before reading other properties. | 
**scale_mode** | **String** | Image scaling mode. | 
**image_ref** | **String** | A reference to an image embedded in this node. To download the image using this reference, use the `GET file images` endpoint to retrieve the mapping from image references to image URLs. | 
**image_transform** | Option<[**Vec<Vec<f64>>**](Vec.md)> | A transformation matrix is standard way in computer graphics to represent translation and rotation. These are the top two rows of a 3x3 matrix. The bottom row of the matrix is assumed to be [0, 0, 1]. This is known as an affine transform and is enough to represent translation, rotation, and skew.  The identity transform is [[1, 0, 0], [0, 1, 0]].  A translation matrix will typically look like:  ``` [[1, 0, tx],   [0, 1, ty]] ```  and a rotation matrix will typically look like:  ``` [[cos(angle), sin(angle), 0],   [-sin(angle), cos(angle), 0]] ```  Another way to think about this transform is as three vectors:  - The x axis (t[0][0], t[1][0]) - The y axis (t[0][1], t[1][1]) - The translation offset (t[0][2], t[1][2])  The most common usage of the Transform matrix is the `relativeTransform property`. This particular usage of the matrix has a few additional restrictions. The translation offset can take on any value but we do enforce that the axis vectors are unit vectors (i.e. have length 1). The axes are not required to be at 90° angles to each other. | [optional]
**scaling_factor** | Option<**f64**> | Amount image is scaled by in tiling, only present if scaleMode is `TILE`. | [optional]
**filters** | Option<[**models::ImageFilters**](ImageFilters.md)> | Defines what image filters have been applied to this paint, if any. If this property is not defined, no filters have been applied. | [optional]
**rotation** | Option<**f64**> | Image rotation, in degrees. | [optional][default to 0]
**gif_ref** | Option<**String**> | A reference to an animated GIF embedded in this node. To download the image using this reference, use the `GET file images` endpoint to retrieve the mapping from image references to image URLs. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


