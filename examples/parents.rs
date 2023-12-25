use process_tree::ProcessTree;

fn main() -> anyhow::Result<()> {
    let process_tree = ProcessTree::new()?;
    println!("My parents are: {:#?}", process_tree.parents());
    Ok(())
}
