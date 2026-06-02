use iztro_core::{PALACE_NAMES, PalaceName};
use iztro_features::{PalaceRelation, PalaceRelationKind, PalaceRelations, all_palace_relations};

#[test]
fn life_palace_relations_use_canonical_offsets() {
    let relations = PalaceRelations::for_palace(PalaceName::Life);

    assert_eq!(relations.target(), PalaceName::Life);
    assert_eq!(relations.opposite(), PalaceName::Migration);
    assert_eq!(relations.triad(), [PalaceName::Wealth, PalaceName::Career]);
    assert_eq!(
        relations.adjacent(),
        [PalaceName::Parents, PalaceName::Siblings]
    );
}

#[test]
fn migration_opposite_is_life() {
    let relations = PalaceRelations::for_palace(PalaceName::Migration);

    assert_eq!(relations.opposite(), PalaceName::Life);
}

#[test]
fn all_palaces_have_complete_relation_aggregates() {
    let all_relations = all_palace_relations();

    assert_eq!(all_relations.len(), PALACE_NAMES.len());

    for (index, relations) in all_relations.iter().enumerate() {
        assert_eq!(relations.target(), PALACE_NAMES[index]);
        assert_eq!(relations.triad().len(), 2);
        assert_eq!(relations.adjacent().len(), 2);

        let opposite_relations = relations
            .to_relations()
            .into_iter()
            .filter(|relation| relation.kind() == PalaceRelationKind::Opposite)
            .collect::<Vec<PalaceRelation>>();
        assert_eq!(opposite_relations.len(), 1);
    }
}

#[test]
fn opposite_relations_are_symmetric() {
    for palace in PALACE_NAMES {
        let relations = PalaceRelations::for_palace(palace);
        let opposite_relations = PalaceRelations::for_palace(relations.opposite());

        assert_eq!(opposite_relations.opposite(), palace);
    }
}

#[test]
fn adjacent_relations_wrap_around_canonical_order() {
    let life_relations = PalaceRelations::for_palace(PalaceName::Life);
    let parents_relations = PalaceRelations::for_palace(PalaceName::Parents);

    assert_eq!(life_relations.adjacent()[0], PalaceName::Parents);
    assert_eq!(parents_relations.adjacent()[1], PalaceName::Life);
}

#[test]
fn aggregate_can_emit_edge_level_relations() {
    let relations = PalaceRelations::for_palace(PalaceName::Life);

    assert_eq!(
        relations.to_relations(),
        [
            PalaceRelation::new(
                PalaceName::Life,
                PalaceName::Migration,
                PalaceRelationKind::Opposite,
            ),
            PalaceRelation::new(
                PalaceName::Life,
                PalaceName::Wealth,
                PalaceRelationKind::Triad
            ),
            PalaceRelation::new(
                PalaceName::Life,
                PalaceName::Career,
                PalaceRelationKind::Triad
            ),
            PalaceRelation::new(
                PalaceName::Life,
                PalaceName::Parents,
                PalaceRelationKind::Adjacent,
            ),
            PalaceRelation::new(
                PalaceName::Life,
                PalaceName::Siblings,
                PalaceRelationKind::Adjacent,
            ),
        ]
    );
}
