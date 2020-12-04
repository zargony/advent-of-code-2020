use advent_of_code_2020::Input;
use futures::stream::TryStreamExt;
use permutator::copy::k_permutation;
use std::error;

fn product_of_summands(numbers: &[u32], sum: u32, k: usize) -> Option<u64> {
    // FIXME: `KPermutationIterator::new` panics, so we use k_permutation() here
    let mut res = None;
    k_permutation(numbers, k, |nums| {
        if nums.iter().fold(0, |a, n| a + *n) == sum {
            res = Some(nums.iter().fold(1u64, |a, n| a * *n as u64));
        }
    });
    res
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let input = Input::day(1).await?;
    let numbers: Vec<_> = input.parsed_lines::<u32>().try_collect().await?;

    let product = product_of_summands(&numbers, 2020, 2).unwrap();
    println!("Product of 2 entries that sum up to 2020: {}", product);

    let product = product_of_summands(&numbers, 2020, 3).unwrap();
    println!("Product of 3 entries that sum up to 2020: {}", product);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const NUMBERS: [u32; 6] = [1721, 979, 366, 299, 675, 1456];

    #[test]
    fn part_1() {
        assert_eq!(product_of_summands(&NUMBERS, 2020, 2), Some(514579));
    }

    #[test]
    fn part_2() {
        assert_eq!(product_of_summands(&NUMBERS, 2020, 3), Some(241861950));
    }
}
