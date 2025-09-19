use the_book::AveragedCollection;

fn main() {
    let mut collection1 = AveragedCollection::new(50);

    collection1.add(40);
    collection1.add(50);
    collection1.add(60);
    collection1.add(70);
    collection1.add(80);
    collection1.add(90);

    dbg!(&collection1);
    dbg!(&collection1);
}
