pub fn day10(input_lines: &[String]) -> (u64, u64) {
    let mut differences: [u64; 4] = [0,0,0,0];
    let mut adapters: Vec<Adapter> = input_lines.iter().map(|line| Adapter { rating: line.parse::<u64>().expect("Invalid input!"), combinations: 0}).collect();
    adapters.push(Adapter {rating: 0, combinations: 1}); // Include the wall socket
    adapters.sort_unstable();
    adapters.push(Adapter {rating: adapters.last().unwrap().rating + 3, combinations: 0}); // Include your device's built-in adapter

    // Both parts 1 and 2 depend on having a sorted list of adapters.
    //
    // Part 1, since it's going to use every adapter, simply requires going through the list, checking
    // the delta to the next adapter up and keeping a tally of each possible delta.
    //
    // Part 2 involves keeping a tally of how many ways we can get to each adapter. If we've reached
    // the Nth adapter and have found X paths here, then we just found X more paths to whichever of
    // the following 3 adapters had a rating within 3 of this adapter (and they might have already had
    // paths found direct from an earlier adapter). So, for example, in a list [0, 1, 2, 3, 5]:
    // - There's obviously only one way of getting to 0, which is why we initialised the wall socket's
    //   `combinations` field to 1 above.
    // - From 0 we can get to 1, 2 or 3, so record that we've found one way to each of those already.
    // - From 1 we can get to 2 or 3, so record that we've found an additional way to get to each of those.
    // - At 2, we've already found 2 ways of getting here (0 -> 1 -> 2 and 0 -> 2), and can reach 3 or 5.
    //   We'd already found two ways of getting to 3 without going through 2, so add the two ways that DO
    //   get through 2.  We haven't previously found any ways of getting to 5, so we've now found 2.
    // - At 3, we've already found four ways of getting here, and can reach 5. We'd found two ways of getting
    //   to 5 without going through 3, so we've now found six total ways of getting to 5.
    // - We're done; the total number of combinations is six.
    // Through this mechanism, we don't actually need to figure out what the combinations ARE, we just
    // know how MANY there are.

    for index in 0..adapters.len() - 1 {
        let difference = adapters[index + 1].rating - adapters[index].rating;
        differences[difference as usize] += 1;

        // The next adapter is guaranteed to exist by the loop conditions, and also guaranteed to be within 3
        // of this adapter.
        adapters[index + 1].combinations += adapters[index].combinations;

        // Adapters N+2 and N+3 might either not exist (because we're almost at the end) or be too big
        // a jump from this adapter.
        if (index + 2 < adapters.len()) && (adapters[index + 2].rating - adapters[index].rating <= 3) {
            adapters[index + 2].combinations += adapters[index].combinations;
        }
        if (index + 3 < adapters.len()) && (adapters[index + 3].rating - adapters[index].rating <= 3) {
            adapters[index + 3].combinations += adapters[index].combinations;
        }
    }

    let part1 = differences[1] * differences[3];
    let part2 = adapters.last().unwrap().combinations;
    (part1,part2)
}

#[derive(PartialEq,Eq,PartialOrd,Ord)]
struct Adapter {
    rating: u64,
    combinations: u64,
}