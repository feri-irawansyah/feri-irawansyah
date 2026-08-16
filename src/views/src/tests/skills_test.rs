use crate::pages::skills::{SkillTier, group_by_tier};
use chrono::Utc;
use modules::skills::SkillView;

fn make_skill(id: i32, star: i32) -> SkillView {
    SkillView {
        skill_id: id,
        title: format!("Skill {id}"),
        description: String::new(),
        url_docs: String::new(),
        image_src: String::new(),
        progress: 50,
        star,
        last_update: Utc::now(),
    }
}

#[test]
fn for_star_5_is_favorite() {
    assert_eq!(SkillTier::for_star(5), SkillTier::Favorite);
}

#[test]
fn for_star_4_is_familiar() {
    assert_eq!(SkillTier::for_star(4), SkillTier::Familiar);
}

#[test]
fn for_star_below_4_is_used_before() {
    assert_eq!(SkillTier::for_star(3), SkillTier::UsedBefore);
    assert_eq!(SkillTier::for_star(0), SkillTier::UsedBefore);
}

#[test]
fn group_by_tier_buckets_correctly_and_orders_favorite_first() {
    let items = vec![make_skill(1, 2), make_skill(2, 5), make_skill(3, 4)];
    let groups = group_by_tier(&items);

    assert_eq!(groups.len(), 3);
    assert_eq!(groups[0].0, SkillTier::Favorite);
    assert_eq!(groups[0].1.len(), 1);
    assert_eq!(groups[0].1[0].skill_id, 2);

    assert_eq!(groups[1].0, SkillTier::Familiar);
    assert_eq!(groups[1].1[0].skill_id, 3);

    assert_eq!(groups[2].0, SkillTier::UsedBefore);
    assert_eq!(groups[2].1[0].skill_id, 1);
}

#[test]
fn group_by_tier_omits_empty_tiers() {
    // Only favorite-tier skills present — familiar/used-before must not
    // show up as empty sections.
    let items = vec![make_skill(1, 5), make_skill(2, 5)];
    let groups = group_by_tier(&items);

    assert_eq!(groups.len(), 1);
    assert_eq!(groups[0].0, SkillTier::Favorite);
    assert_eq!(groups[0].1.len(), 2);
}

#[test]
fn group_by_tier_empty_input_returns_no_groups() {
    assert!(group_by_tier(&[]).is_empty());
}
