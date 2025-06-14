/*
 * Figma API
 *
 * This is the OpenAPI specification for the [Figma REST API](https://www.figma.com/developers/api).  Note: we are releasing the OpenAPI specification as a beta given the large surface area and complexity of the REST API. If you notice any inaccuracies with the specification, please [file an issue](https://github.com/figma/rest-api-spec/issues).
 *
 * The version of the OpenAPI document: 0.31.0
 * Contact: support@figma.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// ShapeType : Geometric shape type.
/// Geometric shape type.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ShapeType {
    #[serde(rename = "SQUARE")]
    Square,
    #[serde(rename = "ELLIPSE")]
    Ellipse,
    #[serde(rename = "ROUNDED_RECTANGLE")]
    RoundedRectangle,
    #[serde(rename = "DIAMOND")]
    Diamond,
    #[serde(rename = "TRIANGLE_UP")]
    TriangleUp,
    #[serde(rename = "TRIANGLE_DOWN")]
    TriangleDown,
    #[serde(rename = "PARALLELOGRAM_RIGHT")]
    ParallelogramRight,
    #[serde(rename = "PARALLELOGRAM_LEFT")]
    ParallelogramLeft,
    #[serde(rename = "ENG_DATABASE")]
    EngDatabase,
    #[serde(rename = "ENG_QUEUE")]
    EngQueue,
    #[serde(rename = "ENG_FILE")]
    EngFile,
    #[serde(rename = "ENG_FOLDER")]
    EngFolder,
    #[serde(rename = "TRAPEZOID")]
    Trapezoid,
    #[serde(rename = "PREDEFINED_PROCESS")]
    PredefinedProcess,
    #[serde(rename = "SHIELD")]
    Shield,
    #[serde(rename = "DOCUMENT_SINGLE")]
    DocumentSingle,
    #[serde(rename = "DOCUMENT_MULTIPLE")]
    DocumentMultiple,
    #[serde(rename = "MANUAL_INPUT")]
    ManualInput,
    #[serde(rename = "HEXAGON")]
    Hexagon,
    #[serde(rename = "CHEVRON")]
    Chevron,
    #[serde(rename = "PENTAGON")]
    Pentagon,
    #[serde(rename = "OCTAGON")]
    Octagon,
    #[serde(rename = "STAR")]
    Star,
    #[serde(rename = "PLUS")]
    Plus,
    #[serde(rename = "ARROW_LEFT")]
    ArrowLeft,
    #[serde(rename = "ARROW_RIGHT")]
    ArrowRight,
    #[serde(rename = "SUMMING_JUNCTION")]
    SummingJunction,
    #[serde(rename = "OR")]
    Or,
    #[serde(rename = "SPEECH_BUBBLE")]
    SpeechBubble,
    #[serde(rename = "INTERNAL_STORAGE")]
    InternalStorage,

}

impl std::fmt::Display for ShapeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Square => write!(f, "SQUARE"),
            Self::Ellipse => write!(f, "ELLIPSE"),
            Self::RoundedRectangle => write!(f, "ROUNDED_RECTANGLE"),
            Self::Diamond => write!(f, "DIAMOND"),
            Self::TriangleUp => write!(f, "TRIANGLE_UP"),
            Self::TriangleDown => write!(f, "TRIANGLE_DOWN"),
            Self::ParallelogramRight => write!(f, "PARALLELOGRAM_RIGHT"),
            Self::ParallelogramLeft => write!(f, "PARALLELOGRAM_LEFT"),
            Self::EngDatabase => write!(f, "ENG_DATABASE"),
            Self::EngQueue => write!(f, "ENG_QUEUE"),
            Self::EngFile => write!(f, "ENG_FILE"),
            Self::EngFolder => write!(f, "ENG_FOLDER"),
            Self::Trapezoid => write!(f, "TRAPEZOID"),
            Self::PredefinedProcess => write!(f, "PREDEFINED_PROCESS"),
            Self::Shield => write!(f, "SHIELD"),
            Self::DocumentSingle => write!(f, "DOCUMENT_SINGLE"),
            Self::DocumentMultiple => write!(f, "DOCUMENT_MULTIPLE"),
            Self::ManualInput => write!(f, "MANUAL_INPUT"),
            Self::Hexagon => write!(f, "HEXAGON"),
            Self::Chevron => write!(f, "CHEVRON"),
            Self::Pentagon => write!(f, "PENTAGON"),
            Self::Octagon => write!(f, "OCTAGON"),
            Self::Star => write!(f, "STAR"),
            Self::Plus => write!(f, "PLUS"),
            Self::ArrowLeft => write!(f, "ARROW_LEFT"),
            Self::ArrowRight => write!(f, "ARROW_RIGHT"),
            Self::SummingJunction => write!(f, "SUMMING_JUNCTION"),
            Self::Or => write!(f, "OR"),
            Self::SpeechBubble => write!(f, "SPEECH_BUBBLE"),
            Self::InternalStorage => write!(f, "INTERNAL_STORAGE"),
        }
    }
}

impl Default for ShapeType {
    fn default() -> ShapeType {
        Self::Square
    }
}

