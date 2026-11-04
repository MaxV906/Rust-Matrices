
pub enum Error {
    InvalidNumberOfData,
    NumberOfRowsExceeded,
    NumberOfColumnsExceeded,
    InvalidMultiplication,
    DimensionsNotEqual,
    RowAndColumnNotEqual,
    SingularityMatrix
}

pub struct Matrix {
    rows: usize,
    columns: usize,
    data: Vec<i64>
}

impl Matrix {

    pub fn new(rows: usize, columns: usize) -> Matrix {

        let data = vec![0; rows*columns];

        Matrix {
            rows,
            columns,
            data
        }

    }

    pub fn get(&self, row: usize, column: usize) -> Result<i64, Error> {

        if row > self.rows - 1 {
            return Err(Error::NumberOfRowsExceeded);
        }

        if column > self.columns - 1 {
            return Err(Error::NumberOfColumnsExceeded);
        }

        Ok(self.data[row * self.columns + column])
    }

    pub fn set_data(&mut self, new_data: Vec<i64>) -> Result<(), Error> {

        if new_data.len() != self.rows*self.columns {
            return Err(Error::InvalidNumberOfData);
        }

        self.data = new_data;

        Ok(())

    }

    pub fn set_value(&mut self, row: usize, column: usize, new_value: i64) -> Result<(), Error> {

        if row > self.rows - 1 {
            return Err(Error::NumberOfRowsExceeded);
        }

        if column > self.columns - 1 {
            return Err(Error::NumberOfColumnsExceeded);
        }

        self.data[row * self.columns + column] = new_value;

        Ok(())

    }

    pub fn transpose(&self) -> Matrix {

        let mut m = Matrix::new(self.columns, self.rows);

        let new_data = self.data.clone();

        let _ = m.set_data(new_data);

        return m;

    }

    pub fn add(&self, matrix: &Matrix) -> Result<Matrix, Error> {

        if self.rows != matrix.rows && self.columns != matrix.columns {
            return Err(Error::DimensionsNotEqual);
        }

        let mut result_matrix = Matrix::new(self.rows, self.columns);
        let mut result_data = Vec::new();

        for i in 0..(self.rows * self.columns) {
            result_data.push(self.data[i] + matrix.data[i]);
        }

        let _ = result_matrix.set_data(result_data);

        Ok(result_matrix)

    }

    pub fn substract(&self, matrix: &Matrix) -> Result<Matrix, Error> {

        if self.rows != matrix.rows && self.columns != matrix.columns {
            return Err(Error::DimensionsNotEqual);
        }

        let mut result_matrix = Matrix::new(self.rows, self.columns);
        let mut result_data = Vec::new();

        for i in 0..(self.rows * self.columns) {
            result_data.push(self.data[i] - matrix.data[i]);
        }

        let _ = result_matrix.set_data(result_data);

        Ok(result_matrix)

    }

    pub fn multiply_by_number(&mut self, number: i64) {

        for i in 0..self.data.len() {
            self.data[i] *= number;
        }

    }

    pub fn multiply_by_matrix(&self, m: &Matrix) -> Result<Matrix, Error> {

        if self.columns != m.rows {
            return Err(Error::InvalidMultiplication);
        }

        let mut res_matrix = Matrix::new(self.rows, m.columns);

        for i in 0..self.rows {
            for j in 0..m.columns {
                let mut sum = 0;
                for k in 0..self.columns {
                    sum += self.data[i * self.columns + k] * m.data[k * m.columns+ j];
                }

                res_matrix.data[i * m.columns + j] = sum;
            }
        }


        Ok(res_matrix)

    }

    pub fn print(&self) {

        for column in 0..self.columns {

            if self.columns == 1 {
                print!("{} ", '[');
            }

            else if column == 0 {
                print!("{} ", '⎡');
            }

            else if column == self.columns - 1 {
                print!("{} ", '⎣');
            }

            else {
                print!("{} ", '⎢');
            }

            for row in 0..self.rows {

                print!("{} ", self.data[row * self.columns + column]);

            }

            if self.columns == 1 {
                print!("{}", ']');
            }

            else if column == 0 {
                print!("{}", '⎤');
            }

            else if column == self.columns - 1 {
                print!("{}", '⎦');
            }

            else {
                print!("{}", '⎢');
            }

            println!();
        }

    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_data() {
        let mut m = Matrix::new(3, 2);
        let _ = m.set_data(vec![1; 6]);

        assert_eq!(m.data[0], 1);

    }

    #[test]
    fn set_value() {
        let mut m = Matrix::new(2, 2);
        let _ = m.set_value(0, 0, 1);

        assert_eq!(m.data[0], 1);
    }

    #[test]
    fn get_data() {
        let mut m = Matrix::new(2, 2);
        let _ = m.set_data(vec![1, 2, 3, 4]);

        let value_res = m.get(0, 0);

        if let Ok(value) = value_res {
            assert_eq!(value, 1);
        }

        else {
            assert!(false);
        }

    }

    #[test]
    fn transpose() {
        let mut m = Matrix::new(2, 1);
        let _ = m.set_data(vec![0, 1]);

        let mt = m.transpose();

        assert!(mt.rows == m.columns && mt.columns == m.rows);
    }

    #[test]
    fn add() {
        let mut a = Matrix::new(3, 3);
        let _ = a.set_data(vec![10; 9]);

        let mut b = Matrix::new(3, 3);
        let _ = b.set_data(vec![1; 9]);

        let ab_res = a.add(&b);

        match ab_res {
            Ok(ab) => assert_eq!(ab.data[0], 11),
            Err(_) => assert!(false)
        }
    }

    #[test]
    fn substract() {
        let mut a = Matrix::new(3, 3);
        let _ = a.set_data(vec![10; 9]);

        let mut b = Matrix::new(3, 3);
        let _ = b.set_data(vec![1; 9]);

        let ab_res = a.substract(&b);

        match ab_res {
            Ok(ab) => assert_eq!(ab.data[0], 9),
            Err(_) => assert!(false)
        }

    }

    #[test]
    fn multiply_by_number() {
        let mut m = Matrix::new(2, 2);
        let _ = m.set_data(vec![1; 4]);

        m.multiply_by_number(2);

        assert_eq!(m.data[0], 2);
    }

    #[test]
    fn multiply_by_matrix() {
        let mut a = Matrix::new(2, 2);
        let mut b = Matrix::new(2, 1);

        let _ = a.set_data(vec![1, 2, 3, 4]);
        let _ = b.set_data(vec![1, 2]);

        let ab_res = a.multiply_by_matrix(&b);

        if let Ok(ab) = ab_res {
            assert_eq!(ab.data[0], 5);
        } 
        
        else {
            assert!(false);
        }
    }
}
