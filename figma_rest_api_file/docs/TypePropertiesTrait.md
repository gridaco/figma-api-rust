# TypePropertiesTrait

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**characters** | **String** | The raw characters in the text node. | 
**style** | [**models::TypeStyle**](TypeStyle.md) | Style of text including font family and weight. | 
**character_style_overrides** | **Vec<f64>** | The array corresponds to characters in the text box, where each element references the 'styleOverrideTable' to apply specific styles to each character. The array's length can be less than or equal to the number of characters due to the removal of trailing zeros. Elements with a value of 0 indicate characters that use the default type style. If the array is shorter than the total number of characters, the characters beyond the array's length also use the default style. | 
**layout_version** | Option<**f64**> | Internal property, preserved for backward compatibility. Avoid using this value. | [optional]
**style_override_table** | [**std::collections::HashMap<String, models::TypeStyle>**](TypeStyle.md) | Map from ID to TypeStyle for looking up style overrides. | 
**line_types** | **Vec<String>** | An array with the same number of elements as lines in the text node, where lines are delimited by newline or paragraph separator characters. Each element in the array corresponds to the list type of a specific line. List types are represented as string enums with one of these possible values:  - `NONE`: Not a list item. - `ORDERED`: Text is an ordered list (numbered). - `UNORDERED`: Text is an unordered list (bulleted). | 
**line_indentations** | **Vec<f64>** | An array with the same number of elements as lines in the text node, where lines are delimited by newline or paragraph separator characters. Each element in the array corresponds to the indentation level of a specific line. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


