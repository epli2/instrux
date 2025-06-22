/// Error types.
pub mod error {
    /// Error from a `TryFrom` or `FromStr` implementation.
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(
            &self,
            f: &mut ::std::fmt::Formatter<'_>,
        ) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(
            &self,
            f: &mut ::std::fmt::Formatter<'_>,
        ) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
///`InstructionItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "anyOf": [
///    {
///      "required": [
///        "body"
///      ]
///    },
///    {
///      "required": [
///        "body_file"
///      ]
///    },
///    {
///      "required": [
///        "instructions"
///      ]
///    }
///  ],
///  "required": [
///    "title"
///  ],
///  "properties": {
///    "body": {
///      "description": "Instruction body content",
///      "type": "string"
///    },
///    "body_file": {
///      "description": "Name of file containing the instruction body",
///      "type": "string"
///    },
///    "description": {
///      "description": "Detailed description of the instruction",
///      "type": "string"
///    },
///    "disable": {
///      "description": "Disable this instruction",
///      "default": false,
///      "type": "boolean"
///    },
///    "instructions": {
///      "description": "Nested instructions",
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/instruction_item"
///      }
///    },
///    "targets": {
///      "default": "all",
///      "oneOf": [
///        {
///          "description": "List of targets to generate this instruction for",
///          "type": "array",
///          "items": {
///            "$ref": "#/$defs/targets"
///          }
///        },
///        {
///          "description": "Apply to all targets",
///          "type": "string",
///          "const": "all"
///        }
///      ]
///    },
///    "title": {
///      "description": "Title of the instruction",
///      "type": "string"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged, deny_unknown_fields)]
pub enum InstructionItem {
    Variant0 {
        ///Instruction body content
        body: ::std::string::String,
        ///Detailed description of the instruction
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        description: ::std::option::Option<::std::string::String>,
        ///Disable this instruction
        #[serde(default)]
        disable: bool,
        #[serde(default = "defaults::instruction_item_variant0_targets")]
        targets: InstructionItemVariant0Targets,
        ///Title of the instruction
        title: ::std::string::String,
    },
    Variant1 {
        ///Name of file containing the instruction body
        body_file: ::std::string::String,
        ///Detailed description of the instruction
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        description: ::std::option::Option<::std::string::String>,
        ///Disable this instruction
        #[serde(default)]
        disable: bool,
        #[serde(default = "defaults::instruction_item_variant1_targets")]
        targets: InstructionItemVariant1Targets,
        ///Title of the instruction
        title: ::std::string::String,
    },
    Variant2 {
        ///Detailed description of the instruction
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        description: ::std::option::Option<::std::string::String>,
        ///Disable this instruction
        #[serde(default)]
        disable: bool,
        ///Nested instructions
        instructions: ::std::vec::Vec<InstructionItem>,
        #[serde(default = "defaults::instruction_item_variant2_targets")]
        targets: InstructionItemVariant2Targets,
        ///Title of the instruction
        title: ::std::string::String,
    },
}
impl ::std::convert::From<&Self> for InstructionItem {
    fn from(value: &InstructionItem) -> Self {
        value.clone()
    }
}
///`InstructionItemVariant0Targets`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "default": "all",
///  "oneOf": [
///    {
///      "description": "List of targets to generate this instruction for",
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/targets"
///      }
///    },
///    {
///      "description": "Apply to all targets",
///      "type": "string",
///      "const": "all"
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum InstructionItemVariant0Targets {
    Variant0(::std::vec::Vec<Targets>),
    Variant1(::std::string::String),
}
impl ::std::convert::From<&Self> for InstructionItemVariant0Targets {
    fn from(value: &InstructionItemVariant0Targets) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for InstructionItemVariant0Targets {
    fn default() -> Self {
        InstructionItemVariant0Targets::Variant1("all".to_string())
    }
}
impl ::std::convert::From<::std::vec::Vec<Targets>> for InstructionItemVariant0Targets {
    fn from(value: ::std::vec::Vec<Targets>) -> Self {
        Self::Variant0(value)
    }
}
///`InstructionItemVariant1Targets`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "default": "all",
///  "oneOf": [
///    {
///      "description": "List of targets to generate this instruction for",
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/targets"
///      }
///    },
///    {
///      "description": "Apply to all targets",
///      "type": "string",
///      "const": "all"
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum InstructionItemVariant1Targets {
    Variant0(::std::vec::Vec<Targets>),
    Variant1(::std::string::String),
}
impl ::std::convert::From<&Self> for InstructionItemVariant1Targets {
    fn from(value: &InstructionItemVariant1Targets) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for InstructionItemVariant1Targets {
    fn default() -> Self {
        InstructionItemVariant1Targets::Variant1("all".to_string())
    }
}
impl ::std::convert::From<::std::vec::Vec<Targets>> for InstructionItemVariant1Targets {
    fn from(value: ::std::vec::Vec<Targets>) -> Self {
        Self::Variant0(value)
    }
}
///`InstructionItemVariant2Targets`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "default": "all",
///  "oneOf": [
///    {
///      "description": "List of targets to generate this instruction for",
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/targets"
///      }
///    },
///    {
///      "description": "Apply to all targets",
///      "type": "string",
///      "const": "all"
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum InstructionItemVariant2Targets {
    Variant0(::std::vec::Vec<Targets>),
    Variant1(::std::string::String),
}
impl ::std::convert::From<&Self> for InstructionItemVariant2Targets {
    fn from(value: &InstructionItemVariant2Targets) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for InstructionItemVariant2Targets {
    fn default() -> Self {
        InstructionItemVariant2Targets::Variant1("all".to_string())
    }
}
impl ::std::convert::From<::std::vec::Vec<Targets>> for InstructionItemVariant2Targets {
    fn from(value: ::std::vec::Vec<Targets>) -> Self {
        Self::Variant0(value)
    }
}
///`InstruxConfiguration`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Instrux Configuration",
///  "type": "object",
///  "required": [
///    "targets",
///    "version"
///  ],
///  "properties": {
///    "instructions": {
///      "description": "List of instructions to generate",
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/instruction_item"
///      },
///      "minItems": 1
///    },
///    "language": {
///      "description": "Human language for generated instructions",
///      "default": "English",
///      "type": "string",
///      "enum": [
///        "English",
///        "Japanese"
///      ]
///    },
///    "targets": {
///      "description": "AI coding tool formats to generate, each as a property with optional settings",
///      "type": "object",
///      "minProperties": 1,
///      "additionalProperties": {
///        "type": "object",
///        "properties": {
///          "outputMode": {
///            "description": "Whether to output to a single file or multiple files for this target",
///            "default": "single",
///            "type": "string",
///            "enum": [
///              "single",
///              "multiple"
///            ]
///          }
///        },
///        "additionalProperties": false
///      },
///      "propertyNames": {
///        "$ref": "#/$defs/targets"
///      }
///    },
///    "version": {
///      "description": "Semantic version of this config file",
///      "type": "string",
///      "pattern": "^\\d+\\.\\d+\\.\\d+(-[a-zA-Z0-9\\.]+)?$"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct InstruxConfiguration {
    ///List of instructions to generate
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub instructions: ::std::vec::Vec<InstructionItem>,
    ///Human language for generated instructions
    #[serde(default = "defaults::instrux_configuration_language")]
    pub language: InstruxConfigurationLanguage,
    ///AI coding tool formats to generate, each as a property with optional settings
    pub targets: ::std::collections::HashMap<Targets, InstruxConfigurationTargetsValue>,
    ///Semantic version of this config file
    pub version: InstruxConfigurationVersion,
}
impl ::std::convert::From<&InstruxConfiguration> for InstruxConfiguration {
    fn from(value: &InstruxConfiguration) -> Self {
        value.clone()
    }
}
impl InstruxConfiguration {
    pub fn builder() -> builder::InstruxConfiguration {
        Default::default()
    }
}
///Human language for generated instructions
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Human language for generated instructions",
///  "default": "English",
///  "type": "string",
///  "enum": [
///    "English",
///    "Japanese"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum InstruxConfigurationLanguage {
    English,
    Japanese,
}
impl ::std::convert::From<&Self> for InstruxConfigurationLanguage {
    fn from(value: &InstruxConfigurationLanguage) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for InstruxConfigurationLanguage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::English => write!(f, "English"),
            Self::Japanese => write!(f, "Japanese"),
        }
    }
}
impl ::std::str::FromStr for InstruxConfigurationLanguage {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "English" => Ok(Self::English),
            "Japanese" => Ok(Self::Japanese),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for InstruxConfigurationLanguage {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for InstruxConfigurationLanguage {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for InstruxConfigurationLanguage {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for InstruxConfigurationLanguage {
    fn default() -> Self {
        InstruxConfigurationLanguage::English
    }
}
///`InstruxConfigurationTargetsValue`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "outputMode": {
///      "description": "Whether to output to a single file or multiple files for this target",
///      "default": "single",
///      "type": "string",
///      "enum": [
///        "single",
///        "multiple"
///      ]
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct InstruxConfigurationTargetsValue {
    ///Whether to output to a single file or multiple files for this target
    #[serde(
        rename = "outputMode",
        default = "defaults::instrux_configuration_targets_value_output_mode"
    )]
    pub output_mode: InstruxConfigurationTargetsValueOutputMode,
}
impl ::std::convert::From<&InstruxConfigurationTargetsValue>
for InstruxConfigurationTargetsValue {
    fn from(value: &InstruxConfigurationTargetsValue) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for InstruxConfigurationTargetsValue {
    fn default() -> Self {
        Self {
            output_mode: defaults::instrux_configuration_targets_value_output_mode(),
        }
    }
}
impl InstruxConfigurationTargetsValue {
    pub fn builder() -> builder::InstruxConfigurationTargetsValue {
        Default::default()
    }
}
///Whether to output to a single file or multiple files for this target
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Whether to output to a single file or multiple files for this target",
///  "default": "single",
///  "type": "string",
///  "enum": [
///    "single",
///    "multiple"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum InstruxConfigurationTargetsValueOutputMode {
    #[serde(rename = "single")]
    Single,
    #[serde(rename = "multiple")]
    Multiple,
}
impl ::std::convert::From<&Self> for InstruxConfigurationTargetsValueOutputMode {
    fn from(value: &InstruxConfigurationTargetsValueOutputMode) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for InstruxConfigurationTargetsValueOutputMode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Single => write!(f, "single"),
            Self::Multiple => write!(f, "multiple"),
        }
    }
}
impl ::std::str::FromStr for InstruxConfigurationTargetsValueOutputMode {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "single" => Ok(Self::Single),
            "multiple" => Ok(Self::Multiple),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for InstruxConfigurationTargetsValueOutputMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for InstruxConfigurationTargetsValueOutputMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for InstruxConfigurationTargetsValueOutputMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for InstruxConfigurationTargetsValueOutputMode {
    fn default() -> Self {
        InstruxConfigurationTargetsValueOutputMode::Single
    }
}
///Semantic version of this config file
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Semantic version of this config file",
///  "type": "string",
///  "pattern": "^\\d+\\.\\d+\\.\\d+(-[a-zA-Z0-9\\.]+)?$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct InstruxConfigurationVersion(::std::string::String);
impl ::std::ops::Deref for InstruxConfigurationVersion {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<InstruxConfigurationVersion> for ::std::string::String {
    fn from(value: InstruxConfigurationVersion) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InstruxConfigurationVersion> for InstruxConfigurationVersion {
    fn from(value: &InstruxConfigurationVersion) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for InstruxConfigurationVersion {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^\\d+\\.\\d+\\.\\d+(-[a-zA-Z0-9\\.]+)?$").unwrap() });
        if (&*PATTERN).find(value).is_none() {
            return Err(
                "doesn't match pattern \"^\\d+\\.\\d+\\.\\d+(-[a-zA-Z0-9\\.]+)?$\""
                    .into(),
            );
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for InstruxConfigurationVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for InstruxConfigurationVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for InstruxConfigurationVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for InstruxConfigurationVersion {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///AI coding tool format to generate
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "AI coding tool format to generate",
///  "type": "string",
///  "enum": [
///    "copilot",
///    "cline",
///    "cursor",
///    "junie",
///    "codex",
///    "agentsmd"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum Targets {
    #[serde(rename = "copilot")]
    Copilot,
    #[serde(rename = "cline")]
    Cline,
    #[serde(rename = "cursor")]
    Cursor,
    #[serde(rename = "junie")]
    Junie,
    #[serde(rename = "codex")]
    Codex,
    #[serde(rename = "agentsmd")]
    Agentsmd,
}
impl ::std::convert::From<&Self> for Targets {
    fn from(value: &Targets) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for Targets {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Copilot => write!(f, "copilot"),
            Self::Cline => write!(f, "cline"),
            Self::Cursor => write!(f, "cursor"),
            Self::Junie => write!(f, "junie"),
            Self::Codex => write!(f, "codex"),
            Self::Agentsmd => write!(f, "agentsmd"),
        }
    }
}
impl ::std::str::FromStr for Targets {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "copilot" => Ok(Self::Copilot),
            "cline" => Ok(Self::Cline),
            "cursor" => Ok(Self::Cursor),
            "junie" => Ok(Self::Junie),
            "codex" => Ok(Self::Codex),
            "agentsmd" => Ok(Self::Agentsmd),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for Targets {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Targets {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Targets {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
/// Types for composing complex structures.
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct InstruxConfiguration {
        instructions: ::std::result::Result<
            ::std::vec::Vec<super::InstructionItem>,
            ::std::string::String,
        >,
        language: ::std::result::Result<
            super::InstruxConfigurationLanguage,
            ::std::string::String,
        >,
        targets: ::std::result::Result<
            ::std::collections::HashMap<
                super::Targets,
                super::InstruxConfigurationTargetsValue,
            >,
            ::std::string::String,
        >,
        version: ::std::result::Result<
            super::InstruxConfigurationVersion,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for InstruxConfiguration {
        fn default() -> Self {
            Self {
                instructions: Ok(Default::default()),
                language: Ok(super::defaults::instrux_configuration_language()),
                targets: Err("no value supplied for targets".to_string()),
                version: Err("no value supplied for version".to_string()),
            }
        }
    }
    impl InstruxConfiguration {
        pub fn instructions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::InstructionItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.instructions = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for instructions: {}", e)
                });
            self
        }
        pub fn language<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::InstruxConfigurationLanguage>,
            T::Error: ::std::fmt::Display,
        {
            self.language = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for language: {}", e)
                });
            self
        }
        pub fn targets<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<
                    super::Targets,
                    super::InstruxConfigurationTargetsValue,
                >,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.targets = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for targets: {}", e)
                });
            self
        }
        pub fn version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::InstruxConfigurationVersion>,
            T::Error: ::std::fmt::Display,
        {
            self.version = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for version: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<InstruxConfiguration> for super::InstruxConfiguration {
        type Error = super::error::ConversionError;
        fn try_from(
            value: InstruxConfiguration,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                instructions: value.instructions?,
                language: value.language?,
                targets: value.targets?,
                version: value.version?,
            })
        }
    }
    impl ::std::convert::From<super::InstruxConfiguration> for InstruxConfiguration {
        fn from(value: super::InstruxConfiguration) -> Self {
            Self {
                instructions: Ok(value.instructions),
                language: Ok(value.language),
                targets: Ok(value.targets),
                version: Ok(value.version),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct InstruxConfigurationTargetsValue {
        output_mode: ::std::result::Result<
            super::InstruxConfigurationTargetsValueOutputMode,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for InstruxConfigurationTargetsValue {
        fn default() -> Self {
            Self {
                output_mode: Ok(
                    super::defaults::instrux_configuration_targets_value_output_mode(),
                ),
            }
        }
    }
    impl InstruxConfigurationTargetsValue {
        pub fn output_mode<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                super::InstruxConfigurationTargetsValueOutputMode,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.output_mode = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for output_mode: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<InstruxConfigurationTargetsValue>
    for super::InstruxConfigurationTargetsValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: InstruxConfigurationTargetsValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                output_mode: value.output_mode?,
            })
        }
    }
    impl ::std::convert::From<super::InstruxConfigurationTargetsValue>
    for InstruxConfigurationTargetsValue {
        fn from(value: super::InstruxConfigurationTargetsValue) -> Self {
            Self {
                output_mode: Ok(value.output_mode),
            }
        }
    }
}
/// Generation of default values for serde.
pub mod defaults {
    pub(super) fn instruction_item_variant0_targets() -> super::InstructionItemVariant0Targets {
        super::InstructionItemVariant0Targets::Variant1("all".to_string())
    }
    pub(super) fn instruction_item_variant1_targets() -> super::InstructionItemVariant1Targets {
        super::InstructionItemVariant1Targets::Variant1("all".to_string())
    }
    pub(super) fn instruction_item_variant2_targets() -> super::InstructionItemVariant2Targets {
        super::InstructionItemVariant2Targets::Variant1("all".to_string())
    }
    pub(super) fn instrux_configuration_language() -> super::InstruxConfigurationLanguage {
        super::InstruxConfigurationLanguage::English
    }
    pub(super) fn instrux_configuration_targets_value_output_mode() -> super::InstruxConfigurationTargetsValueOutputMode {
        super::InstruxConfigurationTargetsValueOutputMode::Single
    }
}
