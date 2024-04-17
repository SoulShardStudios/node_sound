use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sound_queue;
use crate::sounds::{AsGenericSource, SawToothWave};
use egui_node_graph_2::InputParamKind;
use std::collections::HashMap;
use std::convert::Infallible;

pub fn sawtooth_node() -> SoundNode {
    SoundNode {
        name: "Sawtooth Wave".to_string(),
        inputs: HashMap::from([(
            "frequency".to_string(),
            InputParameter {
                data_type: DataType::Float,
                kind: InputParamKind::ConnectionOrConstant,
                name: "frequency".to_string(),
                value: InputValueConfig::Float { value: 0.0 },
            },
        )]),
        outputs: HashMap::from([(
            "out".to_string(),
            Output {
                data_type: DataType::AudioSource,
                name: "out".to_string(),
            },
        )]),
        operation: |props| {
            Ok(HashMap::from([(
                "out".to_string(),
                ValueType::AudioSource {
                    value: sound_queue::push_sound(
                        SawToothWave::new(props.get_float("frequency")?).as_generic(None),
                    ),
                },
            )]))
        },
    }
}
