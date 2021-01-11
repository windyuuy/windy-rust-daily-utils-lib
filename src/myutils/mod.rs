
/// 判断两者是否引用了同一个对象
pub fn is_same<T>(a: &T, b: &T) -> bool {
  let pa = a as *const T;
  let pb = b as *const T;
  let ret = std::ptr::eq(pa, pb);
  return ret;
}

#[test]
mod test{

	struct A{

	};

	#[test]
	fn test_is_same(){
		let a=&A{};
		let b=&A{};
		let c=is_same(a,b);
		assert_eq!(c,true);
	}

}
