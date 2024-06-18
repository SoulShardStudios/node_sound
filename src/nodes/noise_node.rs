use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sound_map;
use crate::sounds::Noise;
use egui_node_graph_2::InputParamKind;
use std::collections::HashMap;

use super::{SoundNodeProps, SoundNodeResult};

pub fn noise_node() -> SoundNode {
    SoundNode {
        name: "Noise".to_string(),
        inputs: HashMap::from([
            (
                "min".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "min".to_string(),
                    value: InputValueConfig::Float { value: 1.0 },
                },
            ),
            (
                "max".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "max".to_string(),
                    value: InputValueConfig::Float { value: 1.0 },
                },
            ),
        ]),
        outputs: HashMap::from([(
            "out".to_string(),
            Output {
                data_type: DataType::AudioSource,
                name: "out".to_string(),
            },
        )]),
    }
}
pub fn noise_logic(props: SoundNodeProps) -> SoundNodeResult {
    Ok(HashMap::from([(
        "out".to_string(),
        ValueType::AudioSource {
            value: sound_map::push_sound::<Noise>(Box::new(Noise::new(
                props.get_float("min")?,
                props.get_float("max")?,
            ))),
        },
    )]))
}
