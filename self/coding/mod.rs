
pub mod index
{

	pub struct foo;
	pub struct bar;

	pub trait foobar{
		pub fn how(self)->bool;
	}

	impl foobar for foo{
		fn how(self)->bool{
			false
		}
	}

	impl foobar for bar{
		fn how(self)->bool{
			true
		}
	}




}