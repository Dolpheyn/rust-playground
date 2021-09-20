use std::{thread, time::Instant};

fn main() {
    // This is our data to process.
    // We will calculate the sum of all digits via a threaded  map-reduce algorithm.
    // Each whitespace separated chunk will be handled in a different thread.
    let data = "86967897737416471853297327050364959
    11861322575564723963297542624962850
    70856234701860851907960690014725639
    38397966707106094172783238747669219
    52380795257888236525459303330302837
    58495327135744041048897885734297812
    69920216438980873548808413720956532
    16278424637452589860345374828574668";

    let mut child_threads = vec![];

    let start = Instant::now();
    let chunked_data = data.split_whitespace();

    for (_, data_segment) in chunked_data.enumerate() {
        // A closure that moves the data segment, take and convert each character to u32 and sum
        // them.
        let parse_and_sum_digits = move || -> u32 {
            data_segment
                .chars()
                .map(|c| c.to_digit(10).expect("Should be a digit."))
                .sum()
        };

        child_threads.push(thread::spawn(parse_and_sum_digits));
    }

    // Run all threads asynchronously and sum the results.
    let final_result = child_threads
        .into_iter()
        .map(|c| c.join().unwrap())
        .sum::<u32>();

    let duration = start.elapsed();
    println!("Final sum result: {}", final_result);
    println!("(MapReduce) Time taken : {:?}", duration);

    // Loop version

    let start = Instant::now();
    let chunked_data = data.split_whitespace();
    let mut total: u32 = 0;

    for (_, data_segment) in chunked_data.enumerate() {
        //println!("data segment {} is \"{}\"", i, data_segment);

        let parse_and_sum_digits = move || -> u32 {
            let result = data_segment
                .chars()
                .map(|c| c.to_digit(10).expect("Should be a digit."))
                .sum();

            //println!("Processed segment {}, result = {}", i, result);
            result
        };

        total += parse_and_sum_digits();
    }

    println!("Final sum result: {}", total);
    let duration = start.elapsed();
    println!("(Loop) Time taken : {:?}", duration);

    // Output:
    //
    // Final sum result: 1342
    // (MapReduce) Time taken : 616.921µs
    // Final sum result: 1342
    // (Loop) Time taken : 164.576µs
    //
    // Loop takes less time because this job is only CPU (not IO) intensive, so we only took more
    // time at creating and waiting for the threads.
}
