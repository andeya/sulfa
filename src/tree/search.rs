// catalan calculate how many binary search trees can be formed by node_num nodes
pub fn catalan(node_num: u64) -> u64 {
    let mut c: f64 = 1.0;
    for i in 0..node_num {
        c = c * 2.0 * (2.0 * i as f64 + 1.0) / (i as f64 + 2.0);
    }
    c as u64
}

#[cfg(test)]
mod tests {
    use super::catalan;

    #[test]
    fn test_catalan() {
        assert_eq!(1, catalan(1));
        assert_eq!(2, catalan(2));
        assert_eq!(5, catalan(3));
        assert_eq!(6564120420, catalan(20))
    }
}
