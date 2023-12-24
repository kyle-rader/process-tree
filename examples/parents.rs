use process_tree::ProcessTree;

fn main() -> anyhow::Result<()> {
    println!("Hello, processes!");

    let process_tree = ProcessTree::new()?;

    println!("My ancestors are: {:#?}", process_tree.ancestry());

    Ok(())
}
