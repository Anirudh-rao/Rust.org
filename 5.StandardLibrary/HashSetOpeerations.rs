use std::collections::HashSet;

fn main() {
    let hashset1 = HashSet::from([2, 7, 8]);
    let hashset2 = HashSet::from([1, 2, 7]);

    // Union of hashsets
    let result: HashSet<_> = hashset1.union(&hashset2).collect();

    println!("hashset1 = {:?}", hashset1);
    println!("hashset2 = {:?}", hashset2);
    println!("union = {:?}", result);

    // Intersection of hashsets
    let result2: HashSet<_> = hashset1.intersection(&hashset2).collect();

    println!("hashset1 = {:?}", hashset1);
    println!("hashset2 = {:?}", hashset2);
    println!("intersection = {:?}", result2);

    //Difference between hashsets
    let result3: HashSet<_> = hashset1.difference(&hashset2).collect();

    println!("hashset1 = {:?}", hashset1);
    println!("hashset2 = {:?}", hashset2);
    println!("difference = {:?}", result3);

    // Symmetric difference of hashsets
    let result4: HashSet<_> = hashset1.symmetric_difference(&hashset2).collect();

    println!("hashset1 = {:?}", hashset1);
    println!("hashset2 = {:?}", hashset2);
    println!("symmetric difference = {:?}", result4);
}
