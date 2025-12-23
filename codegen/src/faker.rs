use std::{collections::BTreeMap, sync::Arc};

use fake::{Dummy, Fake, Faker};

pub struct ArcFaker;
pub struct ArcStrFaker;
pub struct ValueFaker;

impl Dummy<ArcFaker> for Arc<str> {
    fn dummy_with_rng<R: fake::Rng + ?Sized>(_: &ArcFaker, rng: &mut R) -> Self {
        let str: String = Faker.fake_with_rng(rng);
        Arc::from(str)
    }
}

impl<T> Dummy<ArcFaker> for Arc<[T]>
where
    Vec<T>: Dummy<Faker>,
{
    fn dummy_with_rng<R: fake::Rng + ?Sized>(_: &ArcFaker, rng: &mut R) -> Self {
        let str: Vec<T> = Faker.fake_with_rng(rng);
        Arc::from(str)
    }
}

impl Dummy<ArcStrFaker> for Arc<[Arc<str>]> {
    fn dummy_with_rng<R: fake::Rng + ?Sized>(_: &ArcStrFaker, rng: &mut R) -> Self {
        let str: Vec<String> = Faker.fake_with_rng(rng);
        str.iter().map(|x| -> Arc<str> { (&**x).into() }).collect()
    }
}

impl Dummy<ValueFaker> for serde_json::Value {
    fn dummy_with_rng<R: fake::Rng + ?Sized>(_: &ValueFaker, rng: &mut R) -> Self {
        serde_json::Value::Null
    }
}

impl<V: Dummy<Faker>> Dummy<ArcFaker> for Arc<BTreeMap<Arc<str>, V>> {
    fn dummy_with_rng<R: fake::Rng + ?Sized>(_: &ArcFaker, rng: &mut R) -> Self {
        let map: BTreeMap<String, V> = Faker.fake_with_rng(rng);
        let map = map.into_iter().map(|(k, v)| (Arc::from(&*k), v)).collect();
        Arc::new(map)
    }
}
