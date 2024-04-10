use std::{collections::{HashMap, HashSet}, hash::Hash};

use crate::{
    data_definition::{Attribute, DataDefinition, Object},
    decision_tree::{DecNode, DecNodeChild},
};

pub fn create_tree(data: &DataDefinition) -> DecNode<'_> {
    let mut id = 0;
    make_node(&mut id, "", data.attributes.iter(), data.objects.iter())
        .child
        .unwrap()
}

fn make_node<'a, A, O>(
    id: &mut i32,
    attr_class: &'a str,
    attrs: A,
    objs: O,
) -> DecNodeChild<'a>
where
    A: Iterator<Item = &'a Attribute> + Clone,
    O: Iterator<Item = &'a Object> + Clone,
{
    let my_id = *id;
    *id += 1;

    let classes: HashSet<_> = objs.clone().map(|o| o.class.as_str()).collect();

    let info_gains = calc_info_gains(attrs.clone(), objs.clone());
    let child = if info_gains.len() > 1 && classes.len() > 1 {
        let attribute = info_gains
            .iter()
            .max_by(|(_, v1), (_, v2)| v1.total_cmp(v2))
            .unwrap()
            .0;

        let a = attrs
            .clone()
            .filter(|a| a.name == *attribute)
            .next()
            .unwrap();
        let children = a
            .values
            .iter()
            .map(|ac| {
                make_node(
                    id,
                    ac,
                    attrs.clone().filter(|a| a.name != *attribute),
                    objs.clone()
                        .filter(|o| o.attributes.get(&a.name) == Some(ac)),
                )
            })
            .collect();

        Some(DecNode {
            id: my_id,
            attribute,
            info_gains,
            children,
        })
    } else {
        None
    };

    DecNodeChild {
        attr_class,
        objects: objs.collect(),
        child,
        target_id: my_id,
    }
}

fn calc_info_gains<'a, A, O>(attrs: A, objs: O) -> HashMap<&'a str, f64>
where
    A: Iterator<Item = &'a Attribute>,
    O: Iterator<Item = &'a Object> + Clone,
{
    let total_entropy = calc_entropy(objs.clone(), |o| &o.class);
    let mut res = HashMap::new();

    for a in attrs {
        let ig = calc_info_gain(a, total_entropy, objs.clone());
        res.insert(a.name.as_str(), ig);
    }

    res
}

fn calc_info_gain<'a, I>(attr: &Attribute, total_entropy: f64, objs: I) -> f64
where
    I: Iterator<Item = &'a Object> + Clone,
{
    let mut res = 0.0;
    for a in &attr.values {
        res += calc_entropy(
            objs.clone()
                .filter(|o| o.attributes.get(&attr.name) == Some(a)),
            |o| &o.class,
        );
    }

    res / total_entropy
}

fn calc_entropy<'a, I, F, V>(objs: I, by: F) -> f64
where
    I: Iterator<Item = &'a Object>,
    F: Fn(&'a Object) -> V,
    V: Hash + Eq + 'a,
{
    let mut counts = HashMap::<V, usize>::new();
    let mut total = 0;

    for i in objs {
        let k = by(i);
        counts.entry(k).and_modify(|v| *v += 1).or_insert(1);
        total += 1;
    }

    let total = total as f64;
    let mut res = 0.0;
    for i in counts.values() {
        let p = *i as f64 / total;
        res -= p * p.log2();
    }

    res
}
