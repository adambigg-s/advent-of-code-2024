


/// digits are stored in reverse order!! rememeber this
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UIntInf
{
    pub digits: Vec<u8>,
}

impl UIntInf {
    pub fn zero() -> UIntInf
    {
        UIntInf { digits: vec![0] }
    }

    pub fn from_std_int(int: u128) -> UIntInf
    {
        let mut digits = Vec::new();
        let mut num = int;

        if num == 0 {
            digits.push(0);
        } else {
            while num > 0 {
                digits.push((num % 10) as u8);
                num /= 10;
            }
        }

        UIntInf { digits }
    }

    pub fn lhsgreater(lhs: &UIntInf, rhs: &UIntInf) -> bool
    {
        match lhs.digits.len().cmp(&rhs.digits.len()) {
            std::cmp::Ordering::Greater => true,
            std::cmp::Ordering::Less => false,
            std::cmp::Ordering::Equal => {
                for i in (0..lhs.digits.len()).rev() {
                    match lhs.digits[i].cmp(&rhs.digits[i]) {
                        std::cmp::Ordering::Greater => return true,
                        std::cmp::Ordering::Less => return false,
                        std::cmp::Ordering::Equal => continue,
                    }
                }

                false
            }
        }
    }

    pub fn add(lhs: &UIntInf, rhs: &UIntInf) -> UIntInf
    {
        let mut result = Vec::new();
        let mut carry = 0;
        let max_len = lhs.digits.len().max(rhs.digits.len());
        for i in 0..max_len {
            let lhs_digit = if i < lhs.digits.len() { lhs.digits[i] } else { 0 };
            let rhs_digit = if i < rhs.digits.len() { rhs.digits[i] } else { 0 };

            let sum = lhs_digit + rhs_digit + carry;
            result.push(sum % 10);
            carry = sum / 10;
        }
        if carry > 0 {
            result.push(carry);
        }

        UIntInf { digits: result }
    }

    pub fn mul(lhs: &UIntInf, rhs: &UIntInf) -> UIntInf
    {
        let mut result = vec![0; lhs.digits.len() + rhs.digits.len()];
        for i in 0..lhs.digits.len() {
            for j in 0..rhs.digits.len() {
                let product = lhs.digits[i] * rhs.digits[j] + result[i + j];
                result[i + j] = product % 10;
                result[i + j + 1] += product / 10;
            }
        }
        while result.len() > 1 && result[result.len()-1] == 0 {
            result.pop();
        }

        UIntInf { digits: result }
    }

    pub fn concat(lhs: &UIntInf, rhs: &UIntInf) -> UIntInf
    {
        let mut result = lhs.digits.clone();
        let mut other = rhs.digits.clone();
        result.reverse();
        other.reverse();
        result.append(&mut other);
        result.reverse();

        UIntInf { digits: result }
    }

    pub fn to_print(&self) -> String
    {
        self.digits.iter().rev().map(|&dig| (dig + b'0') as char).collect()
    }
}



#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn testing()
    {
        let u = UIntInf::from_std_int(100000);
        let v = UIntInf::from_std_int(100343);
        let y = UIntInf::add(&u, &v);
        
        assert!(y.to_print() == "200343");
    }

    #[test]
    fn ordering_test()
    {
        let bigger = UIntInf::from_std_int(9999999);
        let smaller = UIntInf::from_std_int(2923);

        assert!(UIntInf::lhsgreater(&bigger, &smaller));
    }
}
