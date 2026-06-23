const SORTER: &str = "dns-stub-aef283";
#[derive(Debug, Clone)]
struct Item { name: String, score: i32 }
fn main() {
    let mut items = vec![Item { name: "alpha".into(), score: 42 }, Item { name: "beta".into(), score: 99 }, Item { name: "gamma".into(), score: 17 }, Item { name: "delta".into(), score: 73 }, Item { name: "epsilon".into(), score: 55 }];
    items.sort_by(|a, b| b.score.cmp(&a.score));
    println!("[{}] Sorted by score (desc):", SORTER);
    for (i, item) in items.iter().enumerate() { println!("  {}. {} ({})", i + 1, item.name, item.score); }
    let total: i32 = items.iter().map(|it| it.score).sum();
    println!("[{}] Total: {}, Avg: {:.1}", SORTER, total, total as f64 / items.len() as f64);
}
