
pub fn calculate_one() -> i32 {

  let path = std::path::Path::new("./src/one/data.txt");
  let data_text = std::fs::read_to_string(path).unwrap();

  let per_elf: Vec<&str> = data_text.split("\n\n").collect();
  let fixed: Vec<Vec<i32>> = per_elf
    .iter()
    .map(|x| {
      let result: Vec<&str> = x.split("\n").collect();
      let parsed: Vec<i32> = result
        .iter()
        .map(|x| {
          return x.parse::<i32>().unwrap_or(0);
        })
        .collect();
      return parsed;
    })
    .collect();

  
  let mut sums: Vec<i32> = fixed
    .iter()
    .map(|x| {
      let sum: i32 = x.iter().sum();
      return sum;
    })
    .collect();


  sums.sort_by(|a, b| {

    if a.eq(b) { return std::cmp::Ordering::Equal };
    if a.gt(b) { return std::cmp::Ordering::Less };
    if a.lt(b) { return std::cmp::Ordering::Greater };

    unreachable!();

  });


  let new = &sums[0..3];
  let result: i32 = new.iter().sum();
  return result;
}