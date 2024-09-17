use std::*;

#[derive(Debug,Clone)]
struct Tensor {
	number: Vec<f32>,
	dimension: Vec<usize>,
}
fn main(){
	println!("123");
	let tensor1 = Tensor {
		number: vec![3.14, 2.5, 5.5, 5.6],
		dimension: vec![2, 2],
	};
	let tensor2 = tensor1.clone();
	println!("{:?}", tensor1);
	println!("{:?}", tensor2);
}