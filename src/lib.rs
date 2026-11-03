
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
    fn get() {
        let mut m = Matrix::new(3, 2);
        let _ = m.set_data(vec![1; 6]);

        for c in 0..m.columns {
            for r in 0..m.rows {

                let res = m.get(r, c);

                match res {
                    Ok(n) => { assert_eq!(n, 1); },
                    Err(_) => { assert!(false); }
                }

            }
        }

        let res = m.get(4, 2);

        match res {
            Ok(_) => { assert!(false); },
            Err(e) => {
                match e {
                    Error::NumberOfRowsExceeded => { assert!(true); },
                    _ => { assert!(false); }
                }
            }
        }

        let res = m.get(0, 4);

        match res {
            Ok(_) => { assert!(false); },
            Err(e) => {
                match e {
                    Error::NumberOfColumnsExceeded => { assert!(true); },
                    _ => { assert!(false); }
                }
            }
        }
    }

    #[test]
    fn transpose() {
        let mut m = Matrix::new(2, 1);
        let _ = m.set_data(vec![0, 1]);

        let mt = m.transpose();

        assert_eq!(mt.rows, m.columns);
        assert_eq!(mt.columns, m.rows);
        assert_eq!(mt.data, m.data);
    }

    #[test]

    fn set_data() {

        let mut m = Matrix::new(2, 2);

        let res = m.set_data(vec![1, 9]);

        if let Err(e) = res {
            match e {
                Error::InvalidNumberOfData => {assert!(true);},
                _ => {assert!(false);}
            }
        }

        for i in 1..5 {
            let mut m = Matrix::new(i, i);
            let _ = m.set_data(vec![1; i*i]);

            for j in 0..i*i {
                assert_eq!(1, m.data[j]);
            }
        }

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
}
