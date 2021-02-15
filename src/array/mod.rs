pub mod sum;
pub mod median;
pub mod rev;


#[cfg(test)]
mod tests {
    use std::borrow::Borrow;

    use crate::array::rev::reverse_i32;
    use crate::array::sum::two_sum;

    #[test]
    fn test_two_sum() {
        assert_eq!(vec![0, 1], two_sum(vec![2, 7, 11, 15].borrow(), 9));
        assert_eq!(vec![1, 2], two_sum(&vec![3, 2, 4], 6));
        let r: Vec<i32> = two_sum(&vec![3, 3], 6).iter().map(|x| x.clone() as i32).collect();
        assert_eq!(vec![0, 1], r);
        let r = two_sum(&vec![3, 3], 6).iter().map(|x| x.clone() as i32).collect::<Vec<i32>>();
        assert_eq!(vec![0, 1], r);
    }

    #[test]
    fn test_reverse_i32() {
        assert_eq!(0, reverse_i32(1534236469));
        assert_eq!(0, reverse_i32(-2147483648));
        assert_eq!(54321, reverse_i32(12345));
        assert_eq!(-54321, reverse_i32(-12345));
    }
}
