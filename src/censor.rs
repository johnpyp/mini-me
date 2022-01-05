use censor::Censor;

pub fn get_custom_censor() -> Censor {
    Censor::Standard + "retard"
}
