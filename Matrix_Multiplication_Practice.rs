fn main () {
  let mut result_matrix: Vec<Vec<i32>> = Vec::new();

  let matrix_1 = vec! [
    vec![1, 2, 3],
    vec![2, 3, 4]
  ];

  let matrix_2 = vec! [
    vec![2, 3],
    vec![1, 4],
    vec![3, 2]
  ];
  
  for i in 0..matrix_1.len() {
    let mut new_vec = Vec::new();
    
    for j in 0..matrix_2[0].len() {
      let mut total = 0;
      for k in 0..matrix_2.len() {
        total += matrix_1[i][k] * matrix_2[k][j];
      }

      new_vec.push(total);
    }

    result_matrix.push(new_vec);
  }

  for item in &result_matrix {
    println!("{:?}", item);
  }
}