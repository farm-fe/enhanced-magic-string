pub fn get_locator(code: &str) -> impl Fn(usize) -> Loc {
  let lines = code.lines().collect::<Vec<_>>();
  let line_offsets = lines
    .iter()
    .scan(0, |acc, line| {
      let offset = *acc;
      *acc += line.len() + 1;
      Some(offset)
    })
    .collect::<Vec<_>>();

  move |pos| {
    // binary search
    let mut left = 0;
    let mut right = line_offsets.len();

    while left < right {
      let mid = (left + right) >> 1;

      if pos < line_offsets[mid] {
        right = mid;
      } else {
        left = mid + 1;
      }
    }

    let line = left - 1;
    let column = pos - line_offsets[line];

    Loc { line, column }
  }
}

pub struct Loc {
  pub line: usize,
  pub column: usize,
}
