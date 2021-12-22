use std::ops::Index;
use std::slice::Iter;
use std::collections::{HashMap, LinkedList, BTreeMap};

type Micros = u64;

const INDEX_KEYFRAMES_PER_SECOND: u64 = 30;
const INDEX_KEYFRAME_MICROSECONDS_TIME: u64 = ((1.0 / (INDEX_KEYFRAMES_PER_SECOND as f64)) * 1000000.0) as u64;

pub trait Curve<T> {
    fn get_value(&self, key: Micros) -> Option<T>;
    fn insert_keyframe(&mut self, key: Micros, value: T);
    fn delete_keyframe(&self, key: Micros);
}

struct LinearTimeCurve<T> {
    keyframes: BTreeMap<Micros, T>,
    index_keyframes: HashMap<Micros, usize>
}

impl<T: InterpolateLinear> Curve<T> for LinearTimeCurve<T> {
    fn get_value(&self, key: Micros) -> Option<T> {
        self.keyframes.iter();
        todo!()
    }

    fn insert_keyframe(&mut self, key: Micros, value: T) {
        let val = self.keyframes.insert(key, value).unwrap();
        let last_index_time = (key / INDEX_KEYFRAME_MICROSECONDS_TIME) * INDEX_KEYFRAME_MICROSECONDS_TIME;
        if !self.index_keyframes.contains_key(&key) {
            let (last_time, last_value) = self.keyframes.last_key_value().unwrap();

            let interpolated_value = T::interpolate_linear((*last_time, last_value), (key, &val), last_index_time);
            self.keyframes.insert(last_index_time, interpolated_value);
        }
        todo!()
    }

    fn delete_keyframe(&self, key: Micros) {
        todo!()
    }
}

struct KeyframeSet<'a, T> {
    index_keyframes: HashMap<Micros, &'a T>,
    keyframes: BTreeMap<Micros, T>
}

impl<T> KeyframeSet<'_, T> {
    pub fn insert(&mut self, key: Micros, value: T) {
        let last_index_time = (key / INDEX_KEYFRAME_MICROSECONDS_TIME) * INDEX_KEYFRAME_MICROSECONDS_TIME;

        self.keyframes.insert(key, value);
    }
}

trait InterpolateLinear<T=Self> {
    fn interpolate_linear(first: (Micros, &T), second: (Micros, &T), key: Micros) -> T;
}
